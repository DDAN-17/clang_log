# clang_log
![docs.rs](https://img.shields.io/docsrs/clang_log)
![Crates.io Total Downloads](https://img.shields.io/crates/d/clang_log)
![Crates.io License](https://img.shields.io/crates/l/clang_log)
![Crates.io Dependents](https://img.shields.io/crates/dependents/clang_log)
![Crates.io Version](https://img.shields.io/crates/v/clang_log)

I really like the logging format of `clang` (pronounced klang, not C-lang), so I recreated it as an implementation of the `log` crate. It's licensed under the MIT license, and you can use it as you like. Use it, fork it, steal the code and sell it, I don't really care. If you find bugs, first of all, good job, because how can a logging implementation have bugs, and second of all, you can submit them to the issue tracker, or fix them and submit a PR. Additionally, if you want to improve `clang_log`, go ahead and waste five minuites writing a PR to change something.

## How to use

To use `clang_log`, first include it, and `log` in `Cargo.toml`: 

```toml
[dependencies]
log = "0.4.22"
clang_log = "1.0.2"
```

Then, initialize it at the start of the program:
```rust
use log::Level;

fn main() {
    clang_log::init(Level::Trace, "clang")
}
```

You can also specify a minimum level to start outputting to stderr:
```rust
use log::Level;

fn main() {
    clang_log::init_error(Level::Trace, Level::Warn "clang")
}
```

To use it, just use the macros provided by `log`

```rust
use log::*;

error!("Could not find files!");
```