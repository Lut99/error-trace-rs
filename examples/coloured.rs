//  COLOURED.rs
//    by Lut99
// 
//  Created:
//    22 Sep 2023, 12:49:33
//  Last edited:
//    22 Sep 2023, 12:50:01
//  Auto updated?
//    Yes
// 
//  Description:
//!   Shows how one might use the `coloured`-feature of the
//!   [`error_trace`]-crate.
// 

#[cfg(not(feature = "colours"))]
compile_error!("To run `colours.rs`, please enable the `colours` feature");

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use error_trace::ErrorTrace as _;



/***** ERRORS *****/
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




/***** ENTRYPOINT *****/
fn main() {
    // Create the tiered error
    let err = HigherError {
        msg: "Oh no, something went wrong!".into(),
        child: SomeError {
            msg: "A specific reason".into(),
        }
    };

    // Show it nicely!
    eprintln!("{}", err.trace_coloured());
}
