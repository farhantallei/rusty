# rusty_crab_web 🦀

[![Crates.io](https://img.shields.io/crates/v/rusty_crab_web)](https://crates.io/crates/rusty_crab_web)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://opensource.org/licenses/MIT)

`rusty_crab_web` is a **mini web framework for Rust**, inspired by Elysia.  
It makes it easy to create simple HTTP servers without heavy external dependencies.

---

## Features

- Routing for `GET`, `POST`, `DELETE`, etc.
- Simple handler using `Request` and `Response`
- Modular project structure for scalability
- No large external dependencies

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rusty_crab_web = "0.1.0"
```

---

## Example Usage

```rust
use rusty_crab_web::{Rusty, Request, Response};

fn hello_handler(_req: &Request) -> Response {
    Response::new(200, "Hello, Rusty Crab Web! 🦀")
}

fn main() {
    let mut rusty = Rusty::new();

    // Register a GET route
    rusty.get("/hello", hello_handler);

    // Start the server on port 3000
    rusty.listen(3000);
}
```
