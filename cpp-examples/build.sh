#!/bin/bash
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        GPP_PATH=/usr/bin/g++-10
elif [[ "$OSTYPE" == "darwin"* ]]; then
	# macOS; assuming gcc was installed using homebrew
        GPP_PATH=/usr/local/bin/g++-10
fi

mkdir build
cd build
cmake -GNinja -DCMAKE_BUILD_TYPE:STRING=Release -DCMAKE_CXX_COMPILER:STRING=$GPP_PATH ..
cd ..
