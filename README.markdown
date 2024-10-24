# SHARKEY.RS :crab: :shark:

![GitHub CI](https://github.com/alyxshang/sharkey.rs/actions/workflows/rust.yml/badge.svg)

***A tiny library and CLI tool to perform actions on Sharkey using Rust. :crab: :shark:***

## ABOUT :books:

This repository contains the source code for a Rust library enabling users to perform different actions on the Fediverse platform ***Sharkey***. This library is on [crates.io](https://crates.io/crates/sharkey) under my old name. Using that version of this crate is not advised. I not only rebranded myself, but also made some improvements to this crate.

## INSTALLATION :inbox_tray:

### Installation as a library

To use ***Sharkey.rs*** in your own Rust project, add the following line to the `dependencies` section of your project's `Cargo.toml` file:

```TOML
sharkey = { git = "https://github.com/alyxshang/sharkey.rs", tag="v.0.2.0" }
```

### Installation as a CLI tool

To install ***Sharkey.rs*** as a CLI tool on your own system, run the following command from a terminal session:

```bash
cargo install --git https://github.com/alyxshang/sharkey.rs --tag v.0.2.0 --features=cli
```

To run the command above, you will have to have the Rust toolchain installed.

## USAGE :hammer:

To use ***Sharkey.rs*** as a CLI tool or a library in your Rust project, you will need a Sharkey API token. This token can be obtained in the "API" settings inside your main account settings. 

### API Documentation

To view the documentation of the APIs this crate provides, run the command `cargo doc --open` from the root of this repository.

### CLI tool usage

The CLI provided by ***Sharkey.rs*** features the following eight distinct actions: i) following a user on a Sharkey instance, ii) Unfollowing a user on a Sharkey instance, iii) posting a note from one's own account on a Sharkey instance, iv) deleting a note one has posted on one's own account on a Sharkey instance, v) reacting to a note from one's own account, vi) deleting a reaction to a note from one's own account, vii) getting the current version of the CLI tool, and viii) printing out some information on the usage flags the CLI tool provides.

In the examples featured below, the Sharkey instance I shall be using is [blahaj.zone](https://blahaj.zone). This instance of Sharkey has the API route `api` and the environment variable `BLAHAJ_API_TOKEN` represents the token you generated for your account.

- To follow a user, you can run either of these three commands:

```bash
sharkey --mflow \
	--tauth $BLAHAJ_API_TOKEN \
	--apiad "/api" \
	--inadd "https://blahaj.zone" \
	--namei "9upmnr8igmxe01k3"
# OR
sharkey -m \
    -t $BLAHAJ_API_TOKEN \
    -a "/api" \
    -i "https://blahaj.zone" \
    -n "9upmnr8igmxe01k3"
# OR
sharkey mflow \
    tauth $BLAHAJ_API_TOKEN
    apiad "/api"
    inadd "https://blahaj.zone"
    namei "9upmnr8igmxe01k3"
```

- To unfollow a user, you can run either of these three commands:

```bash
sharkey --dflow \
	--tauth $BLAHAJ_API_TOKEN \
	--apiad "/api" \
	--inadd "https://blahaj.zone" \
	--namei "9upmnr8igmxe01k3"
# OR
sharkey -d \
    -t $BLAHAJ_API_TOKEN \
    -a "/api" \
    -i "https://blahaj.zone" \
    -n "9upmnr8igmxe01k3"
# OR
sharkey dflow \
    tauth $BLAHAJ_API_TOKEN
    apiad "/api"
    inadd "https://blahaj.zone"
    namei "9upmnr8igmxe01k3"
```

- To post a note from your account, you can run either of these three commands:

```bash
sharkey --postn \
    --tauth $BLAHAJ_API_TOKEN \
    --apiad "/api" \
    --inadd "https://blahaj.zone" \
    --conte "Posted from the Sharkey.rs CLI." \
    --visie "public" \
    --etype "LikeOnly"
# OR
sharkey postn \
    tauth $BLAHAJ_API_TOKEN \
    apiad "/api" \
    inadd "https://blahaj.zone" \
    conte "Posted from the Sharkey.rs CLI." \
    visie "public" \
    etype "LikeOnly"
# OR
sharkey -p \
    -t $BLAHAJ_API_TOKEN \
    -a "/api" \
    -i "https://blahaj.zone" \
    -c "Posted from the Sharkey.rs CLI." \
    -v "public" \
    -e "LikeOnly"
```

- To delete a note you have posted from your account, you can run either of these three commands:

```bash
sharkey --rpost \
    --tauth $BLAHAJ_API_TOKEN \
    --apiad "/api" \
    --inadd "https://blahaj.zone" \
    --namei "9zr5dbrdym7x0074"
# OR
sharkey rpost\
    tauth $BLAHAJ_API_TOKEN \
    apiad "/api" \
    inadd "https://blahaj.zone" \
    namei "9zr5dbrdym7x0074"
# OR
sharkey -r \
    -t $BLAHAJ_API_TOKEN \
    -a "/api" \
    -i "https://blahaj.zone" \
    -n "9zr5dbrdym7x0074"
```

- To react to a note, you can run either of these three commands:

```bash
sharkey --liken \
    --tauth $BLAHAJ_API_TOKEN \
    --apiad "/api" \
    --inadd "https://blahaj.zone" \
    --namei "9zpo9el4sh0901es" \
    --conte "like"
# OR
sharkey liken \
    tauth $BLAHAJ_API_TOKEN \
    apiad "/api" \
    inadd "https://blahaj.zone" \
    namei "9zpo9el4sh0901es" \
    conte "like"
# OR
sharkey -l \
    -t $BLAHAJ_API_TOKEN \
    -a "/api" \
    -i "https://blahaj.zone" \
    -n "9zpo9el4sh0901es" \
    -c "like"
```

- To delete a reaction to a note, you can run either of these three commands:

```bash
sharkey --ulike \
    --tauth $BLAHAJ_API_TOKEN \
    --apiad "/api" \
    --inadd "https://blahaj.zone" \
    --namei "9zpo9el4sh0901es" \
    --conte "like"
# OR
sharkey ulike \
    tauth $BLAHAJ_API_TOKEN \
    apiad "/api" \
    inadd "https://blahaj.zone" \
    namei "9zpo9el4sh0901es" \
    conte "like"
# OR
sharkey -u \
    -t $BLAHAJ_API_TOKEN \
    -a "/api" \
    -i "https://blahaj.zone" \
    -n "9zpo9el4sh0901es" \
    -c "like"
```

- To print out version information about the CLI tool, you can run either of these three commands:

```bash
sharkey -v
# OR
sharkey --version
# OR
sharkey version
```

- To print out usage information about the CLI tool, you can run either of these three commands:

```bash
sharkey -h
# OR
sharkey --help
# OR
sharkey help
```



## CHANGELOG :black_nib:

### Version 0.2.0

- Added an optional CLI.
- Added some more documentation.

### Version 0.1.0

- Initial release.
- Upload to GitHub.

## NOTE :scroll:

- *Sharkey.rs :crab: :shark: :crab:* by *Alyx Shang :black_heart:*.
- Licensed under the [FSL v1](https://github.com/alyxshang/fair-software-license).
