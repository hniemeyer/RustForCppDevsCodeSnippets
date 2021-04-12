#!/bin/bash
mkdir build
cd build
cmake -GNinja -DCMAKE_BUILD_TYPE:STRING=Release -DCMAKE_CXX_COMPILER:STRING=/usr/bin/g++-10 ..
cd ..
