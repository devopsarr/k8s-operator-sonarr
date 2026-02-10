# Sonarr Kubernetes Operator

A Kubernetes operator for [Sonarr](https://sonarr.tv/) written in Rust using [kube-rs](https://kube.rs/).

This operator allows you to manage Sonarr resources (tags, root folders, quality profiles, indexers, download clients, notifications, and series) declaratively through Kubernetes Custom Resources.

## Features

- **Declarative Configuration**: Define Sonarr resources as Kubernetes manifests
- **GitOps Ready**: Manage Sonarr configuration through version control
- **Multi-Instance Support**: Manage multiple Sonarr instances from a single operator
- **Automatic Synchronization**: Resources are continuously reconciled with Sonarr
- **Finalizers**: Clean up resources in Sonarr when Kubernetes resources are deleted

## Supported Resources

| CRD | Description |
|-----|-------------|
| `Sonarr` | Connection to a Sonarr instance |
| `SonarrTag` | Manage tags for organizing series |
| `SonarrRootFolder` | Configure root folders for media storage |
| `SonarrQualityProfile` | Define quality profiles |
| `SonarrIndexer` | Configure indexers for searching |
| `SonarrDownloadClient` | Set up download clients (SABnzbd, qBittorrent, etc.) |
| `SonarrNotification` | Configure notifications (Discord, Telegram, etc.) |
| `SonarrSeries` | Add and manage TV series |

## Quick Start

### Prerequisites

- Kubernetes cluster (1.25+)
- Rust toolchain (for building from source)
- A running Sonarr instance

### Installation

1. **Install CRDs**

```bash
make crds
kubectl apply -f deploy/crds/crds.yaml
```

2. **Deploy the Operator**

```bash
# Build and push Docker image (adjust image name as needed)
make docker
docker tag sonarr-operator:latest your-registry/sonarr-operator:latest
docker push your-registry/sonarr-operator:latest

# Deploy to cluster
kubectl apply -f deploy/
```

3. **Create a Sonarr Instance Reference**

First, create a secret with your Sonarr API key:

```bash
kubectl create secret generic sonarr-api-key \
  --from-literal=api-key=YOUR_SONARR_API_KEY
```

Then create the Sonarr instance reference:

```yaml
apiVersion: devopsarr.io/v1alpha1
kind: Sonarr
metadata:
  name: my-sonarr
spec:
  url: "http://sonarr.example.com:8989"
  apiKeySecretRef:
    name: sonarr-api-key
    key: api-key
```

4. **Create Resources**

```yaml
apiVersion: devopsarr.io/v1alpha1
kind: SonarrTag
metadata:
  name: anime
spec:
  sonarrInstanceRef:
    name: my-sonarr
  label: "anime"
---
apiVersion: devopsarr.io/v1alpha1
kind: SonarrRootFolder
metadata:
  name: tv-shows
spec:
  sonarrInstanceRef:
    name: my-sonarr
  path: "/media/tv"
```

## Usage Examples

### Managing Tags

```yaml
apiVersion: devopsarr.io/v1alpha1
kind: SonarrTag
metadata:
  name: documentary
spec:
  sonarrInstanceRef:
    name: my-sonarr
  label: "documentary"
```

### Adding a Series

```yaml
apiVersion: devopsarr.io/v1alpha1
kind: SonarrSeries
metadata:
  name: breaking-bad
spec:
  sonarrInstanceRef:
    name: my-sonarr
  tvdbId: 81189
  qualityProfileId: 1
  rootFolderPath: "/media/tv"
  monitored: true
  seasonFolder: true
  seriesType: Standard
  monitor: All
  searchOnAdd: true
  tags: []
```

### Configuring a Download Client

```yaml
apiVersion: devopsarr.io/v1alpha1
kind: SonarrDownloadClient
metadata:
  name: qbittorrent
spec:
  sonarrInstanceRef:
    name: my-sonarr
  name: "qBittorrent"
  enable: true
  priority: 1
  downloadClientType: QBittorrent
  fields:
    - name: host
      value: "qbittorrent.example.com"
    - name: port
      value: "8080"
    - name: username
      value: "admin"
    - name: password
      secretRef:
        name: qbittorrent-secret
        key: password
```

### Setting Up Notifications

```yaml
apiVersion: devopsarr.io/v1alpha1
kind: SonarrNotification
metadata:
  name: discord
spec:
  sonarrInstanceRef:
    name: my-sonarr
  name: "Discord"
  notificationType: Discord
  triggers:
    onGrab: true
    onDownload: true
    onUpgrade: true
    onRename: false
    onSeriesAdd: true
    onSeriesDelete: false
    onEpisodeFileDelete: false
    onEpisodeFileDeleteForUpgrade: false
    onHealthIssue: true
    onHealthRestored: true
    onApplicationUpdate: false
    onManualInteractionRequired: false
    includeHealthWarnings: false
  tags: []
```

## Building from Source

```bash
# Clone the repository
git clone https://github.com/your-org/kubernetes-operator-sonarr.git
cd kubernetes-operator-sonarr

# Build
make build

# Run locally (requires kubeconfig)
make run
```

## Development

```bash
# Check code
make check

# Run linters
make lint

# Run tests
make test

# Generate CRDs
make crds
```

## Testing

See [docs/TESTING.md](docs/TESTING.md) for comprehensive testing instructions, including:

- Unit testing
- Local development with Kind
- End-to-end testing with a real Sonarr instance
- CI/CD integration

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Kubernetes Cluster                        │
│  ┌─────────────────┐     ┌─────────────────┐               │
│  │  Sonarr CRDs    │     │  Sonarr         │               │
│  │  - Sonarr       │────▶│  Operator       │               │
│  │  - SonarrTag    │     │                 │               │
│  │  - SonarrSeries │     └────────┬────────┘               │
│  │  - ...          │              │                         │
│  └─────────────────┘              │ API Calls               │
└───────────────────────────────────┼─────────────────────────┘
                                    │
                                    ▼
                          ┌─────────────────┐
                          │  Sonarr Server  │
                          │  (External)     │
                          └─────────────────┘
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RUST_LOG` | Log level (trace, debug, info, warn, error) | `info` |

### Operator Flags

The operator currently does not require any command-line flags and uses the default kubeconfig or in-cluster configuration.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Sonarr](https://sonarr.tv/) - The PVR for Usenet and BitTorrent users
- [kube-rs](https://kube.rs/) - Rust client for Kubernetes
- [sonarr-rs](https://github.com/devopsarr/sonarr-rs) - Sonarr API client for Rust
