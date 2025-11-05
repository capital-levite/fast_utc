fast_utc
========

Simple & fast UTC time types

```toml
[dependencies]
fast_utc = "0.1"
```

This library is a fork and modernization of the 5-year-old `utctimestamp` project by Joel Höner (https://github.com/athre0z/utctimestamp).

While [chrono](https://crates.io/crates/chrono) is great for dealing with time
in most cases, its 96-bit integer design can be costly when processing and storing 
large amounts of timestamp data.

This lib solves this problem by providing very simple UTC timestamps that can be
converted from and into their corresponding chrono counterpart using Rust's
`From` and `Into` traits. chrono is then used for all things that aren't expected
to occur in big batches, such as formatting and displaying the timestamps. 

#### Optional features

`serde-support` — Enable (de)serialization support with serde
`coarsetime-support` — Enable `coarsetime` for faster timestamp generation (enabled by default)

#### Benchmarks

Benchmarks were run on a Linux system (specifics omitted for brevity).

**With `coarsetime-support` enabled (default):**

```
UtcTimeStamp::now() (coarsetime) time:   [2.3100 ns 2.3459 ns 2.3881 ns] (Regressed by ~80.4%)
chrono::Utc::now()              time:   [60.333 ns 60.651 ns 61.028 ns] (Improved by ~6.2%)
```

**With `coarsetime-support` disabled (`--no-default-features`):**

```
UtcTimeStamp::now() (chrono fallback) time:   [2.9071 ns 3.0617 ns 3.2538 ns] (Regressed by ~149.5%)
chrono::Utc::now()                  time:   [65.370 ns 67.725 ns 70.497 ns] (Regressed by ~77.1%)
```

**Note:** Deserialization benchmarks are currently not available as they were removed from the benchmark files during recent changes.
