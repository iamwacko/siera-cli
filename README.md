# Aries CLI

> Made by Animo Solutions

## Summary

The Aries-CLI started out as a small project to practise in Rust. It is now at a point where it can publicly be used and benefit everyone. The goal of this project was to make development easier. Think of things like filling your wallet with test credentials, creating quick invitations and sending basic messages. Many of the functionality is already supported. See [example](#example) for some useful starter commands.

## Support

At this moment there is only support for some functionality of [**aries-cloudagent-python**](https://github.com/hyperledger/aries-cloudagent-python).
Support for [**aries-framework-javascript**](https://github.com/hyperledger/aries-framework-javascript) will be added in the future via the [rest api](https://github.com/hyperledger/aries-framework-javascript-ext/tree/main/packages/rest).

## Installation

### MacOS

**Cargo**

```sh
cargo install aries-cli

```

**Git**

```sh
git clone https://github.com/animo/aries-cli.git
cd aries-cli
cargo install --path .
```

**Brew**

```sh
echo "Coming soon!"
```

### Linux

**Git**

```sh
git clone https://github.com/animo/aries-cli.git
cd aries-cli
cargo install --path .
```

### Windows

> Has not been tested extensively

**Git**

```sh
git clone https://github.com/animo/aries-cli.git
cd aries-cli
cargo install --path .
```

## Usage

To see the complete functionality use the `--help` or `-h` flag.
Each individual subcommand also has the `--help` flag, e.g. `acl features --help`.

```
acl --help

Aries CLI 0.1.0
Animo Solutions
A simple Aries Cloudagent Controller

USAGE:
    acl [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -c, --copy               Copies any output to your OS buffer
    -h, --help               Prints help information
    -s, --suppress-output    Suppresses the output to the CLI
    -V, --version            Prints version information

OPTIONS:
    -k, --apikey <apikey>        The admin apikey
    -o, --config <config>        Config file for the CLI
    -e, --endpoint <endpoint>    Required url of the cloudagent

SUBCOMMANDS:
    connections              Connections subcommand
    credential-definition    Credential definition subcommand
    features                 Discover features subcommand
    help                     Prints this message or the help of the given subcommand(s)
    invite                   Invitations subcommand
    issue-credential         Issue credentials subcommand
    message                  Basic message subcommand
    schema                   Schema subcommand

```

## Examples

Here are some code examples for common use cases.

### Creating an invitation for the toolbox

```sh
acl -c -s invite -t
```

The `-t` flag makes sure the invite has an alias as `Toolbox` and sets auto accept to `true`

The `-c` flag copies the output to your clipboard so it can easily be pasted in the toolbox

The `-s` flag suppresses the output to stdout
