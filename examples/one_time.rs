//  ONE TIME.rs
//    by Lut99
//
//  Created:
//    17 Dec 2023, 17:32:27
//  Last edited:
//    17 Dec 2023, 18:06:14
//  Auto updated?
//    Yes
//
//  Description:
//!   Shows the usage of the [`trace!()`]-macro.
//

#[cfg(not(feature = "macros"))]
compile_error!("To run `one_time.rs`, please enable the `macros` feature");

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use error_trace::trace;


/***** ERRORS *****/
/// An error that we would like to wrap.
#[derive(Debug)]
struct NestedError;
impl Display for NestedError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult { write!(f, "Something occurred initially") }
}
impl Error for NestedError {}





/***** ENTRYPOINT *****/
fn main() {
    // Let us create an error that uses [`NestedError`] as a reason
    eprintln!("ERROR: {}", trace!(("Something occurred as a result of something else"), NestedError));

    // We can also use arguments
    let value: u32 = 42;
    eprintln!("ERROR: {}", trace!(("Cannot set to {value}"), NestedError));

    // If we have colour support, why not
    #[cfg(feature = "colours")]
    eprintln!("ERROR: {}", error_trace::trace_coloured!(("Something occurred as a result of something else"), NestedError));
}
