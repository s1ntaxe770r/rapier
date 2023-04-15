use serde::{Serialize,Deserialize};
use actix_web::{
    get,
    web::{Data}, 
    HttpResponse,

};
use crate::info::ClusterInfo;



#[derive(Debug,Serialize,Deserialize)]
pub struct  RegionResponse {
    pub region: String,
}

// create a handler that returns a response with shared state
#[get
("/region")]
pub async fn get_region(ci:Data<ClusterInfo>) -> HttpResponse {
    let region = ci.get_cluster_region().await;
    match region {
        Ok(region) => {
            let resp = RegionResponse {
                region: region,
            };
            HttpResponse::Ok().json(resp)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
