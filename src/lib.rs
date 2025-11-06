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
pub struct UtcTimeStamp(i64);

/// Display timestamp using chrono.
impl fmt::Display for UtcTimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        chrono::DateTime::<chrono::Utc>::from(*self).fmt(f)
    }
}

impl fmt::Debug for UtcTimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UtcTimeStamp({})", self.0)
    }
}

/// Create a dumb timestamp from a chrono date time object.
impl From<chrono::DateTime<chrono::Utc>> for UtcTimeStamp {
    fn from(other: chrono::DateTime<chrono::Utc>) -> Self {
        Self(other.timestamp_millis())
    }
}

/// Create a chrono date time object from a dumb timestamp.
impl From<UtcTimeStamp> for chrono::DateTime<chrono::Utc> {
    fn from(other: UtcTimeStamp) -> Self {
        let sec = other.0 / 1000;
        let ns = (other.0 % 1000 * 1_000_000) as u32;
        chrono::DateTime::<chrono::Utc>::from_timestamp(sec, ns).unwrap_or_else(|| {
            // Fallback for out-of-range timestamps, though unlikely with i64 millis
            chrono::DateTime::<chrono::Utc>::from_timestamp(0, 0).expect("0,0 is a valid timestamp")
        })
    }
}

pub fn now_millis() -> u64 {
	#[cfg(feature = "coarsetime-support")]
	{
		coarsetime::Clock::recent_since_epoch().as_millis()
	}
	#[cfg(not(feature = "coarsetime-support"))]
	{
		chrono::Utc::now().timestamp_millis() as u64
	}
}

pub fn now_nanos() -> u64 {
	#[cfg(feature = "coarsetime-support")]
	{
		coarsetime::Clock::recent_since_epoch().as_nanos() as u64
	}
	#[cfg(not(feature = "coarsetime-support"))]
	{
		(chrono::Utc::now().timestamp_millis() as u64) * 1_000_000
	}
}

impl UtcTimeStamp {
    /// Initialize a timestamp with 0, `1970-01-01 00:00:00 UTC`.
    #[inline]
    pub const fn zero() -> Self {
        UtcTimeStamp(0)
    }

    /// Initialize a timestamp using the current local time converted to UTC.
    #[cfg(not(feature = "coarsetime-support"))]
    pub fn now() -> Self {
        chrono::Utc::now().into()
    }

    /// Initialize a timestamp using the current local time converted to UTC, using `coarsetime`.
    /// For optimal performance, `coarsetime::Clock::update()` should be called periodically.
    #[cfg(feature = "coarsetime-support")]
    pub fn now() -> Self {
        Self::from_nanoseconds(Clock::recent_since_epoch().as_nanos())
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
        UtcTimeStamp::from_nanoseconds(nanos).into()
    }

    #[inline]
    pub const fn from_milliseconds(int: i64) -> Self {
        UtcTimeStamp(int)
    }

    /// Explicit conversion from `i64` seconds.
    #[inline]
    pub const fn from_seconds(int: i64) -> Self {
        UtcTimeStamp(int * 1000)
    }

    /// Explicit conversion from `u64` nanoseconds.
    #[inline]
    pub fn from_nanoseconds(int: u64) -> Self {
        UtcTimeStamp((int / 1_000_000) as i64)
    }

    /// Explicit conversion to `i64`.
    #[inline]
    pub const fn as_milliseconds(self) -> i64 {
        self.0
    }

    /// Align a timestamp to a given frequency.
    pub const fn align_to(self, freq: TimeDelta) -> UtcTimeStamp {
        self.align_to_anchored(UtcTimeStamp::zero(), freq)
    }

    /// Align a timestamp to a given frequency, with a time anchor.
    pub const fn align_to_anchored(self, anchor: UtcTimeStamp, freq: TimeDelta) -> UtcTimeStamp {
        UtcTimeStamp((self.0 - anchor.0) / freq.0 * freq.0 + anchor.0)
    }

    /// Check whether the timestamp is 0 (`1970-01-01 00:00:00 UTC`).
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

/// Calculate the timestamp advanced by a timedelta.
impl ops::Add<TimeDelta> for UtcTimeStamp {
    type Output = UtcTimeStamp;

    fn add(self, rhs: TimeDelta) -> Self::Output {
        UtcTimeStamp(self.0 + rhs.0)
    }
}

impl ops::AddAssign<TimeDelta> for UtcTimeStamp {
    fn add_assign(&mut self, rhs: TimeDelta) {
        *self = *self + rhs;
    }
}

/// Calculate the timestamp lessened by a timedelta.
impl ops::Sub<TimeDelta> for UtcTimeStamp {
    type Output = UtcTimeStamp;

    fn sub(self, rhs: TimeDelta) -> Self::Output {
        UtcTimeStamp(self.0 - rhs.0)
    }
}

impl ops::SubAssign<TimeDelta> for UtcTimeStamp {
    fn sub_assign(&mut self, rhs: TimeDelta) {
        *self = *self - rhs;
    }
}

/// Calculate signed timedelta between two timestamps.
impl ops::Sub<UtcTimeStamp> for UtcTimeStamp {
    type Output = TimeDelta;

    fn sub(self, rhs: UtcTimeStamp) -> Self::Output {
        TimeDelta(self.0 - rhs.0)
    }
}

// /// How far away is the timestamp from being aligned to the given timedelta?
// impl ops::Rem<TimeDelta> for UtcTimeStamp {
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
        Self(other.num_milliseconds())
    }
}

/// Create a chrono duration from a simple timedelta.
impl From<TimeDelta> for chrono::Duration {
    fn from(other: TimeDelta) -> Self {
        chrono::Duration::milliseconds(other.0)
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
        TimeDelta::from_minutes(int * 60)
    }

    #[inline]
    pub const fn from_minutes(int: i64) -> Self {
        TimeDelta::from_seconds(int * 60)
    }

    #[inline]
    pub const fn from_seconds(int: i64) -> Self {
        TimeDelta(int * 1000)
    }

    #[inline]
    pub const fn from_milliseconds(int: i64) -> Self {
        TimeDelta(int)
    }

    #[inline]
    pub const fn as_milliseconds(self) -> i64 {
        self.0
    }

    /// Check whether the timedelta is 0.
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if the timedelta is positive and
    /// `false` if it is zero or negative.
    #[inline]
    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }

    /// Returns `true` if the timedelta is negative and
    /// `false` if it is zero or positive.
    #[inline]
    pub const fn is_negative(self) -> bool {
        self.0 < 0
    }
}

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
    cur: UtcTimeStamp,
    end: UtcTimeStamp,
    step: TimeDelta,
    right_closed: bool,
}

impl TimeRange {
    /// Create a time range that includes the end date.
    pub fn right_closed(
        start: impl Into<UtcTimeStamp>,
        end: impl Into<UtcTimeStamp>,
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
        start: impl Into<UtcTimeStamp>,
        end: impl Into<UtcTimeStamp>,
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
    type Item = UtcTimeStamp;

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
        let c_td = Duration::milliseconds(123456);

        let my_dt = UtcTimeStamp::from(c_dt.clone());
        let my_td = TimeDelta::from_milliseconds(123456);
        assert_eq!(TimeDelta::from(c_td.clone()), my_td);

        let c_result = c_dt + c_td * 555;
        let my_result = my_dt + my_td * 555;
        assert_eq!(UtcTimeStamp::from(c_result.clone()), my_result);
    }

    #[test]
    fn timestamp_ord_eq() {
        let ts1: UtcTimeStamp = UtcTimeStamp::from_milliseconds(111);
        let ts2: UtcTimeStamp = UtcTimeStamp::from_milliseconds(222);
        let ts3: UtcTimeStamp = UtcTimeStamp::from_milliseconds(222);

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

                let ts: UtcTimeStamp = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(

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
        let anchor: UtcTimeStamp = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            day_naive.and_hms_opt(0, 0, 0).unwrap(),
            chrono::Utc,
        ).into();
        let freq = TimeDelta::from_seconds(5 * 60);

        let ts1: UtcTimeStamp = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            day_naive.and_hms_opt(12, 1, 11).unwrap(),
            chrono::Utc,
        ).into();
        let ts2: UtcTimeStamp = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
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
        let coarsetime_now = UtcTimeStamp::now();
        std::thread::sleep(Duration::from_millis(10));
        let chrono_now = chrono::Utc::now();
        let diff = (chrono_now.timestamp_millis() - coarsetime_now.as_milliseconds()).abs();
        // Allow for a small difference due to the nature of coarsetime and thread sleep.
        assert!(diff < 50, "Difference was: {}", diff);
    }

    #[test]
    fn test_fetch_chrono_utc_now() {
        use chrono::Utc;
        #[cfg(feature = "coarsetime-support")]
        coarsetime::Clock::update();
        let now = UtcTimeStamp::fetch_chrono_utc_now();
        let chrono_now = Utc::now();
        // Allow for a small difference due to execution time
        let diff = (chrono_now.timestamp_millis() - now.timestamp_millis()).abs();
        assert!(diff < 50, "Difference was: {}", diff);
    }
}

// ============================================================================================== //

