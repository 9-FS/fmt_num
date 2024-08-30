// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Rounding
{
    Magnitude(i16),        // round statically to digit at 10^n, contains precision n
    SignificantDigits(u8), // round dynamically to n significant numbers, contains precision n
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Scaling
{
    Binary(bool),  // scaling by 2^10 = 1.024 until no more prefixes, then fallback to scientific notation, contains whether or not to put space between number and unit prefix
    Decimal(bool), // scaling by 10^3 = 1.000 until no more prefixes, then fallback to scientific notation, contains whether or not to put space between number and unit prefix
    None,          // no scaling, no fallback to scientific notation
    Scientific,    // always scientific notation
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sign
{
    Always,    // always show sign
    OnlyMinus, // only show sign when negative
}
