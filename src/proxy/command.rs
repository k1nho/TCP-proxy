use crate::proxy::{config::Config, load_balancer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::{
    io,
    net::{TcpListener, TcpStream},
    task::JoinHandle,
};

/// **Proxy**: contains a app_name String, TCPlistener (the connection to route) and **targets** a vector of String of
/// possible targets for the connection.
pub struct Proxy {
    pub app_name: String,
    pub listener: TcpListener,
    pub targets: Vec<String>,
}

/// Generates a map that associates port to App
async fn proxy_map(app_config: &Config) -> HashMap<u16, Vec<String>> {
    let mut port_to_targets = HashMap::new();
    for app in &app_config.apps {
        for port in &app.ports {
            port_to_targets.insert(*port, app.targets.clone());
        }
    }
    port_to_targets
}

/// Generates a map that associates an app's name to a mutex index
async fn port_to_index(app_config: &Config) -> HashMap<String, usize> {
    let mut port_to_idx = HashMap::new();
    for (i, app) in app_config.apps.iter().enumerate() {
        port_to_idx.insert(app.name.clone(), i);
    }

    port_to_idx
}

/// Generate a proxy vector
///
/// # Returns
/// **Vec<Proxy>**: a vector of Proxy structs
async fn generate_proxies(app_config: &Config) -> Vec<Proxy> {
    let mut proxies = Vec::new();
    let mut proxy_to_app = proxy_map(app_config).await;

    for app in &app_config.apps {
        for port in &app.ports {
            let client_addr = format!("0.0.0.0:{}", port);
            let tcp_listener = TcpListener::bind(&client_addr).await;
            let proxy_targets = proxy_to_app.remove(port).unwrap();

            match tcp_listener {
                Ok(tcp_listener) => {
                    proxies.push(Proxy {
                        app_name: app.name.clone(),
                        listener: tcp_listener,
                        targets: proxy_targets,
                    });
                }
                Err(_) => {
                    println!(
                        "could not establish tcp connection. Port {} already in use",
                        port
                    )
                }
            }
        }
    }

    proxies
}

/// Generate mutexes to be used as shared state for the load balancer across the different
/// connections on the app
async fn generate_mutexes(num: usize) -> Vec<Arc<Mutex<usize>>> {
    if num == 0 {
        panic!("Cannot generate share state for 0 apps.");
    }

    let mut mutexes = vec![];
    for _ in 0..num {
        mutexes.push(Arc::new(Mutex::new(0)));
    }
    mutexes
}

/// Runs the TCP proxy service given a Config struct
pub async fn run(app_config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let proxies = generate_proxies(&app_config).await;
    // setup shared counter state for apps
    let number_of_apps = app_config.apps.len();
    let balancer_counters = generate_mutexes(number_of_apps).await;
    let port_to_mutex = port_to_index(&app_config).await;

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for proxy in proxies {
        // acquire mutex for the correct app group
        let idx = port_to_mutex.get(&proxy.app_name).unwrap();
        let shared_counter = Arc::clone(&balancer_counters[*idx]);

        let handle = tokio::spawn(handle_conn(proxy, shared_counter));
        handles.push(handle);
    }

    futures::future::join_all(handles).await;
    Ok(())
}

/// Handle an incomming connection
async fn handle_conn(proxy: Proxy, target_id: Arc<Mutex<usize>>) {
    while let Ok((socket, _)) = proxy.listener.accept().await {
        // Retry until we are able to connect to a target server
        loop {
            // invoke load balancer to select an adequate target
            let target_server = load_balancer::round_robin(&proxy.targets, &target_id);
            match try_proxy(&target_server).await {
                Ok(stream) => {
                    tokio::spawn(async move {
                        match transfer_data(socket, stream, target_server).await {
                            Ok(res) => println!("{}", res),
                            Err(_) => {
                                println!("Could not proxy the connection. All servers occupied");
                            }
                        }
                    });
                    break;
                }
                Err(_) => continue,
            }
        }
    }
}

/// Attempts to proxy a connection
async fn try_proxy(target: &String) -> Result<TcpStream, Box<dyn std::error::Error>> {
    return match TcpStream::connect(target).await {
        Ok(stream) => Ok(stream),
        Err(e) => Err(Box::new(e)),
    };
}

/// Data transfer
async fn transfer_data(
    mut socket: TcpStream,
    mut server: TcpStream,
    target: String,
) -> Result<String, Box<dyn std::error::Error>> {
    match io::copy_bidirectional(&mut socket, &mut server).await {
        Ok((_, _)) => {}
        Err(e) => {
            return Err(Box::new(e));
        }
    }

    let client_port = socket.local_addr().unwrap().port();
    Ok(format!("Proxying :{} to {}", client_port, target))
}
