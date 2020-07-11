use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::dns_records::DNSRecord;
use crate::settings::Settings;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

// A 'CloudFlareClient' is composed of a settings and client var.
pub struct CloudFlareClient {
    pub settings: Settings,
    client: Client,
}

#[async_trait]
pub trait CloudFlareAPI {
    fn new() -> Self;
    async fn get_record(&self) -> Result<DNSRecord>;
    async fn update_record(&self, dns_record: &DNSRecord) -> Result<DNSRecord>;
}

#[async_trait]
impl CloudFlareAPI for CloudFlareClient {
    fn new() -> CloudFlareClient {
        CloudFlareClient {
            settings: Settings::new().unwrap(),
            client: Client::new(),
        }
    }

    async fn get_record(&self) -> Result<DNSRecord> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
            &self.settings.zone_id
        );
        let auth = format!("Bearer {}", &self.settings.token);

        // Format and send request to CF
        let resp = &self
            .client
            .get(&url)
            .header("Authorization", auth)
            .send() // Send request
            .await?
            .text() // Convert text
            .await?;

        // Get DNS Record information
        let record: DNSRecord = serde_json::from_str(resp)?;

        // Return results
        Ok(record)
    }
    async fn update_record(&self, dns_record: &DNSRecord) -> Result<DNSRecord> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            &self.settings.zone_id, dns_record.records[0].id
        );

        let auth = format!("Bearer {}", &self.settings.token);
        let payload = json!({
            "type": "A",
            "name": &self.settings.zone,
            "content": dns_record.records[0].addr,
            "ttl": 1,
            "proxied": true
        });

        let resp = &self
            .client
            .put(&url)
            .header("Authorization", auth)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        let res: DNSRecord = serde_json::from_str(resp)?;
        Ok(res)
    }
}
