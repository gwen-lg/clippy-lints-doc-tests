#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub enum Error {
    DivByZero,
}
/// Divide 100 by the value indicated
pub fn div_hundred_by(value: u32) -> Result<u32, Error> {
    if value == 0 {
        return Err(Error::DivByZero);
    }
    Ok(1000 / value)
}
/// Divide 1000 by the value indicated
pub fn div_thousand_by(value: u32) -> Result<u32, Error> {
    {
        if value == 0 {
            return Err(Error::DivByZero);
        }
        Ok(1000 / value)
    }
}
/// Divide 1000 by the value indicated
pub fn div_thousand_by_2(value: u32) -> Result<u32, Error> {
    {
        if value == 0 {
            return Err(Error::DivByZero);
        }
        Ok(1000 / value)
    }
}
/// Divide 10_000 by the value indicated
///
/// # Errors
/// Will return [`Error::DivByZero`] if `value` is equal to `zero`.
pub fn div_ten_thousand_by(value: u32) -> Result<u32, Error> {
    if value == 0 {
        return Err(Error::DivByZero);
    }
    Ok(10_000 / value)
}
