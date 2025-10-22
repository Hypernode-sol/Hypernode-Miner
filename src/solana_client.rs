use crate::{config, telemetry::MetricsReport, signer::OperatorSigner};
use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{Instruction},
    pubkey::Pubkey,
    transaction::Transaction,
    system_program,
    signer::Signer,
    message::Message,
};

pub async fn submit_metrics(signer: &OperatorSigner, m: MetricsReport) -> Result<()> {
    let client = RpcClient::new(config::rpc_url()?);
    let program_id = Pubkey::from_str(&config::metrics_program_id()?)
        .map_err(|_| anyhow!("Invalid PROGRAM_METRICS_REGISTRY_ID"))?;

    // Instruction placeholder — substitua pelo IDL real (Anchor) do metrics_registry
    let ix = Instruction {
        program_id,
        accounts: vec![], // preencha com as contas exigidas
        data: bincode::serialize(&m).map_err(|e| anyhow!("serialize: {e}"))?,
    };

    let blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&signer.kp.pubkey()),
        &[&signer.kp],
        blockhash,
    );

    client.send_and_confirm_transaction(&tx)?;
    Ok(())
}
