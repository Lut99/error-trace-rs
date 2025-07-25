#![cfg_attr(docsrs, feature(doc_cfg))]
//  LIB.rs
//    by Lut99
//
//  Created:
//    22 Sep 2023, 12:17:19
//  Last edited:
//    05 Nov 2024, 11:38:27
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
//!   If you enable the `colors`-feature, you can additionally print some neat colors:
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
//!   // Requires the `colors`-feature!
//!   eprintln!("{}", err.trace_colored());
//!   ```
//!   ![Showing the same error as above but with some errors](https://github.com/Lut99/error-trace-rs/raw/main/img/example_colors.png)
//!
//!   Finally, when used in a situation where you want to show a quick error but are sure to never
//!   needs its contents, you can use the [`toplevel!()`]-macro:
//!   ```rust
//!   use error_trace::toplevel;
//!
//!   // Do something that fails
//!   let err = std::str::from_utf8(&[0xFF]).unwrap_err();
//!
//!   // Format it with a one-time parent error
//!   eprintln!("{}", toplevel!(("Oh no, everything went wrong!"), err));
//!   ```
//!
//!   For users of the `colors`-feature, there is the associated [`toplevel_colored!()`]-macro:
//!   ```rust
//!   use error_trace::toplevel_colored;
//!
//!   // Do something that fails
//!   let err = std::str::from_utf8(&[0xFF]).unwrap_err();
//!
//!   // Format it with a one-time parent error
//!   eprintln!("{}", toplevel_colored!(("Oh no, everything went wrong!"), err));
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
//!   error-trace = { git = "https://github.com/Lut99/error-trace-rs", tag = "v4.0.0" }
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
//!   - `colors`: Enables the use of [`ErrorTrace::trace_colored()`].
//!   - `macros`: Enables the use of the [`toplevel!()`]- and [`toplevel_colored!()`]-macros.
//!   - `serde`: Implements `Deserialize` and `Serialize` for the [`FrozenTrace`]-structure.
//

// Modules
#[cfg(test)]
mod tests;

// Imports
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

#[cfg(feature = "colors")]
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
/// use error_trace::toplevel;
///
/// // Do something that fails
/// let err = std::str::from_utf8(&[0xFF]).unwrap_err();
///
/// // Format it with a one-time parent error
/// assert_eq!(
///     toplevel!(("Oh no, everything went wrong!"), err).to_string(),
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
/// use error_trace::toplevel;
///
/// // Do something that fails
/// let bytes: [u8; 1] = [0xFF];
/// let err = std::str::from_utf8(&bytes).unwrap_err();
///
/// // Format it with a one-time parent error
/// assert_eq!(
///     toplevel!(("Failed to parse '{:?}'", bytes.as_slice()), err).to_string(),
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
///     toplevel!(("Failed to parse '{bytes:?}'"), err).to_string(),
///     r#"Failed to parse '[255]'
///
/// Caused by:
///  o invalid utf-8 sequence of 1 bytes from index 0
///
/// "#
/// );
/// ```
#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[macro_export]
macro_rules! toplevel {
    (($($args:tt)*), $err:expr) => {
        $crate::ErrorTraceFormatter::new(format!($($args)*), Some(&$err))
    };
}

/// Creates a one-time [`ErrorTrace`]-compatible type from the given string, then calls [`trace_colored()`](ErrorTrace::trace_colored()) on it.
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
/// An [`ErrorTraceColorFormatter`] that can be displayed immediately.
///
/// # Example
/// ```rust
/// use error_trace::toplevel_colored;
///
/// // Do something that fails
/// let err = std::str::from_utf8(&[0xFF]).unwrap_err();
///
/// // Colours aren't visible here, because we're writing to a string; but try writing to stdout/stderr!
/// assert_eq!(
///     toplevel_colored!(("Oh no, everything went wrong!"), err).to_string(),
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
/// use error_trace::toplevel_colored;
///
/// // Do something that fails
/// let bytes: [u8; 1] = [0xFF];
/// let err = std::str::from_utf8(&bytes).unwrap_err();
///
/// // Format it with a one-time parent error
/// assert_eq!(
///     toplevel_colored!(("Failed to parse '{:?}'", bytes.as_slice()), err).to_string(),
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
///     toplevel_colored!(("Failed to parse '{bytes:?}'"), err).to_string(),
///     r#"Failed to parse '[255]'
///
/// Caused by:
///  o invalid utf-8 sequence of 1 bytes from index 0
///
/// "#
/// );
/// ```
#[cfg(all(feature = "colors", feature = "macros"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "colors", feature = "macros"))))]
#[macro_export]
macro_rules! toplevel_colored {
    (($($args:tt)*), $err:expr) => {
        $crate::ErrorTraceColorFormatter::new(format!($($args)*), Some(&$err))
    };
}





/***** FORMATTERS *****/
/// Formats an error and all its dependencies.
///
/// If you have the `colors`-feature enabled, then you can also use [`ErrorTraceColorFormatter`] to
/// do the same but with ANSI-colors.
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
pub struct ErrorTraceFormatter<'s, 'e1, 'e2> {
    /// The message that is the main error message.
    msg: Cow<'s, str>,
    /// An optional nested error to format that is the first element in the tree.
    err: Option<&'e1 (dyn 'e2 + Error)>,
}
impl<'s, 'e1, 'e2> ErrorTraceFormatter<'s, 'e1, 'e2> {
    /// Builds a formatter for a given "anonymous error".
    ///
    /// This is useful for creating one-time error traces where you don't want to create the root type.
    ///
    /// For even more convenience, see the [`toplevel!`]-macro.
    ///
    /// # Arguments
    /// - `msg`: A message that is printed as "current error".
    /// - `err`: An optional error that, if any, will cause this formatter to start printing a
    ///   trace based on the error's [`Error::source()`]-implementation.
    ///
    /// # Returns
    /// A new ErrorTraceFormatter ready to rock-n-roll.
    #[inline]
    pub fn new(msg: impl Into<Cow<'s, str>>, err: Option<&'e1 (dyn 'e2 + Error)>) -> Self { Self { msg: msg.into(), err } }
}
impl<'s, 'e1, 'e2> Display for ErrorTraceFormatter<'s, 'e1, 'e2> {
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

/// Formats an error and all its dependencies using neat ANSI-colors if the formatter to which
/// we're writing supports it.
///
/// Whether colors are enabled or not can be checked by [`console`]'s
/// [`colors_enabled_stderr()`](console::colors_enabled_stderr()) function, and controlled by
/// [`set_colors_enabled_stderr()`](console::set_colors_enabled_stderr()).
///
/// See [`ErrorTraceFormatter`] to do the same but without ANSI colors at all.
///
/// # Example
/// ```rust
/// # use std::error::Error;
/// # use std::fmt::{Display, Formatter, Result as FResult};
/// use error_trace::{ErrorTraceColorFormatter, ErrorTrace as _};
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
/// let fmt: ErrorTraceColorFormatter = err.trace_colored();
///
/// // Colours aren't visible here, because we're writing to a string; but try writing to stdout/stderr!
/// assert_eq!(format!("{fmt}"), "Hello, world!");
/// ```
#[cfg(feature = "colors")]
#[cfg_attr(docsrs, doc(cfg(feature = "colors")))]
pub struct ErrorTraceColorFormatter<'s, 'e1, 'e2> {
    /// The message that is the main error message.
    msg: Cow<'s, str>,
    /// An optional nested error to format that is the first element in the tree.
    err: Option<&'e1 (dyn 'e2 + Error)>,
}
#[cfg(feature = "colors")]
#[cfg_attr(docsrs, doc(cfg(feature = "colors")))]
impl<'s, 'e1, 'e2> ErrorTraceColorFormatter<'s, 'e1, 'e2> {
    /// Builds a formatter for a given "anonymous error".
    ///
    /// This is useful for creating one-time error traces where you don't want to create the root type.
    ///
    /// For even more convenience, see the [`toplevel!`]-macro.
    ///
    /// # Arguments
    /// - `msg`: A message that is printed as "current error".
    /// - `err`: An optional error that, if any, will cause this formatter to start printing a trace based on the error's [`Error::source()`]-implementation.
    ///
    /// # Returns
    /// A new ErrorTraceColourFormatter ready to rock-n-roll.
    #[inline]
    pub fn new(msg: impl Into<Cow<'s, str>>, err: Option<&'e1 (dyn 'e2 + Error)>) -> Self { Self { msg: msg.into(), err } }
}
#[cfg(feature = "colors")]
#[cfg_attr(docsrs, doc(cfg(feature = "colors")))]
impl<'s, 'e1, 'e2> Display for ErrorTraceColorFormatter<'s, 'e1, 'e2> {
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





/***** AUXILLARY *****/
/// A helper type that can be used to "freeze" a trace and then pass it on to a further error.
///
/// This is useful in case you're dealing with errors where you don't want to propagate the type
/// (e.g., due to lifetimes) but do want to propagate the trace.
///
/// # Example
/// ```rust
/// use std::error::Error;
/// use std::fmt::{Display, Formatter, Result as FResult};
///
/// use error_trace::{ErrorTrace as _, FrozenTrace};
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
///     FrozenTrace::new(err).trace().to_string(),
///     r#"Oh no, something went wrong!
///
/// Caused by:
///  o A specific reason
///
/// "#
/// );
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FrozenTrace {
    /// The error on this level.
    pub message: String,
    /// The error on the next level, if any.
    pub source:  Option<Box<Self>>,
}
impl FrozenTrace {
    /// Builds a new FrozenTrace from the given [`Error`].
    ///
    /// # Arguments
    /// - `err`: The error to walk and to freeze the errors of by serializing the messages.
    ///
    /// # Returns
    /// A new FrozenTrace that itself implements [`Error`] again.
    ///
    /// # Example
    /// See [`FrozenTrace`] itself for an example of how to use it, or see [`ErrorTrace::freeze()`].
    #[inline]
    pub fn new(err: impl Error) -> Self { Self { message: err.to_string(), source: err.source().map(|err| Box::new(Self::new(err))) } }

    /// Builds a new Trace from a single [`String`].
    ///
    /// # Arguments
    /// - `msg`: The (already serialized) message to wrap this trace around.
    ///
    /// # Returns
    /// A new Trace that wraps the `msg` implementing [`Error`].
    ///
    /// # Example
    /// ```rust
    /// use std::error::Error as _;
    ///
    /// use error_trace::{ErrorTrace as _, FrozenTrace};
    ///
    /// let trace = FrozenTrace::from_msg("Hello there!");
    /// assert_eq!(trace.trace().to_string(), "Hello there!");
    /// ```
    #[inline]
    pub fn from_msg(msg: impl Into<String>) -> Self { Self { message: msg.into(), source: None } }

    /// Builds a new Trace from a message and a source [`Error`].
    ///
    /// # Arguments
    /// - `msg`: Some toplevel to show as root cause.
    /// - `err`: The first error of the trace that causes `msg`.
    ///
    /// # Returns
    /// A new Trace that wraps the `msg` as error, with `err` as trace, and that implements [`Error`].
    #[inline]
    pub fn from_source(msg: impl Into<String>, err: impl Error) -> Self { Self { message: msg.into(), source: Some(Box::new(err.freeze())) } }

    /// Returns this Trace as an [`Error`] trait object.
    ///
    /// # Returns
    /// A [`&'static dyn Error`](Error) which is even static!
    #[inline]
    pub fn as_error(&self) -> &(dyn 'static + Error) { self }
}
impl Display for FrozenTrace {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult { write!(f, "{}", self.message) }
}
impl Error for FrozenTrace {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> { self.source.as_ref().map(|src| src.as_error()) }
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
    /// "Freezes" the trace of this error.
    ///
    /// This is useful in case you're dealing with errors where you don't want to propagate the
    /// type (e.g., due to lifetimes) but do want to propagate the trace.
    ///
    /// # Returns
    /// A [`FrozenTrace`] that returns the same trace when formatter, except all errors are
    /// serialized to [`String`]s.
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
    /// let err = HigherError {
    ///     msg:   "Oh no, something went wrong!".into(),
    ///     child: SomeError { msg: "A specific reason".into() },
    /// };
    /// assert_eq!(
    ///     err.freeze().trace().to_string(),
    ///     r#"Oh no, something went wrong!
    ///
    /// Caused by:
    ///  o A specific reason
    ///
    /// "#
    /// );
    /// ```
    fn freeze(&self) -> FrozenTrace;



    /// Returns a formatter for showing this Error and all its [source](Error::source())s.
    ///
    /// This function can be used similarly to [`Path::display()`](std::path::Path::display()),
    /// since its result implements [`Display`].
    ///
    /// # Returns
    /// A new [`ErrorTraceFormatter`] that implements [`Display`].
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

    /// Returns a formatter for showing this Error and all its [source](Error::source())s with nice colors.
    ///
    /// This function can be used similarly to [`Path::display()`](std::path::Path::display()),
    /// since its result implements [`Display`].
    ///
    /// # Returns
    /// A new [`ErrorTraceColorFormatter`] that implements [`Display`].
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
    /// assert_eq!(err.trace_colored().to_string(), r#"Oh no, something went wrong!
    ///
    /// Caused by:
    ///  o A specific reason
    ///
    /// "#);
    #[cfg(feature = "colors")]
    #[cfg_attr(docsrs, doc(cfg(feature = "colors")))]
    fn trace_colored(&self) -> ErrorTraceColorFormatter;
}
impl<T: ?Sized + Error> ErrorTrace for T {
    #[inline]
    fn freeze(&self) -> FrozenTrace { FrozenTrace::new(self) }

    #[inline]
    fn trace(&self) -> ErrorTraceFormatter { ErrorTraceFormatter { msg: Cow::Owned(self.to_string()), err: self.source() } }

    #[cfg(feature = "colors")]
    #[cfg_attr(docsrs, doc(cfg(feature = "colors")))]
    #[inline]
    fn trace_colored(&self) -> ErrorTraceColorFormatter { ErrorTraceColorFormatter { msg: Cow::Owned(self.to_string()), err: self.source() } }
}
