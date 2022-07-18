# Authenticator CLI

A TOTP Authenticator (AKA Google Authenticator) command line tool with a minimal UI. 
The system is designed to be a minimal drop in for managing TOTP codes.

All codes are stored encrypted at rest but remain portable across systems.

## Usage

```
authenticator 0.2.2
Dominick Schroer <dominick@schroer.ca>
TOTP Authenticator CLI

USAGE:
    authenticator <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add       Add a new secret
    help      Print this message or the help of the given subcommand(s)
    list      List all secrets
    pin       Change the pin on the secret store
    remove    Remove a secret
    show      Show the value of a secret
```

There is also a [SXMO](https://sxmo.org/) style UI under the command `authenticator_ui.sh`.

## Building

```sh
# build the application
make 

# cross compile the application
make CARGO_ARGS="--target aarch64-unknown-linux-gnu" 

# install the application
make install 
```
