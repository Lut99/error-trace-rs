//  LIB.rs
//    by Lut99
//
//  Created:
//    22 Sep 2023, 12:17:19
//  Last edited:
//    17 Mar 2024, 12:54:12
//  Auto updated?
//    Yes
//
//  Description:
//!   Small Rust crate for printing nice errors traits based on [`Error::source()`].
//!   
//!   # Usage
//!   Using the crate is quite straightforward.
//!   
//!   First, create your errors as usual:
//!   ```rust
//!   # use std::error::Error;
//!   # use std::fmt::{Display, Formatter, Result as FResult};
//!   #[derive(Debug)]
//!   struct SomeError {
//!       msg : String,
//!   }
//!   impl Display for SomeError {
//!       fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
//!           write!(f, "{}", self.msg)
//!       }
//!   }
//!   impl Error for SomeError {}
//!   
//!   #[derive(Debug)]
//!   struct HigherError {
//!       msg   : String,
//!       child : SomeError,
//!   }
//!   impl Display for HigherError {
//!       fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
//!           write!(f, "{}", self.msg)
//!       }
//!   }
//!   impl Error for HigherError {
//!       fn source(&self) -> Option<&(dyn 'static + Error)> {
//!           Some(&self.child)
//!       }
//!   }
//!   ```
//!   
//!   Then, when it is time to report them to the user, do not print them directly but instead use the `ErrorTrace`-trait's `trace()` function:
//!   ```rust
//!   use error_trace::ErrorTrace as _;
//!   
//!   # use std::error::Error;
//!   # use std::fmt::{Display, Formatter, Result as FResult};
//!   # #[derive(Debug)]
//!   # struct SomeError {
//!   #     msg : String,
//!   # }
//!   # impl Display for SomeError {
//!   #     fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
//!   #         write!(f, "{}", self.msg)
//!   #     }
//!   # }
//!   # impl Error for SomeError {}
//!   #
//!   # #[derive(Debug)]
//!   # struct HigherError {
//!   #     msg   : String,
//!   #     child : SomeError,
//!   # }
//!   # impl Display for HigherError {
//!   #     fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
//!   #         write!(f, "{}", self.msg)
//!   #     }
//!   # }
//!   # impl Error for HigherError {
//!   #     fn source(&self) -> Option<&(dyn 'static + Error)> {
//!   #         Some(&self.child)
//!   #     }
//!   # }
//!   // ...
//!   
//!   let err: HigherError = HigherError {
//!       msg: "Oh no, something went wrong!".into(),
//!       child: SomeError{
//!           msg: "A specific reason".into()
//!       }
//!   };
//!   eprintln!("{}", err.trace());
//!   ```
//!   This will show you:
//!   ```text
//!   Oh no, something went wrong!
//!   
//!   Caused by:
//!    o A specific reason
//!   ```
//!   
//!   If you enable the `colours`-feature, you can additionally print some neat colours:
//!   ```rust
//!   use error_trace::ErrorTrace as _;
//!   
//!   # use std::error::Error;
//!   # use std::fmt::{Display, Formatter, Result as FResult};
//!   # #[derive(Debug)]
//!   # struct SomeError {
//!   #     msg : String,
//!   # }
//!   # impl Display for SomeError {
//!   #     fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
//!   #         write!(f, "{}", self.msg)
//!   #     }
//!   # }
//!   # impl Error for SomeError {}
//!   #
//!   # #[derive(Debug)]
//!   # struct HigherError {
//!   #     msg   : String,
//!   #     child : SomeError,
//!   # }
//!   # impl Display for HigherError {
//!   #     fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
//!   #         write!(f, "{}", self.msg)
//!   #     }
//!   # }
//!   # impl Error for HigherError {
//!   #     fn source(&self) -> Option<&(dyn 'static + Error)> {
//!   #         Some(&self.child)
//!   #     }
//!   # }
//!   // ...
//!   
//!   let err: HigherError = HigherError {
//!       msg: "Oh no, something went wrong!".into(),
//!       child: SomeError{
//!           msg: "A specific reason".into()
//!       }
//!   };
//!   
//!   // Requires the `colours`-feature!
//!   eprintln!("{}", err.trace_coloured());
//!   ```
//!   ![Showing the same error as above but with some errors](https://github.com/Lut99/error-trace-rs/raw/main/img/example_colours.png)
//!
//!   Finally, when used in a situation where you want to show a quick error but are sure to never needs its contents, you can use the [`trace!()`]-macro:
//!   ```rust
//!   use error_trace::trace;
//!  
//!   // Do something that fails
//!   let err = std::str::from_utf8(&[0xFF]).unwrap_err();
//!  
//!   // Format it with a one-time parent error
//!   eprintln!("{}", trace!(("Oh no, everything went wrong!"), err));
//!   ```
//!
//!   For users of the `colours`-feature, there is the associated [`trace_coloured!()`]-macro:
//!   ```rust
//!   use error_trace::trace_coloured;
//!  
//!   // Do something that fails
//!   let err = std::str::from_utf8(&[0xFF]).unwrap_err();
//!  
//!   // Format it with a one-time parent error
//!   eprintln!("{}", trace_coloured!(("Oh no, everything went wrong!"), err));
//!   ```
//!   
//!   
//!   # Installation
//!   To use this crate into one of your projects, simply add it to your `Cargo.toml` file:
//!   ```toml
//!   error-trace = { git = "https://github.com/Lut99/error-trace-rs" }
//!   ```
//!   Optionally, you can commit to a particular tag:
//!   ```toml
//!   error-trace = { git = "https://github.com/Lut99/error-trace-rs", tag = "v1.1.0" }
//!   ```
//!   
//!   To build this crate's documentation and open it, run:
//!   ```bash
//!   cargo doc --all-features --no-deps --open
//!   ```
//!   in the root of the repository.
//!   
//!   ## Features
//!   The crate has the following features:
//!   - `colors`: Alias for the `colours`-trait.
//!   - `colours`: Enables the use of [`trace_coloured()`].
//!   - `macros`: Enables the use of the [`trace!()`]- and [`trace_coloured!()`]-macros.
//

use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

#[cfg(feature = "colours")]
use console::style;


/***** MACROS *****/
/// Creates a one-time [`ErrorTrace`]-compatible type from the given string, then calls [`trace()`](ErrorTrace::trace()) on it.
///
/// # Arguments
/// The macro has the following signature:
/// ```plain
/// (`$($args:tt)*), $err:expr
/// ```
/// - `$($args:tt)*`: A message to use for the toplevel error. This can be given the arguments to a [`format!`]-call.
/// - `$err:expr`: The error to embed in the newly built type.
///
/// # Returns
/// An [`ErrorTraceFormatter`] that can be displayed immediately.
///
/// # Example
/// ```rust
/// use error_trace::trace;
///
/// // Do something that fails
/// let err = std::str::from_utf8(&[0xFF]).unwrap_err();
///
/// // Format it with a one-time parent error
/// assert_eq!(
///     trace!(("Oh no, everything went wrong!"), err).to_string(),
///     r#"Oh no, everything went wrong!
///
/// Caused by:
///  o invalid utf-8 sequence of 1 bytes from index 0
///
/// "#
/// );
/// ```
/// One can use full format strings for the message:
/// ```rust
/// use error_trace::trace;
///
/// // Do something that fails
/// let bytes: [u8; 1] = [0xFF];
/// let err = std::str::from_utf8(&bytes).unwrap_err();
///
/// // Format it with a one-time parent error
/// assert_eq!(
///     trace!(("Failed to parse '{:?}'", bytes.as_slice()), err).to_string(),
///     r#"Failed to parse '[255]'
///
/// Caused by:
///  o invalid utf-8 sequence of 1 bytes from index 0
///
/// "#
/// );
///
///
/// // Equivalent to above (but using a neater format syntax!)
/// assert_eq!(
///     trace!(("Failed to parse '{bytes:?}'"), err).to_string(),
///     r#"Failed to parse '[255]'
///
/// Caused by:
///  o invalid utf-8 sequence of 1 bytes from index 0
///
/// "#
/// );
/// ```
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! trace {
    (($($args:tt)*), $err:expr) => {
        ::error_trace::ErrorTraceFormatter::new(format!($($args)*), Some(&$err))
    };
}

/// Creates a one-time [`ErrorTrace`]-compatible type from the given string, then calls [`trace_coloured()`](ErrorTrace::trace_coloured()) on it.
///
/// # Arguments
/// The macro has the following signature:
/// ```plain
/// ($($args:tt)*), $err:expr
/// ```
/// - `$($args:tt)*`: A message to use for the toplevel error. This can be given the arguments to a [`format!`]-call.
/// - `$err:expr`: The error to embed in the newly built type.
///
/// # Returns
/// An [`ErrorTraceColourFormatter`] that can be displayed immediately.
///
/// # Example
/// ```rust
/// use error_trace::trace_coloured;
///
/// // Do something that fails
/// let err = std::str::from_utf8(&[0xFF]).unwrap_err();
///
/// // Colours aren't visible here, because we're writing to a string; but try writing to stdout/stderr!
/// assert_eq!(
///     trace_coloured!(("Oh no, everything went wrong!"), err).to_string(),
///     r#"Oh no, everything went wrong!
///
/// Caused by:
///  o invalid utf-8 sequence of 1 bytes from index 0
///
/// "#
/// );
/// ```
/// One can use full format strings for the message:
/// ```rust
/// use error_trace::trace_coloured;
///
/// // Do something that fails
/// let bytes: [u8; 1] = [0xFF];
/// let err = std::str::from_utf8(&bytes).unwrap_err();
///
/// // Format it with a one-time parent error
/// assert_eq!(
///     trace_coloured!(("Failed to parse '{:?}'", bytes.as_slice()), err).to_string(),
///     r#"Failed to parse '[255]'
///
/// Caused by:
///  o invalid utf-8 sequence of 1 bytes from index 0
///
/// "#
/// );
///
///
/// // Equivalent to above (but using a neater format syntax!)
/// assert_eq!(
///     trace_coloured!(("Failed to parse '{bytes:?}'"), err).to_string(),
///     r#"Failed to parse '[255]'
///
/// Caused by:
///  o invalid utf-8 sequence of 1 bytes from index 0
///
/// "#
/// );
/// ```
#[cfg(all(feature = "colours", feature = "macros"))]
#[macro_export]
macro_rules! trace_coloured {
    (($($args:tt)*), $err:expr) => {
        ::error_trace::ErrorTraceColourFormatter::new(format!($($args)*), Some(&$err))
    };
}





/***** FORMATTERS *****/
/// Formats an error and all its dependencies.
///
/// If you have the `colours`-feature enabled, then you can also use [`ErrorTraceColourFormatter`] to do the same but with colours.
///
/// # Example
/// ```rust
/// # use std::error::Error;
/// # use std::fmt::{Display, Formatter, Result as FResult};
/// use error_trace::{ErrorTrace as _, ErrorTraceFormatter};
///
/// #[derive(Debug)]
/// struct ExampleError {
///     msg: String,
/// }
/// impl Display for ExampleError {
///     fn fmt(&self, f: &mut Formatter<'_>) -> FResult { write!(f, "{}", self.msg) }
/// }
/// impl Error for ExampleError {}
///
/// let err = ExampleError { msg: "Hello, world!".into() };
/// let fmt: ErrorTraceFormatter = err.trace();
/// assert_eq!(format!("{fmt}"), "Hello, world!");
/// ```
pub struct ErrorTraceFormatter<'s, 'e> {
    /// The message that is the main error message.
    msg: Cow<'s, str>,
    /// An optional nested error to format that is the first element in the tree.
    err: Option<&'e (dyn 'static + Error)>,
}
impl<'s, 'e> ErrorTraceFormatter<'s, 'e> {
    /// Builds a formatter for a given "anonymous error".
    ///
    /// This is useful for creating one-time error traces where you don't want to create the root type.
    ///
    /// For even more convenience, see the [`trace!`]-macro.
    ///
    /// # Arguments
    /// - `msg`: A message that is printed as "current error".
    /// - `err`: An optional error that, if any, will cause this formatter to start printing a trace based on the error's [`Error::source()`]-implementation.
    ///
    /// # Returns
    /// A new ErrorTraceFormatter ready to rock-n-roll.
    #[inline]
    pub fn new(msg: impl Into<Cow<'s, str>>, err: Option<&'e (dyn 'static + Error)>) -> Self { Self { msg: msg.into(), err } }
}
impl<'s, 'e> Display for ErrorTraceFormatter<'s, 'e> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Match on beautiness
        if f.alternate() {
            // Always print the thing
            write!(f, "{:#}", self.msg)?;

            // Print any deps if any
            if let Some(source) = self.err {
                // Write the thingy
                write!(f, "\n\nCaused by:")?;

                let mut source: Option<&dyn Error> = Some(source);
                while let Some(err) = source.take() {
                    // Print it
                    write!(f, "\n o {err:#}")?;
                    source = err.source();
                }

                // Write closing enters
                writeln!(f, "\n")?;
            }
        } else {
            // Always print the thing
            write!(f, "{}", self.msg)?;

            // Print any deps if any
            if let Some(source) = self.err {
                // Write the thingy
                write!(f, "\n\nCaused by:")?;

                let mut source: Option<&dyn Error> = Some(source);
                while let Some(err) = source.take() {
                    // Print it
                    write!(f, "\n o {err}")?;
                    source = err.source();
                }

                // Write closing enters
                writeln!(f, "\n")?;
            }
        }

        // Done!
        Ok(())
    }
}

/// Formats an error and all its dependencies using neat ANSI-colours if the formatter to which we're writing supports it.
///
/// Whether colours are enabled or not can be checked by [`console`]'s [`colors_enabled_stderr()`](console::colors_enabled_stderr()) function, and controlled by [`set_colors_enabled_stderr()`](console::set_colors_enabled_stderr()).
///
/// See [`ErrorTraceFormatter`] to do the same but without ANSI colours at all.
///
/// # Example
/// ```rust
/// # use std::error::Error;
/// # use std::fmt::{Display, Formatter, Result as FResult};
/// use error_trace::{ErrorTraceColourFormatter, ErrorTrace as _};
///
/// #[derive(Debug)]
/// struct ExampleError {
///     msg : String,
/// }
/// impl Display for ExampleError {
///     fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
///         write!(f, "{}", self.msg)
///     }
/// }
/// impl Error for ExampleError {}
///
/// let err = ExampleError { msg: "Hello, world!".into() };
/// let fmt: ErrorTraceColourFormatter = err.trace_coloured();
///
/// // Colours aren't visible here, because we're writing to a string; but try writing to stdout/stderr!
/// assert_eq!(format!("{fmt}"), "Hello, world!");
/// ```
#[cfg(feature = "colours")]
pub struct ErrorTraceColourFormatter<'s, 'e> {
    /// The message that is the main error message.
    msg: Cow<'s, str>,
    /// An optional nested error to format that is the first element in the tree.
    err: Option<&'e (dyn 'static + Error)>,
}
#[cfg(feature = "colours")]
impl<'s, 'e> ErrorTraceColourFormatter<'s, 'e> {
    /// Builds a formatter for a given "anonymous error".
    ///
    /// This is useful for creating one-time error traces where you don't want to create the root type.
    ///
    /// For even more convenience, see the [`trace!`]-macro.
    ///
    /// # Arguments
    /// - `msg`: A message that is printed as "current error".
    /// - `err`: An optional error that, if any, will cause this formatter to start printing a trace based on the error's [`Error::source()`]-implementation.
    ///
    /// # Returns
    /// A new ErrorTraceColourFormatter ready to rock-n-roll.
    #[inline]
    pub fn new(msg: impl Into<Cow<'s, str>>, err: Option<&'e (dyn 'static + Error)>) -> Self { Self { msg: msg.into(), err } }
}
#[cfg(feature = "colours")]
impl<'s, 'e> Display for ErrorTraceColourFormatter<'s, 'e> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Match on beautiness
        if f.alternate() {
            // Always print the thing
            write!(f, "{}", style(format!("{:#}", self.msg)).for_stderr().bold())?;

            // Print any deps if any
            if let Some(source) = self.err {
                // Write the thingy
                write!(f, "\n\n{}", style("Caused by:").for_stderr().red().bold())?;

                let mut source: Option<&dyn Error> = Some(source);
                while let Some(err) = source.take() {
                    // Print it
                    write!(f, "\n o {}", style(format!("{err:#}")).for_stderr().bold())?;
                    source = err.source();
                }

                // Write closing enters
                writeln!(f, "\n")?;
            }
        } else {
            // Always print the thing
            write!(f, "{}", style(&self.msg).for_stderr().bold())?;

            // Print any deps if any
            if let Some(source) = self.err {
                // Write the thingy
                write!(f, "\n\n{}", style("Caused by:").for_stderr().red().bold())?;

                let mut source: Option<&dyn Error> = Some(source);
                while let Some(err) = source.take() {
                    // Print it
                    write!(f, "\n o {}", style(err).for_stderr().bold())?;
                    source = err.source();
                }

                // Write closing enters
                writeln!(f, "\n")?;
            }
        }

        // Done!
        Ok(())
    }
}





/***** LIBRARY *****/
/// Allows one to write an error and all of its dependencies.
///
/// # Example
/// ```rust
/// use std::error::Error;
/// use std::fmt::{Display, Formatter, Result as FResult};
///
/// use error_trace::ErrorTrace as _;
///
/// #[derive(Debug)]
/// struct SomeError {
///     msg: String,
/// }
/// impl Display for SomeError {
///     fn fmt(&self, f: &mut Formatter<'_>) -> FResult { write!(f, "{}", self.msg) }
/// }
/// impl Error for SomeError {}
///
/// #[derive(Debug)]
/// struct HigherError {
///     msg:   String,
///     child: SomeError,
/// }
/// impl Display for HigherError {
///     fn fmt(&self, f: &mut Formatter<'_>) -> FResult { write!(f, "{}", self.msg) }
/// }
/// impl Error for HigherError {
///     fn source(&self) -> Option<&(dyn 'static + Error)> { Some(&self.child) }
/// }
///
///
///
/// let err = HigherError {
///     msg:   "Oh no, something went wrong!".into(),
///     child: SomeError { msg: "A specific reason".into() },
/// };
/// assert_eq!(
///     err.trace().to_string(),
///     r#"Oh no, something went wrong!
///
/// Caused by:
///  o A specific reason
///
/// "#
/// );
/// ```
pub trait ErrorTrace: Error {
    /// Returns a formatter for showing this Error and all its [source](Error::source())s.
    ///
    /// This function can be used similarly to [`Path::display()`](std::path::Path::display()), since its result
    /// implements both [`Debug`] and [`Display`].
    ///
    /// # Returns
    /// A new [`ErrorTraceFormatter`] that implements [`Debug`] and [`Display`].
    ///
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use std::fmt::{Display, Formatter, Result as FResult};
    /// #
    /// use error_trace::ErrorTrace as _;
    ///
    /// # #[derive(Debug)]
    /// # struct SomeError {
    /// #     msg : String,
    /// # }
    /// # impl Display for SomeError {
    /// #     fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
    /// #         write!(f, "{}", self.msg)
    /// #     }
    /// # }
    /// # impl Error for SomeError {}
    /// #
    /// # #[derive(Debug)]
    /// # struct HigherError {
    /// #     msg   : String,
    /// #     child : SomeError,
    /// # }
    /// # impl Display for HigherError {
    /// #     fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
    /// #         write!(f, "{}", self.msg)
    /// #     }
    /// # }
    /// # impl Error for HigherError {
    /// #     fn source(&self) -> Option<&(dyn 'static + Error)> {
    /// #         Some(&self.child)
    /// #     }
    /// # }
    /// #
    /// #
    /// #
    /// let err = HigherError { msg: "Oh no, something went wrong!".into(), child: SomeError { msg: "A specific reason".into() } };
    /// assert_eq!(err.trace().to_string(), r#"Oh no, something went wrong!
    ///
    /// Caused by:
    ///  o A specific reason
    ///
    /// "#);
    fn trace(&self) -> ErrorTraceFormatter;

    /// Returns a formatter for showing this Error and all its [source](Error::source())s with nice colours.
    ///
    /// This function can be used similarly to [`Path::display()`](std::path::Path::display()), since its result
    /// implements both [`Debug`] and [`Display`].
    ///
    /// # Returns
    /// A new [`ErrorTraceColourFormatter`] that implements [`Debug`] and [`Display`].
    ///
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use std::fmt::{Display, Formatter, Result as FResult};
    /// #
    /// use error_trace::ErrorTrace as _;
    ///
    /// # #[derive(Debug)]
    /// # struct SomeError {
    /// #     msg : String,
    /// # }
    /// # impl Display for SomeError {
    /// #     fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
    /// #         write!(f, "{}", self.msg)
    /// #     }
    /// # }
    /// # impl Error for SomeError {}
    /// #
    /// # #[derive(Debug)]
    /// # struct HigherError {
    /// #     msg   : String,
    /// #     child : SomeError,
    /// # }
    /// # impl Display for HigherError {
    /// #     fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
    /// #         write!(f, "{}", self.msg)
    /// #     }
    /// # }
    /// # impl Error for HigherError {
    /// #     fn source(&self) -> Option<&(dyn 'static + Error)> {
    /// #         Some(&self.child)
    /// #     }
    /// # }
    /// #
    /// #
    /// #
    /// let err = HigherError { msg: "Oh no, something went wrong!".into(), child: SomeError { msg: "A specific reason".into() } };
    /// assert_eq!(err.trace_coloured().to_string(), r#"Oh no, something went wrong!
    ///
    /// Caused by:
    ///  o A specific reason
    ///
    /// "#);
    #[cfg(feature = "colours")]
    fn trace_coloured(&self) -> ErrorTraceColourFormatter;
}
impl<T: ?Sized + Error> ErrorTrace for T {
    fn trace(&self) -> ErrorTraceFormatter { ErrorTraceFormatter { msg: Cow::Owned(self.to_string()), err: self.source() } }

    #[cfg(feature = "colours")]
    fn trace_coloured(&self) -> ErrorTraceColourFormatter { ErrorTraceColourFormatter { msg: Cow::Owned(self.to_string()), err: self.source() } }
}
