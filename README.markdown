# SHARKEY.RS :crab: :shark:

![GitHub CI](https://github.com/alyxshang/sharkey.rs/actions/workflows/rust.yml/badge.svg)

***A tiny library to perform actions on Sharkey using Rust. :crab: :shark:***

## ABOUT

This repository contains the source code for a Rust library enabling users to perform different actions on the Fediverse platform ***Sharkey***. This library is on [crates.io](https://crates.io/crates/sharkey) under my old name. Using that version of this crate is not advised. I not only rebranded myself, but also made some improvements to this crate.

## INSTALLATION

### FOR RUST PROJECTS

To use ***Sharkey.rs*** in your Rust project, add this line to your project's `Cargo.toml`:

```TOML
sharkey = { git = "https://github.com/alyxshang/sharkey.rs", tag="0.1.0" }
```

## USAGE

To perform any actions on any Sharkey server, you will need an API token for your Sharkey account. This token can be obtained in the "API" settings inside your main account settings. To run the provided unit tests, please set the value of the `BLAHAJ_API_TOKEN` environment variable to the API token you generated for your account. To view the documentation of the APIs this crate provides, run the command `cargo doc --open` from the root of this repository.

## CHANGELOG

### Version 0.1.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *Sharkey.rs :crab: :shark: :crab:* by *Alyx Shang :black_heart:*.
- Licensed under the [FSL v1](https://github.com/alyxshang/fair-software-license).
