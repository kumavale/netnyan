# netnyan 🐱

[![Actions Status](https://github.com/kumavale/netnyan/workflows/Rust/badge.svg)](https://github.com/kumavale/netnyan/actions)
[![Crate](https://img.shields.io/crates/v/netnyan.svg)](https://crates.io/crates/netnyan)
[![license](https://img.shields.io/badge/License-MIT-blue.svg?style=flat)](LICENSE-MIT)
[![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat)](LICENSE-APACHE)

**netnyan** is a command written in Rust like netcat.  
netcat (often abbreviated to nc) is a computer networking utility for reading from and writing to network connections using TCP or UDP.

<img src="https://media.giphy.com/media/4QZK21zlzVIyc/giphy.gif" align="right" />

## Features

- Outbound or inbound connections, TCP ~or UDP~, to or from any ports
- I/O with pipes

## Non-goals

- Complete replacement for netcat
- Support for protocols above TCP

## Install

### Cargo

```
cargo install netnyan
```

## Usage

listen:

```
nn -lp 22222
```

connect:

```
nn localhost 22222
```

HTTP request:

```
echo -n "GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n" | nn example.com 80 | grep OK
```

[](https://support.giphy.com/hc/en-us/articles/360020027752-GIPHY-User-Terms-of-Service)

## Contributing

This project welcomes your PR and issues. For example, fixing bugs, adding features, refactoring, etc.
