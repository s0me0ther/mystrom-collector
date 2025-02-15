use reqwest;
use serde_json::Value;
use tokio;
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <ip1> <ip2> ...", args[0]);
        return;
    }

    let ips = &args[1..];
    let results = fetch_all_ips(ips).await;

    match serde_json::to_string(&results) {
        Ok(json_str) => println!("{}", json_str),
        Err(e) => eprintln!("Failed to serialize results: {}", e),
    }
}

async fn fetch_all_ips(ips: &[String]) -> Vec<Value> {
    let client = reqwest::Client::new();
    let mut handles = Vec::new();

    for ip in ips {
        let url = format!("http://{}/report", ip);
        let client = client.clone();
        let ip = ip.clone();
        handles.push(tokio::spawn(async move {
            fetch_json(&client, &ip, &url).await
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(Ok(json)) => results.push(json),
            Ok(Err(e)) => eprintln!("Request failed: {}", e),
            Err(e) => eprintln!("Task failed: {}", e),
        }
    }

    results
}

async fn fetch_json(client: &reqwest::Client, ip: &str, url: &str) -> Result<Value, reqwest::Error> {
    let response = client.get(url).timeout(Duration::from_secs(5)).send().await?;
    let mut json: Value = response.json().await?;
    
    if let Some(obj) = json.as_object_mut() {
        obj.insert("ip".to_string(), Value::String(ip.to_string()));
    }
    
    Ok(json)
}