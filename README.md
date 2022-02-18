# Looks Roar

A project for working with NFTs and NFT markets.

```
roar 
> An NFT roarity inspector

USAGE:
    roar [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -v, --verbose    verbosity

SUBCOMMANDS:
    help     Print this message or the help of the given subcommand(s)
    iface    list supported interfaces
    top      list top rarities
```

# Interface Commands

```
roar-iface 
list supported interfaces

USAGE:
    roar iface [OPTIONS] --contract <contract>

OPTIONS:
    -c, --contract <contract>    contract to target (can be '0xABC' string or path to file)
    -d, --db <db>                path to db (created automatically if absent)
    -f, --fresh                  sync fresh data to db
    -h, --help                   Print help information
    -N, --no-db                  do not use db
    -p, --provider <provider>    ethereum provider (such as 'http://localhost:8545'
    -t, --testnet                use testnet
    -v, --verbose                verbosity

```
