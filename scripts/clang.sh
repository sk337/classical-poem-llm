rm llvm.sh

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