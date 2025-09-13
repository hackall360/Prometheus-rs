# Rust Port Outline

This document sketches the initial steps for rewriting Prometheus, originally a Lua obfuscator written in Lua, into a pure Rust implementation.

## 1. Project Setup

* [x] Create a new Rust library crate (`prometheus-rs`).
* [ ] Establish continuous integration for the Rust code.
* [ ] Expose a command line interface using `clap`.

## 2. Lexer

* Design a `Token` enum covering Lua syntax.
* Implement a tokenizer (potentially with `nom`).
* Add unit tests comparing output to the Lua implementation.

## 3. Parser and AST

* Define AST node structures.
* Build a parser that converts tokens into the AST.
* Validate the parser against existing Lua tests.

## 4. Obfuscation Pipeline

* Port each transformation step from the Lua pipeline.
* Use traits to compose steps and allow configurability.

## 5. Code Generation

* Implement an "unparser" that converts the AST back into obfuscated Lua source code.
* Ensure generated code round-trips through the parser.

## 6. Integration

* Replace Lua entry points with a Rust binary.
* Provide an API so other tools can embed the obfuscator.

## Contribution Notes

* Format code with `cargo fmt` and lint with `cargo clippy`.
* Add tests for new modules.

