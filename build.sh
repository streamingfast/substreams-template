#!/bin/bash

cargo update
PROTOC_INCLUDE=. wasm-pack build --target nodejs
