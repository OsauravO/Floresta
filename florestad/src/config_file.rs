use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct Wallet {
    pub xpubs: Option<Vec<String>>,
    pub descriptors: Option<Vec<String>>,
    pub addresses: Option<Vec<String>>,
}

#[derive(Default, Debug, Deserialize)]
pub struct ConfigFile {
    pub wallet: Wallet,
}

impl ConfigFile {
    pub fn from_file(filename: &str) -> Result<Self, crate::error::Error> {
        let file = std::fs::read_to_string(filename)?;
        Ok(toml::from_str(&file)?)
    }
}
