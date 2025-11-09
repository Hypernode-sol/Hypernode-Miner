# Hypernode Miner

> **⚠️ DEPRECATED - November 9, 2025**
>
> This repository is not compatible with the Hypernode network architecture.
>
> **Correct Alternative:**
> - Node operators should use [hypernode-node-client](https://github.com/Hypernode-sol/hypernode-node-client) for GPU compute participation
> - This integrates properly with hypernode-jobs, hypernode-nodes, and hypernode-staking programs
>
> **No New Development:** This repository is archived for historical reference only.

---

## Historical Build Instructions

### Requirements
- **OS**: Linux (Ubuntu 20.04+), macOS, or Windows (with WSL2)
- **CPU**: Multi-core processor (4+ cores recommended)
- **RAM**: 4 GB minimum (8 GB recommended)
- **GPU**: Optional (NVIDIA/AMD with OpenCL support)

### Software Requirements
- **C Compiler**: GCC 9+ or Clang 10+
- **CMake**: 3.15+
- **OpenCL**: For GPU acceleration (optional)

### Build

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y build-essential cmake git
sudo apt install -y ocl-icd-opencl-dev  # For GPU support
```

**Build:**
```bash
git clone https://github.com/Hypernode-sol/Hypernode-Miner.git
cd Hypernode-Miner

mkdir build && cd build
cmake ..
make

# Or use provided script
chmod +x release.sh
./release.sh
```

### Run

```bash
# CPU mode
./hypernode_miner

# GPU mode (if OpenCL available)
./hypernode_miner --gpu

# With custom configuration
./hypernode_miner --config /path/to/config
```

---

## Project Structure

```
Hypernode-Miner/
├── hypernode_miner.c       # Main implementation
├── hypernode_compiler.c    # Compiler
├── miner.h                 # Header definitions
├── elist.h                 # Data structures
├── ocl.c                   # OpenCL GPU support
├── util.c                  # Utility functions
├── crypto/                 # Cryptographic functions
├── HYPERNODEPL/            # Language files
├── CMakeLists.txt          # CMake build configuration
└── README.md               # This file
```

---

## License

See [LICENSE](LICENSE) file for details (GPL v2).

---

## Disclaimer

- This code is not affiliated with the current Hypernode distributed GPU compute network
- Use at your own risk
