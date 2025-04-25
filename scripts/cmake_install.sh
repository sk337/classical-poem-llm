rm -f cmake.sh

wget https://github.com/Kitware/CMake/releases/download/v3.31.7/cmake-3.31.7-linux-$(arch).sh -O cmake.sh

chmod +x cmake.sh

sudo ./cmake.sh --skip-license --prefix=/usr

rm cmake.sh
