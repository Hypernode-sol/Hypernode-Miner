use anyhow::{anyhow, Result};
use solana_sdk::{signature::{Keypair, read_keypair_file}, signer::Signer};

pub struct OperatorSigner {
    pub kp: Keypair,
}

impl OperatorSigner {
    pub fn from_env() -> Result<Self> {
        let path = std::env::var("OPERATOR_KEYPAIR").map_err(|_| anyhow!("OPERATOR_KEYPAIR not set"))?;
        let kp = read_keypair_file(path).map_err(|e| anyhow!("read_keypair_file: {e}"))?;
        Ok(Self { kp })
    }
}
