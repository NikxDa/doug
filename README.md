# doug - A DiG clone

## Introduction
Doug is a DNS querying tool that I am building for educational purposes. It's output is much nicer and more readable than that of DiG, and it has an interactive mode when you do not supply any arguments. It is a WiP, so this page is subject to change.

## Features
- [x] Interactive Mode
- [x] Beautiful Output
- [x] Basic Records like A, MX, ...
- [ ] Full DNS Spec Support
- [ ] EDNS Support

## Installation
Clone this repository and run `cargo build --release`. You can then find the binary file in `target/release/`. Move that to your PATH, and you're good to go!

## Usage
> doug [domain] [@nameserver] [type]

## Example
> doug nikx.io A @8.8.8.8

![doug output](https://raw.githubusercontent.com/nikxda/doug/master/doug.jpg)