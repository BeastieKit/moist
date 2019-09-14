<!-- <p align="center">
  <img src="libzetta.png">
</p> -->

> Moist is std facade for FreeBSD kernel development for stable Rust

## Installation
I haven't tested this yet, at all. However, due to nature of libstd in Rust I will provide a tool that downloads pre-built sysroot. That's the only way this library can be used in _stable_ Rust.

## Limitations
This is a shim for 1.35.0 version of libstd. That's just because 1.35.0 is the latest version of Rust for FreeBSD that has pre-built components like RLS.

Once I confirm this actually works I will start work on making is available for current stable rust.
