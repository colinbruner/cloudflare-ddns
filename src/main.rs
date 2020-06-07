extern crate config;
extern crate serde;
extern crate serde_json;

use serde_json::{json, Value};

use reqwest::blocking::Client;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
//use reqwest::header::{ContentType, Headers, UserAgent};
mod settings;
use settings::Settings;

fn get_ip(client: &Client) -> Result<String> {
    let ip: Vec<u8> = client
        .get("https://api.ipify.org")
        .send()?
        .text()?
        .split(".")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    Ok(format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]))
}

fn get_zone_dns_records(client: &Client, settings: &Settings) -> Result<(String, String)> {
    // Returns the "content" of the A record
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
        settings.zone_id
    );
    let auth = format!("Bearer {}", settings.token);
    let resp = client.get(&url).header("Authorization", auth).send()?;
    let v: Value = serde_json::from_str(&resp.text().unwrap())?;
    let ip: &str = v["result"][0]["content"].as_str().unwrap();
    let id: &str = v["result"][0]["id"].as_str().unwrap();
    Ok((String::from(ip), String::from(id)))
}

fn update_zone_a_record_ip(
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
        .send()?;

    let v: Value = serde_json::from_str(&resp.text().unwrap())?;
    Ok(v["success"].to_string())
}

fn main() -> Result<()> {
    let client = Client::new();
    let settings = Settings::new().unwrap();

    let current_ip = get_ip(&client).unwrap();
    let (zone_ip, id) = get_zone_dns_records(&client, &settings).unwrap();

    if current_ip != zone_ip {
        println!("IP: {} == A Record {}. Doing nothing.", current_ip, zone_ip);
    } else {
        println!(
            "Updating A Record for Zone: {} to IP: {}.",
            settings.zone, current_ip
        );
        let status = update_zone_a_record_ip(&client, &id, &current_ip, &settings).unwrap();
        println! {"Success: {:?}", status};
    }
    Ok(())
}
