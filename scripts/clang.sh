#!/bin/bash
set -euo pipefail

rm -f llvm.sh

sudo add-apt-repository ppa:ubuntu-toolchain-r/test -y
sudo apt update
sudo apt install -y lsb-release wget software-properties-common gnupg g++-13

# 0. Check iff LLVM is installed
if [ -d /usr/lib/llvm-18 ]; then
    echo "LLVM 18 is already installed."
    exit 0
fi

# 1. Add the official LLVM GPG key
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh

# 2. Run the script for version 18
sudo ./llvm.sh 18



# 3. Use clang-18 directly
clang-18 --version
clang++-18 --version

sudo update-alternatives --install /usr/bin/clang clang /usr/bin/clang-18 100
sudo update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-18 100

rm llvm.sh