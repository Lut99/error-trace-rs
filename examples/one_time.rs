//  ONE TIME.rs
//    by Lut99
//
//  Created:
//    17 Dec 2023, 17:32:27
//  Last edited:
//    17 Dec 2023, 17:37:11
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

    // If we have colour support, why not
    #[cfg(feature = "colours")]
    eprintln!("ERROR: {}", error_trace::trace_coloured!(("Something occurred as a result of something else"), NestedError));
}
