# StreamingFast Substreams Template
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Quick Start (Gitpod)

Use these steps to conveniently open your repository in a Gitpod.

1. First, [copy this repository](https://github.com/streamingfast/substreams-template/generate)
2. Grab a StreamingFast key from [https://app.streamingfast.io/](https://app.streamingfast.io/)
3. Create a [Gitpod](https://gitpod.io) account
4. Configure a `STREAMINGFAST_KEY` variable in your Gitpod account settings
5. Open your repository as a [Gitpod workspace](https://gitpod.io/workspaces)

## Quick Start (Locally)

Use this quickstart guide to set up your environment to use Substreams locally.

First, [copy this repository](https://github.com/streamingfast/substreams-template/generate) and clone it.

## Install Dependencies

Follow [Installation Requirements](https://substreams.streamingfast.io/getting-started/installing-the-cli) instructions on official Substreams documentation wesite.

### Validation

Ensure that `substreams` CLI works as expected:

```
substreams -v
version (...)
```

## Generating Protobuf

```bash
substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"
```

## Compile

At this point, we're ready to build our WASM binary and Protobuf definitions.

```bash
cargo build --target wasm32-unknown-unknown --release
```

The resulting WASM artifact will be found at `./target/wasm32-unknown-unknown/release/substreams.wasm`

## Run your Substream

We're now ready to run our example Substream!

> Don't forget to be at the root of the project to run the following commands

```bash
# to run the map module
substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_transfers --start-block 12292922 --stop-block +1

# to run the store module (and the map module in the background)
substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml store_transfers --start-block 12292922 --stop-block +1
```

Let's break down everything happening above.

- `substreams` is our executable
- `-e mainnet.eth.streamingfast.io:443` is the provider going to run our Substreams
- `substream.yaml` is the path where we have defined our Substreams Manifest
- `map_transfers` (or `store_transfers`) this is the module which we want to run, defined in the manifest
- `--start-block 12292922` start from block `12292922`
- `--stop-block +1` only request a single block (stop block will be manifest's start block + 1)

Here is the example of an output of the `map_transfers` starting at `12292922` block for only `1` block.
The `[...]` was added to abbreviate the JSON output as there was a lot of ERC20 transfers.

```bash
----------- IRREVERSIBLE BLOCK #12,292,922 (12292922) ---------------
map_transfers: message "eth.erc721.v1.Transfers": {
  "transfers": [
    {
      "from": "AAAAAAAAAAAAAAAAAAAAAAAAAAA=",
      "to": "q6cWGn+2nIjhbtn0Vc5it5HuTQM=",
      "trxHash": "z7GX9i7Fx/DnGhHsDEoOOUo6pB21OG6FUm+GyEs/J5Y=",
      "ordinal": "85"
    },
    <continued>,
    {
      "from": "AAAAAAAAAAAAAAAAAAAAAAAAAAA=",
      "to": "q6cWGn+2nIjhbtn0Vc5it5HuTQM=",
      "tokenId": "29",
      "trxHash": "z7GX9i7Fx/DnGhHsDEoOOUo6pB21OG6FUm+GyEs/J5Y=",
      "ordinal": "114"
    }
  ]
}
```

> Bytes are rendered with base64 encoding by default, so it might be a little troubling to see `q6cWGn+2nIjhbtn0Vc5it5HuTQM=` as an Ethereum address, but it's actually `aba7161a7fb69c88e16ed9f455ce62b791ee4d03`, you can use `string` instead of `bytes` if you prefer that in your Protobuf definitions.

## Next Steps

Congratulations! You've successfully run a Substreams.

- Read the documentation at https://substreams.streamingfast.io.
- Look at [Playground](https://github.com/streamingfast/substreams-playground) for more learning examples.
