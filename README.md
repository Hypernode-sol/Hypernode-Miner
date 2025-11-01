# Hypernode Miner

**High-performance cryptocurrency miner for Hypernode network**

The Hypernode Miner is the operator node that:
- Collects local metrics (uptime, latency, throughput)
- Signs performance reports
- Submits metrics on-chain to the `metrics_registry` program on Solana
- Performs mining operations using CPU/GPU

![C](https://img.shields.io/badge/Language-C-blue)
![OpenCL](https://img.shields.io/badge/OpenCL-Supported-green)
![Solana](https://img.shields.io/badge/Blockchain-Solana-blueviolet)

---

## 🎯 Overview

This miner integrates with the Hypernode ecosystem to:
- Validate and record node performance on-chain
- Participate in consensus mechanisms
- Earn rewards based on computational contribution
- Support the decentralized GPU compute network

---

## 💻 Requirements

### System Requirements
- **OS**: Linux (Ubuntu 20.04+), macOS, or Windows (with WSL2)
- **CPU**: Multi-core processor (4+ cores recommended)
- **RAM**: 4 GB minimum (8 GB recommended)
- **GPU**: Optional but recommended (NVIDIA with OpenCL support)

### Software Requirements
- **C Compiler**: GCC 9+ or Clang 10+
- **CMake**: 3.15+
- **OpenCL**: For GPU acceleration (optional)
- **Rust**: 1.77+ (stable)
- **Solana CLI**: 1.18+
- **Anchor**: 0.30+

### Blockchain Requirements
- Solana RPC endpoint (mainnet or devnet)
- Operator keypair (Solana wallet)
- Access to `metrics_registry` program

---

## 🚀 Quick Start

### 1. Install Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y build-essential cmake git
sudo apt install -y ocl-icd-opencl-dev  # For GPU support
```

**macOS:**
```bash
brew install cmake
# OpenCL is included in macOS
```

### 2. Clone and Build

```bash
git clone https://github.com/Hypernode-sol/Hypernode-Miner.git
cd Hypernode-Miner

# Build with CMake
mkdir build && cd build
cmake ..
make

# Or use provided script
chmod +x release.sh
./release.sh
```

### 3. Configure

Copy the example configuration and adjust settings:

```bash
cp .env.example .env
```

Edit `.env` with your configuration:
```bash
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
OPERATOR_KEYPAIR=/path/to/your/keypair.json
PROGRAM_METRICS_REGISTRY_ID=YourProgramIdHere
```

### 4. Run the Miner

```bash
# CPU mining
./hypernode_miner

# GPU mining (if OpenCL available)
./hypernode_miner --gpu

# With custom configuration
./hypernode_miner --config /path/to/config
```

---

## 🔧 Building

### Using CMake (Recommended)

```bash
mkdir build
cd build
cmake ..
make
```

### Using Provided Scripts

**Linux/macOS:**
```bash
chmod +x release.sh
./release.sh
```

**macOS (with specific optimizations):**
```bash
chmod +x cmake_mac.sh
./cmake_mac.sh
```

**Windows:**
```batch
compile_dll.bat
```

---

## ⚙️ Configuration Options

### Environment Variables

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `SOLANA_RPC_URL` | Solana RPC endpoint | Yes | - |
| `OPERATOR_KEYPAIR` | Path to operator keypair | Yes | - |
| `PROGRAM_METRICS_REGISTRY_ID` | Metrics registry program ID | Yes | - |
| `MINING_THREADS` | Number of CPU threads | No | Auto-detect |
| `GPU_DEVICE` | GPU device index | No | 0 |
| `LOG_LEVEL` | Logging verbosity | No | info |

### Command Line Options

```bash
# Show help
./hypernode_miner --help

# GPU mining
./hypernode_miner --gpu

# Specify threads
./hypernode_miner --threads 8

# Specify GPU device
./hypernode_miner --gpu-device 0

# Custom config file
./hypernode_miner --config config.json

# Verbose logging
./hypernode_miner --verbose
```

---

## 🏗️ Project Structure

```
Hypernode-Miner/
├── hypernode_miner.c       # Main miner implementation
├── hypernode_compiler.c    # EPL compiler
├── miner.h                 # Miner header definitions
├── elist.h                 # ElasticPL data structures
├── ocl.c                   # OpenCL GPU support
├── util.c                  # Utility functions
├── crypto/                 # Cryptographic functions
├── docker/                 # Docker configuration
├── HYPERNODEPL/            # ElasticPL language files
├── src/                    # Additional source files
├── CMakeLists.txt          # CMake build configuration
└── README.md               # This file
```

---

## 🎮 GPU Mining

### Prerequisites

Install OpenCL:

**Ubuntu/Debian:**
```bash
sudo apt install -y ocl-icd-opencl-dev
```

**NVIDIA GPU:**
```bash
sudo apt install -y nvidia-opencl-dev
```

**AMD GPU:**
```bash
sudo apt install -y amd-opencl-dev
```

### Running with GPU

```bash
# Single GPU
./hypernode_miner --gpu

# Specific GPU device
./hypernode_miner --gpu-device 1

# Multiple GPUs
./hypernode_miner --gpu --multi-gpu
```

### Verify GPU Access

```bash
# Check OpenCL devices
clinfo

# Test GPU mining (1 minute)
./hypernode_miner --gpu --test
```

---

## 📊 Monitoring

### View Mining Statistics

The miner outputs performance metrics in real-time:

```
[INFO] Hypernode Miner v1.0.0
[INFO] Using 8 CPU threads
[INFO] GPU 0: NVIDIA GeForce RTX 3080
[INFO] Hashrate: 1250 H/s
[INFO] Submitted metrics to Solana: TxID abc123...
[INFO] Uptime: 3600s | Latency: 45ms | Throughput: 125 MB/s
```

### Logging

Logs are written to:
- Console (stdout/stderr)
- `miner.log` file
- Solana on-chain (metrics registry)

---

## 🔐 Security

### Key Management

- **Never share your operator keypair**
- Store keypair securely (use hardware wallet for production)
- Use file permissions to protect keypair:
  ```bash
  chmod 600 /path/to/keypair.json
  ```

### Network Security

- Use trusted Solana RPC endpoints
- Consider running your own Solana validator
- Monitor on-chain transactions

---

## 🐛 Troubleshooting

### Build Errors

**Problem**: CMake not found

**Solution**:
```bash
sudo apt install -y cmake
```

**Problem**: OpenCL headers not found

**Solution**:
```bash
sudo apt install -y ocl-icd-opencl-dev
```

### Runtime Errors

**Problem**: GPU not detected

**Solution**:
```bash
# Check OpenCL installation
clinfo

# Reinstall GPU drivers
sudo ubuntu-drivers autoinstall
sudo reboot
```

**Problem**: Connection to Solana RPC failed

**Solution**:
- Verify `SOLANA_RPC_URL` in `.env`
- Check internet connectivity
- Try alternative RPC endpoint

---

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📚 Resources

- [Hypernode Main Site](https://hypernodesolana.org)
- [Hypernode Core Protocol](https://github.com/Hypernode-sol/hypernode-core-protocol)
- [Solana Documentation](https://docs.solana.com)
- [OpenCL Programming Guide](https://www.khronos.org/opencl/)

---

## 📄 License

See [LICENSE](LICENSE) file for details.

---

## ⚠️ Disclaimer

- Mining uses computational resources and electricity
- Ensure adequate cooling for CPU/GPU
- Monitor system temperatures
- Use at your own risk

---

**Join the Hypernode network and start mining today!**
