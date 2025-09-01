//! Runtime daemon for watching and regenerating types

use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

pub struct Daemon {
    watch_paths: Vec<PathBuf>,
    output_dir: PathBuf,
}

impl Daemon {
    pub fn new(output_dir: PathBuf) -> Self {
        Self {
            watch_paths: Vec::new(),
            output_dir,
        }
    }

    pub fn add_watch_path(&mut self, path: PathBuf) {
        self.watch_paths.push(path);
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting amalgam daemon");
        info!("Watching paths: {:?}", self.watch_paths);
        info!("Output directory: {:?}", self.output_dir);

        // TODO: Implement file watching
        // TODO: Implement incremental compilation
        // TODO: Implement caching

        Ok(())
    }
}

#[cfg(feature = "kubernetes")]
pub mod k8s {
    use super::*;
    use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
    use kube::{Api, Client};

    pub struct K8sWatcher {
        client: Client,
    }

    impl K8sWatcher {
        pub async fn new() -> Result<Self> {
            let client = Client::try_default().await?;
            Ok(Self { client })
        }

        pub async fn watch_crds(&self) -> Result<()> {
            let _crds: Api<CustomResourceDefinition> = Api::all(self.client.clone());

            // TODO: Implement CRD watching
            // TODO: Generate types when CRDs change

            Ok(())
        }
    }
}
