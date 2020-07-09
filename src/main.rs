// External libs
use ipify_async;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};

// Local libs
mod settings;
use settings::Settings;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Deserialize)]
struct DNSRecord {
    #[serde(alias = "result")]
    records: Vec<Records>,
}

#[derive(Deserialize)]
struct Records {
    #[serde(alias = "content")]
    addr: String,
    id: String,
}

async fn get_zone_dns_records(client: &Client, settings: &Settings) -> Result<DNSRecord> {
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

    // Get DNS Record information
    let record: DNSRecord = serde_json::from_str(&resp.text().await?)?;

    // Get 'addr' and 'id' from record
    //let addr: &str = record.result[0].content.as_str();
    //let id: &str = record.result[0].id.as_str();

    // Return results
    Ok(record)
}

async fn update_zone_a_record_ip(
    client: &Client,
    dns_record: &DNSRecord,
    settings: &Settings,
) -> Result<String> {
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        settings.zone_id, dns_record.records[0].id
    );
    let auth = format!("Bearer {}", settings.token);
    let payload = json!({"type": "A", "name":settings.zone, "content": dns_record.records[0].addr, "ttl":1, "proxied":true});

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

    let current_ip = ipify_async::get_ip().await.unwrap().to_string();
    let dns_record: DNSRecord = get_zone_dns_records(&client, &settings).await?;

    if current_ip == dns_record.records[0].addr {
        println!(
            "IP: {} == A Record {}. Doing nothing.",
            current_ip, dns_record.records[0].addr
        );
    } else {
        println!(
            "Updating A Record for Zone: {} to IP: {}.",
            settings.zone, dns_record.records[0].addr
        );
        let status = update_zone_a_record_ip(&client, &dns_record, &settings).await?;
        println! {"Success: {:?}", status};
    }
    Ok(())
}
