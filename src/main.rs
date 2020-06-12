// External libs
use reqwest::Client;
use serde_json::{json, Value};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

// Local libs
mod settings;
use settings::Settings;

async fn get_ip(client: &Client) -> Result<String> {
    let ip: Vec<u8> = client
        .get("https://api.ipify.org")
        .send()
        .await?
        .text()
        .await?
        .split(".")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    Ok(format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]))
}

async fn get_zone_dns_records(client: &Client, settings: &Settings) -> Result<(String, String)> {
    // Returns the "content" of the A record
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
        settings.zone_id
    );
    let auth = format!("Bearer {}", settings.token);
    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;
    let v: Value = serde_json::from_str(&resp.text().await?)?;
    let ip: &str = v["result"][0]["content"].as_str().unwrap();
    let id: &str = v["result"][0]["id"].as_str().unwrap();
    Ok((String::from(ip), String::from(id)))
}

async fn update_zone_a_record_ip(
    client: &Client,
    id: &str,
    current_ip: &str,
    settings: &Settings,
) -> Result<String> {
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        settings.zone_id, id
    );
    let auth = format!("Bearer {}", settings.token);
    let payload =
        json!({"type": "A", "name":settings.zone, "content": current_ip, "ttl":1, "proxied":true});

    let resp = client
        .put(&url)
        .header("Authorization", auth)
        .json(&payload)
        .send()
        .await?;

    let v: Value = serde_json::from_str(&resp.text().await?)?;
    Ok(v["success"].to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let settings = Settings::new().unwrap();

    let current_ip = get_ip(&client).await?;
    let (zone_ip, id) = get_zone_dns_records(&client, &settings).await?;

    if current_ip != zone_ip {
        println!("IP: {} == A Record {}. Doing nothing.", current_ip, zone_ip);
    } else {
        println!(
            "Updating A Record for Zone: {} to IP: {}.",
            settings.zone, current_ip
        );
        let status = update_zone_a_record_ip(&client, &id, &current_ip, &settings).await?;
        println! {"Success: {:?}", status};
    }
    Ok(())
}
