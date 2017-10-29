# HTTPTest

This library is in eairly stages of development the api may change frequently
as I start to make use of it. Feedback, suggestions and pull requests are
welcome.

A creates simple HTTP servers designed for use in unit and intergration
testing. Servers a simple to create, run in a background thread and cleanly
shutdown when they go out of scope.

## Quick start

With [cargo-edit](https://github.com/killercup/cargo-edit) installed simply run

```bash
cargo add --dev --git https://github.com/mdaffin/httptest.git httptest
```

or add the following to your `Cargo.toml`

```toml
[dev-dependencies.httptest]
git = "https://github.com/mdaffin/httptest.git"
```

To serve a simple string:

```rust
let ts = httptest::serve_str("hello world".to_string());

let mut response = reqwest::get(&ts.url()).unwrap();
let mut body = String::new();
response.read_to_string(&mut body).unwrap();

assert_eq!(body, "hello world");
```

See the [examples](examples) directory for more complete examples.
