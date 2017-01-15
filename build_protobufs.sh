#!/bin/bash
set -x

protoc --rust_out=src/nes/debugger src/nes/debugger/*.proto
protoc --rust-grpc_out=src/nes/debugger src/nes/debugger/*.proto

grpc_tools_ruby_protoc -I src/nes/debugger/ --ruby_out=repl --grpc_out=repl src/nes/debugger/*.proto
