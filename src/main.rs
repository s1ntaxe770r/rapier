use rapier::info::ClusterInfo;



#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cluster_info = ClusterInfo::new().await;
    println!("Cluster region: {}", cluster_info.get_cluster_region().await);
    Ok(())
}
