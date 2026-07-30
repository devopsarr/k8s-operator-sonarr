use std::sync::Arc;

use kube::api::{Api, Patch, PatchParams};
use kube::runtime::controller::Action;
use kube::{Client, ResourceExt};
use tracing::info;

use sonarr::apis::notification_api;
use sonarr::models::NotificationResource;

use crate::Context;
use crate::crds::notification::NotificationType;
use crate::crds::{SonarrNotification, SonarrNotificationStatus};
use crate::error::{Error, Result};

use super::tag::get_sonarr_config;
use super::traits::{REQUEUE_DURATION, reconcile_with_finalizer, run_controller};
use super::{ready_condition, update_conditions};

/// Start the SonarrNotification controller
pub async fn run(client: Client, context: Arc<Context>) {
    run_controller::<SonarrNotification, _, _>(client, context, "SonarrNotification", reconcile)
        .await;
}

async fn reconcile(obj: Arc<SonarrNotification>, ctx: Arc<Context>) -> Result<Action> {
    reconcile_with_finalizer(obj, ctx, reconcile_apply, reconcile_cleanup).await
}

async fn reconcile_apply(
    notification: Arc<SonarrNotification>,
    ctx: Arc<Context>,
) -> Result<Action> {
    let client = &ctx.client;
    let namespace = notification
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;
    let name = notification.name_any();

    info!("Reconciling SonarrNotification: {}/{}", namespace, name);

    let config = get_sonarr_config(
        &ctx,
        client,
        &namespace,
        &notification.spec.sonarr_instance_ref,
    )
    .await?;

    // Build notification resource
    let mut n_resource = NotificationResource::new();
    n_resource.name = Some(Some(notification.spec.name.clone()));
    n_resource.implementation = Some(Some(
        get_implementation_name(&notification.spec.notification_type).to_string(),
    ));
    n_resource.on_grab = Some(notification.spec.triggers.on_grab);
    n_resource.on_download = Some(notification.spec.triggers.on_download);
    n_resource.on_upgrade = Some(notification.spec.triggers.on_upgrade);
    n_resource.on_rename = Some(notification.spec.triggers.on_rename);
    n_resource.on_series_add = Some(notification.spec.triggers.on_series_add);
    n_resource.on_series_delete = Some(notification.spec.triggers.on_series_delete);
    n_resource.on_episode_file_delete = Some(notification.spec.triggers.on_episode_file_delete);
    n_resource.on_episode_file_delete_for_upgrade = Some(
        notification
            .spec
            .triggers
            .on_episode_file_delete_for_upgrade,
    );
    n_resource.on_health_issue = Some(notification.spec.triggers.on_health_issue);
    n_resource.on_health_restored = Some(notification.spec.triggers.on_health_restored);
    n_resource.on_application_update = Some(notification.spec.triggers.on_application_update);
    n_resource.on_manual_interaction_required =
        Some(notification.spec.triggers.on_manual_interaction_required);
    n_resource.include_health_warnings = Some(notification.spec.triggers.include_health_warnings);
    n_resource.tags = Some(Some(notification.spec.tags.clone()));

    let sonarr_notification = if let Some(id) = notification.status.as_ref().and_then(|s| s.id) {
        n_resource.id = Some(id);
        match notification_api::update_notification(
            &config,
            id,
            Some(false),
            Some(n_resource.clone()),
        )
        .await
        {
            Ok(n) => n,
            Err(_) => {
                n_resource.id = None;
                notification_api::create_notification(&config, Some(false), Some(n_resource))
                    .await?
            }
        }
    } else {
        let existing = notification_api::list_notification(&config).await?;
        if let Some(existing_n) = existing
            .iter()
            .find(|n| n.name.as_ref().and_then(|nm| nm.as_ref()) == Some(&notification.spec.name))
        {
            existing_n.clone()
        } else {
            notification_api::create_notification(&config, Some(false), Some(n_resource)).await?
        }
    };

    // Update status
    let notifications_api: Api<SonarrNotification> = Api::namespaced(client.clone(), &namespace);
    let mut conditions = notification
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    update_conditions(
        &mut conditions,
        ready_condition(true, "Synced", "Notification synchronized with Sonarr"),
    );

    let status = SonarrNotificationStatus {
        conditions,
        id: sonarr_notification.id,
        observed_generation: notification.metadata.generation.unwrap_or(0),
    };

    let status_patch = serde_json::json!({ "status": status });
    notifications_api
        .patch_status(&name, &PatchParams::default(), &Patch::Merge(&status_patch))
        .await?;

    Ok(Action::requeue(REQUEUE_DURATION))
}

fn get_implementation_name(notification_type: &NotificationType) -> &'static str {
    match notification_type {
        NotificationType::Apprise => "Apprise",
        NotificationType::CustomScript => "CustomScript",
        NotificationType::Discord => "Discord",
        NotificationType::Email => "Email",
        NotificationType::Emby => "MediaBrowser",
        NotificationType::Gotify => "Gotify",
        NotificationType::Join => "Join",
        NotificationType::Kodi => "Xbmc",
        NotificationType::Mailgun => "Mailgun",
        NotificationType::Ntfy => "Ntfy",
        NotificationType::Plex => "PlexServer",
        NotificationType::Prowl => "Prowl",
        NotificationType::Pushbullet => "Pushbullet",
        NotificationType::Pushover => "Pushover",
        NotificationType::SendGrid => "SendGrid",
        NotificationType::Signal => "Signal",
        NotificationType::Simplepush => "Simplepush",
        NotificationType::Slack => "Slack",
        NotificationType::SynologyIndexer => "SynologyIndexer",
        NotificationType::Telegram => "Telegram",
        NotificationType::Trakt => "Trakt",
        NotificationType::Twitter => "Twitter",
        NotificationType::Webhook => "Webhook",
    }
}

async fn reconcile_cleanup(
    notification: Arc<SonarrNotification>,
    ctx: Arc<Context>,
) -> Result<Action> {
    let client = &ctx.client;
    let namespace = notification
        .namespace()
        .ok_or(Error::MissingObjectKey(".metadata.namespace"))?;

    info!(
        "Cleaning up SonarrNotification: {}/{}",
        namespace,
        notification.name_any()
    );

    if let Some(id) = notification.status.as_ref().and_then(|s| s.id)
        && let Ok(config) = get_sonarr_config(
            &ctx,
            client,
            &namespace,
            &notification.spec.sonarr_instance_ref,
        )
        .await
    {
        let _ = notification_api::delete_notification(&config, id).await;
    }

    Ok(Action::await_change())
}
