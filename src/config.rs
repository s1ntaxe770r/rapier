use std::env;


pub struct ApiConfig {
    pub server_port: u16,
}

impl ApiConfig {
    pub fn new() -> ApiConfig {
        // check for env var
        let server_port = match env::var("SERVER_PORT") {
            Ok(val) => {
                match val.parse::<u16>() {
                    Ok(port) => {
                        port
                    }
                    Err(e) => {
                       // if it is not a valid port, use default
                       println!("Error parsing SERVER_PORT: {} falling back", e);
                        8080
                    }
                }
            }
            Err(e) => {
                // if it is not set, use default
                println!("Error getting SERVER_PORT: {} falling back", e);
                8080
            }
        };
        ApiConfig {
            server_port,
        }
    }
    
}