// External libs
use ipify_async;
use reqwest::Client;

// Local libs
mod cloudflare;
mod dns_records;
mod settings;
use cloudflare::{get_record, update_zone, Result};
use dns_records::DNSRecord;
use settings::Settings;

#[tokio::main]
async fn main() -> Result<()> {
    // Spawn a new async client
    let client = Client::new();
    // Attempt to configure settings from file.
    let settings = Settings::new().unwrap();

    // Grab the systems current public IP address through ipify API
    let current_ip = ipify_async::get_ip().await.unwrap().to_string();
    // Fetch Cloudflare DNS records based off setting values
    let dns_record: DNSRecord = get_record(&client, &settings).await?;

    // Main logic point - either update if current IP is not equal to
    // cloudflare's records, or print out IPs and do nothing.
    if current_ip != dns_record.records[0].addr {
        println!(
            "Updating A Record for Zone: {} to IP: {}.",
            settings.zone, dns_record.records[0].addr
        );
        update_zone(&client, &dns_record, &settings).await?;
    } else {
        println!(
            "IP: {} == A Record {}. Doing nothing.",
            current_ip, dns_record.records[0].addr
        );
    }

    Ok(())
}
