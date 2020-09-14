cmake_minimum_required(VERSION 3.17)
project(meetupCpp)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_FLAGS "-O3")

add_executable(oop opp.cpp)
add_executable(ownership ownership.cpp)
add_executable(ownership2 ownership2.cpp)
add_executable(sort sort.cpp)
add_executable(bench bench.cpp)