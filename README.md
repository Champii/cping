# Colorful Ping (cping)

This is a simple wrapper around unix `ping` program to add some colors and a nice interface.

## Demo

![demo](/media/demo.jpg)
![demo](/media/demo2.jpg)

## Features

- Interactive
- Responsive interface
- Configurable
- Cool colors
- Cool histogram graph

## Usage

```
CPing 0.1
Champii <contact@champii.io>
Colorful Ping

USAGE:
    cping [FLAGS] <address> [-- <ping-args>]

FLAGS:
    -h, --help          Prints help information
    -G, --no-graph      Hide the graph
    -H, --no-history    Hide the history
    -L, --no-legend     Hide the legend
    -T, --no-title      Hide the title
    -V, --version       Prints version information

ARGS:
    <address>      the address/domain to ping
    <ping-args>    Pass arguments to ping
```

## Build

Dependancies: Rust (Tested with rust 1.37 nightly, but should work on previous stable releases)

`cargo build`

## Install

`cargo install --path .`

To reinstall or update, add the `--force` flag

## Todo

- Live stats
- Keyboard events
- Tests
