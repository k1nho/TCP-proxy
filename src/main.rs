use tcp_proxy::proxy::{command, config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the configuration for the proxy from the config.json file
    let app_config: config::Config = config::config(String::from("./config.json"));
    // println!("Config: {:?}", app_config);

    match command::run(app_config).await {
        Ok(()) => (),
        Err(e) => panic!("shutdown {}", e),
    };
    Ok(())
}
