// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
mod format;
// mod from_str;
pub mod options;
pub use options::*;
pub mod round;
pub use round::*;


/// # Summary
/// A convenient formatter to scale, round, and display numbers. More information about available options and can be found at the setter functions and the format function itself.
#[derive(Debug, PartialEq)]
pub struct Formatter
{
    decimal_separator: String,
    group_separator:   String,
    rounding:          Rounding,
    scaling:           Scaling,
    sign:              Sign,
}


impl Formatter
{
    /// # Summary
    /// Constructs default Formatter with only sign when negative, decimal scaling, rounding to 4 significant digits, "." as thousand separator, and "," as decimal separator.
    ///
    /// # Returns
    /// - Formatter
    pub fn new() -> Self
    {
        return Self {
            decimal_separator: ",".to_string(),
            group_separator:   ".".to_string(),
            rounding:          Rounding::SignificantDigits(4),
            scaling:           Scaling::Decimal(true),
            sign:              Sign::OnlyMinus,
        };
    }


    /// # Summary
    /// Sets the group and decimal separator. Warns if decimal separator is empty or if they are the same.
    ///
    /// # Arguments
    /// - `group_separator`: new group separator, a string that separates groups every 3 digits before the decimal separator
    /// - `decimal_separator`: new decimal separator, a string that separates the integer and fractional parts of a number
    ///
    /// # Returns
    /// - modified self
    pub fn set_separators(mut self, group_separator: &str, decimal_separator: &str) -> Self
    {
        self.group_separator = group_separator.to_string();
        self.decimal_separator = decimal_separator.to_string();

        if self.decimal_separator == ""
        {
            log::warn!("Decimal separator is empty. This may lead to ambiguous formatting.");
        }
        else if self.decimal_separator == self.group_separator
        {
            log::warn!(
                "Group separator \"{}\" and decimal separator \"{}\" are the same. This may lead to ambiguous formatting.",
                self.group_separator,
                self.decimal_separator
            );
        }

        return self;
    }


    /// # Summary
    /// Sets the rounding mode and precision.
    ///
    /// # Arguments
    /// - `rounding_mode`: new rounding mode, contains precision
    ///     - Magnitude: round statically to digit at 10^n, contains precision n
    ///     - SignificantDigits: round dynamically to n significant numbers, contains precision n
    ///
    /// # Returns
    /// - modified self
    pub fn set_rounding(mut self, rounding: Rounding) -> Self
    {
        self.rounding = rounding;
        return self;
    }


    /// # Summary
    /// Sets the scaling mode.
    ///
    /// # Arguments
    /// - `scaling`: new scaling mode
    ///     - Binary: scaling by 2^10 = 1.024 until no more prefixes, then fallback to scientific notation, contains whether or not to put space between number and unit prefix
    ///     - Decimal: scaling by 10^3 = 1.000 until no more prefixes, then fallback to scientific notation, contains whether or not to put space between number and unit prefix
    ///     - None: no scaling, no fallback to scientific notation
    ///     - Scientific: always scientific notation
    ///
    /// # Returns
    /// - modified self
    pub fn set_scaling(mut self, scaling: Scaling) -> Self
    {
        self.scaling = scaling;
        return self;
    }


    /// # Summary
    /// Sets the sign mode.
    ///
    /// # Arguments
    /// - `sign`: new sign mode
    ///     - Always: always show sign
    ///     - OnlyMinus: only show sign when negative
    ///
    /// # Returns
    /// - modified self
    pub fn set_sign(mut self, sign: Sign) -> Self
    {
        self.sign = sign;
        return self;
    }
}


impl Default for Formatter
{
    /// # Summary
    /// Constructs default Formatter with only sign when negative, decimal scaling, rounding to 4 significant digits, "." as thousand separator, and "," as decimal separator.
    ///
    /// # Returns
    /// - default Formatter
    fn default() -> Self
    {
        return Self::new();
    }
}


// inspiration: https://github.com/kurtlawrence/numfmt/
