# `fast_utc` API Description for LLMs

This document provides a comprehensive overview of the `fast_utc` Rust crate's API, designed to enable Large Language Models (LLMs) to understand its functionality and usage without requiring direct access to the Rust source code.

The primary goal of `fast_utc` is to offer a simple, fast, and lightweight set of UTC timestamp primitives suitable for high‑performance applications where:

- You want efficient storage and arithmetic over UTC timestamps,
- You prefer a compact 64‑bit representation,
- You want straightforward interoperability with `chrono` and (optionally) `coarsetime`.

`fast_utc` centers around:

- `Timestamp`: a millisecond‑precision UTC timestamp stored as `i64` milliseconds since the Unix epoch.
- `TimeDelta`: a millisecond‑precision duration stored as `i64` milliseconds.
- `TimeRange`: an iterator that produces regularly spaced `Timestamp` values.

It also provides optional integration with `serde` for serialization and with `coarsetime` for ultra‑fast `now()` retrieval.

---

## 1. Crate Overview

`fast_utc` is a Rust library providing simple and fast 64‑bit UTC time types. It offers a lightweight alternative to heavier date‑time libraries like `chrono` for scenarios requiring high‑performance timestamp processing and storage.

**Core ideas:**

- Represent UTC timestamps as `i64` milliseconds since the Unix epoch (`1970-01-01T00:00:00Z`).
- Provide minimal but expressive arithmetic via a `TimeDelta` duration type.
- Offer conversions to and from `chrono` types for interoperability.
- Optionally use `coarsetime` to get extremely fast, cached timestamps for `Timestamp::now()`.

The library’s central representation is:

- `Timestamp`: `i64` milliseconds since Unix epoch, millisecond precision.
- `TimeDelta`: `i64` milliseconds, representing durations between timestamps.

This representation enables:

- Fast arithmetic (simple integer math),
- Compact storage (8 bytes per timestamp),
- Easy alignment, iteration, and range operations.

---

## 2. Installation (Cargo.toml)

To use `fast_utc` in a Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
fast_utc = "0.1" # Use the latest version available for this project
```

> If the published crate name differs (for example, if this repository is named `fast_utc` but the crate on crates.io has a different name), adjust the dependency line accordingly. The rest of this document assumes the crate is imported as `fast_utc`.

### Optional Features

The crate supports several optional features that can be enabled in `Cargo.toml`:

- `serde-support`: Enables serialization and deserialization of `Timestamp` and `TimeDelta` using `serde`.

  ```toml
  [dependencies]
  fast_utc = { version = "0.1", features = ["serde-support"] }
  ```

- `coarsetime-support`: (Enabled by default) Integrates with the `coarsetime` crate for extremely fast timestamp generation. When this feature is enabled, `Timestamp::now()` leverages `coarsetime`’s cached timestamp value for high performance.

  ```toml
  [dependencies]
  fast_utc = { version = "0.1", default-features = false, features = ["coarsetime-support"] } # Explicitly enable
  ```

  To disable `coarsetime-support` (and fall back to `chrono` for `now()`), use:

  ```toml
  [dependencies]
  fast_utc = { version = "0.1", default-features = false }
  ```

---

## 3. Core Types

### 3.1 `Timestamp`

Represents a UTC timestamp with millisecond precision, stored internally as an `i64` representing milliseconds since the Unix epoch.

Conceptually:

- `Timestamp(0)` corresponds to `1970-01-01T00:00:00Z`.
- Positive values are times after the epoch; negative values are times before.

#### Key Methods

- **`Timestamp::zero() -> Self`**
  - **Description:** Initializes a timestamp representing `1970-01-01 00:00:00 UTC` (Unix epoch).
  - **Returns:** A `Timestamp` instance.

- **`Timestamp::now() -> Self`**
  - **Description:** Initializes a timestamp representing the current UTC time.
    - If the `coarsetime-support` feature is enabled (default), this uses `coarsetime::Clock::recent_since_epoch()` under the hood for high performance. For best results, the application should periodically call `coarsetime::Clock::update()` or run a `coarsetime::Updater` in the background.
    - If `coarsetime-support` is disabled, this falls back to `chrono::Utc::now()`.
  - **Returns:** A `Timestamp` instance representing "now" in UTC.

- **`Timestamp::fetch_chrono_utc_now() -> chrono::DateTime<chrono::Utc>`**
  - **Description:** Fetches the current UTC time directly as a `chrono::DateTime<chrono::Utc>` instance.
    - With `coarsetime-support` enabled, still uses `coarsetime::Clock::recent_since_epoch()` for performance, then converts to `chrono`.
    - With `coarsetime-support` disabled, uses `chrono::Utc::now()` directly.
  - **Returns:** A `chrono::DateTime<chrono::Utc>` instance.

- **`Timestamp::from_milliseconds(int: i64) -> Self`**
  - **Description:** Creates a `Timestamp` from an `i64` representing milliseconds since the Unix epoch.
  - **Parameters:** `int` (`i64`) – milliseconds since epoch.
  - **Returns:** A `Timestamp` instance.

- **`Timestamp::from_seconds(int: i64) -> Self`**
  - **Description:** Creates a `Timestamp` from an `i64` representing seconds since the Unix epoch.
  - **Parameters:** `int` (`i64`) – seconds since epoch.
  - **Returns:** A `Timestamp` instance.

- **`Timestamp::from_nanoseconds(int: u64) -> Self`**
  - **Description:** Creates a `Timestamp` from a `u64` representing nanoseconds since the Unix epoch.
    - Note: `Timestamp` itself has millisecond precision, so any sub‑millisecond component will be truncated.
  - **Parameters:** `int` (`u64`) – nanoseconds since epoch.
  - **Returns:** A `Timestamp` instance.

- **`Timestamp::as_milliseconds(self) -> i64`**
  - **Description:** Returns the timestamp as an `i64` representing milliseconds since the Unix epoch.
  - **Returns:** `i64` milliseconds.

- **`Timestamp::align_to(self, freq: TimeDelta) -> Timestamp`**
  - **Description:** Aligns the timestamp to a given frequency (e.g., nearest 5 minutes) relative to the Unix epoch.
  - **Parameters:** `freq` (`TimeDelta`) – the frequency to align to.
  - **Returns:** A new `Timestamp` aligned to the given frequency.

- **`Timestamp::align_to_anchored(self, anchor: Timestamp, freq: TimeDelta) -> Timestamp`**
  - **Description:** Aligns the timestamp to a given frequency, using a specified anchor point instead of the Unix epoch. This is useful for application‑specific alignment (e.g., custom trading session start).
  - **Parameters:**
    - `anchor` (`Timestamp`) – the anchor timestamp.
    - `freq` (`TimeDelta`) – the frequency to align to.
  - **Returns:** A new `Timestamp` aligned to the frequency relative to the anchor.

- **`Timestamp::is_zero(self) -> bool`**
  - **Description:** Checks if the timestamp equals the Unix epoch (`1970-01-01 00:00:00 UTC`).
  - **Returns:** `true` if zero, `false` otherwise.

#### Operator Overloads

`Timestamp` supports arithmetic with `TimeDelta` and other `Timestamp`s:

- `Timestamp + TimeDelta -> Timestamp`
- `Timestamp - TimeDelta -> Timestamp`
- `Timestamp - Timestamp -> TimeDelta`

This allows for natural usage patterns:

```rust
use fast_utc::{Timestamp, TimeDelta};

let start = Timestamp::now();
let one_minute = TimeDelta::from_minutes(1);
let end = start + one_minute;

let elapsed: TimeDelta = end - start;
assert!(elapsed.is_positive());
```

#### Conversions

To maintain interoperability with `chrono`, `fast_utc` provides `From` implementations:

- `From<chrono::DateTime<chrono::Utc>> for Timestamp`
- `From<Timestamp> for chrono::DateTime<chrono::Utc>`

This allows simple conversions:

```rust
use fast_utc::Timestamp;
use chrono::{DateTime, Utc};

let chrono_now: DateTime<Utc> = Utc::now();
let fast_now: Timestamp = chrono_now.into();

let back_to_chrono: DateTime<Utc> = fast_now.into();
```

---

### 3.2 `TimeDelta`

Represents a duration with millisecond precision, stored internally as an `i64` representing milliseconds.

#### Key Methods

- **`TimeDelta::zero() -> Self`**
  - **Description:** Initializes a zero duration.
  - **Returns:** A `TimeDelta` instance.

- **`TimeDelta::from_hours(int: i64) -> Self`**
  - **Description:** Creates a `TimeDelta` from a number of hours.
  - **Parameters:** `int` (`i64`) – number of hours.
  - **Returns:** A `TimeDelta` instance.

- **`TimeDelta::from_minutes(int: i64) -> Self`**
  - **Description:** Creates a `TimeDelta` from a number of minutes.
  - **Parameters:** `int` (`i64`) – number of minutes.
  - **Returns:** A `TimeDelta` instance.

- **`TimeDelta::from_seconds(int: i64) -> Self`**
  - **Description:** Creates a `TimeDelta` from a number of seconds.
  - **Parameters:** `int` (`i64`) – number of seconds.
  - **Returns:** A `TimeDelta` instance.

- **`TimeDelta::from_milliseconds(int: i64) -> Self`**
  - **Description:** Creates a `TimeDelta` from a number of milliseconds.
  - **Parameters:** `int` (`i64`) – number of milliseconds.
  - **Returns:** A `TimeDelta` instance.

- **`TimeDelta::as_milliseconds(self) -> i64`**
  - **Description:** Returns the duration as an `i64` representing milliseconds.
  - **Returns:** `i64` milliseconds.

- **`TimeDelta::is_zero(self) -> bool`**
  - **Description:** Checks if the duration is zero.
  - **Returns:** `true` if zero, `false` otherwise.

- **`TimeDelta::is_positive(self) -> bool`**
  - **Description:** Checks if the duration is positive.
  - **Returns:** `true` if positive, `false` otherwise.

- **`TimeDelta::is_negative(self) -> bool`**
  - **Description:** Checks if the duration is negative.
  - **Returns:** `true` if negative, `false` otherwise.

#### Operator Overloads

`TimeDelta` supports common arithmetic operations:

- `TimeDelta + TimeDelta -> TimeDelta`
- `TimeDelta - TimeDelta -> TimeDelta`
- `TimeDelta * i64 -> TimeDelta`
- `TimeDelta / i64 -> TimeDelta`
- `TimeDelta / TimeDelta -> i64` (ratio between two durations)
- `TimeDelta % TimeDelta -> TimeDelta` (remainder)

Example:

```rust
use fast_utc::TimeDelta;

let hour = TimeDelta::from_hours(1);
let half_hour = hour / 2;
let remainder = hour % half_hour;

assert!(remainder.is_zero());
```

#### Conversions

To integrate with `chrono::Duration`, `fast_utc` provides:

- `From<chrono::Duration> for TimeDelta`
- `From<TimeDelta> for chrono::Duration`

Example:

```rust
use fast_utc::TimeDelta;
use chrono::Duration;

let chrono_dur = Duration::seconds(30);
let fast_dur: TimeDelta = chrono_dur.into();

let back_to_chrono: Duration = fast_dur.into();
```

---

## 4. `TimeRange`

`TimeRange` is an iterator for looping over `Timestamp`s given a start, end, and step `TimeDelta`.

This is useful for generating evenly spaced time grids (e.g., every minute, every 5 seconds, etc.) between two timestamps.

### Key Constructors

- **`TimeRange::right_closed(start: impl Into<Timestamp>, end: impl Into<Timestamp>, step: impl Into<TimeDelta>) -> Self`**
  - **Description:** Creates a time range that includes the `end` timestamp, if it lies exactly on a step boundary.
  - **Parameters:**
    - `start` (Into<`Timestamp`>)
    - `end` (Into<`Timestamp`>)
    - `step` (Into<`TimeDelta`>)
  - **Returns:** A `TimeRange` iterator.

- **`TimeRange::right_open(start: impl Into<Timestamp>, end: impl Into<Timestamp>, step: impl Into<TimeDelta>) -> Self`**
  - **Description:** Creates a time range that excludes the `end` timestamp, even if it falls exactly on a step boundary.
  - **Parameters:**
    - `start` (Into<`Timestamp`>)
    - `end` (Into<`Timestamp`>)
    - `step` (Into<`TimeDelta`>)
  - **Returns:** A `TimeRange` iterator.

### Example Usage

```rust
use fast_utc::{Timestamp, TimeDelta, TimeRange};

let start = Timestamp::from_seconds(0);
let end   = Timestamp::from_seconds(10);
let step  = TimeDelta::from_seconds(2);

let mut count = 0;

for ts in TimeRange::right_closed(start, end, step) {
    println!("Tick at {} ms", ts.as_milliseconds());
    count += 1;
}

// For 0..=10 seconds with 2-second steps, count will be 6: 0, 2, 4, 6, 8, 10
assert_eq!(count, 6);
```

---

## 5. Utilizing `coarsetime` for Maximum Performance

When the `coarsetime-support` feature is enabled (which is the default for `fast_utc`), `Timestamp::now()` leverages `coarsetime`’s highly optimized, cached timestamp.

To ensure this cached value stays fresh and monotonic, your application should:

- Call `coarsetime::Clock::update()` periodically in a loop, or
- Use the `coarsetime::Updater` utility to manage updates in a background thread.

### Example: Manual Clock Updates

```rust
use fast_utc::Timestamp;
use std::time::Duration;

fn main() {
    loop {
        // Update the coarsetime clock cache
        coarsetime::Clock::update();

        // Get the current timestamp using fast_utc (uses cached coarsetime value)
        let now = Timestamp::now();

        // ... your application logic using `now` ...

        // Sleep or yield to avoid busy-waiting
        std::thread::sleep(Duration::from_millis(1)); // Tune as appropriate
    }
}
```

### Example: Using `coarsetime::Updater`

```rust
use fast_utc::Timestamp;

fn main() {
    // Start an updater that refreshes the coarsetime cache every 1 ms
    let updater = coarsetime::Updater::new(1 /* milliseconds */)
        .start()
        .expect("Failed to start coarsetime updater");

    // In your application logic, you can now call Timestamp::now()
    // without explicitly calling Clock::update().
    let now = Timestamp::now();
    println!("Now: {} ms since epoch", now.as_milliseconds());

    // ... rest of your application ...

    // On shutdown:
    updater.stop().expect("Failed to stop coarsetime updater");
}
```

By following this pattern, LLMs should understand that `Timestamp::now()` in `fast_utc` offers extremely low‑latency timestamp acquisition, provided the `coarsetime` cache is actively maintained.

---

## 6. Benchmarking

`fast_utc` includes benchmarking support to compare different approaches to timestamp acquisition (e.g., via `chrono` and via `coarsetime`).

A convenience script is provided to run benchmarks with specific warm‑up and measurement times (for example, 0.2 seconds warm‑up and 0.5 seconds measurement time).

From the project root, run:

```bash
./scripts/bench_fast.sh
```

This script executes commands similar to:

```bash
cargo bench --bench timestamp_bench_chrono -- --warm-up-time 0.2 --measurement-time 0.5
cargo bench --bench timestamp_bench_coarsetime -- --warm-up-time 0.2 --measurement-time 0.5
```

Adjust these commands as needed based on the actual benchmark targets defined in this repository.

---

## 7. The Analytical Engineering Protocol (For LLM Task Execution)

The following protocol is **meta‑documentation** for LLMs working on the `@capital-levite/fast_utc` project. It describes how to approach software engineering tasks in this repository with maximum quality, precision, and insight.

**Objective:**  
Execute software engineering tasks in `fast_utc` with maximal rigor, by following a strict two‑phase protocol:

1. **Deep analytical phase** – understand context, constraints, incentives, and patterns.
2. **Precise implementation phase** – provide concrete, executable changes (code and commands).

### Core Principles

1. **Analysis First, Code Second**  
   Never provide implementation without first demonstrating a deep understanding of the problem’s context, constraints, and underlying patterns.

2. **Precision and Actionability**  
   All implementation instructions must be specific, deterministic, and ready for execution. Avoid vague or hand‑wavy suggestions.

3. **Systems Thinking**  
   Analyze problems across multiple dimensions: code, architecture, benchmarks, crate features, and how they are used by consuming applications.

4. **Source & Incentive Awareness**  
   Critically evaluate information sources and underlying incentives: performance targets, API stability, error handling, and ergonomics.

### Phase 1: Analytical Discernment

Before proposing any changes to `fast_utc`, perform and present this analysis.

**A. Contextual Survey**

- List all relevant components: files, modules, types (`Timestamp`, `TimeDelta`, `TimeRange`), features (`serde-support`, `coarsetime-support`), benchmarks, and documentation.
- Identify the stated goal and any unstated constraints (e.g., hard performance budgets, binary size constraints, compatibility requirements).

**B. Multi‑Dimensional Analysis**

- **Technical Dimension:**  
  Analyze code structure, data flow, error handling, trait implementations, and API boundaries. Look for technical debt, potential bottlenecks, and failure points (e.g., overflow handling, time alignment edge cases).

- **Historical Dimension:**  
  Identify relevant historical precedents—either in this repository (e.g., previous performance regressions or API changes) or in similar time libraries.

- **Incentive Dimension:**  
  Map who cares about what:
  - Maintainers: API stability, clean abstractions, performance.
  - Consumers: predictable behavior, easy interop with `chrono`, simple types.
  - Tooling: benchmarks, CI, feature flags.

- **Information Asymmetry Dimension:**  
  Identify what information is missing, obscured, or assumed: exact crate name on crates.io, supported Rust versions, feature defaults, etc.

**C. Problem Reframing**

Reframe the initial problem statement based on the above. For example:

- Surface: “make `now()` faster.”
- Core: “ensure `now()` is both fast and predictable under `coarsetime`, including on machines with skewed clocks or irregular `Clock::update()` calls.”

State the **core problem** clearly before moving to implementation.

---

### Phase 2: Precise Implementation

Only after completing Phase 1, provide the solution using this structured format.

**1. Solution Strategy**

- Briefly summarize the chosen approach and explicitly link it back to the insights from Phase 1.
- For example: “To preserve API stability while improving performance, we will introduce an internal helper that abstracts `coarsetime` usage, guarded strictly by the `coarsetime-support` feature.”

**2. Implementation Steps**

For each action:

- Specify the **exact file path** and location (e.g., “add this function near other `Timestamp` constructors”).
- Provide the **exact code** to be written, edited, or deleted.
- Provide **exact shell commands** to run (e.g., tests, benches, fmt, clippy).

Example format:

> **Step 1: Refactor internal `now()` implementation**  
> **File:** `src/utc_timestamp.rs`  
> **Action:** Replace direct `coarsetime::Clock::recent_since_epoch()` calls with a small wrapper that handles fallbacks.  
> ```rust
> // NEW helper, near other constructors
> fn now_millis_since_epoch() -> i64 {
>     #[cfg(feature = "coarsetime-support")]
>     {
>         coarsetime::Clock::recent_since_epoch().as_millis() as i64
>     }
>     #[cfg(not(feature = "coarsetime-support"))]
>     {
>         chrono::Utc::now().timestamp_millis()
>     }
> }
> 
> impl Timestamp {
>     pub fn now() -> Self {
>         Self::from_milliseconds(now_millis_since_epoch())
>     }
> }
> ```
> **Command:**  
> `cargo test`  
> `cargo bench --bench timestamp_bench_coarsetime -- --warm-up-time 0.2 --measurement-time 0.5`

**3. Validation & Verification**

- Provide precise test commands (e.g., `cargo test`, `cargo bench`, `cargo clippy`, `cargo fmt -- --check`).
- Specify expected outcomes:
  - Tests all pass.
  - Benchmark results show acceptable performance behavior.
- Identify potential side effects or regression risks:
  - Changes in `now()` semantics,
  - Edge cases for negative timestamps,
  - Serialization behavior under `serde-support`.

---

### Prohibitions (Hard Constraints for LLMs Working on `fast_utc`)

- **Do NOT** provide code or commands without first completing Phase 1 analysis.
- **Do NOT** use hedging language like “probably,” “I think,” or “you might consider.” State analysis and actions definitively.
- **Do NOT** propose solutions that address only superficial symptoms when a deeper systemic issue was identified.
- **Do NOT** ignore performance and correctness constraints identified in analysis (e.g., monotonicity, overflow, feature interactions).

By adhering to this protocol, any contributions or automated changes to `@capital-levite/fast_utc` will be more robust, well‑reasoned, and aligned with the project’s goals of providing fast, reliable UTC time primitives.
