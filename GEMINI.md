# `utctimestamp` API Description for LLMs

This document provides a comprehensive overview of the `utctimestamp` Rust crate's API, designed to enable Large Language Models (LLMs) to understand its functionality and usage without direct Rust code analysis. It focuses on key types, functions, and features relevant for integration and utilization.

## 1. Crate Overview

`utctimestamp` is a Rust library providing simple and fast 64-bit UTC time types. It offers a lightweight alternative to `chrono` for scenarios requiring high-performance timestamp processing and storage, particularly in applications like High-Frequency Trading (HFT).

The library's core idea is to represent UTC timestamps as `i64` milliseconds since the Unix epoch, allowing for efficient arithmetic and storage. It provides seamless conversion to and from `chrono::DateTime<chrono::Utc>` for more complex operations like formatting and display.

## 2. Installation (Cargo.toml)

To use `utctimestamp` in a Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
utctimestamp = "0.1" # Use the latest version available on crates.io
```

### Optional Features

The crate supports the following optional features, which can be enabled in `Cargo.toml`:

*   `serde-support`: Enables serialization and deserialization of `UtcTimeStamp` and `TimeDelta` using the `serde` framework.
    ```toml
    [dependencies]
    utctimestamp = { version = "0.1", features = ["serde-support"] }
    ```
*   `coarsetime-support`: (Enabled by default) Integrates with the `coarsetime` crate for extremely fast timestamp generation. When this feature is enabled, `UtcTimeStamp::now()` leverages `coarsetime`'s cached timestamp for optimal performance.
    ```toml
    [dependencies]
    utctimestamp = { version = "0.1", default-features = false, features = ["coarsetime-support"] } # Example of explicitly enabling
    ```
    To disable `coarsetime-support` (and fall back to `chrono` for `now()`), you would use:
    ```toml
    [dependencies]
    utctimestamp = { version = "0.1", default-features = false }
    ```

## 3. Core Types

### `UtcTimeStamp`

Represents a UTC timestamp with millisecond precision, stored internally as an `i64` representing milliseconds since the Unix epoch.

**Key Methods:**

*   **`UtcTimeStamp::zero() -> Self`**
    *   **Description**: Initializes a timestamp representing `1970-01-01 00:00:00 UTC` (Unix epoch).
    *   **Returns**: A `UtcTimeStamp` instance.

*   **`UtcTimeStamp::now() -> Self`**
    *   **Description**: Initializes a timestamp representing the current UTC time.
        *   If `coarsetime-support` feature is enabled (default), this uses `coarsetime::Clock::recent_since_epoch()` for high performance. For optimal results, `coarsetime::Clock::update()` should be called periodically in the application's main loop or by a background thread (e.g., using `coarsetime::Updater`).
        *   If `coarsetime-support` is disabled, this falls back to `chrono::Utc::now()`.
    *   **Returns**: A `UtcTimeStamp` instance.

*   **`UtcTimeStamp::from_milliseconds(int: i64) -> Self`**
    *   **Description**: Creates a `UtcTimeStamp` from an `i64` representing milliseconds since the Unix epoch.
    *   **Parameters**: `int` (i64) - Milliseconds since epoch.
    *   **Returns**: A `UtcTimeStamp` instance.

*   **`UtcTimeStamp::from_seconds(int: i64) -> Self`**
    *   **Description**: Creates a `UtcTimeStamp` from an `i64` representing seconds since the Unix epoch.
    *   **Parameters**: `int` (i64) - Seconds since epoch.
    *   **Returns**: A `UtcTimeStamp` instance.

*   **`UtcTimeStamp::from_nanoseconds(int: u64) -> Self`**
    *   **Description**: Creates a `UtcTimeStamp` from a `u64` representing nanoseconds since the Unix epoch. Note that `UtcTimeStamp` itself has millisecond precision, so nanoseconds will be truncated.
    *   **Parameters**: `int` (u64) - Nanoseconds since epoch.
    *   **Returns**: A `UtcTimeStamp` instance.

*   **`UtcTimeStamp::as_milliseconds(self) -> i64`**
    *   **Description**: Returns the timestamp as an `i64` representing milliseconds since the Unix epoch.
    *   **Returns**: `i64` milliseconds.

*   **`UtcTimeStamp::align_to(self, freq: TimeDelta) -> UtcTimeStamp`**
    *   **Description**: Aligns the timestamp to a given frequency (e.g., nearest 5 minutes) relative to the Unix epoch.
    *   **Parameters**: `freq` (`TimeDelta`) - The frequency to align to.
    *   **Returns**: A new `UtcTimeStamp` aligned to the frequency.

*   **`UtcTimeStamp::align_to_anchored(self, anchor: UtcTimeStamp, freq: TimeDelta) -> UtcTimeStamp`**
    *   **Description**: Aligns the timestamp to a given frequency, using a specified anchor point instead of the Unix epoch.
    *   **Parameters**: `anchor` (`UtcTimeStamp`) - The anchor timestamp; `freq` (`TimeDelta`) - The frequency to align to.
    *   **Returns**: A new `UtcTimeStamp` aligned to the frequency relative to the anchor.

*   **`UtcTimeStamp::is_zero(self) -> bool`**
    *   **Description**: Checks if the timestamp is `1970-01-01 00:00:00 UTC`.
    *   **Returns**: `true` if zero, `false` otherwise.

**Operator Overloads:**

*   `+ TimeDelta`: Adds a `TimeDelta` to a `UtcTimeStamp`, returning a new `UtcTimeStamp`.
*   `- TimeDelta`: Subtracts a `TimeDelta` from a `UtcTimeStamp`, returning a new `UtcTimeStamp`.
*   `- UtcTimeStamp`: Calculates the difference between two `UtcTimeStamp`s, returning a `TimeDelta`.

**Conversions:**

*   `From<chrono::DateTime<chrono::Utc>> for UtcTimeStamp`
*   `From<UtcTimeStamp> for chrono::DateTime<chrono::Utc>`

### `TimeDelta`

Represents a duration with millisecond precision, stored internally as an `i64` representing milliseconds.

**Key Methods:**

*   **`TimeDelta::zero() -> Self`**
    *   **Description**: Initializes a zero duration.
    *   **Returns**: A `TimeDelta` instance.

*   **`TimeDelta::from_hours(int: i64) -> Self`**
    *   **Description**: Creates a `TimeDelta` from a number of hours.
    *   **Parameters**: `int` (i64) - Number of hours.
    *   **Returns**: A `TimeDelta` instance.

*   **`TimeDelta::from_minutes(int: i64) -> Self`**
    *   **Description**: Creates a `TimeDelta` from a number of minutes.
    *   **Parameters**: `int` (i64) - Number of minutes.
    *   **Returns**: A `TimeDelta` instance.

*   **`TimeDelta::from_seconds(int: i64) -> Self`**
    *   **Description**: Creates a `TimeDelta` from a number of seconds.
    *   **Parameters**: `int` (i64) - Number of seconds.
    *   **Returns**: A `TimeDelta` instance.

*   **`TimeDelta::from_milliseconds(int: i64) -> Self`**
    *   **Description**: Creates a `TimeDelta` from a number of milliseconds.
    *   **Parameters**: `int` (i64) - Number of milliseconds.
    *   **Returns**: A `TimeDelta` instance.

*   **`TimeDelta::as_milliseconds(self) -> i64`**
    *   **Description**: Returns the duration as an `i64` representing milliseconds.
    *   **Returns**: `i64` milliseconds.

*   **`TimeDelta::is_zero(self) -> bool`**
    *   **Description**: Checks if the duration is zero.
    *   **Returns**: `true` if zero, `false` otherwise.

*   **`TimeDelta::is_positive(self) -> bool`**
    *   **Description**: Checks if the duration is positive.
    *   **Returns**: `true` if positive, `false` otherwise.

*   **`TimeDelta::is_negative(self) -> bool`**
    *   **Description**: Checks if the duration is negative.
    *   **Returns**: `true` if negative, `false` otherwise.

**Operator Overloads:**

*   `+ TimeDelta`: Adds two `TimeDelta`s.
*   `- TimeDelta`: Subtracts one `TimeDelta` from another.
*   `* i64`: Multiplies a `TimeDelta` by an `i64` scalar.
*   `/ i64`: Divides a `TimeDelta` by an `i64` scalar.
*   `/ TimeDelta`: Divides one `TimeDelta` by another, returning an `i64`.
*   `% TimeDelta`: Calculates the remainder of one `TimeDelta` divided by another.

**Conversions:**

*   `From<chrono::Duration> for TimeDelta`
*   `From<TimeDelta> for chrono::Duration`

## 4. `TimeRange`

An iterator for looping over `UtcTimeStamp`s given a start, end, and step `TimeDelta`.

**Key Methods:**

*   **`TimeRange::right_closed(start: impl Into<UtcTimeStamp>, end: impl Into<UtcTimeStamp>, step: impl Into<TimeDelta>) -> Self`**
    *   **Description**: Creates a time range that includes the `end` timestamp.
    *   **Parameters**: `start` (Into<UtcTimeStamp>), `end` (Into<UtcTimeStamp>), `step` (Into<TimeDelta>).
    *   **Returns**: A `TimeRange` iterator.

*   **`TimeRange::right_open(start: impl Into<UtcTimeStamp>, end: impl Into<UtcTimeStamp>, step: impl Into<TimeDelta>) -> Self`**
    *   **Description**: Creates a time range that excludes the `end` timestamp.
    *   **Parameters**: `start` (Into<UtcTimeStamp>), `end` (Into<UtcTimeStamp>), `step` (Into<TimeDelta>).
    *   **Returns**: A `TimeRange` iterator.

## 5. Utilizing `coarsetime` for Maximum Performance

When the `coarsetime-support` feature is enabled (which is the default), `UtcTimeStamp::now()` leverages `coarsetime`'s highly optimized, cached timestamp. To ensure this cached value is always up-to-date and provides the fastest possible results, it is crucial to periodically call `coarsetime::Clock::update()`.

**Example Usage Pattern:**

```rust
// In your application's main loop or a dedicated background task:
loop {
    // Update the coarsetime clock cache
    coarsetime::Clock::update();

    // ... your application logic ...

    // Get the current timestamp using utctimestamp (which now uses the cached coarsetime value)
    let now = utctimestamp::UtcTimeStamp::now();

    // ... further processing with 'now' ...

    // Sleep or yield to prevent busy-waiting
    std::thread::sleep(std::time::Duration::from_millis(1)); // Adjust update frequency as needed
}
```

Alternatively, `coarsetime` provides an `Updater` utility to manage background updates:

```rust
// In your application initialization:
let updater = coarsetime::Updater::new(1 /* milliseconds */).start().expect("Failed to start coarsetime updater");

// In your application logic, you can now call UtcTimeStamp::now() without explicitly calling Clock::update()
let now = utctimestamp::UtcTimeStamp::now();

// When your application shuts down:
updater.stop().expect("Failed to stop coarsetime updater");
```

By following this pattern, LLMs can understand that `UtcTimeStamp::now()` offers extremely low-latency timestamp acquisition, provided the `coarsetime` cache is actively maintained by the consuming application.