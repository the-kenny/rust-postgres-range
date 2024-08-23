# rust-postgres-range
[![Build Status](https://travis-ci.org/sfackler/rust-postgres-range.svg?branch=master)](https://travis-ci.org/sfackler/rust-postgres-range)

[Documentation](https://sfackler.github.io/rust-postgres-range/doc/v0.8.2/postgres_range)

Support for PostgreSQL ranges in [rust-postgres](https://github.com/sfackler/rust-postgres).


|   **Feature**   |                     **Description**                     | **Extra Dependencies** |
|:---------------:|:-------------------------------------------------------:|:----------------------:|
| with-chrono-0_4 | Enable conversions for ts_range, tstz_range, date_range | [chrono](https://crates.io/crates/chrono) 0.4             |
| with-decimal-1  | Enable conversions for num_range type                   | [rust_decimal](https://crates.io/crates/rust-decimal) 1.0       |
