// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


/// # Summary
/// Convenience functions to round numbers to a specific magnitude or number of significant digits.
pub trait Round
{
    /// # Summary
    /// Rounds a number x to a specific magnitude m where x ≈ 10^m. Meaning if x shall be rounded to whole numbers, magnitude is 0. If x shall be rounded to 10s, magnitude is 1. If x shall be rounded to 0.1s, magnitude is -1.
    ///
    /// # Arguments
    /// - `magnitude`: the magnitude to round to
    ///
    /// # Returns
    /// - the rounded number
    ///
    /// # Examples
    /// ```
    /// use scaler::round::Round;
    /// let x: f64 = 42.069;
    /// assert_eq!(x.round_mag(-4), 42.069);
    /// assert_eq!(x.round_mag(-3), 42.069);
    /// assert_eq!(x.round_mag(-2), 42.07);
    /// assert_eq!(x.round_mag(-1), 42.1);
    /// assert_eq!(x.round_mag(0), 42.0);
    /// assert_eq!(x.round_mag(1), 40.0);
    /// assert_eq!(x.round_mag(2), 0.0);
    /// ```
    /// ```
    /// use scaler::round::Round;
    /// let x: f64 = 0.5;
    /// assert_eq!(x.round_mag(0), 0.0);
    /// let x: f64 = 1.5;
    /// assert_eq!(x.round_mag(0), 2.0);
    /// ```
    fn round_mag(&self, magnitude: i16) -> Self;


    /// # Summary
    /// Rounds a number x to a specific number of significant digits. This is useful for formatting numbers to a certain precision irrespective of the magnitude.
    ///
    /// # Arguments
    /// - `significants`: the number of significant digits to round to, rounding to 0 significant digits always returns 0
    ///
    /// # Returns
    /// - the rounded number
    ///
    /// # Examples
    /// ```
    /// use scaler::round::Round;
    /// let x: f64 = 123.45;
    /// assert_eq!(x.round_sig(0), 0.0);
    /// assert_eq!(x.round_sig(1), 100.0);
    /// assert_eq!(x.round_sig(2), 120.0);
    /// assert_eq!(x.round_sig(3), 123.0);
    /// assert_eq!(x.round_sig(4), 123.4);
    /// assert_eq!(x.round_sig(5), 123.45);
    /// assert_eq!(x.round_sig(6), 123.450);
    /// ```
    ///
    /// ```
    /// use scaler::round::Round;
    /// let x: f64 = 0.789;
    /// assert_eq!(x.round_sig(0), 0.0);
    /// assert_eq!(x.round_sig(1), 0.8);
    /// assert_eq!(x.round_sig(2), 0.79);
    /// assert_eq!(x.round_sig(3), 0.789);
    /// assert_eq!(x.round_sig(4), 0.7890);
    /// ```
    fn round_sig(&self, significants: u8) -> Self;
}


impl Round for f64 // TODO implement for all number types
{
    fn round_mag(&self, magnitude: i16) -> Self
    {
        let x_rounded: Self;


        if *self == 0 as Self
        // rounded 0 is always 0
        {
            return 0 as Self;
        }


        x_rounded = (*self * Self::powi(10 as Self, (-magnitude).into())).round_ties_even() * Self::powi(10 as Self, magnitude.into()); // multiply by 10^(-magnitude), round, multiply by 10^(magnitude)

        return x_rounded;
    }


    fn round_sig(&self, significants: u8) -> Self
    {
        let magnitude: i16;
        let x_rounded: Self;


        if *self == 0 as Self || significants == 0
        // rounded 0 or rounded to 0 significants is always 0
        {
            return 0 as Self;
        }


        magnitude = self.abs().log10().floor() as i16; // current magnitude of x
        x_rounded = self.round_mag(magnitude - i16::from(significants) + 1); // round to significants

        return x_rounded;
    }
}
