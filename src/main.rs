use reqwest;
use serde_json::Value;
use tokio;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <ip1> <ip2> ...", args[0]);
        return;
    }

    let ips = &args[1..];
    let futures: Vec<_> = ips.iter().map(|ip| {
        let url = format!("http://{}/report", ip);
        fetch_json(ip.clone(), url)
    }).collect();
    
    let results = futures::future::join_all(futures).await;
    let json_results: Vec<Value> = results.into_iter().filter_map(Result::ok).collect();
    
    println!("{}", serde_json::to_string_pretty(&json_results).unwrap());
}

async fn fetch_json(ip: String, url: String) -> Result<Value, reqwest::Error> {
    let response = reqwest::get(&url).await?;
    let mut json: Value = response.json().await?;
    
    if let Some(obj) = json.as_object_mut() {
        obj.insert("ip".to_string(), Value::String(ip));
    }
    
    Ok(json)
}
