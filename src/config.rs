use anyhow::{anyhow, Result};

pub fn rpc_url() -> Result<String> {
    std::env::var("SOLANA_RPC_URL").map_err(|_| anyhow!("SOLANA_RPC_URL not set"))
}

pub fn metrics_program_id() -> Result<String> {
    std::env::var("PROGRAM_METRICS_REGISTRY_ID").map_err(|_| anyhow!("PROGRAM_METRICS_REGISTRY_ID not set"))
}

pub fn operator_keypair_path() -> Result<String> {
    std::env::var("OPERATOR_KEYPAIR").map_err(|_| anyhow!("OPERATOR_KEYPAIR not set"))
}
