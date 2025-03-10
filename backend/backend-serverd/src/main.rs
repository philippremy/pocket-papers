use std::{error::Error, process::Command};

fn start_reverse_proxy() {
    // Launch Reverse Proxy
    Command::new(std::env::current_exe().unwrap().parent().unwrap().join("rpxy").to_str().unwrap())
    .args([
        "--watch",
        "--config",
        std::env::current_exe().unwrap().parent().unwrap().join("reverse_proxy.toml").to_str().unwrap()
    ])
    .output()
    .map(|output| {
        println!("Reverse proxy process exited early: {output:?}!");
        start_reverse_proxy()
    }).unwrap();
}

fn start_backend_server() {
    // Launch Backend Server
    Command::new(std::env::current_exe().unwrap().parent().unwrap().join("backend-server").to_str().unwrap())
    .output()
    .map(|output| {
        println!("Backend Server process exited early: {output:?}!");
        start_backend_server()
    }).unwrap();
}

async fn get_current_ip() -> Result<String, Box<dyn Error>> {
    Ok(reqwest::blocking::get("https://api.ipify.org")?.text()?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let f1 = tokio::spawn(async {
        loop {
            if let Ok(ip) = get_current_ip().await {
                let req = format!("https://api.1984.is/1.0/freedns/?apikey={}&domain={}&ip={}", "yV6bA1uV9eG1gJ6iP8vH6pO7qN9uI4zW0nT6uW6kA9rZ3uC2yE2vX2fS2iT5sA2", "api.dtb-kampfrichter.de", ip);
                Command::new("curl")
                    .arg(req)
                    .status()
                    .unwrap();
            } else {
                eprintln!("IPv4 Address could not be fetched. The server might be offline!");
            }
            tokio::time::sleep(std::time::Duration::from_secs(300)).await;
        }
    });

    let f2 = tokio::spawn(async {
        start_reverse_proxy()
    });

    let f3 = tokio::spawn(async {
        start_backend_server()
    });

    f1.await?;
    f2.await?;
    f3.await?;

    Ok(())
}
