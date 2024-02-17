# Rust Intel DPDK Hello World

Complete Guide of Building RUST DPDK Hello World.

I've tried to cover from all the basics

## Dev Environment
[If you have DPDK Compliant NIC you can skip this]

A virtual box VM with following attributes was used to build and run the example due NIC dep.

- `Ubuntu 22.04`
- `4 vcpu`
- `8gb RAM`
- `NIC: default adapter[NAT]`
- `NIC: Intel PRO/1000 MT Desktop (82540EM) [bridged]`
## Installation

### 1. Installation of prerequisites
```sh
sudo apt install \
build-essential \
meson \
python3-pyelftools \
libnuma-dev \
pkgconf \
libclang-dev \
clang \
llvm-dev \
libbsd-dev
```

### 2. DPDK Installation
```sh
tar xf dpdk-<version>.tar.gz # or checkout specific version 
cd dpdk
meson setup -Dplatform=native build
cd build
ninja
meson install #sudo 
ldconfig #sudo
```

### 3. Enable Hugepages

```sh

  mkdir -p /dev/hugepages
  mountpoint -q /dev/hugepages || mount -t hugetlbfs nodev /dev/hugepages
  echo 64 > /sys/devices/system/node/node0/hugepages/hugepages-2048kB/nr_hugepages # prefer to run with root

```

### 4. Install Rust
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 5. Create new rust project
```sh
cargo new demo-rust-dpdk-hello-world
```

### 6. Add DPDK wrapper dependencies
```toml
[dependencies]
# rust dpdk from github
rust-dpdk = { git = "https://github.com/ANLAB-KAIST/rust-dpdk.git", package = "rust-dpdk-sys" }
```

### 7. Building project
```sh
cargo build #--release
```

### 8. Running App
```console
$dpdk-user@vbox: sudo ./target/release/demo-rust-dpdk-hello-world
EAL: Detected CPU lcores: 4
EAL: Detected NUMA nodes: 1
EAL: Detected shared linkage of DPDK
EAL: Multi-process socket /var/run/dpdk/rte/mp_socket
EAL: Selected IOVA mode 'PA'
EAL: VFIO support initialized
TELEMETRY: No legacy callbacks, legacy socket not created
hello from core 1
hello from core 2
hello from core 3
hello from core 0
```