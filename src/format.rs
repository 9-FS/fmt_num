// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use crate::*;


impl Formatter
{
    /// # Summary
    /// Intermediate collection of formatting options to then scale, round, and display numbers.
    ///
    /// # Arguments
    /// - `x`: the number to format
    ///     - must be copy convertable to f64, from into expects lossless conversion
    ///     - lossy conversion must be explicitly handled by caller beforehand to avoid unexpected behaviour
    ///
    /// # Returns
    /// - the formatted number
    ///
    /// # Examples
    /// ## Practical Formatters
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new().set_rounding(scaler::Rounding::SignificantDigits(2)); // general display
    /// assert_eq!(f.format(123), "120");
    /// assert_eq!(f.format(4.56), "4,6");
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new(); // calculation results
    /// assert_eq!(f.format(456789), "456,8 k");
    /// assert_eq!(f.format(0.1), "100,0 m");
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new().set_scaling(scaler::Scaling::None).set_rounding(scaler::Rounding::Magnitude(0)); // absolute values
    /// assert_eq!(f.format(0.1), "0");
    /// assert_eq!(f.format(1), "1");
    /// assert_eq!(f.format(1000), "1.000");
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new().set_scaling(scaler::Scaling::Binary(true)); // data sizes
    /// assert_eq!(f.format(0.1), "1,600 * 2^(-4)");
    /// assert_eq!(f.format(1023), "1.023");
    /// assert_eq!(f.format(1024), "1,000 Ki");
    /// ```
    ///
    /// ## Sign
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new().set_sign(scaler::Sign::OnlyMinus);
    /// assert_eq!(f.format(-1), "-1,000");
    /// assert_eq!(f.format(0), "0,000");
    /// assert_eq!(f.format(1), "1,000");
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new().set_sign(scaler::Sign::Always);
    /// assert_eq!(f.format(-1), "-1,000");
    /// assert_eq!(f.format(0), "+0,000");
    /// assert_eq!(f.format(1), "+1,000");
    /// ```
    ///
    /// ## Scaling
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new().set_scaling(scaler::Scaling::Binary(true));
    /// assert_eq!(f.format(2_f64.powi(-10)), "1,000 * 2^(-10)");
    /// assert_eq!(f.format(2), "2,000");
    /// assert_eq!(f.format(1023), "1.023");
    /// assert_eq!(f.format(1024), "1,000 Ki");
    /// assert_eq!(f.format(2_f64.powi(10)), "1,000 Ki");
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new().set_scaling(scaler::Scaling::Decimal(true));
    /// assert_eq!(f.format(1e-31), "1,000 * 10^(-31)");
    /// assert_eq!(f.format(1e-3), "1,000 m");
    /// assert_eq!(f.format(10), "10,00");
    /// assert_eq!(f.format(1e3), "1,000 k");
    /// assert_eq!(f.format(1e33), "1,000 * 10^(33)");
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new().set_scaling(scaler::Scaling::None);
    /// assert_eq!(f.format(1e-10), "0,0000000001000");
    /// assert_eq!(f.format(0.1), "0,1000");
    /// assert_eq!(f.format(1), "1,000");
    /// assert_eq!(f.format(1000), "1.000");
    /// assert_eq!(f.format(1e10), "10.000.000.000");
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new().set_scaling(scaler::Scaling::Scientific);
    /// assert_eq!(f.format(1e-1), "1,000 * 10^(-1)");
    /// assert_eq!(f.format(1), "1,000 * 10^(0)");
    /// assert_eq!(f.format(1e3), "1,000 * 10^(3)");
    /// ```
    ///
    /// ## Rounding Mode and Rounding Precision
    /// ### Rounding Mode Magnitude
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new().set_rounding(scaler::Rounding::Magnitude(-10));
    /// assert_eq!(f.format(0.000000123456789), "123,5 n");
    /// assert_eq!(f.format(123.45), "123,4500000000");
    /// assert_eq!(f.format(0.9), "900,0000000 m");
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new().set_rounding(scaler::Rounding::Magnitude(-1));
    /// assert_eq!(f.format(123456), "123,4560 k");
    /// assert_eq!(f.format(123.456), "123,5");
    /// assert_eq!(f.format(0.9), "900 m");
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new().set_rounding(scaler::Rounding::Magnitude(0));
    /// assert_eq!(f.format(123456), "123,456 k");
    /// assert_eq!(f.format(123.456), "123");
    /// assert_eq!(f.format(0.9), "1");
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new().set_rounding(scaler::Rounding::Magnitude(1));
    /// assert_eq!(f.format(123456), "123,46 k");
    /// assert_eq!(f.format(123.456), "120");
    /// assert_eq!(f.format(0.9), "0");
    /// ```
    ///
    /// ### Rounding Mode Significant Digits
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new().set_rounding(scaler::Rounding::SignificantDigits(0));
    /// assert_eq!(f.format(123456), "0");
    /// assert_eq!(f.format(123.456), "0");
    /// assert_eq!(f.format(0.9), "0");
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new().set_rounding(scaler::Rounding::SignificantDigits(1));
    /// assert_eq!(f.format(123456), "100 k");
    /// assert_eq!(f.format(123.456), "100");
    /// assert_eq!(f.format(0.9), "900 m");
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new().set_rounding(scaler::Rounding::SignificantDigits(10));
    /// assert_eq!(f.format(123456), "123,4560000 k");
    /// assert_eq!(f.format(123.456), "123,4560000");
    /// assert_eq!(f.format(0.9), "900,0000000 m");
    /// ```
    ///
    /// ## Separators
    /// ```
    /// let f: scaler::Formatter = scaler::Formatter::new().set_scaling(scaler::Scaling::None).set_separators(".", ",");
    /// assert_eq!(f.format(123456), "123.500");
    /// assert_eq!(f.format(123.456), "123,5");
    /// assert_eq!(f.format(0.9), "0,9000");
    ///
    ///
    /// let f: scaler::Formatter = scaler::Formatter::new().set_scaling(scaler::Scaling::None).set_separators(",", ".");
    /// assert_eq!(f.format(123456), "123,500");
    /// assert_eq!(f.format(123.456), "123.5");
    /// assert_eq!(f.format(0.9), "0.9000");
    /// ```
    pub fn format<T>(&self, x: T) -> String
    where
        T: Clone + Into<f64>, // T must be copy convertable to f64
    {
        const BINARY_PREFIXES: [(i16, i16, &str); 9] = [
            (0, 10, ""),
            (10, 20, "Ki"),
            (20, 30, "Mi"),
            (30, 40, "Gi"),
            (40, 50, "Ti"),
            (50, 60, "Pi"),
            (60, 70, "Ei"),
            (70, 80, "Zi"),
            (80, 90, "Yi"),
        ]; // unit prefixes for binary mode, [lower bound magnitude; upper bound magnitude[, unit prefix
        const DECIMAL_PREFIXES: [(i16, i16, &str); 21] = [
            (-30, -27, "q"),
            (-27, -24, "r"),
            (-24, -21, "y"),
            (-21, -18, "z"),
            (-18, -15, "a"),
            (-15, -12, "f"),
            (-12, -9, "p"),
            (-9, -6, "n"),
            (-6, -3, "µ"),
            (-3, 0, "m"),
            (0, 3, ""),
            (3, 6, "k"),
            (6, 9, "M"),
            (9, 12, "G"),
            (12, 15, "T"),
            (15, 18, "P"),
            (18, 21, "E"),
            (21, 24, "Z"),
            (24, 27, "Y"),
            (27, 30, "R"),
            (30, 33, "Q"),
        ]; // SI unit prefixes for decimal mode, [lower bound magnitude; upper bound magnitude[, unit prefix
        let mut dec_places: i16; // number of decimal places to use, i16 instead of u16 to allow negative values during intermediate steps
        let magnitude: f64; // magnitude of the number, decimal 10^magnitude or binary 2^magnitude, exact f64 instead of floored i16 to enable scaling binary with rounding significant digits correctly when number is [1.000; 1.024[
        let mut s: String; // formatted number string, result


        let mut x: f64 = x.into(); // &T -> f64
        if x.is_infinite() && x.is_sign_positive()
        // edge cases
        {
            return "∞".to_string(); // positive infinity
        }
        else if x.is_infinite() && x.is_sign_negative()
        {
            return "-∞".to_string(); // negative infinity
        }
        else if x.is_nan()
        {
            return "NaN".to_string(); // not a number
        }


        x = match self.rounding // rounded here already in case rounding changes magnitude
        {
            Rounding::Magnitude(precision) => x.round_mag(precision), // round statically to digit at 10^magnitude
            Rounding::SignificantDigits(precision) => x.round_sig(precision), // round dynamically to significant numbers
        };

        if x == 0.0
        {
            magnitude = 0.0; // 0 has default magnitude and no unit prefix, here because log(0) would shit itself
        }
        else
        {
            magnitude = match self.scaling // determine magnitude with rounded value in case rounding changes magnitude
            {
                Scaling::Binary(_) => x.abs().log2(), // if scaling binary: binary magnitude 2^magnitude
                _ => x.abs().log10(), // usually: decimal magnitude 10^magnitude
            }
        }

        dec_places = match (&self.scaling, &self.rounding) // decimal places required depending on scaling and rounding mode
        {
            (Scaling::Binary(_), Rounding::Magnitude(precision)) => // convert binary magnitude to decimal magnitude, exact f64 magnitude required for this case, then business as usual
            {
                match BINARY_PREFIXES.iter().find(|(lower, upper, _prefix)| *lower as f64 <= magnitude && magnitude < *upper as f64) // try to find binary unit prefix for magnitude
                {
                    Some(_) =>{2.0_f64.powf(magnitude - magnitude.rem_euclid(10.0)).log10().floor() as i16 - precision - 1},
                    None => {magnitude.floor() as i16} // fallback to scientific notation
                }
            },
            (Scaling::Binary(_), Rounding::SignificantDigits(precision)) => // convert binary magnitude to decimal magnitude, exact f64 magnitude required for this case, then business as usual
            {
                match BINARY_PREFIXES.iter().find(|(lower, upper, _prefix)| *lower as f64 <= magnitude && magnitude < *upper as f64) // try to find binary unit prefix for magnitude
                {
                    Some(_) =>{-1 * (2.0_f64.powf(magnitude.rem_euclid(10.0)).log10().floor()) as i16 + *precision as i16 - 1},
                    None => {*precision as i16 - 1} // fallback to scientific notation
                }
            }
            (Scaling::Decimal(_), Rounding::Magnitude(precision)) =>
            {
                match DECIMAL_PREFIXES.iter().find(|(lower, upper, _prefix)| *lower as f64 <= magnitude && magnitude < *upper as f64) // try to find decimal unit prefix for magnitude
                {
                    Some(_) =>{(magnitude - magnitude.rem_euclid(3.0)).floor() as i16 - precision},
                    None => {magnitude.floor() as i16} // fallback to scientific notation
                }
            },
            (Scaling::Decimal(_), Rounding::SignificantDigits(precision)) =>
            {
                match DECIMAL_PREFIXES.iter().find(|(lower, upper, _prefix)| *lower as f64 <= magnitude && magnitude < *upper as f64) // try to find decimal unit prefix for magnitude
                {
                    Some(_) =>{-1 * magnitude.rem_euclid(3.0).floor() as i16+ *precision as i16 - 1},
                    None => {*precision as i16 - 1} // fallback to scientific notation
                }
            }
            (Scaling::None, Rounding::Magnitude(precision)) => -1 * precision,
            (Scaling::None, Rounding::SignificantDigits(precision)) => -1 * magnitude.floor() as i16 + *precision as i16 - 1,
            (Scaling::Scientific, Rounding::Magnitude(_)) => magnitude.floor() as i16,
            (Scaling::Scientific, Rounding::SignificantDigits(precision)) => *precision as i16 - 1,
        };
        if dec_places < 0
        {
            dec_places = 0; // negative number of decimal places are not allowed
        }

        match self.scaling //  apply magnitude shift for scaling, f64 -> String, append unit prefix
        {
            Scaling::None => {s = format!("{:.*}", dec_places as usize, x);} // no scaling
            Scaling::Binary(whitespace_separation) =>
            {
                match BINARY_PREFIXES.iter().find(|(lower, upper, _prefix)| *lower as f64 <= magnitude && magnitude < *upper as f64) // try to find binary unit prefix for magnitude
                {
                    Some((_lower, _upper, prefix)) =>
                    {
                        s = format!("{:.*}", dec_places as usize, x / 2.0_f64.powf(magnitude - magnitude.rem_euclid(10.0))); // divide by 2^magnitude
                        if whitespace_separation {s += " ";} // add whitespace between number and unit prefix
                        s += prefix; // append binary unit prefix
                        s = s.trim_end().to_string(); // remove possible trailing whitespace
                    },
                    None => {s = format!("{:.*} * 2^({})", dec_places as usize, x / 2.0_f64.powf(x.log2().floor()), x.log2().floor())} // fallback to base 2 scientific notation
                }
            }
            Scaling::Decimal(whitespace_separation) =>
            {
                match DECIMAL_PREFIXES.iter().find(|(lower, upper, _prefix)| *lower as f64 <= magnitude && magnitude < *upper as f64) // try to find decimal unit prefix for magnitude
                {
                    Some((_lower, _upper, prefix)) =>
                    {
                        s = format!("{:.*}", dec_places as usize, x / 10.0_f64.powf(magnitude - magnitude.rem_euclid(3.0))); // divide by 10^magnitude
                        if whitespace_separation {s += " ";} // add whitespace between number and unit prefix
                        s += prefix; // append decimal unit prefix
                        s = s.trim_end().to_string(); // remove possible trailing whitespace
                    },
                    None => {s = format!("{:.*} * 10^({})", dec_places as usize, x / 10.0_f64.powf(x.log10().floor()), x.log10().floor())} // fallback to base 10 scientific notation
                }
            }
            Scaling::Scientific => {s = format!("{:.*} * 10^({})", dec_places as usize, x / 10.0_f64.powf(x.log10().floor()), x.log10().floor());} // scientific notation
        }

        if self.sign == Sign::Always && x.is_sign_positive()
        // if always sign and positive
        {
            s = format!("+{s}"); // manually add plus sign
        }

        if self.group_separator != ""
        // add thousands separators markers, done here already with default decimal separator "." in case user defined empty or otherwise trolling decimal separator
        {
            let group_separator_i_earliest: usize = s
                .chars()
                .position(|c| c.is_digit(10))
                .expect(format!("Could not find first digit in `s` = \"{s}\", formerly `x` = \"{x}\".").as_str())
                + 1; // earliest possible index of group separator, find first digit
            let mut i: usize = s
                .find(".") // find default decimal separator
                .unwrap_or_else(|| {
                    s.chars()
                        .rev()
                        .position(|c| c.is_digit(10))
                        .map(|pos| s.len() - pos)
                        .expect(format!("Could not find last digit in `s` = \"{s}\", formerly `x` = \"{x}\".").as_str())
                }); // if none assume no decimal separator and start at last digit

            while group_separator_i_earliest + 3 <= i
            // insert group separators
            {
                i -= 3; // move to previous group of 3 digits
                s.insert_str(i, "{GROUP SEPARATOR}"); // insert group separator marker, not actual separator to avoid confusion with decimal separator replacement
            }
        }
        s = s.replace(".", self.decimal_separator.to_string().as_str()); // replace decimal separator
        s = s.replace("{GROUP SEPARATOR}", self.group_separator.to_string().as_str()); // replace group separator

        return s;
    }
}


// TODO sign handling, separators, scaling, scientific notation, prefix handling,
