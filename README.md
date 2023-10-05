# ShSSH

An insecure but slightly more useable ssh client. Ssh into servers using the credentials inside the configurataion file.

## Install

Currently this can only be installed through cargo.

```sh
cargo install --git https://github.com/connorvoisey/shssh
```

## Usage

Get help.

```sh
shssh --help
```

Ssh into a server.

```sh
shssh ssh personal
```

List servers.

```sh
shssh ls
```

## Config file

The config file location is set in the env variable "SHSSH_CONFIG" or it will default to "$HOME/.config/shssh.json". The json file must follow the correct schema. 

At some point I'll probably write some docs explaining the config files schema but for now see [example config file](https://github.com/connorvoisey/shssh/config.example.json).
