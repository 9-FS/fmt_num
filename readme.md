# scaler
## Introduction

This crate provides a convenient `Formatter` to scale, round, and display numbers.

Scaling describes the usage of [decimal / metric / SI unit prefixes](https://en.wikipedia.org/wiki/Metric_prefix) or [binary / IEC unit prefixes](https://en.wikipedia.org/wiki/Binary_prefix) to increase readability; though no scaling and scientific notation are also supported.\
Rounding can be done either to a specified magnitude or to a number of significant digits.\
Separators can be freely adjusted. The group separator separates groups of digits every 3 digits before the decimal separator, while the decimal separator separates the integer and fractional parts of a number.\
The sign behaviour can be set to only show the sign when the number is negative ("-"), which is the default, or always show the sign ("+" and "-"). The latter can be useful for highlighting differences. \
By default rounding can create trailing zeros. They can optionally be removed.

## Installation

The feature `warn_about_problematic_separators` warns using `log::warn!` if separators are being set with `Formatter::set_separators` that could lead to ambiguous formatting. It depends on the [`log`](https://crates.io/crates/log) crate and is the only dependency. If a dependencyless build should be desired, it can be disabled by specifying `default-features = false` in your Cargo.toml entry.

## Usage

1. Execute `Formatter::new` to create a new `Formatter` with default settings.
1. Adjust separators, rounding, scaling, and sign behaviour as necessary using the setters.
1. Format numbers with `Formatter::format`.

### `Rounding`

Examples have scaling disabled for easier understanding.

- `Magnitude`:
    - Round to digit at magnitude $10^m$.
    - Contains $m$.

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
        .set_scaling(scaler::Scaling::None)
        .set_rounding(scaler::Rounding::Magnitude(-2));
    assert_eq!(f.format(123.456), "123,46");
    assert_eq!(f.format(0.789), "0,79");
    assert_eq!(f.format(42069), "42.069,00");
    ```

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
        .set_scaling(scaler::Scaling::None)
        .set_rounding(scaler::Rounding::Magnitude(-1));
    assert_eq!(f.format(123.456), "123,5");
    assert_eq!(f.format(0.789), "0,8");
    assert_eq!(f.format(42069), "42.069,0");
    ```

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::None)
       .set_rounding(scaler::Rounding::Magnitude(0));
    assert_eq!(f.format(123.456), "123");
    assert_eq!(f.format(0.789), "1");
    assert_eq!(f.format(42069), "42.069");
    ```

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::None)
       .set_rounding(scaler::Rounding::Magnitude(1));
    assert_eq!(f.format(123.456), "120");
    assert_eq!(f.format(0.789), "0");
    assert_eq!(f.format(42069), "42.070");
    ```

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::None)
       .set_rounding(scaler::Rounding::Magnitude(2));
    assert_eq!(f.format(123.456), "100");
    assert_eq!(f.format(0.789), "0");
    assert_eq!(f.format(42069), "42.100");
    ```

- `SignificantDigits`:
    - Round to $n$ significant numbers.
    - Contains $n$.

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::None)
       .set_rounding(scaler::Rounding::SignificantDigits(0));
    assert_eq!(f.format(123.456), "0");
    assert_eq!(f.format(0.789), "0");
    assert_eq!(f.format(42069), "0");
    ```

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::None)
       .set_rounding(scaler::Rounding::SignificantDigits(1));
    assert_eq!(f.format(123.456), "100");
    assert_eq!(f.format(0.789), "0,8");
    assert_eq!(f.format(42069), "40.000");
    ```

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::None)
       .set_rounding(scaler::Rounding::SignificantDigits(2));
    assert_eq!(f.format(123.456), "120");
    assert_eq!(f.format(0.789), "0,79");
    assert_eq!(f.format(42069), "42.000");
    ```

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::None)
       .set_rounding(scaler::Rounding::SignificantDigits(3));
    assert_eq!(f.format(123.456), "123");
    assert_eq!(f.format(0.789), "0,789");
    assert_eq!(f.format(42069), "42.100");
    ```

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::None)
       .set_rounding(scaler::Rounding::SignificantDigits(4));
    assert_eq!(f.format(123.456), "123,5");
    assert_eq!(f.format(0.789), "0,7890");
    assert_eq!(f.format(42069), "42.070");
    ```

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::None)
       .set_rounding(scaler::Rounding::SignificantDigits(5));
    assert_eq!(f.format(123.456), "123,46");
    assert_eq!(f.format(0.789), "0,78900");
    assert_eq!(f.format(42069), "42.069");
    ```

### `Scaling`

- `Binary`:
    - Scales by factor $2^(10) = 1024$.
    - If no prefix for that magnitude defined: Fallback to scientific notation.
    - Contains whether or not to put space between number and unit prefix.

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::Binary(true));
    assert_eq!(f.format(0.5), "1,000 * 2^(-1)");
    assert_eq!(f.format(1), "1,000");
    assert_eq!(f.format(64), "64,00");
    assert_eq!(f.format(128), "128,0");
    assert_eq!(f.format(1023), "1.023");
    assert_eq!(f.format(1024), "1,000 Ki");
    assert_eq!(f.format(2_f64.powi(10)), "1,000 Ki");
    assert_eq!(f.format(2_f64.powi(20)), "1,000 Mi");
    assert_eq!(f.format(2_f64.powi(30)), "1,000 Gi");
    assert_eq!(f.format(2_f64.powi(40)), "1,000 Ti");
    assert_eq!(f.format(2_f64.powi(50)), "1,000 Pi");
    assert_eq!(f.format(2_f64.powi(60)), "1,000 Ei");
    assert_eq!(f.format(2_f64.powi(70)), "1,000 Zi");
    assert_eq!(f.format(2_f64.powi(80)), "1,000 Yi");
    assert_eq!(f.format(2_f64.powi(90)), "1,000 * 2^(90)");
    ```

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::Binary(false));
    assert_eq!(f.format(1024), "1,000Ki");
    ```

- `Decimal`:
    - Scales by factor $10^(3) = 1000$.
    - If no prefix for that magnitude defined: Fallback to scientific notation.
    - Contains whether or not to put space between number and unit prefix.

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::Decimal(true));
    assert_eq!(f.format(1e-31), "1,000 * 10^(-31)");
    assert_eq!(f.format(1e-30), "1,000 q");
    assert_eq!(f.format(1e-27), "1,000 r");
    assert_eq!(f.format(1e-24), "1,000 y");
    assert_eq!(f.format(1e-21), "1,000 z");
    assert_eq!(f.format(1e-18), "1,000 a");
    assert_eq!(f.format(1e-15), "1,000 f");
    assert_eq!(f.format(1e-12), "1,000 p");
    assert_eq!(f.format(1e-9), "1,000 n");
    assert_eq!(f.format(1e-6), "1,000 µ");
    assert_eq!(f.format(1e-3), "1,000 m");
    assert_eq!(f.format(1), "1,000");
    assert_eq!(f.format(10), "10,00");
    assert_eq!(f.format(100), "100,0");
    assert_eq!(f.format(999), "999,0");
    assert_eq!(f.format(1000), "1,000 k");
    assert_eq!(f.format(1e3), "1,000 k");
    assert_eq!(f.format(1e6), "1,000 M");
    assert_eq!(f.format(1e9), "1,000 G");
    assert_eq!(f.format(1e12), "1,000 T");
    assert_eq!(f.format(1e15), "1,000 P");
    assert_eq!(f.format(1e18), "1,000 E");
    assert_eq!(f.format(1e21), "1,000 Z");
    assert_eq!(f.format(1e24), "1,000 Y");
    assert_eq!(f.format(1e27), "1,000 R");
    assert_eq!(f.format(1e30), "1,000 Q");
    assert_eq!(f.format(1e33), "1,000 * 10^(33)");
    ```

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::Decimal(false));
    assert_eq!(f.format(1000), "1,000k");
    ```

- `None`:
    - no scaling
    - no fallback to scientific notation

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::None);
    assert_eq!(f.format(1e-10), "0,0000000001000");
    assert_eq!(f.format(0.1), "0,1000");
    assert_eq!(f.format(1), "1,000");
    assert_eq!(f.format(10), "10,00");
    assert_eq!(f.format(100), "100,0");
    assert_eq!(f.format(1000), "1.000");
    assert_eq!(f.format(1e10), "10.000.000.000");
    ```

- `Scientific`:
    - always scientific notation

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_scaling(scaler::Scaling::Scientific);
    assert_eq!(f.format(0.1), "1,000 * 10^(-1)");
    assert_eq!(f.format(1), "1,000 * 10^(0)");
    assert_eq!(f.format(10), "1,000 * 10^(1)");
    ```

### Separators

- `group_separator`
    - Separates groups every 3 digits before the decimal separator.
- `decimal_separator`
    - Separates the integer and fractional parts of a number.

Examples have scaling disabled for easier understanding.

```Rust
let f: scaler::Formatter = scaler::Formatter::new()
   .set_scaling(scaler::Scaling::None)
   .set_separators(".", ",");
assert_eq!(f.format(1), "1,000");
assert_eq!(f.format(10), "10,00");
assert_eq!(f.format(100), "100,0");
assert_eq!(f.format(1000), "1.000");
assert_eq!(f.format(10000), "10.000");
```

```Rust
let f: scaler::Formatter = scaler::Formatter::new()
   .set_scaling(scaler::Scaling::None)
   .set_separators("", ",");
assert_eq!(f.format(1), "1,000");
assert_eq!(f.format(10), "10,00");
assert_eq!(f.format(100), "100,0");
assert_eq!(f.format(1000), "1000");
assert_eq!(f.format(10000), "10000");
```

```Rust
let f: scaler::Formatter = scaler::Formatter::new()
   .set_scaling(scaler::Scaling::None)
   .set_separators(",", ".");
assert_eq!(f.format(1), "1.000");
assert_eq!(f.format(10), "10.00");
assert_eq!(f.format(100), "100.0");
assert_eq!(f.format(1000), "1,000");
assert_eq!(f.format(10000), "10,000");
```

### `Sign`

- `Always`
    - Always show sign, even when number is positive.

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_sign(scaler::Sign::Always);
    assert_eq!(f.format(std::f64::NEG_INFINITY), "-∞");
    assert_eq!(f.format(-1), "-1,000");
    assert_eq!(f.format(0), "+0,000");
    assert_eq!(f.format(1), "+1,000");
    assert_eq!(f.format(std::f64::INFINITY), "+∞");
    ```

- `OnlyMinus`
    - Only show sign when number is negative.

    ```Rust
    let f: scaler::Formatter = scaler::Formatter::new()
       .set_sign(scaler::Sign::OnlyMinus);
    assert_eq!(f.format(std::f64::NEG_INFINITY), "-∞");
    assert_eq!(f.format(-1), "-1,000");
    assert_eq!(f.format(0), "0,000");
    assert_eq!(f.format(1), "1,000");
    assert_eq!(f.format(std::f64::INFINITY), "∞");
    ```

### Trailing Zeros

- `true`
   ```Rust
   let f: scaler::Formatter = scaler::Formatter::new()
      .set_trailing_zeros(true);
   assert_eq!(f.format(1), "1,000");
   assert_eq!(f.format(1.2), "1,200");
   assert_eq!(f.format(1.23), "1,230");
   assert_eq!(f.format(1.234), "1,234");
   assert_eq!(f.format(1.2345), "1,234");
   ```

- `false`
   ```Rust
   let f: scaler::Formatter = scaler::Formatter::new()
      .set_trailing_zeros(false);
   assert_eq!(f.format(1), "1");
   assert_eq!(f.format(1.2), "1,2");
   assert_eq!(f.format(1.23), "1,23");
   assert_eq!(f.format(1.234), "1,234");
   assert_eq!(f.format(1.2345), "1,234");
   ```