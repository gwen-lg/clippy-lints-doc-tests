pub enum Error {
    DivByZero,
}

/// Divide 100 by the value indicated
/// => Reference which trigger the lint.
pub fn div_hundred_by(value: u32) -> Result<u32, Error> {
    if value == 0 {
        return Err(Error::DivByZero);
    }
    Ok(100 / value)
}

/// Divide 1000 by the value indicated
#[profiling::function]
pub fn div_thousand_by(value: u32) -> Result<u32, Error> {
    if value == 0 {
        return Err(Error::DivByZero);
    }
    Ok(1000 / value)
}

/// Divide 10_000 by the value indicated
///
/// # Errors
/// Will return [`Error::DivByZero`] if `value` is equal to `zero`.
///
/// => variation with Errors doc, no lint to trigger.
pub fn div_ten_thousand_by(value: u32) -> Result<u32, Error> {
    if value == 0 {
        return Err(Error::DivByZero);
    }
    Ok(10_000 / value)
}
