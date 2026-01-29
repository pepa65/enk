[![version](https://img.shields.io/crates/v/enk.svg)](https://crates.io/crates/enk)
[![build](https://github.com/pepa65/enk/actions/workflows/ci.yml/badge.svg)](https://github.com/pepa65/enk/actions/workflows/ci.yml)
[![dependencies](https://deps.rs/repo/github/pepa65/enk/status.svg)](https://deps.rs/repo/github/pepa65/enk)
[![docs](https://img.shields.io/badge/docs-enk-blue.svg)](https://docs.rs/crate/enk/latest)
[![license](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://github.com/pepa65/enk/blob/main/LICENSE)
[![downloads](https://img.shields.io/crates/d/enk.svg)](https://crates.io/crates/enk)

# enk 0.3.5
**Simple data en/decryption**

* License: GPLv3.0
* Authors: github.com/pepa65, Ariel Horwitz
* Repo: https:/github.com/pepa65/enk
* After: https://github.com/ArielHorwitz/rhinopuffin

## Usage
```
enk 0.3.5 - Simple data en/decryption
Usage: enk [OPTIONS] [FILE]
Arguments:
  [FILE]  Input file (omit to read from stdin)

Options:
  -d, --decrypt              Decrypt [default: encrypt]
  -k, --keyfile <KEYFILE>    Use a file as encryption/decryption key
  -p, --password <PASSWORD>  Password as argument (instead of prompting)
  -r, --remove               Remove input file
  -h, --help                 Print help
  -V, --version              Print version
```

## Install
### Download static single-binary
```
wget https://github.com/pepa65/enk/releases/download/0.3.5/enk
sudo mv enk /usr/local/bin
sudo chown root:root /usr/local/bin/enk
sudo chmod +x /usr/local/bin/enk
```

### Using cargo (rust toolchain)
If not installed yet, install a **Rust toolchain**, see https://www.rust-lang.org/tools/install

### Cargo from crates.io
`cargo install enk`

#### Cargo from git

`cargo install --git https://github.com/pepa65/enk`

#### Cargo static build (avoid GLIBC incompatibilities)
```
rustup target add x86_64-unknown-linux-musl
git clone https://github.com/pepa65/enk
cd enk
cargo rel
```

### Install with cargo-binstall
Even without a full Rust toolchain, rust binaries can be installed with the static binary `cargo-binstall`:

```
# Install cargo-binstall for Linux x86_64
# (Other versions are available at https://crates.io/crates/cargo-binstall)
wget github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
tar xf cargo-binstall-x86_64-unknown-linux-musl.tgz
sudo chown root:root cargo-binstall
sudo mv cargo-binstall /usr/local/bin/
```

Only a linux-x86_64 (musl) binary available: `cargo-binstall enk`

Then `enk` will be installed in `~/.cargo/bin/` which will need to be added to `PATH`!

