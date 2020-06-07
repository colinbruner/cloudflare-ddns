use config::{Config, ConfigError, File};
use serde::Deserialize;

//CONFIGURATION_FILE = os.path.expanduser('~/') + '.cloudflare-ddns'
//
//EXTERNAL_IP_QUERY_APIS = ['https://api.ipify.org', 'https://ifconfig.io/ip', 'https://ident.me/',
//                          'https://ifconfig.me/ip', 'https://icanhazip.com/']
//CLOUDFLARE_ZONE_QUERY_API = 'https://api.cloudflare.com/client/v4/zones'  # GET
//CLOUDFLARE_ZONE_DNS_RECORDS_QUERY_API = 'https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records'  # GET
//CLOUDFLARE_ZONE_DNS_RECORDS_UPDATE_API = 'https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records/{dns_record_id}'

#[derive(Debug, Deserialize)]
pub struct Settings {
    //pub key: String,
    //pub email: String,
    pub zone: String,
    pub zone_id: String,
    pub token: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(".cloudflare-ddns"))?;
        s.try_into()
    }
}
