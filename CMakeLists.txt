cmake_minimum_required(VERSION 3.20)

project(Encrypt)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/target)

include_directories(deps)
add_executable(Encrypt src/main.cpp)

find_package(OpenSSL REQUIRED)
target_link_libraries(Encrypt OpenSSL::SSL OpenSSL::Crypto)
