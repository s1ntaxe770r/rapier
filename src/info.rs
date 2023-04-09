use k8s_openapi::api::core::v1::Node;
use kube::api::ListParams;
use kube::Api;
pub struct ClusterInfo {
    // in cluster client
    pub in_cluster_client: kube::Client,
}

impl ClusterInfo {
    pub async fn new() -> ClusterInfo {
        let in_cluster_client = kube::Client::try_default().await;
        match in_cluster_client {
            Ok(client) => {
                ClusterInfo {
                    in_cluster_client: client,
                }
            }
            Err(e) => {
                panic!("Error creating in cluster client: {}", e)
            }
        }
    }
    pub async fn get_cluster_region(&self) -> String {
        let nodes = Api::<Node>::all(self.in_cluster_client.clone());
        let list_params = ListParams::default();
        let node_list = nodes.list(&list_params).await;
        // get node region 
        match node_list {
            Ok(node_list) => {
                let node = node_list.items.get(0).unwrap();
                match &node.metadata.labels {
                    Some(labels) => {
                        match labels.get("failure-domain.beta.kubernetes.io/region") {
                            Some(region) => {
                                region.to_string()
                            }
                            None => {
                                "unknown".to_string()
                            }
                        }
                    }
                    None => {
                        "unknown".to_string()
                    }
                }
            }
            Err(e) => {
                panic!("Error getting node list: {}", e)
            }
        }
    }
}

// 
