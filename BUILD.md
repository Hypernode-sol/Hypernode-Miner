# Build Instructions

## Prerequisites

### Linux/Ubuntu
```bash
sudo apt-get update
sudo apt-get install -y build-essential cmake git libssl-dev libcurl4-openssl-dev
```

### macOS
```bash
brew install cmake openssl curl
```

### Windows
- Install [Visual Studio 2019+](https://visualstudio.microsoft.com/) with C++ tools
- Install [CMake](https://cmake.org/download/)
- Install [OpenSSL](https://slproweb.com/products/Win32OpenSSL.html)

## Building

### Linux/macOS

```bash
# Clone repository
git clone https://github.com/Hypernode-sol/Hypernode-Miner.git
cd Hypernode-Miner

# Create build directory
mkdir build && cd build

# Configure and build
cmake ..
make

# Run
./hypernode_miner
```

### macOS (using provided script)

```bash
chmod +x cmake_mac.sh
./cmake_mac.sh
```

### Windows

```bash
# Using provided batch file
compile_dll.bat
```

Or manually:
```cmd
mkdir build
cd build
cmake ..
cmake --build . --config Release
```

## Build Options

### OpenCL Support (for GPU mining)

```bash
cmake -DUSE_OPENCL=ON ..
make
```

### Debug Build

```bash
cmake -DCMAKE_BUILD_TYPE=Debug ..
make
```

### Release Build with Optimizations

```bash
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_FLAGS="-O3 -march=native" ..
make
```

## Running

### Basic usage

```bash
./hypernode_miner
```

### With configuration

```bash
./hypernode_miner --config config.json
```

### GPU mining

```bash
./hypernode_miner --opencl
```

## Troubleshooting

### OpenSSL not found

**Linux**:
```bash
sudo apt-get install libssl-dev
```

**macOS**:
```bash
brew install openssl
export OPENSSL_ROOT_DIR=/usr/local/opt/openssl
```

### CMake version too old

```bash
# Ubuntu/Debian
sudo apt-get install cmake

# Or download latest from cmake.org
```

### Compilation errors

```bash
# Clean build
rm -rf build
mkdir build && cd build
cmake ..
make clean
make
```

## Platform-Specific Notes

### Linux
- Requires GCC 7+ or Clang 6+
- OpenCL headers: `sudo apt-get install opencl-headers`

### macOS
- Requires Xcode Command Line Tools
- May need to specify OpenSSL path

### Windows
- Requires Visual Studio 2019 or later
- OpenCL SDK for GPU support

## Performance Optimization

### CPU Mining
- Build with `-O3` flag
- Enable architecture-specific optimizations: `-march=native`

### GPU Mining
- Install latest GPU drivers
- Use OpenCL build option
- Tune work size for your GPU

## Support

For build issues, open an issue at:
https://github.com/Hypernode-sol/Hypernode-Miner/issues
