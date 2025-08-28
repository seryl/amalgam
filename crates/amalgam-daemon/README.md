# amalgam-daemon

Runtime daemon for amalgam that watches for schema changes and regenerates configurations.

## Overview

`amalgam-daemon` provides continuous monitoring of schema sources and automatic regeneration of type definitions when changes are detected.

## Features

- **File Watching**: Monitor local directories for schema changes
- **Kubernetes Integration**: Watch CRD updates in live clusters
- **GitHub Integration**: Poll repositories for schema updates
- **Incremental Updates**: Only regenerate affected types
- **Cache Management**: Smart caching of parsed schemas

## Usage

```rust
use amalgam_daemon::{Daemon, WatchConfig};

// Configure the daemon
let config = WatchConfig::new()
    .watch_directory("./schemas")
    .watch_kubernetes(true)
    .output_directory("./generated");

// Start the daemon
let daemon = Daemon::new(config);
daemon.run().await?;
```

## CLI Usage

```bash
# Watch local directory
amalgam-daemon --watch ./schemas --output ./generated

# Watch Kubernetes cluster
amalgam-daemon --k8s --context production --output ./k8s-types

# Watch with specific interval
amalgam-daemon --watch ./schemas --interval 30s
```

