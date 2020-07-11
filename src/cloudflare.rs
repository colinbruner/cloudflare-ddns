use reqwest::Client;
use serde_json::json;

use crate::dns_records::DNSRecord;
use crate::settings::Settings;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub async fn get_record(client: &Client, settings: &Settings) -> Result<DNSRecord> {
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

    // Return results
    Ok(record)
}
pub async fn update_zone(
    client: &Client,
    dns_record: &DNSRecord,
    settings: &Settings,
) -> Result<()> {
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        settings.zone_id, dns_record.records[0].id
    );

    let auth = format!("Bearer {}", settings.token);
    let payload = json!({
        "type": "A",
        "name": settings.zone,
        "content": dns_record.records[0].addr,
        "ttl": 1,
        "proxied": true
    });

    let resp = client
        .put(&url)
        .header("Authorization", auth)
        .json(&payload)
        .send()
        .await?;

    let res: DNSRecord = serde_json::from_str(&resp.text().await?)?;
    println! {"Success: {:?}", res.success};
    Ok(())
}
