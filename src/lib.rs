//  LIB.rs
//    by Lut99
// 
//  Created:
//    22 Sep 2023, 12:17:19
//  Last edited:
//    22 Sep 2023, 12:46:56
//  Auto updated?
//    Yes
// 
//  Description:
//!   Small Rust crate for printing nice errors traits based on
//!   [`Error::source()`].
// 

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FResult};

#[cfg(feature = "colours")]
use console::style;


/***** FORMATTERS *****/
/// Formats an error and all its dependencies.
/// 
/// If you have the `colours`-feature enabled, then you can also use [`ErrorTraceColourFormatter`] to do the same but with colours.
/// 
/// # Example
/// ```rust
/// # use std::error::Error;
/// # use std::fmt::{Display, Formatter, Result as FResult};
/// use error_trace::{ErrorTraceFormatter, ErrorTrace as _};
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
/// let fmt: ErrorTraceFormatter<ExampleError> = err.trace();
/// assert_eq!(format!("{fmt}"), "Hello, world!");
/// assert_eq!(format!("{fmt:?}"), "ExampleError { msg: \"Hello, world!\" }");
/// ```
pub struct ErrorTraceFormatter<'e, E: ?Sized> {
    /// The error to format.
    err : &'e E,
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

/// Formats an error and all its dependencies.
/// 
/// If you have the `colours`-feature enabled, then you can also use [`ErrorTraceColourFormatter`] to do the same but with colours.
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
/// assert_eq!(format!("{fmt}"), "error: Hello, world!");
/// assert_eq!(format!("{fmt:?}"), "error: ExampleError { msg: \"Hello, world!\" }");
/// ```
#[cfg(feature = "colours")]
pub struct ErrorTraceColourFormatter<'e, E: ?Sized> {
    /// The error to format.
    err : &'e E,
}
#[cfg(feature = "colours")]
impl<'e, E: Error> Debug for ErrorTraceColourFormatter<'e, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Match on beautiness
        if f.alternate() {
            // Always print the thing
            write!(f, "{}: {}", style("error").red().bold(), style(format!("{:#?}", self.err)).bold())?;

            // Print any deps if any
            if let Some(source) = self.err.source() {
                // Write the thingy
                write!(f, "\n\n{}", style("Caused by:").red().bold())?;

                let mut source: Option<&dyn Error> = Some(source);
                while let Some(err) = source.take() {
                    // Print it
                    write!(f, "\n o {}", style(format!("{err:#?}")).bold())?;
                    source = err.source();
                }

                // Write closing enters
                writeln!(f, "\n")?;
            }
        } else {
            // Always print the thing
            write!(f, "{}: {}", style("error").red().bold(), style(format!("{:?}", self.err)).bold())?;

            // Print any deps if any
            if let Some(source) = self.err.source() {
                // Write the thingy
                write!(f, "\n\n{}", style("Caused by:").red().bold())?;

                let mut source: Option<&dyn Error> = Some(source);
                while let Some(err) = source.take() {
                    // Print it
                    write!(f, "\n o {}", style(format!("{err:?}")).bold())?;
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
            write!(f, "{}: {}", style("error").red().bold(), style(format!("{:#}", self.err)).bold())?;

            // Print any deps if any
            if let Some(source) = self.err.source() {
                // Write the thingy
                write!(f, "\n\n{}", style("Caused by:").red().bold())?;

                let mut source: Option<&dyn Error> = Some(source);
                while let Some(err) = source.take() {
                    // Print it
                    write!(f, "\n o {}", style(format!("{err:#}")).bold())?;
                    source = err.source();
                }

                // Write closing enters
                writeln!(f, "\n")?;
            }
        } else {
            // Always print the thing
            write!(f, "{}: {}", style("error").red().bold(), style(self.err).bold())?;

            // Print any deps if any
            if let Some(source) = self.err.source() {
                // Write the thingy
                write!(f, "\n\n{}", style("Caused by:").red().bold())?;

                let mut source: Option<&dyn Error> = Some(source);
                while let Some(err) = source.take() {
                    // Print it
                    write!(f, "\n o {}", style(err).bold())?;
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
///     msg : String,
/// }
/// impl Display for SomeError {
///     fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
///         write!(f, "{}", self.msg)
///     }
/// }
/// impl Error for SomeError {}
/// 
/// #[derive(Debug)]
/// struct HigherError {
///     msg   : String,
///     child : SomeError,
/// }
/// impl Display for HigherError {
///     fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
///         write!(f, "{}", self.msg)
///     }
/// }
/// impl Error for HigherError {
///     fn source(&self) -> Option<&(dyn 'static + Error)> {
///         Some(&self.child)
///     }
/// }
/// 
/// 
/// 
/// let err = HigherError { msg: "Oh no, something went wrong!".into(), child: SomeError { msg: "A specific reason".into() } };
/// assert_eq!(err.trace().to_string(), r#"Oh no, something went wrong!
///
/// Caused by:
///  o A specific reason
///
/// "#);
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
    /// assert_eq!(err.trace_coloured().to_string(), r#"error: Oh no, something went wrong!
    ///
    /// Caused by:
    ///  o A specific reason
    ///
    /// "#);
    #[cfg(feature = "colours")]
    fn trace_coloured(&self) -> ErrorTraceColourFormatter<Self>;
}
impl<T: ?Sized + Error> ErrorTrace for T {
    fn trace(&self) -> ErrorTraceFormatter<Self> {
        ErrorTraceFormatter {
            err : self,
        }
    }

    #[cfg(feature = "colours")]
    fn trace_coloured(&self) -> ErrorTraceColourFormatter<Self> {
        ErrorTraceColourFormatter {
            err : self,
        }
    }
}
