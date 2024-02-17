# Quantlib on Rust

An example of calling [Quantlib](https://github.com/lballabio/quantlib) C++ functionalities inside Rust using CxxBridge.

## Usage

Clone the repository with `--recurse-submodules`:

```bash
git clone https://github.com/randommm/quantlib-on-rust.git --recurse-submodules --shallow-submodules --depth=1
```

Install Boost if you don't have it already, e.g.:

```bash
sudo apt-get install libboost-all-dev
```

And then just run with Cargo which will automatically build Quantlib and link it with the Rust executable:

```bash
cargo run
```
