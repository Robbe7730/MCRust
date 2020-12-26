pub mod clientbound;
pub mod serverbound;
pub mod packet_reader;
pub mod packet_writer;

use clientbound::*;
use serverbound::*;

#[macro_export]
macro_rules! expect_equal {
    ( $actual:expr, $expected:expr ) => {{
        if ($actual != $expected) {
            Err(ErrorType::Fatal(format!("Expected {} but got {}", $expected, $actual)))?
        } else {
            $expected
        }
    }};
}

