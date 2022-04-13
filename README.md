# substreams-template
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Overview
This repo has for objective to demo how you can build your own substreams. We will go over some examples from easy to 
more complex use-cases to have an in-depth understanding of how we can create substreams.

## Download local blocks
Download gsutils from [here](https://cloud.google.com/storage/docs/gsutil_install) if you do not have the cli installed.

Choose any of the below lines of gsutil to copy some blocks locally to speed things up:

```
mkdir localblocks 
gsutil -m cp 'gs://dfuseio-global-blocks-us/eth-mainnet/v5/001444*' ./localblocks/
```

> If you get an issue while downloading blocks please contact StreamingFast to get an auth key.

## Install rust, wasmer and wasm-pack

#### Rust
Install rust [here](https://www.rust-lang.org/tools/install)

#### Wasmer
Install wasmer [here](https://wasmer.io/)

#### Wasm-pack
install wasm-pack [here](https://rustwasm.github.io/wasm-pack/installer/)

## Download or build substreams-alpha client

#### Download client
substreams-alpha client lives [here](https://github.com/streamingfast/sf-ethereum/releases/tag/v0.10.0-rc.2) 
choose the version best suited for you. Once you have unzipped it, copy and paste the executable at the root of substreams-template.

#### Build from source code from release page
substreams-alpha client lives [here](https://github.com/streamingfast/sf-ethereum/releases/tag/v0.10.0-rc.2)
choose the source code (zip or tar) and download it. Once you have the source code, you need to build it. Let's assume
you have unzipped the source code on your Desktop.

```bash
cd ~/Desktop/sf-ethereum-0.10.0-rc.2
go install -v ./cmd/substreams-alpha
```

> don't forget to `cd` back to where this repo is on your computer

#### Build from GitHub repository
```bash
git clone git@github.com:streamingfast/sf-ethereum.git
cd sf-ethereum
go install -v ./cmd/substreams-alpha
```

## Build wasm
```bash
./build.sh
```

This will build the wasm binary and proto definitions. The wasm binary will be created in:
```bash
./demo/target/wasm32-unknown-unknown/release/substreams_template.wasm
```

## Set up environment variable for RPC
```bash
export STREAMING_FAST_ETH_MAINNET_RPC={{ YOUR_FAVORITE_RPC_ENDPOINT_HERE }}
```

## Run substreams-alpha with wasm binary
```bash
substreams-alpha local ./wasm_demo.yaml block_to_hello_world 10000 --blocks-store-url=./localblocks --rpc-endpoint=$STREAMING_FAST_ETH_MAINNET_RPC
```

Let's break down everything happening above.

- `substreams-alpha` is the executable which was either downloaded or built
- `./wasm_demo.yaml` is the path where we have defined our manifest of what we want to run (i.e. modules with inputs and outputs)
- `hello_world` this is the module which we want to run
- `10000` the number of blocks to process from the starting block (set in wasm_demo.yaml)

> Don't forget to be at the root of the project to run the above command

Here is the example of an output of the `block_to_erc_20_transfer` starting at `14440000` block for only `1` block.
The `[...]` was added to abbreviate the JSON output as there was a lot of ERC20 transfers.

```bash
Mermaid graph:

```mermaid
graph TD;
  sf.ethereum.type.v1.Block -- "source:sf.ethereum.type.v1.Block" --> block_to_hello_world
  sf.ethereum.type.v1.Block -- "source:sf.ethereum.type.v1.Block" --> block_to_erc_20_transfer
  block_to_erc_20_transfer -- "map:block_to_erc_20_transfer" --> erc_20_transfer

Using RPC endpoint: {{ YOUR_FAVORITE_RPC_ENDPOINT_HERE }}
Adding mapper for module "block_to_erc_20_transfer"
-------------------------------------------------------------------
BLOCK +0 14440000 42777bef0aaef0468697a1c5f9d130ec0ff210b653624f1cb207f057d4bd99bf

{
  "transfers": [
    {
      "from": "0x48acf41d10a063f9a6b718b9aad2e2ff5b319ca2",
      "to": "0x109403ab5c5896711699dd3de01c1d520f79801a",
      "amount": "7752235492486228145381410794440202021481973102607839495265831900745419456512",
      "balanceChangeFrom": [
        {
          "oldBalance": "13569385457497991651199724805705614201555076328004753598373935625927319879680",
          "newBalance": "14021698306081258039573048965895801341606912205604912051653066813458230542336"
        }
      ],
      "balanceChangeTo": [
        {
          "oldBalance": "9498569820248594155839807363993929941088553429603327518861754938149123915776",
          "newBalance": "9950882668831860544213131524184117081140389307203485972140886125680034578432"
        }
      ]
    },
    [...]
}
```