use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

#[derive(Clone, Default, Eq, PartialEq)]
pub struct Config {
    pub rpc_urls: HashMap<String, String>,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let rpc_url_dev = env::var("BRYTESCHEMES_API_RPC_URL_DEV")
            .unwrap_or("https://api.devnet.solana.com".to_string());
        let rpc_url_test = env::var("BRYTESCHEMES_API_RPC_URL_TEST")
            .unwrap_or("https://api.testnet.solana.com".to_string());
        let rpc_url_main = env::var("BRYTESCHEMES_API_RPC_URL_DEV")
            .unwrap_or("https://api.mainnet.solana.com".to_string());
        let rpc_url_local = env::var("BRYTESCHEMES_API_RPC_URL_LOCAL")
            .unwrap_or("http://localhost:8000".to_string());

        let mut rpc_urls = HashMap::new();
        rpc_urls.insert("Dev".to_string(), rpc_url_dev);
        rpc_urls.insert("Test".to_string(), rpc_url_test);
        rpc_urls.insert("Main".to_string(), rpc_url_main);
        rpc_urls.insert("Local".to_string(), rpc_url_local);

        Ok(Config { rpc_urls })
    }

    pub fn rpc_url(&self, input: &str) -> Result<&str> {
        match input {
            "Dev" => Ok(self.rpc_urls.get("Dev").unwrap()),
            "Test" => Ok(self.rpc_urls.get("Test").unwrap()),
            "Main" => Ok(self.rpc_urls.get("Main").unwrap()),
            "Local" => Ok(self.rpc_urls.get("Local").unwrap()),
            _ => Ok(self.rpc_urls.get("Local").unwrap()),
        }
    }
}
