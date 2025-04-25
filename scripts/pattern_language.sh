sudo clang.sh && sudo cmake.sh

sudo apt update
sudo apt install -y       \
    build-essential       \
    lld                   \
    ninja-build           \
    ccache

gh repo clone WerWolv/PatternLanguage

cd PatternLanguage

# goto the 
git reset --hard 9833500

git submodule update --init --recursive

git apply ../pattern_language.patch

rm -rf build

mkdir build
cd build

CC=clang-18 CXX=clang++-18 cmake            \
    -DCMAKE_BUILD_TYPE=Debug                \
    -DCMAKE_INSTALL_PREFIX="/usr"           \
    -DLIBPL_ENABLE_TESTS=OFF                \
    -DLIBPL_ENABLE_CLI=ON                   \
    -DCMAKE_C_COMPILER_LAUNCHER=ccache      \
    -DCMAKE_CXX_COMPILER_LAUNCHER=ccache    \
    -DCMAKE_INSTALL_PREFIX="./install"      \
    -G Ninja                                \
    ..

ninja -j$(nproc)