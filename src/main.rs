// External libs
use ipify_async;

// Local libs
mod cloudflare;
mod dns_records;
mod settings;
use cloudflare::{CloudFlareAPI, CloudFlareClient, Result};
use dns_records::DNSRecord;

#[tokio::main]
async fn main() -> Result<()> {
    let client: CloudFlareClient = CloudFlareAPI::new();

    // Grab the systems current public IP address through ipify API
    let current_ip = ipify_async::get_ip().await.unwrap().to_string();

    // Fetch Cloudflare DNS records based off setting values
    let dns_record: DNSRecord = client.get_record().await?;

    // Main logic point - either update if current IP is not equal to
    // cloudflare's records, or print out IPs and do nothing.
    if current_ip != dns_record.records[0].addr {
        println!(
            "Updating A Record for Zone: {} to IP: {}.",
            client.settings.zone, dns_record.records[0].addr
        );
        client.update_record(&dns_record).await?;
    } else {
        println!(
            "IP: {} == A Record {}. Doing nothing.",
            current_ip, dns_record.records[0].addr
        );
    }

    Ok(())
}
