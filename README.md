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
UtcTimeStamp::now() (coarsetime) time:   [1.2601 ns 1.2737 ns 1.2913 ns]
chrono::Utc::now()              time:   [36.935 ns 37.078 ns 37.231 ns]
```

**With `coarsetime-support` disabled (`--no-default-features`):**

```
UtcTimeStamp::now() (chrono fallback) time:   [41.372 ns 41.499 ns 41.626 ns]
chrono::Utc::now()                  time:   [36.981 ns 37.095 ns 37.214 ns]
```
