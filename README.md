# Hypernode Miner

O Hypernode Miner é o nó do operador. Ele:
- coleta métricas locais (uptime, latência, throughput),
- assina os relatórios,
- envia as métricas on-chain para o programa `metrics_registry` na Solana.

## Requisitos
- Rust 1.77+ (stable)
- Solana CLI 1.18+
- Anchor 0.30+
- RPC de Solana (mainnet ou devnet)
- Chave do operador (keypair)

## Configuração
Copie `.env.example` para `.env` e ajuste:
- `SOLANA_RPC_URL`
- `OPERATOR_KEYPAIR`
- `PROGRAM_METRICS_REGISTRY_ID`

## Execução
```bash
cargo run --release
