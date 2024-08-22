#import "@preview/wrap-it:0.1.0": wrap-content  // https://github.com/ntjess/wrap-it/blob/main/docs/manual.pdf
#import "./doc_templates/src/note.typ": *
#import "./doc_templates/src/style.typ": set_style


#show: doc => set_style(
    topic: "scaler",
    author: "구FS",
    language: "EN",
    doc
)
#set text(size: 3.5mm)


#align(center, text(size: 2em, weight: "bold")[scaler])
#line(length: 100%, stroke: 0.3mm)
\
\
= Introduction

This crate provides a convenient `Formatter` to scale, round, and display numbers.

Scaling describes the usage of #link("https://en.wikipedia.org/wiki/Metric_prefix")[decimal] or #link("https://en.wikipedia.org/wiki/Binary_prefix")[binary unit prefixes] to increase readability; though no scaling and scientific notation are also supported. Rounding can be done either to a specified magnitude or to a number of significant digits.

= Table of Contents

#outline()

#pagebreak(weak: true)

= Usage

+ Execute `Formatter::new` to create a new `Formatter` with default settings.
+ Adjust separators, rounding, scaling, and sign behaviour as necessary using the setters.
+ Format numbers with `Formatter::format`.

== Example

```Rust
let f: scaler::Formatter = scaler::Formatter::new()
    .set_rounding(scaler::Rounding::SignificantDigits(2)); // general display
assert_eq!(f.format(123), "120");
assert_eq!(f.format(4.56), "4,6");
```

== Example

```Rust
let f: scaler::Formatter = scaler::Formatter::new(); // calculation results
assert_eq!(f.format(456789), "456,8 k");
assert_eq!(f.format(0.1), "100,0 m");
```

== Example

```Rust
let f: scaler::Formatter = scaler::Formatter::new()
    .set_scaling(scaler::Scaling::None)
    .set_rounding(scaler::Rounding::Magnitude(0)); // absolute values
assert_eq!(f.format(0.1), "0");
assert_eq!(f.format(1), "1");
assert_eq!(f.format(1000), "1.000");
```

== Example

```Rust
let f: scaler::Formatter = scaler::Formatter::new()
    .set_scaling(scaler::Scaling::Binary(true)); // data sizes
assert_eq!(f.format(0.1), "1,600 * 2^(-4)");
assert_eq!(f.format(1023), "1.023");
assert_eq!(f.format(1024), "1,000 Ki");
```