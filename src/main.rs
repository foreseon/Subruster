use subruster::session_manager;

#[tokio::main]
async fn main() {
    session_manager::start_session_operations().await;
    
}