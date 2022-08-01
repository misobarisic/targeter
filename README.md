## targeter

[![Continuous Integration](https://github.com/misobarisic/targeter/actions/workflows/ci.yml/badge.svg)](https://github.com/misobarisic/targeter/actions/workflows/ci.yml)
[![Continuous Deployment](https://github.com/misobarisic/targeter/actions/workflows/cd.yml/badge.svg)](https://github.com/misobarisic/targeter/actions/workflows/cd.yml)
[![License](https://img.shields.io/github/license/misobarisic/targeter?color=blue)](./COPYING.md)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/misobarisic/targeter)](https://github.com/misobarisic/targeter/releases/latest)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/misobarisic/targeter)

targeter is a cleanup tool for Rust (Cargo) projects. Remove target directories easily.

---

## Usage

Search through the whole home directory for directories containing `target` and `Cargo.toml`

```shell
targeter -i ~/
```

If no `--input/-i` is specified, current directory is used.


## Installation

### Latest release

Binary releases are available [here](https://github.com/misobarisic/targeter/releases).

### Build from source (latest)

Requires `cargo` to be installed:

1. Clone the repository with `git clone https://github.com/misobarisic/targeter.git` and cd into it
2. Run `cargo build --release --features delay` or `cargo build --release`
3. Move the binary to your place of choice `mv target/release/targeter $destination`

### Arch Linux

3 different packages are available in the Arch Linux User Repository:
- `targeter-bin` (latest binary release)
- `targeter` (latest release, built locally)
- `targeter-git` (latest commit, built locally)

---

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

---

## License
This project is licensed under [GPLv3](https://choosealicense.com/licenses/gpl-3.0/).
