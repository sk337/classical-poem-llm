#!/bin/bash
set -euo pipefail

rm -f cmake.sh

#check if CMake is installed
if [ -d /usr/bin/cmake ]; then
    echo "CMake is already installed."
    exit 0
fi

wget https://github.com/Kitware/CMake/releases/download/v3.31.7/cmake-3.31.7-linux-$(arch).sh -O cmake.sh

chmod +x cmake.sh

sudo ./cmake.sh --skip-license --prefix=/usr

rm cmake.sh
