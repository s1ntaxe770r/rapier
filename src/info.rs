use k8s_openapi::api::core::v1::Node;
use k8s_openapi::api::core::v1::Pod;
use kube::api::ListParams;
use kube::Api;
use kube::Error;

// write rustdoc

/// ClusterInfo is a struct that contains information about the cluster, specifically the regions 
#[derive(Clone)]
pub struct ClusterInfo {
    // in cluster client
    pub in_cluster_client: kube::Client,
}

impl ClusterInfo {
    /// new creates a new ClusterInfo struct
    /// # Examples
    /// ```
    /// let cluster_info = ClusterInfo::new().await;
    /// ```
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
    /// get_cluster_region gets the region of the cluster
    /// # Examples
    /// ``` 
    /// let cluster_info = ClusterInfo::new().await;    
    /// let region = cluster_info.get_cluster_region().await;
    /// ```
    /// # Panics
    /// Panics if it cannot get the node list
    /// # Returns
    /// A string containing the region of the cluster
    /// # Errors
    /// Returns an error if it cannot get the node list
    pub async fn get_cluster_region(&self) -> Result<String,Error> {
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
                               Ok( region.to_string() )
                            }
                            None => {
                                Ok("unknown".to_string())
                            }
                        }
                    }
                    None => {
                        Ok("unknown".to_string())
                    }
                }
            }
            Err(e) => {
                // log error using log crate
                log::error!("Error getting node list: {}", e);
                Err(Error::from(e))
               
            }
        }
    }

    // pub async fn get_pod_name(&self) -> Result<String,Error> {
    //     let current_pod = Api::<Pod>::
    //     // match current_pod 


    // }
}


// 
