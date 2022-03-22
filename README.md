# Authenticator CLI

A TOTP Authenticator (AKA Google Authenticator) command line tool with a minimal UI. 
The system is designed to be a minimal drop in for managing TOTP codes.

## Usage

```
authenticator 0.1.0
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
    remove    Remove a secret
    show      Show the value of a secret
```

## Building

```sh
# build the application
make 

# cross compile the application
make CARGO_ARGS="--target aarch64-unknown-linux-gnu" 

# install the application
make install 
```

