cmake_minimum_required(VERSION 3.1)
project(cpp-examples LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
add_executable(vector-example vector.cpp)
add_executable(pattern-matching-example patternmatching.cpp)
add_executable(move-example move.cpp)
