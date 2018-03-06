Nal
=====

[![Build Status](https://travis-ci.org/HyeonuPark/Nal.svg?branch=master)](https://travis-ci.org/HyeonuPark/Nal)
[![codecov](https://codecov.io/gh/HyeonuPark/Nal/branch/master/graph/badge.svg)](https://codecov.io/gh/HyeonuPark/Nal)

Nal - Your daily programming language

> Note: This repository is under heavy development.
Do not use this in any serious project.

## Goals

- Asynchronous without hassle
- Type safe without verbosity
- Structural typing FTW

## // TODO:

### Nal v0.2 should have:

  - Simple function that accepts source code and execute it
  - Web playground that can execute `Nal` and show its output

### Nal v0.2 should not have:

  - Api stability
  - Any syntactic sugar
  - Compound types
  - Static type checker
  - User-friendly documentations

### Components in Nal v0.2

  - `nal_ast`: Abstract Syntax Tree structure for source code.
  - `nal_ir`: Intermediate Representation as a desugared control flow semantics.
  - `nalc_parser`: Parser that produces AST from source code.
  - `nalc_atoi`: Convert `a`st `to` `i`r.
  - `nali`: IR interpreter with dynamic type check.

## Planned features

- [Algebraic data types][adt]
- Type notation and static type checker with inference
- Compile to binary with LLVM
- Coroutine/Generator
- Turing-complete metaprogramming
- Fiber based goroutine-like runtime
- Async, lock-free IO with future API
- Safe [Rust][rust] FFI
- Runtime code replacement

## License

This repository is dual-licensed under the [MIT license][license-mit]
and [Apache license 2.0][license-apl] at your option.
By contributing to Nal you agree that your contributions will be licensed
under these two licenses.

<!-- links -->

[adt]: https://en.wikipedia.org/wiki/Algebraic_data_type
[rust]: https://www.rust-lang.org/
[license-mit]: ./LICENSE-MIT
[license-apl]: ./LICENSE-APACHE
