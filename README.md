# StreamingFast Substream Template
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Quick Start

### Compilation

You will need:
- Rust (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` to install)
- Protobuf Compiler `protoc` (https://grpc.io/docs/protoc-installation/#install-using-a-package-manager)

```
./build.sh
```

#### Using Docker

If you don't want to install all the dependencies, you can use a pre-built image:

```
docker run -it --rm -v `pwd`:/substreams --entrypoint '' ghcr.io/streamingfast/substreams:develop bash -c 'cd /substreams && ./build.sh'
```

### Running

You will need:
- Substreams (https://github.com/streamingfast/substreams-docs/blob/master/docs/guides/quickstart.md#obtain-the-substreams-cli-tool)

```
substreams run mainnet.ethereum.streamingfast.io substream.yaml block_to_hello_world
```

## Next Steps

Read the documentation at https://github.com/streamingfast/substreams-docs/blob/master/docs.

