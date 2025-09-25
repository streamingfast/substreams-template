# StreamingFast Substreams Template
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Quick Start (Locally)

Use this quick start guide to set up your environment to use Substreams locally.

First, [copy this repository](https://github.com/streamingfast/substreams-template/generate) and clone it.

## Install Dependencies & Authentication

Follow [Installation Requirements](https://docs.substreams.dev/reference-material/substreams-cli/installing-the-cli) instructions on official Substreams documentation website.

Also make sure that you grabbed your StreamingFast API key and generated a Substreams API token set to environment `SUBSTREAMS_API_TOKEN`, see [authentication instructions](https://docs.substreams.dev/reference-material/substreams-cli/authentication) for how to do it.

### Validation

Ensure that `substreams` CLI works as expected:

```
substreams -v
substreams version 1.15.5 (Commit 85fbda4, Commit Date 2025-05-07T12:58:13Z)
```

> **Note** Your version may differ.

### Generate, Build & Run

Two simple commands:

```bash
# Ensure you have SUBSTREAMS_API_TOKEN environment variable is set

substreams build
substreams run substreams-template-v0.3.1.spkg --start-block 12292922 --stop-block +1
```

Let's break down everything happening above.

- `substreams` is our executable
- the `build` command generates required protobuf bindings and build our Substreams Rust module into a package `substreams-template-v0.3.1.spkg`
- `substreams-template-v0.3.1.spkg` is a compiled Substreams package, it contains your Rust code compiled down to WASM and the manifest extract from `substreams.yaml` source
- the `run` command send your compiled Substreams package to our server to consume network [mainnet](./substreams.yaml#L51)
- `db_out` this is the module which we want to run, defined in the manifest (must be of `map` kind)
- `--start-block 12292922` start from block `12292922`
- `--stop-block +1` only request a single block (stop block will be manifest's start block + 1)

Here is the example of an output of the `map_transfers` starting at `12292922` block for only `1` block:

 > **Note** Using `[...]` to abbreviate the JSON output

```bash
substreams run substreams-template-v0.3.1.spkg db_out --start-block 12292922 --stop-block +10
Connected (trace ID f787f4cd6b170c0da30ce57721d80c81)
Progress messages received: 39 (5/sec)
Backprocessing history up to requested target block 12292922:
(hit 'm' to switch mode)

Stage 0: map_transfers,store_transfers

stage 0 (0 jobs)           12287507  ::  12287507-12292000

# Output above will be different on your machine, what is happening is that we requested block
# 12292922 but the `substreams.yaml` start block is 12287507 which means we have 5 415 blocks to
# catch to build up 'store_transfers' state up to block 12292922. This is done on parallel worker
# and the output above is displaying the advanced of the backward parallel processing.
...

# Once store reconciliation is done, you will start to receive the output of `db_out` module:

----------- BLOCK #12,292,922 (e2d521d11856591b77506a383033cf85e1d46f1669321859154ab38643244293) ---------------
{
  "@module": "db_out",
  "@block": 12292922,
  "@type": "sf.substreams.sink.database.v1.DatabaseChanges",
  "@data": {
    "tableChanges": [
      {
        "table": "transfer",
        "pk": "cfb197f62ec5c7f0e71a11ec0c4a0e394a3aa41db5386e85526f86c84b3f2796-87",
        "operation": "OPERATION_CREATE",
        "fields": [
          {
            "name": "trx_hash",
            "newValue": "cfb197f62ec5c7f0e71a11ec0c4a0e394a3aa41db5386e85526f86c84b3f2796"
          },
          [...]
```

### SQL Sink

This template has a `db_out` module that can be pushed to an `SQL` database, here the steps needed to sink it:

1. Install `substreams-sink-sql` from Brew with `brew install streamingfast/tap/substreams-sink-sql` or by using the pre-built binary release [available in the releases page](https://github.com/streamingfast/substreams-sink-sql/releases) (extract `substreams-sink-sql` binary into a folder and ensure this folder is referenced globally via your `PATH` environment variable).

1. Start a local development Postgres database instance and SQL viewer (`pgweb`) connected to the database, then set up the database schema and sink data:

   ```bash
   # Start the database and pgweb (available on http://localhost:8081)
   docker compose up -d
   
   # Set DSN for convenience
   export DSN="psql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node?sslmode=disable"
   
   # Create the necessary table schema(s)
   substreams-sink-sql setup $DSN substreams-template-v0.3.1.spkg
   
   # Sink to database
   substreams-sink-sql run $DSN substreams-template-v0.3.1.spkg :+100
   ```

   > [!NOTE]
   > Use `:` as the block range argument instead of `:+100` to sink the full chain and follow chain's head block.

1. Read more at https://github.com/streamingfast/substreams-sink-sql?tab=readme-ov-file#substreamssql-sink.

## Next Steps

Congratulations! You've successfully run a Substreams.

Read the documentation at https://substreams.streamingfast.io to continue your Substreams learning.
