use serde::Deserialize;

#[derive(Deserialize)]
pub struct DNSRecord {
    #[serde(alias = "result")]
    pub records: Vec<Records>,
    pub success: String,
}

#[derive(Deserialize)]
pub struct Records {
    #[serde(alias = "content")]
    pub addr: String,
    pub id: String,
}
