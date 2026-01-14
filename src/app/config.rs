use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub http_addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            http_addr: "127.0.0.1:3000".parse()?,
        })
    }
}
