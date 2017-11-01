Nal
=====

Nal - Your daily programming language

> Note: This repository is under heavy development.
Do not use this in any serious project.

## Goals

- Asynchronous without hassle
- Type safe without verbosity
- Structural typing FTW

## Milestones

- [x] Parser that define AST
- [x] Simple tree-walk interpreter
- [ ] [Algebraic data types][adt]
- [ ] Type notation and static type checker with inference
- [ ] Compile to binary with LLVM
- [ ] Coroutine/Generator
- [ ] Fiber based goroutine-like runtime
- [ ] Async, lock-free IO with future API
- [ ] Safe [Rust][rust] FFI
- [ ] Runtime code replacement

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
