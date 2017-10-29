# HTTPTest

A creates simple HTTP servers designed for use in unit and intergration
testing. Servers a simple to create, run in a background thread and cleanly
shutdown when they go out of scope.

```rust
let ts = httptest::serve_str("hello world".to_string());

let mut response = reqwest::get(&ts.url()).unwrap();
let mut body = String::new();
response.read_to_string(&mut body).unwrap();

assert_eq!(body, "hello world");
```

See the [examples](tree/master/examples) directory for more complete examples.
