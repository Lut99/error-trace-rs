//  LIB.rs
//    by Lut99
//
//  Created:
//    22 Sep 2023, 12:17:19
//  Last edited:
//    17 Dec 2023, 18:00:55
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

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};

#[cfg(feature = "colours")]
use console::style;


/***** MACROS *****/
/// Creates a one-time [`ErrorTrace`]-compatible type from the given string, then calls [`trace()`](ErrorTrace::trace()) on it.
///
/// # Arguments
/// The macro has the following signature:
/// ```plain
/// ($fmt:literal $(, $args:expr)*), $err:expr
/// ```
/// - `$fmt:literal`: The format string (as a string literal) to use to generate the message.
/// - `$(, $args:expr)*`: Zero or more arguments for the format string that can be used to build the message.
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
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! trace {
    (($fmt:literal $(, $args:tt)*), $err:expr) => {
        {
            // Build the one-time type
            #[derive(::std::fmt::Debug)]
            struct _OneTimeError<E>(::std::string::String, E);
            impl<E> ::std::fmt::Display for _OneTimeError<E> {
                #[inline]
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { ::std::write!(f, "{}", self.0) }
            }
            impl<E: 'static + ::std::error::Error> ::std::error::Error for _OneTimeError<E> {
                #[inline]
                fn source(&self) -> ::std::option::Option<&(dyn 'static + ::std::error::Error)> { Some(&self.1) }
            }

            // Populate it, then trace
            <_OneTimeError<_> as ::error_trace::ErrorTrace>::trace(&_OneTimeError(format!($fmt $(, $args)*), $err))
        }
    };
}

/// Creates a one-time [`ErrorTrace`]-compatible type from the given string, then calls [`trace_coloured()`](ErrorTrace::trace_coloured()) on it.
///
/// # Arguments
/// The macro has the following signature:
/// ```plain
/// ($fmt:literal $(, $args:expr)*), $err:expr
/// ```
/// - `$fmt:literal`: The format string (as a string literal) to use to generate the message.
/// - `$(, $args:expr)*`: Zero or more arguments for the format string that can be used to build the message.
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
#[cfg(all(feature = "colours", feature = "macros"))]
#[macro_export]
macro_rules! trace_coloured {
    (($fmt:literal $(, $args:tt)*), $err:expr) => {
        {
            // Build the one-time type
            #[derive(::std::fmt::Debug)]
            struct _OneTimeError<E>(::std::string::String, E);
            impl<E> ::std::fmt::Display for _OneTimeError<E> {
                #[inline]
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result { ::std::write!(f, "{}", self.0) }
            }
            impl<E: 'static + ::std::error::Error> ::std::error::Error for _OneTimeError<E> {
                #[inline]
                fn source(&self) -> ::std::option::Option<&(dyn 'static + ::std::error::Error)> { Some(&self.1) }
            }

            // Populate it, then trace
            <_OneTimeError<_> as ::error_trace::ErrorTrace>::trace_coloured(&_OneTimeError(format!($fmt $(, $args)*), $err))
        }
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
/// let fmt: ErrorTraceFormatter<ExampleError> = err.trace();
/// assert_eq!(format!("{fmt}"), "Hello, world!");
/// assert_eq!(format!("{fmt:?}"), "ExampleError { msg: \"Hello, world!\" }");
/// ```
pub struct ErrorTraceFormatter<'e, E: ?Sized> {
    /// The error to format.
    err: &'e E,
}
impl<'e, E: Error> Debug for ErrorTraceFormatter<'e, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Match on beautiness
        if f.alternate() {
            // Always print the thing
            write!(f, "{:#?}", self.err)?;

            // Print any deps if any
            if let Some(source) = self.err.source() {
                // Write the thingy
                write!(f, "\n\nCaused by:")?;

                let mut source: Option<&dyn Error> = Some(source);
                while let Some(err) = source.take() {
                    // Print it
                    write!(f, "\n o {err:#?}")?;
                    source = err.source();
                }

                // Write closing enters
                writeln!(f, "\n")?;
            }
        } else {
            // Always print the thing
            write!(f, "{:?}", self.err)?;

            // Print any deps if any
            if let Some(source) = self.err.source() {
                // Write the thingy
                write!(f, "\n\nCaused by:")?;

                let mut source: Option<&dyn Error> = Some(source);
                while let Some(err) = source.take() {
                    // Print it
                    write!(f, "\n o {err:?}")?;
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
impl<'e, E: Error> Display for ErrorTraceFormatter<'e, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Match on beautiness
        if f.alternate() {
            // Always print the thing
            write!(f, "{:#}", self.err)?;

            // Print any deps if any
            if let Some(source) = self.err.source() {
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
            write!(f, "{}", self.err)?;

            // Print any deps if any
            if let Some(source) = self.err.source() {
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
/// let fmt: ErrorTraceColourFormatter<ExampleError> = err.trace_coloured();
///
/// // Colours aren't visible here, because we're writing to a string; but try writing to stdout/stderr!
/// assert_eq!(format!("{fmt}"), "Hello, world!");
/// assert_eq!(format!("{fmt:?}"), "ExampleError { msg: \"Hello, world!\" }");
/// ```
#[cfg(feature = "colours")]
pub struct ErrorTraceColourFormatter<'e, E: ?Sized> {
    /// The error to format.
    err: &'e E,
}
#[cfg(feature = "colours")]
impl<'e, E: Error> Debug for ErrorTraceColourFormatter<'e, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Match on beautiness
        if f.alternate() {
            // Always print the thing
            write!(f, "{}", style(format!("{:#?}", self.err)).for_stderr().bold())?;

            // Print any deps if any
            if let Some(source) = self.err.source() {
                // Write the thingy
                write!(f, "\n\n{}", style("Caused by:").for_stderr().red().bold())?;

                let mut source: Option<&dyn Error> = Some(source);
                while let Some(err) = source.take() {
                    // Print it
                    write!(f, "\n o {}", style(format!("{err:#?}")).for_stderr().bold())?;
                    source = err.source();
                }

                // Write closing enters
                writeln!(f, "\n")?;
            }
        } else {
            // Always print the thing
            write!(f, "{}", style(format!("{:?}", self.err)).for_stderr().bold())?;

            // Print any deps if any
            if let Some(source) = self.err.source() {
                // Write the thingy
                write!(f, "\n\n{}", style("Caused by:").for_stderr().red().bold())?;

                let mut source: Option<&dyn Error> = Some(source);
                while let Some(err) = source.take() {
                    // Print it
                    write!(f, "\n o {}", style(format!("{err:?}")).for_stderr().bold())?;
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
#[cfg(feature = "colours")]
impl<'e, E: Error> Display for ErrorTraceColourFormatter<'e, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Match on beautiness
        if f.alternate() {
            // Always print the thing
            write!(f, "{}", style(format!("{:#}", self.err)).for_stderr().bold())?;

            // Print any deps if any
            if let Some(source) = self.err.source() {
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
            write!(f, "{}", style(self.err).for_stderr().bold())?;

            // Print any deps if any
            if let Some(source) = self.err.source() {
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
    fn trace(&self) -> ErrorTraceFormatter<Self>;

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
    fn trace_coloured(&self) -> ErrorTraceColourFormatter<Self>;
}
impl<T: ?Sized + Error> ErrorTrace for T {
    fn trace(&self) -> ErrorTraceFormatter<Self> { ErrorTraceFormatter { err: self } }

    #[cfg(feature = "colours")]
    fn trace_coloured(&self) -> ErrorTraceColourFormatter<Self> { ErrorTraceColourFormatter { err: self } }
}
