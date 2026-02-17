use mimic_ai::registry::Registry;
use mimic_ai::server::MimicServer;
use rmcp::{service::RunningService, RoleClient, ServiceExt};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub async fn spawn_server(project_dir: Option<PathBuf>) -> RunningService<RoleClient, ()> {
    let registry = Arc::new(RwLock::new(Registry::new(project_dir)));
    let handler = MimicServer::new(registry, None);

    let (client_stream, server_stream) = tokio::io::duplex(4096);

    // Spawn the server in the background
    tokio::spawn(async move {
        let service = rmcp::serve_server(handler, server_stream).await.unwrap();
        service.waiting().await.unwrap();
    });

    // Connect a client
    ().serve(client_stream).await.unwrap()
}
