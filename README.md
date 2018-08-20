# Test framework for XPIR and SealPIR

Framework to measure latency and throughput of [xpir-rust](https://github.com/pung-project/xpir-rust) and [sealpir-rust](https://github.com/pung-project/sealpir-rust).


# Compiling test framework

There are currently some issues with the RPC library that we use (tarpc) on the latest Rust compiler.
We have tested this code on Rust 1.29.0. Use ``rustup override set nightly-2018-07-30`` to set the rust compiler
to the exact version that we have tested.

You must first install xpir-rust and sealpir-rust as described in the respective repositories (see above). Add the corresponding paths to ``Cargo.toml`` (currently it assumes both repositories are in the directory ``..`` relative to the directory of the pir-test repository). Run ``cargo build --release``.

After compiling, there will be 4 binaries in the ``target/release/`` directory: ``client_sealpir``, ``client_xpir``, ``server_sealpir``, and ``server_xpir``. You can run them with the ``--help`` flag for the available options.

# Reproducing the results in the paper

See the ``client_master.rb`` and ``server_master.rb`` scripts. You can use these scripts to launch a PIR server and one or more PIR clients.
