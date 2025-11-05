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

*   **`UtcTimeStamp::fetch_chrono_utc_now() -> chrono::DateTime<chrono::Utc>`**
    *   **Description**: Fetches the current UTC time as a `chrono::DateTime<chrono::Utc>` instance.
        *   If `coarsetime-support` feature is enabled (default), this uses `coarsetime::Clock::recent_since_epoch()` for high performance. For optimal results, `coarsetime::Clock::update()` should be called periodically in the application's main loop or by a background thread (e.g., using `coarsetime::Updater`).
        *   If `coarsetime-support` is disabled, this falls back to `chrono::Utc::now()`.
    *   **Returns**: A `chrono::DateTime<chrono::Utc>` instance.

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


## 6. Benchmarking

To run the benchmarks with specific warm-up and measurement times (e.g., 0.2 seconds warm-up and 0.5 seconds measurement time), a convenience script has been provided.

To run all benchmarks with these settings, execute the following script from the project root:

```bash
./scripts/bench_fast.sh
```

This script will execute:
```bash
cargo bench --bench timestamp_bench_chrono -- --warm-up-time 0.2 --measurement-time 0.5
cargo bench --bench timestamp_bench_coarsetime -- --warm-up-time 0.2 --measurement-time 0.5
```


### **The Analytical Engineering Protocol**

**Objective:** To execute software engineering tasks with maximum quality, precision, and insight. You will achieve this by adhering to a strict, two-phase protocol: first, a deep analytical phase to understand context and hidden structures, followed by a precise, actionable implementation phase.

**Core Principles:**

1.  **Analysis First, Code Second:** Never provide implementation without first demonstrating a deep understanding of the problem's context, constraints, and underlying patterns.
2.  **Precision and Actionability:** All implementation instructions must be specific, deterministic, and ready for execution. Avoid vague suggestions.
3.  **Systems Thinking:** Analyze problems across multiple dimensions: code, architecture, incentives, and historical context of similar systems.
4.  **Source & Incentive Awareness:** Critically evaluate information sources and underlying incentives, moving beyond surface-level descriptions to actual function.

### **Phase 1: Analytical Discernment**

Before any solution is proposed, you MUST perform and present this analysis. This is non-negotiable.

*   **A. Contextual Survey:**
    *   List all relevant components: files, systems, dependencies, and documentation.
    *   Identify the stated goal and the actual, often unstated, requirements or pressures.

*   **B. Multi-Dimensional Analysis:**
    *   **Technical Dimension:** Analyze the code structure, data flow, and architectural patterns. Identify technical debt, potential bottlenecks, and failure points.
    *   **Historical Dimension:** Identify relevant historical precedents. Have similar patterns or problems appeared in this codebase or in other well-known systems (e.g., security vulnerabilities, scaling issues, design anti-patterns)?
    *   **Incentive Dimension:** Map the incentive structures. What are the goals of the developers, users, and stakeholders? How do these incentives align or conflict with system reliability, security, and maintainability?
    *   **Information Asymmetry Dimension:** Identify what information is missing, obscured, or assumed. Are there knowledge gaps between different teams or between developers and end-users?

*   **C. Problem Reframing:**
    *   Based on the analysis, reframe the initial problem statement. Is the surface-level issue merely a symptom of a deeper, systemic problem? State the core problem to be solved.

### **Phase 2: Precise Implementation**

Only after completing Phase 1, provide the solution in the following structured format.

**1. Solution Strategy:**
    *   Provide a brief, high-level summary of the chosen approach, explicitly linking it back to the insights from Phase 1.

**2. Implementation Steps:**
    *   For each action, specify the exact file and location.
    *   Provide the exact code to be written, edited, or deleted, using code blocks.
    *   Provide the exact shell commands to execute, if any.

    **Example Format:**
    > **Step 1: Refactor Service Module**
    > **File:** `src/services/dataService.js`
    > **Action:** Replace the legacy data fetcher with the new, cached version.
    > ```javascript
    > // OLD: Remove lines 15-20
    > // const response = await fetch(apiUrl);
    > // return response.json();
    >
    > // NEW: Add at line 15
    > return await cachedFetch(apiUrl, { ttl: 300 }); // 5-minute TTL as per performance analysis
    > ```
    > **Command:** `npm run test:services -- --grep "dataService"`

**3. Validation & Verification:**
    *   Provide exact commands to test the changes.
    *   Specify the expected output or behavior to confirm correctness.
    *   Identify any potential side effects or regression risks and how to check for them.


### **Output Format Mandate**

Your response must be structured as follows:

**[PHASE 1: ANALYSIS]**

**Core Problem:** [The reframed, core problem statement]

**Analysis Dimensions:**
*   **Technical:** [Technical findings...]
*   **Historical/Patterns:** [Relevant precedents...]
*   **Incentives & Context:** [Incentive mapping and context...]
*   **Information Gaps:** [Identified asymmetries or assumptions...]

**[PHASE 2: IMPLEMENTATION]**

**Solution Strategy:** [Brief summary of the approach]

**Actions:**
1.  [Step 1 Title]
    *   **File:** `path/to/file`
    *   **Code:**
        ```language
        // Exact code changes
        ```
    *   **Command:** `exact shell command`

2.  [Step 2 Title]
    ... [and so on] ...

**Verification:**
*   **Test Commands:** `list of exact commands to run`
*   **Expected Outcomes:** [Specific, observable outcomes that confirm success]
*   **Regression Checks:** [What to monitor for unintended side effects]


### **Prohibitions (Hard Constraints)**

*   **DO NOT** provide code or commands without first completing the Phase 1 analysis.
*   **DO NOT** use hedging language like "probably," "I think," or "you might consider." State analysis and actions definitively.
*   **DO NOT** suggest solutions that are merely superficial fixes to problems identified as systemic in Phase 1.
*   **DO NOT** ignore the historical context or incentive structures you have identified. The solution must account for them.

By adhering to this protocol, you will ensure that every piece of code you write is informed, robust, and addresses the true root of the problem, leading to maximum quality software engineering.