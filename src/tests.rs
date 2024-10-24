//  TESTS.rs
//    by Lut99
//
//  Created:
//    24 Oct 2024, 13:19:12
//  Last edited:
//    24 Oct 2024, 13:22:22
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements some tests.
//

use super::*;


/***** LIBRARY *****/
#[cfg(feature = "macros")]
#[test]
fn test_static_trace() {
    use std::error::Error;

    #[allow(unused)]
    trait Test {
        type Error: Error;

        fn failure(&self) -> Self::Error;
    }

    #[allow(unused)]
    fn test(test: impl Test) {
        // Let's try to trace the resulting error
        eprintln!("{}", trace!(crate("Failed to test"), test.failure()));
    }
}
