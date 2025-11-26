use core::{fmt, ops};

#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "coarsetime-support")]
use coarsetime::Clock;

#[cfg(feature = "coarsetime-support")]
pub fn coarsetime_update() {
	coarsetime::Clock::update();
}

#[cfg(feature = "coarsetime-support")]
pub fn coarsetime_init_updater() {
	coarsetime::Updater::new(1).start().expect("Failed to start coarsetime updater");
}

// ============================================================================================== //
// [UTC timestamp]                                                                                //
// ============================================================================================== //

/// Represents a dumb but fast UTC timestamp.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct Timestamp(u64);

/// Display timestamp using chrono.
impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        chrono::DateTime::<chrono::Utc>::from(*self).fmt(f)
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Timestamp({})", self.0)
    }
}

/// Create a dumb timestamp from a chrono date time object.
impl From<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(other: chrono::DateTime<chrono::Utc>) -> Self {
        let nanos = other.timestamp_nanos_opt().unwrap_or(0);
        if nanos < 0 {
            Self(0) // Clamp negative timestamps to 0
        } else {
            Self(nanos as u64)
        }
    }
}

/// Create a chrono date time object from a dumb timestamp.
impl From<Timestamp> for chrono::DateTime<chrono::Utc> {
    fn from(other: Timestamp) -> Self {
        let nanoseconds_u64 = other.0;
        let nanos_in_sec: u64 = 1_000_000_000;

        // Safely convert u64 seconds to i64 seconds, clamping at i64::MAX if it overflows
        let sec_i64: i64 = nanoseconds_u64
            .checked_div(nanos_in_sec)
            .and_then(|s| s.try_into().ok())
            .unwrap_or(i64::MAX);

        let ns_u32: u32 = nanoseconds_u64
            .checked_rem(nanos_in_sec)
            .and_then(|n| n.try_into().ok())
            .unwrap_or(0);

        chrono::DateTime::<chrono::Utc>::from_timestamp(sec_i64, ns_u32).unwrap_or_else(|| {
            // Fallback for out-of-range timestamps or conversion issues
            chrono::DateTime::<chrono::Utc>::from_timestamp(0, 0).expect("0,0 is a valid timestamp")
        })
    }
}


impl Timestamp {
    /// Initialize a timestamp with 0, `1970-01-01 00:00:00 UTC`.
    #[inline]
    pub const fn zero() -> Self {
        Timestamp(0)
    }

    /// Initialize a timestamp using the current local time converted to UTC.
    #[cfg(not(feature = "coarsetime-support"))]
    pub fn now() -> Self {
        let nanos = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
        if nanos < 0 {
            Self(0)
        } else {
            Self(nanos as u64)
        }
    }

    /// Initialize a timestamp using the current local time converted to UTC, using `coarsetime`.
    /// For optimal performance, `coarsetime::Clock::update()` should be called periodically.
    #[cfg(feature = "coarsetime-support")]
    pub fn now() -> Self {
        Self(Clock::recent_since_epoch().as_nanos())
    }

    /// Fetches the current UTC time using `chrono::Utc::now()`.
    #[cfg(not(feature = "coarsetime-support"))]
    pub fn fetch_chrono_utc_now() -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }

    /// Fetches the current UTC time using `coarsetime` and converts it to `chrono::DateTime<chrono::Utc>`.
    /// For optimal performance, `coarsetime::Clock::update()` should be called periodically.
    #[cfg(feature = "coarsetime-support")]
    pub fn fetch_chrono_utc_now() -> chrono::DateTime<chrono::Utc> {
        let nanos = coarsetime::Clock::recent_since_epoch().as_nanos();
        Timestamp(nanos).into()
    }

    #[inline]
    pub const fn from_milliseconds(int: u64) -> Self {
        Timestamp(int * 1_000_000)
    }

    /// Explicit conversion from `u64` seconds.
    #[inline]
    pub const fn from_seconds(int: u64) -> Self {
        Timestamp(int * 1_000_000_000)
    }

    /// Explicit conversion from `u64` nanoseconds.
    #[inline]
    pub fn from_nanoseconds(int: u64) -> Self {
        Timestamp(int)
    }

    /// Explicit conversion to `u64` milliseconds.
    #[inline]
    pub const fn as_milliseconds(self) -> u64 {
        self.0 / 1_000_000
    }

    /// Explicit conversion to `u64` nanoseconds.
    #[inline]
    pub const fn as_nanoseconds(self) -> u64 {
        self.0
    }

    /// Align a timestamp to a given frequency.
    pub const fn align_to(self, freq: TimeDelta) -> Timestamp {
        self.align_to_anchored(Timestamp::zero(), freq)
    }

    /// Align a timestamp to a given frequency, with a time anchor.
    pub const fn align_to_anchored(self, anchor: Timestamp, freq: TimeDelta) -> Timestamp {
        // Perform arithmetic with i64 to handle potential negative intermediate results
        // then clamp back to u64
        let self_i64 = self.0 as i64;
        let anchor_i64 = anchor.0 as i64;
        let freq_i64 = freq.0; // TimeDelta is i64

        let aligned_i64 = (self_i64 - anchor_i64) / freq_i64 * freq_i64 + anchor_i64;
        Self(if aligned_i64 < 0 { 0 } else { aligned_i64 as u64 }) // Clamp to 0
    }

    /// Check whether the timestamp is 0 (`1970-01-01 00:00:00 UTC`).
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

/// Calculate the timestamp advanced by a timedelta.
impl ops::Add<TimeDelta> for Timestamp {
    type Output = Timestamp;

    fn add(self, rhs: TimeDelta) -> Self::Output {
        // Convert to i64 for arithmetic, then clamp to 0 and convert back to u64
        let result_i64 = (self.0 as i64) + rhs.0;
        Self(result_i64.max(0) as u64)
    }
}

impl ops::AddAssign<TimeDelta> for Timestamp {
    fn add_assign(&mut self, rhs: TimeDelta) {
        *self = *self + rhs;
    }
}

/// Calculate the timestamp lessened by a timedelta.
impl ops::Sub<TimeDelta> for Timestamp {
    type Output = Timestamp;

    fn sub(self, rhs: TimeDelta) -> Self::Output {
        // Convert to i64 for arithmetic, then clamp to 0 and convert back to u64
        let result_i64 = (self.0 as i64) - rhs.0;
        Self(result_i64.max(0) as u64)
    }
}

impl ops::SubAssign<TimeDelta> for Timestamp {
    fn sub_assign(&mut self, rhs: TimeDelta) {
        *self = *self - rhs;
    }
}

/// Calculate signed timedelta between two timestamps.
impl ops::Sub<Timestamp> for Timestamp {
    type Output = TimeDelta;

    fn sub(self, rhs: Timestamp) -> Self::Output {
        TimeDelta((self.0 as i64) - (rhs.0 as i64))
    }
}

// /// How far away is the timestamp from being aligned to the given timedelta?
// impl ops::Rem<TimeDelta> for Timestamp {
//     type Output = TimeDelta;
//
//     fn rem(self, rhs: TimeDelta) -> Self::Output {
//         TimeDelta(self.0 % rhs.0)
//     }
// }

// ============================================================================================== //
// [TimeDelta]                                                                                    //
// ============================================================================================== //

/// Millisecond precision time delta.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct TimeDelta(i64);

/// Display timedelta using chrono.
impl fmt::Display for TimeDelta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        chrono::Duration::from(*self).fmt(f)
    }
}

/// Create a simple timedelta from a chrono duration.
impl From<chrono::Duration> for TimeDelta {
    fn from(other: chrono::Duration) -> Self {
        // chrono::Duration::num_nanoseconds() returns Option<i64>
        // If the duration is too large to fit in i64 nanoseconds, it returns None.
        // We handle this by clamping to 0, consistent with Timestamp's i64 nanosecond limits.
        Self(other.num_nanoseconds().unwrap_or(0))
    }
}

/// Create a chrono duration from a simple timedelta.
impl From<TimeDelta> for chrono::Duration {
    fn from(other: TimeDelta) -> Self {
        chrono::Duration::nanoseconds(other.0)
    }
}

impl ops::Add<TimeDelta> for TimeDelta {
    type Output = TimeDelta;

    fn add(self, rhs: TimeDelta) -> Self::Output {
        TimeDelta(self.0 + rhs.0)
    }
}

impl ops::Sub<TimeDelta> for TimeDelta {
    type Output = TimeDelta;

    fn sub(self, rhs: TimeDelta) -> Self::Output {
        TimeDelta(self.0 - rhs.0)
    }
}

/// Multiply the delta to be n times as long.
impl ops::Mul<i64> for TimeDelta {
    type Output = TimeDelta;

    fn mul(self, rhs: i64) -> Self::Output {
        TimeDelta(self.0 * rhs)
    }
}

/// Shorten the delta by a given factor. Integer div.
impl ops::Div<i64> for TimeDelta {
    type Output = TimeDelta;

    fn div(self, rhs: i64) -> Self::Output {
        TimeDelta(self.0 / rhs)
    }
}

/// How many times does the timestamp fit into another?
impl ops::Div<TimeDelta> for TimeDelta {
    type Output = i64;

    fn div(self, rhs: TimeDelta) -> Self::Output {
        self.0 / rhs.0
    }
}

/// How far away is the delta from being aligned to another delta?
impl ops::Rem<TimeDelta> for TimeDelta {
    type Output = TimeDelta;

    fn rem(self, rhs: TimeDelta) -> Self::Output {
        TimeDelta(self.0 % rhs.0)
    }
}

/// Explicit conversion from and to `i64`.
impl TimeDelta {
    #[inline]
    pub const fn zero() -> Self {
        TimeDelta(0)
    }

    #[inline]
    pub const fn from_hours(int: i64) -> Self {
        TimeDelta(int * 60 * 60 * 1_000_000_000)
    }

    #[inline]
    pub const fn from_minutes(int: i64) -> Self {
        TimeDelta(int * 60 * 1_000_000_000)
    }

    #[inline]
    pub const fn from_seconds(int: i64) -> Self {
        TimeDelta(int * 1_000_000_000)
    }

    #[inline]
    pub const fn from_milliseconds(int: i64) -> Self {
        TimeDelta(int * 1_000_000)
    }

    #[inline]
    pub const fn from_nanoseconds(int: i64) -> Self {
        TimeDelta(int)
    }

    #[inline]
    pub const fn as_milliseconds(self) -> i64 {
        self.0 / 1_000_000
    }

    #[inline]
    pub const fn as_nanoseconds(self) -> i64 {
        self.0
    }
} // This brace was missing

// ============================================================================================== //
// [TimeRange]                                                                                    //
// ============================================================================================== //

/// An iterator looping over dates given a time delta as step.
///
/// The range is either right open or right closed depending on the
/// constructor chosen, but always left closed.
///
/// Examples:
///
/// ```
/// use fast_utc::TimeRange;
/// use chrono::{offset::TimeZone, Duration, Utc};
///
/// let start = Utc.with_ymd_and_hms(2019, 4, 14, 0, 0, 0).unwrap();
/// let end = Utc.with_ymd_and_hms(2019, 4, 16, 0, 0, 0).unwrap();
/// let step = Duration::hours(12);
/// let tr: Vec<_> = TimeRange::right_closed(start, end, step).collect();
///
/// assert_eq!(tr, vec![
///     Utc.with_ymd_and_hms(2019, 4, 14, 0, 0, 0).unwrap().into(),
///     Utc.with_ymd_and_hms(2019, 4, 14, 12, 0, 0).unwrap().into(),
///     Utc.with_ymd_and_hms(2019, 4, 15, 0, 0, 0).unwrap().into(),
///     Utc.with_ymd_and_hms(2019, 4, 15, 12, 0, 0).unwrap().into(),
///     Utc.with_ymd_and_hms(2019, 4, 16, 0, 0, 0).unwrap().into(),
/// ]);
/// ```
#[derive(Debug)]
pub struct TimeRange {
    cur: Timestamp,
    end: Timestamp,
    step: TimeDelta,
    right_closed: bool,
}

impl TimeRange {
    /// Create a time range that includes the end date.
    pub fn right_closed(
        start: impl Into<Timestamp>,
        end: impl Into<Timestamp>,
        step: impl Into<TimeDelta>,
    ) -> Self {
        TimeRange {
            cur: start.into(),
            end: end.into(),
            step: step.into(),
            right_closed: true,
        }
    }

    /// Create a time range that excludes the end date.
    pub fn right_open(
        start: impl Into<Timestamp>,
        end: impl Into<Timestamp>,
        step: impl Into<TimeDelta>,
    ) -> Self {
        TimeRange {
            cur: start.into(),
            end: end.into(),
            step: step.into(),
            right_closed: false,
        }
    }
}

impl Iterator for TimeRange {
    type Item = Timestamp;

    fn next(&mut self) -> Option<Self::Item> {
        let exhausted = if self.right_closed {
            self.cur > self.end
        } else {
            self.cur >= self.end
        };

        if exhausted {
            None
        } else {
            let cur = self.cur;
            self.cur += self.step;
            Some(cur)
        }
    }
}

// ============================================================================================== //
// [Tests]                                                                                        //
// ============================================================================================== //

#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::{offset::TimeZone, Duration, Utc};

    #[test]
    fn open_time_range() {
        let start = Utc.with_ymd_and_hms(2019, 4, 14, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2019, 4, 16, 0, 0, 0).unwrap();
        let step = Duration::hours(12);
        let tr: Vec<_> = Iterator::collect(TimeRange::right_closed(start, end, step));
        assert_eq!(tr, vec![
            Utc.with_ymd_and_hms(2019, 4, 14, 0, 0, 0).unwrap().into(),
            Utc.with_ymd_and_hms(2019, 4, 14, 12, 0, 0).unwrap().into(),
            Utc.with_ymd_and_hms(2019, 4, 15, 0, 0, 0).unwrap().into(),
            Utc.with_ymd_and_hms(2019, 4, 15, 12, 0, 0).unwrap().into(),
            Utc.with_ymd_and_hms(2019, 4, 16, 0, 0, 0).unwrap().into(),
        ]);
    }

    #[test]
    fn timestamp_and_delta_vs_chrono() {
        let c_dt = Utc.with_ymd_and_hms(2019, 3, 13, 16, 14, 9).unwrap();
        let c_td = Duration::nanoseconds(123456000000); // 123456 milliseconds as nanoseconds

        let my_dt = Timestamp::from(c_dt.clone());
        let my_td = TimeDelta::from_nanoseconds(123456000000); // 123456 milliseconds as nanoseconds
        assert_eq!(TimeDelta::from(c_td.clone()), my_td);

        let c_result = c_dt + c_td * 555;
        let my_result = my_dt + my_td * 555;
        assert_eq!(Timestamp::from(c_result.clone()), my_result);
    }

    #[test]
    fn timestamp_ord_eq() {
        let ts1: Timestamp = Timestamp::from_nanoseconds(111);
        let ts2: Timestamp = Timestamp::from_nanoseconds(222);
        let ts3: Timestamp = Timestamp::from_nanoseconds(222);

        assert!(ts1 < ts2);
        assert!(ts2 > ts1);
        assert!(ts1 <= ts2);
        assert!(ts2 >= ts3);
        assert!(ts2 <= ts3);
        assert!(ts2 >= ts3);
        assert_eq!(ts2, ts3);
        assert_ne!(ts1, ts3);
    }

            #[test]

            fn align_to_anchored() {

                let day_naive = chrono::NaiveDate::from_ymd_opt(2020, 9, 28).unwrap();

                let ts: Timestamp = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(

                    day_naive.and_hms_opt(19, 32, 51).unwrap(),

                    chrono::Utc,

                ).into();

        

                assert_eq!(

                    ts.align_to_anchored(

                        chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(

                            day_naive.and_hms_opt(0, 0, 0).unwrap(),

                            chrono::Utc,

                        ).into(),

                        TimeDelta::from_seconds(60 * 5)

                    ),

                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(

                        day_naive.and_hms_opt(19, 30, 0).unwrap(),

                        chrono::Utc,

                    ).into(),

                );

        

                assert_eq!(

                    ts.align_to_anchored(

                        chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(

                            day_naive.and_hms_opt(9 /* irrelevant */, 1, 3).unwrap(),

                            chrono::Utc,

                        ).into(),

                        TimeDelta::from_seconds(60 * 5)

                    ),

                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(

                        day_naive.and_hms_opt(19, 31, 3).unwrap(),

                        chrono::Utc,

                    ).into(),

                );

            }

    #[test]
    fn align_to_anchored_eq() {
        let day_naive = chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let anchor: Timestamp = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            day_naive.and_hms_opt(0, 0, 0).unwrap(),
            chrono::Utc,
        ).into();
        let freq = TimeDelta::from_seconds(5 * 60);

        let ts1: Timestamp = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            day_naive.and_hms_opt(12, 1, 11).unwrap(),
            chrono::Utc,
        ).into();
        let ts2: Timestamp = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            day_naive.and_hms_opt(12, 4, 11).unwrap(),
            chrono::Utc,
        ).into();
        assert_eq!(
            ts1.align_to_anchored(anchor, freq),
            ts2.align_to_anchored(anchor, freq),
        );
    }

    #[cfg(feature = "coarsetime-support")]
    #[test]
    fn coarsetime_now_test() {
        use core::time::Duration;
        coarsetime::Clock::update();
        let coarsetime_now = Timestamp::now();
        std::thread::sleep(Duration::from_millis(10));
        let chrono_now = chrono::Utc::now();
        let diff = (chrono_now.timestamp_nanos_opt().unwrap_or(0) - (coarsetime_now.as_nanoseconds() as i64)).abs();
        // Allow for a small difference due to the nature of coarsetime and thread sleep.
        // 50ms = 50_000_000ns
        assert!(diff < 50_000_000, "Difference was: {}", diff);
    }

    #[test]
    fn test_fetch_chrono_utc_now() {
        use chrono::Utc;
        #[cfg(feature = "coarsetime-support")]
        coarsetime::Clock::update();
        let now = Timestamp::fetch_chrono_utc_now();
        let chrono_now = Utc::now();
        // Allow for a small difference due to execution time
        // 50ms = 50_000_000ns
        let diff = (chrono_now.timestamp_nanos_opt().unwrap_or(0) - now.timestamp_nanos_opt().unwrap_or(0)).abs();
        assert!(diff < 50_000_000, "Difference was: {}", diff);
    }
}

// ============================================================================================== //

