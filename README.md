# katype

<p align="center">
    <img height="200px" alt="Katype Logo" src="./assets/logo.png" />
    <br>
    <strong>A fast typing test from terminal writted in Rust 🦀</strong>
    <br>
    <img alt="GitHub License" src="https://img.shields.io/github/license/kadircy/katype">
    <img alt="GitHub Release" src="https://img.shields.io/github/v/release/kadircy/katype">
    <img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/kadircy/katype">
</p>

[![asciicast](https://asciinema.org/a/6RccCcS2xqrZ39DFaMxWQFYF7.svg)](https://asciinema.org/a/6RccCcS2xqrZ39DFaMxWQFYF7)

## Features

- **Fast**: `katype` can start blazingly fast and run it without any lags or delay.
- **Lightweight**: There are a lot of features and `katype` only uses around **2.5 Mb** of RAM (in debug build).
- **Single-file**: Don't need for additional files or packages to run. Just download it and test your fingers.
- **Customizable**: Bring your own settings, stylings and experience different universes.
- **Statistics**: See your own results and share with your friends in fancy way.
- **And much more**: You can add your wanted feature to this project. Please look at our [CONTRIBUTING area](#contributing)

## Getting Started

```bash
katype            # test your limits in typing
katype -h         # see all avaliable options on cli
katype -v         # see version of the program. Update katype if it is old.
katype -a 5       # Use 5 random words for test.
katype -a 123     # Use 123 random words for test without delay.
```

## Installation
`katype` can be installed in just 1 easy step.

1. **Install Binary**:
`katype` runs on many major platforms. If you are having issues with `katype` on your platform, please [open an issue](https://github.com/kadircy/katype/issues/new)

The recommended way to install is from install script:

```bash
curl -sSfL https://raw.githubusercontent.com/kadircy/katype/master/install.sh | sh
```

## Options

When running `katype`, the following arguments are:
- `--amount, -a`
  * Sets the amount of words to type. Default is `15`.
  * `--amount 22` will set the amount of words to `22`.
  * `-a 5` will set the amount of words to `5`.
- `--language, -l`
  * Sets the language of words to type. Default is `en`.
  * `--language es` will set the language to `Spanish`.
  * `-l en` will set the language to `English`.

## Contributing

Thank you for your interest in contributing to `katype`! We welcome contributions, whether it's fixing bugs, adding features, improving documentation, or helping with testing.

### How to Contribute

1. **Fork the Repository**: Start by forking this repository to your own GitHub account.
2. **Create a New Branch**: Make sure to create a new branch for your changes.
   ```
   git checkout -b feature-branch
   ```
3. **Make Changes**: Implement your changes or additions. Be sure to follow the coding style and include relevant tests if applicable.
4. **Commit Your Changes**: Write clear, concise commit messages that explain your changes.
   ```
   git commit -m "Description of your changes"
   ```
5. **Push to Your Fork**: Push your branch to your forked repository.
   ```
   git push origin feature-branch
   ```
6. **Create a Pull Request**: Open a pull request from your branch to the `master` branch of the `katype` repository. Please ensure the PR description clearly explains what your changes do.

### Code of Conduct

Please follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/conduct) in all interactions. Be respectful and considerate to all contributors.

If you have any questions or need assistance, feel free to reach out via issues or discussions.

## License
This project is licensed under the MIT license - See the [LICENSE](./LICENSE) file for details.
