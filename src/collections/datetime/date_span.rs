use std::{
    cmp,
    ffi::{c_void, CStr, CString},
    fmt::Debug,
    hash::Hash,
    ops::{BitAnd, Range, RangeInclusive},
    ptr,
};

use chrono::{Datelike, NaiveDate, TimeDelta};

use crate::{
    collections::{base::*, datetime::DAYS_UNTIL_2000},
    errors::ParseError,
    utils::from_interval,
};

pub struct DateSpan {
    _inner: ptr::NonNull<meos_sys::Span>,
}

impl Drop for DateSpan {
    fn drop(&mut self) {
        unsafe {
            libc::free(self._inner.as_ptr() as *mut c_void);
        }
    }
}

impl Collection for DateSpan {
    impl_collection!(span, NaiveDate);

    fn contains(&self, content: &NaiveDate) -> bool {
        unsafe { meos_sys::contains_span_date(self.inner(), content.num_days_from_ce()) }
    }
}

impl Span for DateSpan {
    type SubsetType = TimeDelta;
    fn inner(&self) -> *const meos_sys::Span {
        self._inner.as_ptr()
    }

    /// Creates a new `DateSpan` from an inner podateer to a `meos_sys::Span`.
    ///
    /// # Arguments
    /// * `inner` - A podateer to the inner `meos_sys::Span`.
    ///
    /// ## Returns
    /// * A new `DateSpan` instance.
    fn from_inner(inner: *mut meos_sys::Span) -> Self {
        Self {
            _inner: ptr::NonNull::new(inner).expect("No null pointers allowed"),
        }
    }

    /// Returns the lower bound of the span.
    ///
    /// ## Returns
    /// * The lower bound as a `NaiveDate`.
    ///
    /// ## Example
    /// ```
    /// # use meos::DateSpan;
    /// # use meos::Span;
    /// # use chrono::naive::NaiveDate;
    ///
    /// let from_ymd_opt = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
    ///
    /// let span: DateSpan = (from_ymd_opt(2023, 1, 1)..from_ymd_opt(2023, 1, 15)).into();
    /// let lower = span.lower();
    /// assert_eq!(lower, from_ymd_opt(2023, 1, 1));
    /// ```
    fn lower(&self) -> Self::Type {
        let num_of_days = unsafe { meos_sys::datespan_lower(self.inner()) };
        NaiveDate::from_num_days_from_ce_opt(num_of_days)
            .expect("Wrong date returned from meos")
            .checked_add_days(DAYS_UNTIL_2000)
            .unwrap()
    }

    /// Returns the upper bound of the span.
    ///
    /// ## Returns
    /// * The upper bound as a `NaiveDate`.
    ///
    /// ## Example
    /// ```
    /// # use meos::DateSpan;
    /// # use meos::Span;
    /// # use chrono::naive::NaiveDate;
    ///
    /// let from_ymd_opt = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
    ///
    /// let span: DateSpan = (from_ymd_opt(2023, 1, 1)..from_ymd_opt(2023, 1, 15)).into();
    /// let upper = span.upper();
    /// assert_eq!(upper, from_ymd_opt(2023, 1, 15));
    /// ```
    fn upper(&self) -> Self::Type {
        let num_of_days = unsafe { meos_sys::datespan_upper(self.inner()) };
        NaiveDate::from_num_days_from_ce_opt(num_of_days)
            .expect("Wrong date returned from meos")
            .checked_add_days(DAYS_UNTIL_2000)
            .unwrap()
    }

    /// Return a new `DateSpan` with the lower and upper bounds shifted by `delta`.
    ///
    /// # Arguments
    /// * `delta` - The value to shift by, as a `NaiveDate`.
    ///
    /// # Returns
    /// A new `DateSpan` instance.
    ///
    /// # Example
    /// ```
    /// # use meos::DateSpan;
    /// # use meos::Span;
    /// use chrono::naive::NaiveDate;
    /// use chrono::TimeDelta;
    ///
    /// let from_ymd_opt = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
    ///
    /// let span: DateSpan = (from_ymd_opt(2023, 1, 1)..from_ymd_opt(2023, 1, 15)).into();
    /// let shifted_span = span.shift(TimeDelta::days(5));
    /// let expected_span: DateSpan = (from_ymd_opt(2023, 1, 6)..from_ymd_opt(2023, 1, 20)).into();
    /// assert_eq!(shifted_span, expected_span);
    /// ```
    fn shift(&self, delta: TimeDelta) -> DateSpan {
        self.shift_scale(Some(delta), None)
    }

    /// Return a new `DateSpan` with the lower and upper bounds scaled so that the width is `width`.
    ///
    /// # Arguments
    /// * `width` - The new width, as a `NaiveDate`.
    ///
    /// # Returns
    /// A new `DateSpan` instance.
    ///
    /// # Example
    /// ```
    /// # use meos::DateSpan;
    /// # use meos::Span;
    /// use chrono::naive::NaiveDate;
    /// use chrono::TimeDelta;
    ///
    /// let from_ymd_opt = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
    ///
    /// let span: DateSpan = (from_ymd_opt(2023, 1, 1)..from_ymd_opt(2023, 1, 15)).into();
    /// let scaled_span = span.scale(TimeDelta::days(5));
    /// let expected_span: DateSpan = (from_ymd_opt(2023, 1, 1)..from_ymd_opt(2023, 1, 07)).into();
    /// assert_eq!(scaled_span, expected_span);
    /// ```
    fn scale(&self, width: TimeDelta) -> DateSpan {
        self.shift_scale(None, Some(width))
    }

    /// Return a new `DateSpan` with the lower and upper bounds shifted by `delta` and scaled so that the width is `width`.
    ///
    /// # Arguments
    /// * `delta` - The value to shift by, as a `NaiveDate`.
    /// * `width` - The new width, as a `NaiveDate`.
    ///
    /// # Returns
    /// A new `DateSpan` instance.
    ///
    /// # Example
    /// ```
    /// # use meos::DateSpan;
    /// # use meos::Span;
    /// use chrono::naive::NaiveDate;
    /// use chrono::TimeDelta;
    ///
    /// let from_ymd_opt = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
    ///
    /// let span: DateSpan = (from_ymd_opt(2023, 1, 1)..from_ymd_opt(2023, 1, 15)).into();
    /// let shifted_scaled_span = span.shift_scale(Some(TimeDelta::days(5)), Some(TimeDelta::days(10)));
    /// let expected_span: DateSpan = (from_ymd_opt(2023, 1, 6)..from_ymd_opt(2023, 1, 17)).into();
    /// assert_eq!(shifted_scaled_span, expected_span);
    /// ```
    fn shift_scale(&self, delta: Option<TimeDelta>, width: Option<TimeDelta>) -> DateSpan {
        let d = delta
            .unwrap_or_default()
            .num_days()
            .try_into()
            .expect("Number too big");
        let w = width
            .unwrap_or_default()
            .num_days()
            .try_into()
            .expect("Number too big");
        let modified = unsafe {
            meos_sys::datespan_shift_scale(
                self._inner.as_ptr(),
                d,
                w,
                delta.is_some(),
                width.is_some(),
            )
        };
        DateSpan::from_inner(modified)
    }

    /// Calculates the distance between this `DateSpan` and a specific timestamp (`value`).
    ///
    /// ## Arguments
    /// * `value` - Anvalue `DateSpan` to calculate the distance to.
    ///
    /// ## Returns
    /// A `TimeDelta` representing the distance in seconds between the two spans.
    ///
    /// ## Example
    /// ```
    /// # use meos::DateSpan;
    /// # use meos::meos_initialize;
    /// use std::str::FromStr;
    /// # use meos::Span;
    /// use chrono::TimeDelta;
    /// # meos_initialize();
    /// let span1 = DateSpan::from_str("[2019-09-08, 2019-09-10]").unwrap();
    /// let span2 = DateSpan::from_str("[2019-09-12, 2019-09-14]").unwrap();
    /// let distance = span1.distance_to_span(&span2);
    /// assert_eq!(distance, TimeDelta::days(2));
    /// ```
    fn distance_to_value(&self, other: &Self::Type) -> TimeDelta {
        unsafe {
            TimeDelta::days(
                meos_sys::distance_span_date(
                    self.inner(),
                    other
                        .checked_sub_days(DAYS_UNTIL_2000)
                        .unwrap()
                        .num_days_from_ce(),
                )
                .into(),
            )
        }
    }

    /// Calculates the distance between this `DateSpan` and another `DateSpan`.
    ///
    /// ## Arguments
    /// * `other` - Another `DateSpan` to calculate the distance to.
    ///
    /// ## Returns
    /// A `TimeDelta` representing the distance in seconds between the two spans.
    ///
    /// ## Example
    /// ```
    /// # use meos::DateSpan;
    /// # use meos::Span;
    /// # use chrono::{TimeDelta, TimeZone, Utc};
    /// # use meos::meos_initialize;
    /// use std::str::FromStr;
    /// # meos_initialize();
    /// let span_set1 = DateSpan::from_str("[2019-09-08, 2019-09-10]").unwrap();
    /// let span_set2 = DateSpan::from_str("[2018-08-07, 2018-08-17]").unwrap();
    /// let distance = span_set1.distance_to_span(&span_set2);
    /// assert_eq!(distance, TimeDelta::days(387));
    /// ```
    fn distance_to_span(&self, other: &Self) -> TimeDelta {
        unsafe {
            TimeDelta::days(
                meos_sys::distance_datespan_datespan(self.inner(), other.inner()).into(),
            )
        }
    }
}

impl DateSpan {
    pub fn duration(&self) -> TimeDelta {
        from_interval(unsafe { meos_sys::datespan_duration(self._inner.as_ptr()).read() })
    }
}

impl Clone for DateSpan {
    fn clone(&self) -> Self {
        unsafe { Self::from_inner(meos_sys::span_copy(self._inner.as_ptr())) }
    }
}

impl Hash for DateSpan {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let hash = unsafe { meos_sys::span_hash(self._inner.as_ptr()) };
        state.write_u32(hash);

        let _ = state.finish();
    }
}

impl std::str::FromStr for DateSpan {
    type Err = ParseError;
    /// Parses a `DateSpan` from a string representation.
    ///
    /// ## Arguments
    /// * `string` - A string slice containing the representation.
    ///
    /// ## Returns
    /// * A `DateSpan` instance.
    ///
    /// ## Errors
    /// * Returns `ParseSpanError` if the string cannot be parsed.
    ///
    /// ## Example
    /// ```
    /// # use meos::DateSpan;
    /// # use meos::Span;
    /// # use std::str::FromStr;
    /// # use meos::meos_initialize;
    /// use chrono::NaiveDate;
    /// # meos_initialize();
    /// let from_ymd_opt = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
    ///
    /// let span: DateSpan = "(2019-09-08, 2019-09-10)".parse().expect("Failed to parse span");
    /// assert_eq!(span.lower(), from_ymd_opt(2019, 9, 9));
    /// assert_eq!(span.upper(), from_ymd_opt(2019, 9, 10));
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        CString::new(string).map_err(|_| ParseError).map(|string| {
            let inner = unsafe { meos_sys::datespan_in(string.as_ptr()) };
            Self::from_inner(inner)
        })
    }
}

impl cmp::PartialEq for DateSpan {
    /// Checks if two `DateSpan` instances are equal.
    ///
    /// # Arguments
    /// * `other` - Another `DateSpan` instance.
    ///
    /// ## Returns
    /// * `true` if the spans are equal, `false` otherwise.
    ///
    /// ## Example
    /// ```
    /// # use meos::DateSpan;
    /// # use meos::Span;
    /// use chrono::naive::NaiveDate;
    ///
    /// let from_ymd_opt = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
    ///
    /// let span1: DateSpan = (from_ymd_opt(1, 1, 1)..from_ymd_opt(2, 2, 2)).into();
    /// let span2: DateSpan = (from_ymd_opt(1, 1, 1)..from_ymd_opt(2, 2, 2)).into();
    /// assert_eq!(span1, span2);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        unsafe { meos_sys::span_eq(self._inner.as_ptr(), other._inner.as_ptr()) }
    }
}

impl cmp::Eq for DateSpan {}

impl From<Range<NaiveDate>> for DateSpan {
    fn from(Range { start, end }: Range<NaiveDate>) -> Self {
        let inner = unsafe {
            meos_sys::datespan_make(
                start
                    .checked_sub_days(DAYS_UNTIL_2000)
                    .unwrap()
                    .num_days_from_ce(),
                end.checked_sub_days(DAYS_UNTIL_2000)
                    .unwrap()
                    .num_days_from_ce(),
                true,
                false,
            )
        };
        Self::from_inner(inner)
    }
}

impl From<RangeInclusive<NaiveDate>> for DateSpan {
    fn from(range: RangeInclusive<NaiveDate>) -> Self {
        let inner = unsafe {
            meos_sys::datespan_make(
                range
                    .start()
                    .checked_sub_days(DAYS_UNTIL_2000)
                    .unwrap()
                    .num_days_from_ce(),
                range
                    .end()
                    .checked_sub_days(DAYS_UNTIL_2000)
                    .unwrap()
                    .num_days_from_ce(),
                true,
                true,
            )
        };
        Self::from_inner(inner)
    }
}

impl Debug for DateSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out_str = unsafe { meos_sys::datespan_out(self._inner.as_ptr()) };
        let c_str = unsafe { CStr::from_ptr(out_str) };
        let str = c_str.to_str().map_err(|_| std::fmt::Error)?;
        let result = f.write_str(str);
        unsafe { libc::free(out_str as *mut c_void) };
        result
    }
}

// Implement BitAnd for dateersection with DateSpan
impl BitAnd for DateSpan {
    type Output = Option<DateSpan>;
    /// Computes the dateersection of two `DateSpan` instances.
    ///
    /// # Arguments
    /// * `other` - Another `DateSpan` instance.
    ///
    /// ## Returns
    /// * An `Option<DateSpan>` containing the dateersection, or `None` if there is no dateersection.
    ///
    /// ## Example
    /// ```
    /// # use meos::DateSpan;
    /// # use meos::Span;
    /// # use std::str::FromStr;
    /// use chrono::naive::NaiveDate;
    ///
    /// let from_ymd_opt = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();
    ///
    /// let span1: DateSpan = (from_ymd_opt(1, 1, 1)..from_ymd_opt(1, 1, 11)).into();
    /// let span2: DateSpan = (from_ymd_opt(1, 1, 9)..from_ymd_opt(2, 1, 11)).into();
    /// let date_intersection = (span1 & span2).unwrap();
    ///
    /// assert_eq!(date_intersection, (from_ymd_opt(1, 1, 9)..from_ymd_opt(1, 1, 11)).into())
    /// ```
    fn bitand(self, other: Self) -> Self::Output {
        // Replace with actual function call or logic
        let result = unsafe {
            meos_sys::intersection_span_span(self._inner.as_ptr(), other._inner.as_ptr())
        };
        if !result.is_null() {
            Some(DateSpan::from_inner(result))
        } else {
            None
        }
    }
}

impl PartialOrd for DateSpan {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let cmp = unsafe { meos_sys::span_cmp(self._inner.as_ptr(), other._inner.as_ptr()) };
        match cmp {
            -1 => Some(cmp::Ordering::Less),
            0 => Some(cmp::Ordering::Equal),
            1 => Some(cmp::Ordering::Greater),
            _ => None,
        }
    }
}

impl Ord for DateSpan {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).expect(
            "Unreachable since for non-null and same types spans, we only return -1, 0, or 1",
        )
    }
}
