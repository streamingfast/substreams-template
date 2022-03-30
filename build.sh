#!/bin/bash

cargo update
PROTOC_INCLUDE=$(pwd)/proto wasm-pack build --target nodejs
