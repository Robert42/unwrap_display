# unwrap_display
Like `unwrap` for Result but formats using `core::fmt::Display` trait instead of `core::fmt::Debug`.

This crate is most useful for tests, so I recommend adding it to the `dev-dependencies` section instead of `dependencies` section of the `Cargo.toml` file.
```toml
[dev-dependencies]
unwrap_display = "0.0"
```
