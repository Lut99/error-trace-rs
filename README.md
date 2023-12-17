# error-trace-rs
Small Rust crate for printing nice errors traits based on [`Error::source()`].


## Usage
Using the crate is quite straightforward.

First, create your errors as usual:
```rust
#[derive(Debug)]
struct SomeError {
    msg : String,
}
impl Display for SomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", self.msg)
    }
}
impl Error for SomeError {}

#[derive(Debug)]
struct HigherError {
    msg   : String,
    child : SomeError,
}
impl Display for HigherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", self.msg)
    }
}
impl Error for HigherError {
    fn source(&self) -> Option<&(dyn 'static + Error)> {
        Some(&self.child)
    }
}
```

Then, when it is time to report them to the user, do not print them directly but instead use the `ErrorTrace`-trait's `trace()` function:
```rust
use error_trace::ErrorTrace as _;

// ...

let err: HigherError = HigherError {
    msg: "Oh no, something went wrong!".into(),
    child: SomeError{
        msg: "A specific reason".into()
    }
};
eprintln!("{}", err.trace());
```
This will show you:
```text
Oh no, something went wrong!

Caused by:
 o A specific reason
```

If you enable the `colours`-feature, you can additionally print some neat colours:
```rust
use error_trace::ErrorTrace as _;

// ...

let err: HigherError = HigherError {
    msg: "Oh no, something went wrong!".into(),
    child: SomeError{
        msg: "A specific reason".into()
    }
};

// Requires the `colours`-feature!
eprintln!("{}", err.trace_coloured());
```
![Showing the same error as above but with some errors](https://github.com/Lut99/error-trace-rs/raw/main/img/example_colours.png)

Finally, when used in a situation where you want to show a quick error but are sure to never needs its contents, you can use the [`trace!()`]-macro:
```rust
use error_trace::trace;
// Do something that fails
let err = std::str::from_utf8(&[0xFF]).unwrap_err();
// Format it with a one-time parent error
eprintln!("{}", trace!(("Oh no, everything went wrong!"), err));
```

For users of the `colours`-feature, there is the associated [`trace_coloured!()`]-macro:
```rust
use error_trace::trace_coloured;
// Do something that fails
let err = std::str::from_utf8(&[0xFF]).unwrap_err();
// Format it with a one-time parent error
eprintln!("{}", trace_coloured!(("Oh no, everything went wrong!"), err));
```


## Installation
To use this crate into one of your projects, simply add it to your `Cargo.toml` file:
```toml
error-trace = { git = "https://github.com/Lut99/error-trace-rs" }
```
Optionally, you can commit to a particular tag:
```toml
error-trace = { git = "https://github.com/Lut99/error-trace-rs", tag = "v1.1.0" }
```

To build this crate's documentation and open it, run:
```bash
cargo doc --all-features --no-deps --open
```
in the root of the repository.

### Features
The crate has the following features:
- `colors`: Alias for the `colours`-trait.
- `colours`: Enables the use of [`trace_coloured()`].
- `macros`: Enables the use of the [`trace!()`]- and [`trace_coloured!()`]-macros.


## Contribution
If you are interested in contributing in this project, feel free to raise [an issue](https://github.com/Lut99/error-trace-rs/issues) or create [a pull request](https://github.com/Lut99/error-trace-rs/pulls). Note that this is mainly a hobby project, so not all suggestions may be accepted no matter how good it is ;)


## License
This project is licensed under the GPLv3 license. See [LICENSE](./LICENSE) for more information.
