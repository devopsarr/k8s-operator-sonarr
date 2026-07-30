use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

fn now() -> k8s_openapi::apimachinery::pkg::apis::meta::v1::Time {
    k8s_openapi::apimachinery::pkg::apis::meta::v1::Time(k8s_openapi::jiff::Timestamp::now())
}

/// Create a Ready condition with the given status and message
pub fn ready_condition(status: bool, reason: &str, message: &str) -> Condition {
    Condition {
        type_: "Ready".to_string(),
        status: if status { "True" } else { "False" }.to_string(),
        reason: reason.to_string(),
        message: message.to_string(),
        last_transition_time: now(),
        observed_generation: None,
    }
}

/// Create a Progressing condition
pub fn progressing_condition(status: bool, reason: &str, message: &str) -> Condition {
    Condition {
        type_: "Progressing".to_string(),
        status: if status { "True" } else { "False" }.to_string(),
        reason: reason.to_string(),
        message: message.to_string(),
        last_transition_time: now(),
        observed_generation: None,
    }
}

/// Update conditions list, replacing existing conditions of the same type
pub fn update_conditions(conditions: &mut Vec<Condition>, new_condition: Condition) {
    if let Some(existing) = conditions
        .iter_mut()
        .find(|c| c.type_ == new_condition.type_)
    {
        if existing.status != new_condition.status || existing.reason != new_condition.reason {
            *existing = new_condition;
        }
    } else {
        conditions.push(new_condition);
    }
}
