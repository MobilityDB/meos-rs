use std::{
    ffi::{CStr, CString},
    hash::Hash,
    ptr,
};

use crate::{
    collections::{
        base::*,
        datetime::{TsTzSpan, TsTzSpanSet},
    },
    factory,
    utils::{create_interval, from_interval, from_meos_timestamp, to_meos_timestamp},
    BoundingBox, MeosEnum,
};
use chrono::{DateTime, TimeDelta, TimeZone, Utc};

use super::{
    interpolation::TInterpolation, tbool::TBoolTrait, tinstant::TInstant, tsequence::TSequence,
    tsequence_set::TSequenceSet,
};

pub trait Temporal: Collection + Hash {
    type TI: TInstant;
    type TS: TSequence;
    type TSS: TSequenceSet;
    type TBB: BoundingBox;
    type Enum: MeosEnum;
    type TBoolType: TBoolTrait;

    fn from_inner_as_temporal(inner: *mut meos_sys::Temporal) -> Self;

    fn inner(&self) -> *const meos_sys::Temporal;

    /// Returns the bounding box of the temporal object.
    ///
    /// ## Returns
    /// The bounding box of the temporal object.
    fn bounding_box(&self) -> Self::TBB;

    /// Returns the interpolation method of the temporal object.
    ///
    /// ## Returns
    /// The interpolation method.
    fn interpolation(&self) -> TInterpolation {
        let string = unsafe { CStr::from_ptr(meos_sys::temporal_interp(self.inner())) };
        string.to_str().unwrap().parse().unwrap()
    }

    /// Returns the set of unique values in the temporal object.
    ///
    /// ## Returns
    /// A set of unique values.
    // fn value_set(&self) -> HashSet<Self::Type>;

    /// Returns the list of values taken by the temporal object.
    ///
    /// ## Returns
    /// A list of values.
    fn values(&self) -> Vec<Self::Type>;

    /// Returns the starting value of the temporal object.
    ///
    /// ## Returns
    /// The starting value.
    fn start_value(&self) -> Self::Type;

    /// Returns the ending value of the temporal object.
    ///
    /// ## Returns
    /// The ending value.
    fn end_value(&self) -> Self::Type;

    /// Returns the value of the temporal object at a specific timestamp.
    ///
    /// ## Arguments
    /// * `timestamp` - The timestamp.
    ///
    /// ## Returns
    /// The value at the given timestamp.
    fn value_at_timestamp<Tz: TimeZone>(&self, timestamp: DateTime<Tz>) -> Option<Self::Type>;

    /// Returns the time span on which the temporal object is defined.
    ///
    /// ## Returns
    /// The time span.
    #[doc(alias = "temporal_time")]
    fn time(&self) -> TsTzSpanSet {
        TsTzSpanSet::from_inner(unsafe { meos_sys::temporal_time(self.inner()) })
    }

    /// Returns the time span on which the temporal object is defined.
    ///
    /// ## Returns
    /// The time span.
    #[doc(alias = "temporal_to_tstzspan")]
    fn timespan(&self) -> TsTzSpan {
        unsafe { TsTzSpan::from_inner(meos_sys::temporal_to_tstzspan(self.inner())) }
    }

    /// Returns the duration of the temporal object.
    ///
    /// ## Arguments
    /// * `ignore_gaps` - Whether to ignore gaps in the temporal value.
    ///
    /// ## Returns
    /// The duration of the temporal object.
    #[doc(alias = "temporal_duration")]
    fn duration(&self, ignore_gaps: bool) -> TimeDelta {
        from_interval(unsafe { meos_sys::temporal_duration(self.inner(), ignore_gaps).read() })
    }

    /// Returns the number of instants in the temporal object.
    ///
    /// ## Returns
    /// The number of instants.
    #[doc(alias = "temporal_num_instants")]
    fn num_instants(&self) -> i32 {
        unsafe { meos_sys::temporal_num_instants(self.inner()) }
    }

    /// Returns the first instant in the temporal object.
    ///
    /// ## Returns
    /// The first instant.
    #[doc(alias = "temporal_start_instant")]
    fn start_instant(&self) -> Self::TI {
        <Self::TI as TInstant>::from_inner(unsafe {
            meos_sys::temporal_start_instant(self.inner())
        })
    }

    /// Returns the last instant in the temporal object.
    ///
    /// ## Returns
    /// The last instant.
    #[doc(alias = "temporal_end_instant")]
    fn end_instant(&self) -> Self::TI {
        <Self::TI as TInstant>::from_inner(unsafe { meos_sys::temporal_end_instant(self.inner()) })
    }

    /// Returns the instant with the minimum value in the temporal object.
    ///
    /// ## Returns
    /// The instant with the minimum value.
    #[doc(alias = "temporal_min_instant")]
    fn min_instant(&self) -> Self::TI {
        <Self::TI as TInstant>::from_inner(unsafe { meos_sys::temporal_min_instant(self.inner()) })
    }

    /// Returns the instant with the maximum value in the temporal object.
    ///
    /// ## Returns
    /// The instant with the maximum value.
    #[doc(alias = "temporal_max_instant")]
    fn max_instant(&self) -> Self::TI {
        <Self::TI as TInstant>::from_inner(unsafe { meos_sys::temporal_max_instant(self.inner()) })
    }

    /// Returns the n-th instant in the temporal object.
    ///
    /// ## Arguments
    /// * `n` - The index (0-based).
    ///
    /// ## Return
    /// The n-th instant if exists, None otherwise.
    #[doc(alias = "temporal_instant_n")]
    fn instant_n(&self, n: i32) -> Option<Self::TI> {
        let result = unsafe { meos_sys::temporal_instant_n(self.inner(), n) };
        if !result.is_null() {
            Some(<Self::TI as TInstant>::from_inner(result))
        } else {
            None
        }
    }

    /// Returns the list of instants in the temporal object.
    ///
    /// ## Returns
    /// A list of instants.
    #[doc(alias = "temporal_instants")]
    fn instants(&self) -> Vec<Self::TI> {
        let mut count = 0;
        unsafe {
            let instants = meos_sys::temporal_instants(self.inner(), ptr::addr_of_mut!(count));

            Vec::from_raw_parts(instants, count as usize, count as usize)
                .iter()
                .map(|&instant| <Self::TI as TInstant>::from_inner(instant))
                .collect()
        }
    }

    /// Returns the number of timestamps in the temporal object.
    ///
    /// ## Returns
    /// The number of timestamps.
    #[doc(alias = "temporal_num_timestamps")]
    fn num_timestamps(&self) -> i32 {
        unsafe { meos_sys::temporal_num_timestamps(self.inner()) }
    }

    /// Returns the first timestamp in the temporal object.
    ///
    /// ## Returns
    /// The first timestamp.
    #[doc(alias = "temporal_start_timestampz")]
    fn start_timestamp(&self) -> DateTime<Utc> {
        from_meos_timestamp(unsafe { meos_sys::temporal_start_timestamptz(self.inner()) })
    }

    /// Returns the last timestamp in the temporal object.
    ///
    /// ## Returns
    /// The last timestamp.
    #[doc(alias = "temporal_end_timestampz")]
    fn end_timestamp(&self) -> DateTime<Utc> {
        from_meos_timestamp(unsafe { meos_sys::temporal_end_timestamptz(self.inner()) })
    }

    /// Returns the n-th timestamp in the temporal object.
    ///
    /// ## Arguments
    /// * `n` - The index (0-based).
    ///
    /// ## Returns
    /// The n-th timestamp if exists, None otherwise.
    #[doc(alias = "temporal_timestampz_n")]
    fn timestamp_n(&self, n: i32) -> Option<DateTime<Utc>> {
        let mut timestamp = 0;
        unsafe {
            let success =
                meos_sys::temporal_timestamptz_n(self.inner(), n, ptr::addr_of_mut!(timestamp));
            if success {
                Some(from_meos_timestamp(timestamp))
            } else {
                None
            }
        }
    }

    /// Returns the list of timestamps in the temporal object.
    ///
    /// ## Returns
    /// A list of timestamps.
    #[doc(alias = "temporal_timestamps")]
    fn timestamps(&self) -> Vec<DateTime<Utc>> {
        let mut count = 0;
        let timestamps =
            unsafe { meos_sys::temporal_timestamps(self.inner(), ptr::addr_of_mut!(count)) };
        unsafe {
            Vec::from_raw_parts(timestamps, count as usize, count as usize)
                .iter()
                .map(|&timestamp| from_meos_timestamp(timestamp))
                .collect()
        }
    }

    /// Returns the list of segments in the temporal object.
    ///
    /// ## Returns
    /// A list of segments.
    #[doc(alias = "temporal_segments")]
    fn segments(&self) -> Vec<Self::TS> {
        let mut count = 0;
        let segments =
            unsafe { meos_sys::temporal_segments(self.inner(), ptr::addr_of_mut!(count)) };
        unsafe {
            Vec::from_raw_parts(segments, count as usize, count as usize)
                .iter()
                .map(|&segment| <Self::TS as TSequence>::from_inner(segment))
                .collect()
        }
    }

    // ------------------------- Transformations -------------------------------

    /// Returns a new `Temporal` object with the given interpolation.
    #[doc(alias = "temporal_set_interp")]
    fn set_interpolation(&self, interpolation: TInterpolation) -> Self {
        Self::from_inner_as_temporal(unsafe {
            meos_sys::temporal_set_interp(self.inner(), interpolation as u32)
        })
    }

    /// Returns a new `Temporal` with the temporal dimension shifted by `delta`.
    ///
    /// ## Arguments
    /// * `delta` - TimeDelta to shift the temporal dimension.
    #[doc(alias = "temporal_shift_scale_time")]
    fn shift_time(&self, delta: TimeDelta) -> Self {
        self.shift_scale_time(Some(delta), None)
    }

    /// Returns a new `Temporal` scaled so the temporal dimension has duration `duration`.
    ///
    /// ## Arguments
    /// * `duration` - TimeDelta representing the new temporal duration.
    #[doc(alias = "temporal_shift_scale_time")]
    fn scale_time(&self, duration: TimeDelta) -> Self {
        self.shift_scale_time(None, Some(duration))
    }

    /// Returns a new `Temporal` with the time dimension shifted and scaled.
    ///
    /// ## Arguments
    /// * `shift` - TimeDelta to shift the time dimension.
    /// * `duration` - TimeDelta representing the new temporal duration.
    #[doc(alias = "temporal_shift_scale_time")]
    fn shift_scale_time(&self, shift: Option<TimeDelta>, duration: Option<TimeDelta>) -> Self {
        let d = {
            if let Some(d) = shift {
                &*Box::new(create_interval(d)) as *const meos_sys::Interval
            } else {
                std::ptr::null()
            }
        };

        let w = {
            if let Some(w) = duration {
                &*Box::new(create_interval(w)) as *const meos_sys::Interval
            } else {
                std::ptr::null()
            }
        };

        let modified = unsafe { meos_sys::temporal_shift_scale_time(self.inner(), d, w) };
        Self::from_inner_as_temporal(modified)
    }

    /// Returns a new `Temporal` downsampled with respect to `duration`.
    ///
    /// ## Arguments
    /// * `duration` - TimeDelta of the temporal tiles.
    /// * `start` - Start time of the temporal tiles.
    /// * `interpolation`- Interpolation of the resulting temporal object.
    #[doc(alias = "temporal_tsample")]
    fn temporal_sample<Tz: TimeZone>(
        self,
        duration: TimeDelta,
        start: DateTime<Tz>,
        interpolation: TInterpolation,
    ) -> Self {
        let interval = create_interval(duration);
        Self::from_inner_as_temporal(unsafe {
            meos_sys::temporal_tsample(
                self.inner(),
                ptr::addr_of!(interval),
                to_meos_timestamp(&start),
                interpolation as u32,
            )
        })
    }

    /// Returns a new `Temporal` with precision reduced to `duration`.
    ///
    /// ## Arguments
    /// * `duration` - TimeDelta of the temporal tiles.
    /// * `start` - Start time of the temporal tiles.
    #[doc(alias = "temporal_tprecision")]
    fn temporal_precision<Tz: TimeZone>(self, duration: TimeDelta, start: DateTime<Tz>) -> Self {
        let interval = create_interval(duration);
        Self::from_inner_as_temporal(unsafe {
            meos_sys::temporal_tprecision(
                self.inner(),
                ptr::addr_of!(interval),
                to_meos_timestamp(&start),
            )
        })
    }

    /// Converts `self` into a `TInstant`.
    #[doc(alias = "temporal_to_instant")]
    fn to_instant(&self) -> Self::TI {
        TInstant::from_inner(unsafe { meos_sys::temporal_to_tinstant(self.inner()) })
    }

    /// Converts `self` into a `TSequence`.
    ///
    /// ## Arguments
    /// * `interpolation` - The interpolation type for the sequence.
    #[doc(alias = "temporal_to_tsequence")]
    fn to_sequence(&self, interpolation: TInterpolation) -> Self::TS {
        let c_str = CString::new(interpolation.to_string()).unwrap();
        TSequence::from_inner(unsafe {
            meos_sys::temporal_to_tsequence(self.inner(), c_str.as_ptr())
        })
    }

    /// Converts `self` into a `TSequenceSet`.
    ///
    /// ## Arguments
    /// * `interpolation` - The interpolation type for the sequence set.
    #[doc(alias = "temporal_to_tsequenceset")]
    fn to_sequence_set(&self, interpolation: TInterpolation) -> Self::TSS {
        let c_str = CString::new(interpolation.to_string()).unwrap();
        TSequenceSet::from_inner(unsafe {
            meos_sys::temporal_to_tsequenceset(self.inner(), c_str.as_ptr())
        })
    }

    // ------------------------- Modifications ---------------------------------

    /// Appends `instant` to `self`.
    ///
    /// ## Arguments
    /// * `instant` - Instant to append.
    /// * `max_dist` - Maximum distance for defining a gap.
    /// * `max_time` - Maximum time for defining a gap.
    #[doc(alias = "temporal_append_tinstant")]
    fn append_instant(
        self,
        instant: Self::TI,
        max_dist: Option<f64>,
        max_time: Option<TimeDelta>,
    ) -> Self::Enum {
        let td = create_interval(max_time.unwrap_or_default());
        let max_time_ptr = if max_time.is_some() {
            ptr::addr_of!(td)
        } else {
            ptr::null()
        };
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_append_tinstant(
                self.inner() as *mut _,
                instant.inner_as_tinstant(),
                max_dist.unwrap_or_default(),
                max_time_ptr,
                false,
            )
        })
    }

    /// Appends `sequence` to `self`.
    ///
    /// ## Arguments
    /// * `sequence` - Sequence to append.
    #[doc(alias = "temporal_append_tsequence")]
    fn append_sequence(&self, sequence: Self::TS) -> Self::Enum {
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_append_tsequence(
                self.inner() as *mut _,
                sequence.inner_as_tsequence(),
                false,
            )
        })
    }

    /// Merges `self` with `other`.
    ///
    /// ## Arguments
    /// * `other` - Another temporal object
    #[doc(alias = "temporal_merge")]
    fn merge_other(&self, other: Self::Enum) -> Self::Enum {
        factory::<Self::Enum>(unsafe { meos_sys::temporal_merge(self.inner(), other.inner()) })
    }

    /// Inserts `other` into `self`.
    ///
    /// ## Arguments
    /// * `other` - Temporal object to insert.
    /// * `connect` - Whether to connect inserted elements with existing ones.
    #[doc(alias = "temporal_insert")]
    fn insert(&self, other: Self::Enum, connect: bool) -> Self::Enum {
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_insert(self.inner(), other.inner(), connect)
        })
    }

    /// Updates `self` with `other`.
    ///
    /// ## Arguments
    /// * `other` - Temporal object to update with.
    /// * `connect` - Whether to connect updated elements with existing ones.
    #[doc(alias = "temporal_update")]
    fn update(&self, other: Self::Enum, connect: bool) -> Self::Enum {
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_update(self.inner(), other.inner(), connect)
        })
    }

    /// Deletes elements from `self` at `other`.
    ///
    /// ## Arguments
    /// * `other` - Time object specifying the elements to delete.
    /// * `connect` - Whether to connect the potential gaps generated by the deletions.
    #[doc(alias = "temporal_delete_timestamptz")]
    fn delete_at_timestamp<Tz: TimeZone>(&self, other: DateTime<Tz>, connect: bool) -> Self::Enum {
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_delete_timestamptz(self.inner(), to_meos_timestamp(&other), connect)
        })
    }

    /// Deletes elements from `self` at `time_span`.
    ///
    /// ## Arguments
    /// * `time_span` - Time span object specifying the elements to delete.
    /// * `connect` - Whether to connect the potential gaps generated by the deletions.
    #[doc(alias = "temporal_delete_tstzspan")]
    fn delete_at_tstz_span(&self, time_span: TsTzSpan, connect: bool) -> Self::Enum {
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_delete_tstzspan(self.inner(), time_span.inner(), connect)
        })
    }

    /// Deletes elements from `self` at `time_span_set`.
    ///
    /// ## Arguments
    /// * `time_span_set` - Time span set object specifying the elements to delete.
    /// * `connect` - Whether to connect the potential gaps generated by the deletions.
    #[doc(alias = "temporal_delete_tstzspanset")]
    fn delete_at_tstz_span_set(&self, time_span_set: TsTzSpanSet, connect: bool) -> Self::Enum {
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_delete_tstzspanset(self.inner(), time_span_set.inner(), connect)
        })
    }

    // ------------------------- Restrictions ----------------------------------

    /// Returns a new temporal object with values restricted to the time `other`.
    ///
    /// ## Arguments
    /// * `other` - A timestamp to restrict the values to.
    #[doc(alias = "temporal_at_timestamptz")]
    fn at_timestamp<Tz: TimeZone>(&self, other: DateTime<Tz>) -> Self::TI {
        <Self::TI as Temporal>::from_inner_as_temporal(unsafe {
            meos_sys::temporal_at_timestamptz(self.inner(), to_meos_timestamp(&other))
        })
    }

    /// Returns a new temporal object with values restricted to the time `time_span`.
    ///
    /// ## Arguments
    /// * `time_span` - A time span to restrict the values to.
    #[doc(alias = "temporal_at_tstzspan")]
    fn at_tstz_span(&self, time_span: TsTzSpan) -> Self {
        Self::from_inner_as_temporal(unsafe {
            meos_sys::temporal_at_tstzspan(self.inner(), time_span.inner())
        })
    }

    /// Returns a new temporal object with values restricted to the time `time_span_set`.
    ///
    /// ## Arguments
    /// * `time_span_set` - A time span set to restrict the values to.
    #[doc(alias = "temporal_at_tstzspanset")]
    fn at_tstz_span_set(&self, time_span_set: TsTzSpanSet) -> Self {
        Self::from_inner_as_temporal(unsafe {
            meos_sys::temporal_at_tstzspanset(self.inner(), time_span_set.inner())
        })
    }

    /// Returns a new temporal object containing the times `self` is at `value`.
    fn at_value(&self, value: &Self::Type) -> Option<Self::Enum>;

    /// Returns a new temporal object containing the times `self` is in any of the values of `values`.
    fn at_values(&self, values: &[Self::Type]) -> Option<Self::Enum>;

    /// Returns a new temporal object with values at `timestamp` removed.
    ///
    /// ## Arguments
    /// * `timestamp` - A timestamp specifying the values to remove.
    #[doc(alias = "temporal_minus_timestampz")]
    fn minus_timestamp<Tz: TimeZone>(&self, timestamp: DateTime<Tz>) -> Self::Enum {
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_minus_timestamptz(self.inner(), to_meos_timestamp(&timestamp))
        })
    }

    /// Returns a new temporal object with values at any of the values of `timestamps` removed.
    ///
    /// ## Arguments
    /// * `timestamps` - A timestamp specifying the values to remove.
    #[doc(alias = "temporal_minus_tstzset")]
    fn minus_timestamp_set<Tz: TimeZone>(&self, timestamps: &[DateTime<Tz>]) -> Self::Enum {
        let timestamps: Vec<_> = timestamps.iter().map(to_meos_timestamp).collect();
        let set = unsafe { meos_sys::tstzset_make(timestamps.as_ptr(), timestamps.len() as i32) };
        factory::<Self::Enum>(unsafe { meos_sys::temporal_minus_tstzset(self.inner(), set) })
    }

    /// Returns a new temporal object with values at `time_span` removed.
    ///
    /// ## Arguments
    /// * `time_span` - A time span specifying the values to remove.
    #[doc(alias = "temporal_minus_tstzspan")]
    fn minus_tstz_span(&self, time_span: TsTzSpan) -> Self::Enum {
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_minus_tstzspan(self.inner(), time_span.inner())
        })
    }

    /// Returns a new temporal object with values at `time_span_set` removed.
    ///
    /// ## Arguments
    /// * `time_span_set` - A time span set specifying the values to remove.
    #[doc(alias = "temporal_minus_tstzspanset")]
    fn minus_tstz_span_set(&self, time_span_set: TsTzSpanSet) -> Self::Enum {
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_minus_tstzspanset(self.inner(), time_span_set.inner())
        })
    }

    /// Returns a new temporal object containing the times `self` is not at `value`.
    fn minus_value(&self, value: Self::Type) -> Self::Enum;

    /// Returns a new temporal object containing the times `self` is not at `values`.
    fn minus_values(&self, values: &[Self::Type]) -> Self::Enum;

    // ------------------------- Topological Operations ------------------------

    /// Returns a `TBool` representing whether the bounding box of `self` is adjacent to the bounding box of `other` accross time.
    ///
    /// ## Arguments
    /// * `other` - A time or temporal object to compare.
    ///
    /// See also:
    ///     `Collection.is_adjacent`
    #[doc(alias = "adjacent_temporal_temporal")]
    fn is_adjacent(&self, other: Self::Enum) -> bool {
        unsafe { meos_sys::adjacent_temporal_temporal(self.inner(), other.inner()) }
    }

    /// Returns a `TBool` representing whether the bounding timespan of `self` is temporally adjacent to the bounding timespan of `other` accross time.
    ///
    /// ## Arguments
    /// * `other` - A time or temporal object to compare.
    ///
    /// See also:
    ///     `Collection.is_adjacent`
    fn is_temporally_adjacent(&self, other: Self) -> bool {
        self.timespan().is_adjacent(&other.timespan())
    }

    /// Returns a `TBool` representing whether the bounding-box of `self` is contained in the bounding-box of `container` accross time.
    ///
    /// ## Arguments
    /// * `container` - A time or temporal object to compare.
    ///
    /// See also:
    ///     `Collection.is_contained_in`
    #[doc(alias = "contained_temporal_temporal")]
    fn is_contained_in(&self, other: Self::Enum) -> bool {
        unsafe { meos_sys::contained_temporal_temporal(self.inner(), other.inner()) }
    }

    /// Returns a `TBool` representing whether the bounding timespan of `self` is contained in the bounding timespan of `container` accross time.
    ///
    /// ## Arguments
    /// * `container` - A time or temporal object to compare.
    ///
    /// See also:
    ///     `Collection.is_contained_in`
    fn is_temporally_contained_in(&self, other: Self) -> bool {
        self.timespan().is_contained_in(&other.timespan())
    }

    /// Returns a `TBool` representing whether the bounding timespan of `self` contains the bounding timespan of `other` accross time.
    ///
    /// ## Arguments
    /// * `other` - A time or temporal object to compare.
    #[doc(alias = "contains_temporal_temporal")]
    fn contains(&self, other: Self::Enum) -> bool {
        unsafe { meos_sys::contains_temporal_temporal(self.inner(), other.inner()) }
    }

    /// Returns a `TBool` representing whether the bounding timespan of `self` temporally contains the bounding timespan of `other` accross time.
    ///
    /// ## Arguments
    /// * `other` - A time or temporal object to compare.
    fn temporally_contains(&self, other: Self) -> bool {
        other.timespan().is_contained_in(&self.timespan())
    }

    /// Returns a `TBool` representing whether the bounding timespan of `self` overlaps with the bounding timespan of `other` accross time.
    ///
    /// ## Arguments
    /// * `other` - A time or temporal object to compare.
    ///
    /// See also:
    ///     `Collection.overlaps`
    #[doc(alias = "overlaps_temporal_temporal")]
    fn overlaps(&self, other: Self) -> bool {
        unsafe { meos_sys::overlaps_temporal_temporal(self.inner(), other.inner()) }
    }

    /// Returns a `TBool` representing whether the bounding timespan of `self` temporally overlaps with the bounding timespan of `other` accross time.
    ///
    /// ## Arguments
    /// * `other` - A time or temporal object to compare.
    ///
    /// See also:
    ///     `TsTzSpan.overlaps`
    fn temporally_overlaps(&self, other: Self) -> bool {
        self.timespan().overlaps(&other.timespan())
    }

    // ------------------------- Position Operations ---------------------------
    /// Returns whether `self` is before `other`.
    ///
    /// ## Arguments
    /// * `other` - A time or temporal object to compare.
    ///
    /// ## Returns
    /// True if `self` is before `other`, False otherwise.
    ///
    /// See also:
    ///     `TsTzSpan.is_left`
    #[doc(alias = "before_temporal_temporal")]
    fn is_before(&self, other: Self::Enum) -> bool {
        unsafe { meos_sys::before_temporal_temporal(self.inner(), other.inner()) }
    }

    /// Returns whether `self` is before `other` allowing overlap.
    ///
    /// ## Arguments
    /// * `other` - A time or temporal object to compare.
    ///
    /// ## Returns
    /// True if `self` is before `other` allowing overlap, False otherwise.
    ///
    /// See also:
    ///     `TsTzSpan.is_over_or_left`
    #[doc(alias = "overbefore_temporal_temporal")]
    fn is_over_or_before(&self, other: Self::Enum) -> bool {
        unsafe { meos_sys::overbefore_temporal_temporal(self.inner(), other.inner()) }
    }

    /// Returns whether `self` is after `other`.
    ///
    /// ## Arguments
    /// * `other` - A time or temporal object to compare.
    ///
    /// ## Returns
    /// True if `self` is after `other`, False otherwise.
    ///
    /// See also:
    ///     `TsTzSpan.is_right`
    #[doc(alias = "after_temporal_temporal")]
    fn is_after(&self, other: Self::Enum) -> bool {
        unsafe { meos_sys::after_temporal_temporal(self.inner(), other.inner()) }
    }

    /// Returns whether `self` is after `other` allowing overlap.
    ///
    /// ## Arguments
    /// * `other` - A time or temporal object to compare.
    ///
    /// ## Returns
    /// True if `self` is after `other` allowing overlap, False otherwise.
    ///
    /// See also:
    ///     `TsTzSpan.is_over_or_right`
    #[doc(alias = "overafter_temporal_temporal")]
    fn is_over_or_after(&self, other: Self::Enum) -> bool {
        unsafe { meos_sys::overafter_temporal_temporal(self.inner(), other.inner()) }
    }

    // ------------------------- Similarity Operations -------------------------
    /// Returns the Frechet distance between `self` and `other`.
    ///
    /// ## Arguments
    /// * `other` - A temporal object to compare.
    ///
    /// ## Returns
    /// A float with the Frechet distance.
    #[doc(alias = "temporal_frechet_distance")]
    fn frechet_distance(&self, other: Self) -> f64 {
        unsafe { meos_sys::temporal_frechet_distance(self.inner(), other.inner()) }
    }

    /// Returns the Dynamic Time Warp distance between `self` and `other`.
    ///
    /// ## Arguments
    /// * `other` - A temporal object to compare.
    ///
    /// ## Returns
    /// A float with the Dynamic Time Warp distance.
    #[doc(alias = "temporal_dyntimewarp_distance")]
    fn dyntimewarp_distance(&self, other: Self) -> f64 {
        unsafe { meos_sys::temporal_dyntimewarp_distance(self.inner(), other.inner()) }
    }

    /// Returns the Hausdorff distance between `self` and `other`.
    ///
    /// ## Arguments
    /// * `other` - A temporal object to compare.
    ///
    /// ## Returns
    /// A float with the Hausdorff distance.
    #[doc(alias = "temporal_hausdorff_distance")]
    fn hausdorff_distance(&self, other: Self) -> f64 {
        unsafe { meos_sys::temporal_hausdorff_distance(self.inner(), other.inner()) }
    }

    // ------------------------- Split Operations ------------------------------
    /// Splits the temporal object into multiple pieces based on the given duration.
    ///
    /// ## Arguments
    /// * `duration` - Duration of each temporal tile.
    /// * `start` - Start time for the tiles.
    ///
    /// ## Returns
    /// A list of temporal objects representing the split tiles.
    #[doc(alias = "temporal_time_split")]
    fn time_split<Tz: TimeZone>(&self, duration: TimeDelta, start: DateTime<Tz>) -> Vec<Self> {
        let duration = create_interval(duration);
        let start = to_meos_timestamp(&start);
        let mut count = 0;
        let _buckets = Vec::new().as_mut_ptr();
        unsafe {
            let temps = meos_sys::temporal_time_split(
                self.inner(),
                ptr::addr_of!(duration),
                start,
                _buckets,
                ptr::addr_of_mut!(count),
            );

            Vec::from_raw_parts(temps, count as usize, count as usize)
                .iter()
                .map(|&t| Temporal::from_inner_as_temporal(t))
                .collect()
        }
    }

    /// Splits the temporal object into `n` equal-duration parts.
    ///
    /// ## Arguments
    /// * `n` - Number of parts to split into.
    ///
    /// ## Returns
    /// A list of temporal objects representing the split parts.
    fn time_split_n(&self, n: usize) -> Vec<Self> {
        let start = self.start_timestamp();
        let duration = (self.end_timestamp() - start) / n as i32;
        self.time_split(duration, start)
    }

    /// Extracts the subsequences where the object stays within a certain distance for a specified duration.
    ///
    /// ## Arguments
    /// * `max_distance` - Maximum distance of a stop.
    /// * `min_duration` - Minimum duration of a stop.
    ///
    /// ## Returns
    /// A sequence set of stops.
    #[doc(alias = "temporal_stops")]
    fn stops(&self, max_distance: f64, min_duration: TimeDelta) -> Self::TSS {
        let interval = create_interval(min_duration);
        unsafe {
            <Self::TSS as TSequenceSet>::from_inner(meos_sys::temporal_stops(
                self.inner(),
                max_distance,
                ptr::addr_of!(interval),
            ))
        }
    }

    /// Returns whether the values of `self` are always equal to `other`.
    ///
    /// ## Arguments
    ///
    /// * `other` - Another temporal instance to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are always equal to `other`, `false` otherwise.
    #[doc(alias = "always_eq_temporal_temporal")]
    fn always_equal(&self, other: &Self) -> Option<bool> {
        let result = unsafe { meos_sys::always_eq_temporal_temporal(self.inner(), other.inner()) };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        }
    }

    /// Returns whether the values of `self` are always not equal to `other`.
    ///
    /// ## Arguments
    ///
    /// * `other` - Another temporal instance to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are always not equal to `other`, `false` otherwise.
    #[doc(alias = "always_ne_temporal_temporal")]
    fn always_not_equal(&self, other: &Self) -> Option<bool> {
        let result = unsafe { meos_sys::always_ne_temporal_temporal(self.inner(), other.inner()) };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        }
    }

    /// Returns whether the values of `self` are ever equal to `other`.
    ///
    /// ## Arguments
    ///
    /// * `other` - Another temporal instance to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are ever equal to `other`, `false` otherwise.
    #[doc(alias = "ever_eq_temporal_temporal")]
    fn ever_equal(&self, other: &Self) -> Option<bool> {
        let result = unsafe { meos_sys::ever_eq_temporal_temporal(self.inner(), other.inner()) };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        }
    }

    /// Returns whether the values of `self` are ever not equal to `other`.
    ///
    /// ## Arguments
    ///
    /// * `other` - Another temporal instance to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are ever not equal to `other`, `false` otherwise.
    #[doc(alias = "ever_ne_temporal_temporal")]
    fn ever_not_equal(&self, other: &Self) -> Option<bool> {
        let result = unsafe { meos_sys::ever_ne_temporal_temporal(self.inner(), other.inner()) };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        }
    }

    /// Returns whether the values of `self` are always equal to `value`.
    ///
    /// ## Arguments
    ///
    /// * `value` - Value to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are always equal to `value`, `false` otherwise.
    fn always_equal_than_value(&self, value: Self::Type) -> Option<bool>;

    /// Returns whether the values of `self` are always not equal to `value`.
    ///
    /// ## Arguments
    ///
    /// * `value` - Value to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are always not equal to `value`, `false` otherwise.
    fn always_not_equal_than_value(&self, value: Self::Type) -> Option<bool>;

    /// Returns whether the values of `self` are ever equal to `value`.
    ///
    /// ## Arguments
    ///
    /// * `value` - Value to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are ever equal to `value`, `false` otherwise.
    fn ever_equal_than_value(&self, value: Self::Type) -> Option<bool>;

    /// Returns whether the values of `self` are ever not equal to `value`.
    ///
    /// ## Arguments
    ///
    /// * `value` - Value to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are ever not equal to `value`, `false` otherwise.
    fn ever_not_equal_than_value(&self, value: Self::Type) -> Option<bool>;

    /// Returns a `TBool` representing whether `self` is equal to `other` accross time.
    ///
    /// ## Arguments
    ///
    /// * `other` - A reference to another temporal object to compare with.
    ///
    /// ## Returns
    ///
    /// A temporal boolean indicating if `self` is equal to `other`.
    fn temporal_equal(&self, other: &Self) -> Self::TBoolType {
        Self::TBoolType::from_inner_as_temporal(unsafe {
            meos_sys::teq_temporal_temporal(self.inner(), other.inner())
        })
    }

    /// Returns a `TBool` representing whether `self` is not equal to `other` accross time.
    ///
    /// ## Arguments
    ///
    /// * `other` - A reference to another temporal object to compare with.
    ///
    /// ## Returns
    ///
    /// A temporal boolean indicating if `self` is not equal to `other`.
    fn temporal_not_equal(&self, other: &Self) -> Self::TBoolType {
        Self::TBoolType::from_inner_as_temporal(unsafe {
            meos_sys::tne_temporal_temporal(self.inner(), other.inner())
        })
    }

    /// Returns a `TBool` representing whether `self` is equal to the given value accross time.
    ///
    /// ## Arguments
    ///
    /// * `other` - A reference to a value to compare with.
    ///
    /// ## Returns
    ///
    /// A temporal boolean indicating if `self` is equal to the given value.
    fn temporal_equal_value(&self, other: &Self::Type) -> Self::TBoolType;

    /// Returns a `TBool` representing whether `self` is not equal to the given value accross time.
    ///
    /// ## Arguments
    ///
    /// * `other` - A reference to a value to compare with.
    ///
    /// ## Returns
    ///
    /// A temporal boolean indicating if `self` is not equal to the given value.
    fn temporal_not_equal_value(&self, other: &Self::Type) -> Self::TBoolType;
}

pub trait OrderedTemporal: Temporal {
    /// Returns the minimum value of the temporal object.
    ///
    /// ## Returns
    /// The minimum value.
    fn min_value(&self) -> Self::Type;

    /// Returns the maximum value of the temporal object.
    ///
    /// ## Returns
    /// The maximum value.
    fn max_value(&self) -> Self::Type;

    /// Returns a new temporal object containing the times `self` is at its minimum value.
    fn at_min(&self) -> Self {
        Self::from_inner_as_temporal(unsafe { meos_sys::temporal_at_min(self.inner()) })
    }

    /// Returns a new temporal object containing the times `self` is at its maximum value.
    fn at_max(&self) -> Self {
        Self::from_inner_as_temporal(unsafe { meos_sys::temporal_at_max(self.inner()) })
    }
    /// Returns a new temporal object containing the times `self` is not at its minimum value.
    fn minus_min(&self) -> Self {
        Self::from_inner_as_temporal(unsafe { meos_sys::temporal_minus_min(self.inner()) })
    }

    /// Returns a new temporal object containing the times `self` is not at its maximum value.
    fn minus_max(&self) -> Self {
        Self::from_inner_as_temporal(unsafe { meos_sys::temporal_minus_max(self.inner()) })
    }

    /// Returns a `TBool` representing whether `self` is greater than `other` accross time.
    ///
    /// ## Arguments
    ///
    /// * `other` - A reference to another temporal object to compare with.
    ///
    /// ## Returns
    ///
    /// A temporal boolean indicating if `self` is greater than `other`.
    fn temporal_greater_than(&self, other: &Self) -> Self::TBoolType {
        Self::TBoolType::from_inner_as_temporal(unsafe {
            meos_sys::tgt_temporal_temporal(self.inner(), other.inner())
        })
    }

    /// Returns a `TBool` representing whether `self` is greater than or equal to `other` accross time.
    ///
    /// ## Arguments
    ///
    /// * `other` - A reference to another temporal object to compare with.
    ///
    /// ## Returns
    ///
    /// A temporal boolean indicating if `self` is greater than or equal to `other`.
    fn temporal_greater_or_equal_than(&self, other: &Self) -> Self::TBoolType {
        Self::TBoolType::from_inner_as_temporal(unsafe {
            meos_sys::tge_temporal_temporal(self.inner(), other.inner())
        })
    }

    /// Returns a `TBool` representing whether `self` is less than `other` accross time.
    ///
    /// ## Arguments
    ///
    /// * `other` - A reference to another temporal object to compare with.
    ///
    /// ## Returns
    ///
    /// A temporal boolean indicating if `self` is less than `other`.
    fn temporal_lower_than(&self, other: &Self) -> Self::TBoolType {
        Self::TBoolType::from_inner_as_temporal(unsafe {
            meos_sys::tlt_temporal_temporal(self.inner(), other.inner())
        })
    }

    /// Returns a `TBool` representing whether `self` is less than or equal to `other` accross time.
    ///
    /// ## Arguments
    ///
    /// * `other` - A reference to another temporal object to compare with.
    ///
    /// ## Returns
    ///
    /// A temporal boolean indicating if `self` is less than or equal to `other`.
    fn temporal_lower_or_equal_than(&self, other: &Self) -> Self::TBoolType {
        Self::TBoolType::from_inner_as_temporal(unsafe {
            meos_sys::tle_temporal_temporal(self.inner(), other.inner())
        })
    }

    /// Returns a `TBool` representing whether `self` is greater than the given value accross time.
    ///
    /// ## Arguments
    ///
    /// * `other` - A reference to a value to compare with.
    ///
    /// ## Returns
    ///
    /// A temporal boolean indicating if `self` is greater than the given value.
    fn temporal_greater_than_value(&self, other: &Self::Type) -> Self::TBoolType;

    /// Returns a `TBool` representing whether `self` is greater than or equal to the given value accross time.
    ///
    /// ## Arguments
    ///
    /// * `other` - A reference to a value to compare with.
    ///
    /// ## Returns
    ///
    /// A temporal boolean indicating if `self` is greater than or equal to the given value.
    fn temporal_greater_or_equal_than_value(&self, other: &Self::Type) -> Self::TBoolType;

    /// Returns a `TBool` representing whether `self` is less than the given value accross time.
    ///
    /// ## Arguments
    ///
    /// * `other` - A reference to a value to compare with.
    ///
    /// ## Returns
    ///
    /// A temporal boolean indicating if `self` is less than the given value.
    fn temporal_lower_than_value(&self, other: &Self::Type) -> Self::TBoolType;

    /// Returns a `TBool` representing whether `self` is less than or equal to the given value accross time.
    ///
    /// ## Arguments
    ///
    /// * `other` - A reference to a value to compare with.
    ///
    /// ## Returns
    ///
    /// A temporal boolean indicating if `self` is less than or equal to the given value.
    fn temporal_lower_or_equal_than_value(&self, other: &Self::Type) -> Self::TBoolType;

    /// Returns whether the values of `self` are always less than `other`.
    ///
    /// ## Arguments
    ///
    /// * `other` - Another temporal instance to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are always less than `other`, `false` otherwise.
    #[doc(alias = "always_lt_temporal_temporal")]
    fn always_less(&self, other: &Self::Enum) -> Option<bool> {
        let result = unsafe { meos_sys::always_lt_temporal_temporal(self.inner(), other.inner()) };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        }
    }

    /// Returns whether the values of `self` are always less than or equal to `other`.
    ///
    /// ## Arguments
    ///
    /// * `other` - Another temporal instance to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are always less than or equal to `other`, `false` otherwise.
    #[doc(alias = "always_le_temporal_temporal")]
    fn always_less_or_equal(&self, other: &Self::Enum) -> Option<bool> {
        let result = unsafe { meos_sys::always_le_temporal_temporal(self.inner(), other.inner()) };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        }
    }

    /// Returns whether the values of `self` are always greater than or equal to `other`.
    ///
    /// ## Arguments
    ///
    /// * `other` - Another temporal instance to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are always greater than or equal to `other`, `false` otherwise.
    #[doc(alias = "always_ge_temporal_temporal")]
    fn always_greater_or_equal(&self, other: &Self::Enum) -> Option<bool> {
        let result = unsafe { meos_sys::always_ge_temporal_temporal(self.inner(), other.inner()) };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        }
    }

    /// Returns whether the values of `self` are always greater than `other`.
    ///
    /// ## Arguments
    ///
    /// * `other` - Another temporal instance to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are always greater than `other`, `false` otherwise.
    #[doc(alias = "always_gt_temporal_temporal")]
    fn always_greater(&self, other: &Self::Enum) -> Option<bool> {
        let result = unsafe { meos_sys::always_gt_temporal_temporal(self.inner(), other.inner()) };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        }
    }
    /// Returns whether the values of `self` are ever less than `other`.
    ///
    /// ## Arguments
    ///
    /// * `other` - Another temporal instance to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are ever less than `other`, `false` otherwise.
    #[doc(alias = "ever_lt_temporal_temporal")]
    fn ever_less(&self, other: &Self::Enum) -> Option<bool> {
        let result = unsafe { meos_sys::ever_lt_temporal_temporal(self.inner(), other.inner()) };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        }
    }
    /// Returns whether the values of `self` are ever less than or equal to `other`.
    ///
    /// ## Arguments
    ///
    /// * `other` - Another temporal instance to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are ever less than or equal to `other`, `false` otherwise.
    #[doc(alias = "ever_le_temporal_temporal")]
    fn ever_less_or_equal(&self, other: &Self::Enum) -> Option<bool> {
        let result = unsafe { meos_sys::ever_le_temporal_temporal(self.inner(), other.inner()) };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        }
    }

    /// Returns whether the values of `self` are ever greater than or equal to `other`.
    ///
    /// ## Arguments
    ///
    /// * `other` - Another temporal instance to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are ever greater than or equal to `other`, `false` otherwise.
    #[doc(alias = "ever_ge_temporal_temporal")]
    fn ever_greater_or_equal(&self, other: &Self::Enum) -> Option<bool> {
        let result = unsafe { meos_sys::ever_ge_temporal_temporal(self.inner(), other.inner()) };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        }
    }

    /// Returns whether the values of `self` are ever greater than `other`.
    ///
    /// ## Arguments
    ///
    /// * `other` - Another temporal instance to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are ever greater than `other`, `false` otherwise.
    #[doc(alias = "ever_gt_temporal_temporal")]
    fn ever_greater(&self, other: &Self::Enum) -> Option<bool> {
        let result = unsafe { meos_sys::ever_gt_temporal_temporal(self.inner(), other.inner()) };
        if result != -1 {
            Some(result == 1)
        } else {
            None
        }
    }

    /// Returns whether the values of `self` are always less than `value`.
    ///
    /// ## Arguments
    ///
    /// * `value` - Value to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are always less than `value`, `false` otherwise.
    fn always_less_than_value(&self, value: Self::Type) -> Option<bool>;

    /// Returns whether the values of `self` are always less than or equal to `value`.
    ///
    /// ## Arguments
    ///
    /// * `value` - Value to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are always less than or equal to `value`, `false` otherwise.
    fn always_less_or_equal_than_value(&self, value: Self::Type) -> Option<bool>;

    /// Returns whether the values of `self` are always greater than or equal to `value`.
    ///
    /// ## Arguments
    ///
    /// * `value` - Value to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are always greater than or equal to `value`, `false` otherwise.
    fn always_greater_or_equal_than_value(&self, value: Self::Type) -> Option<bool>;

    /// Returns whether the values of `self` are always greater than `value`.
    ///
    /// ## Arguments
    ///
    /// * `value` - Value to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are always greater than `value`, `false` otherwise.
    fn always_greater_than_value(&self, value: Self::Type) -> Option<bool>;

    /// Returns whether the values of `self` are ever less than `value`.
    ///
    /// ## Arguments
    ///
    /// * `value` - Value to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are ever less than `value`, `false` otherwise.
    fn ever_less_than_value(&self, value: Self::Type) -> Option<bool>;

    /// Returns whether the values of `self` are ever less than or equal to `value`.
    ///
    /// ## Arguments
    ///
    /// * `value` - Value to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are ever less than or equal to `value`, `false` otherwise.
    fn ever_less_or_equal_than_value(&self, value: Self::Type) -> Option<bool>;

    /// Returns whether the values of `self` are ever greater than or equal to `value`.
    ///
    /// ## Arguments
    ///
    /// * `value` - Value to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are ever greater than or equal to `value`, `false` otherwise.
    fn ever_greater_or_equal_than_value(&self, value: Self::Type) -> Option<bool>;

    /// Returns whether the values of `self` are ever greater than `value`.
    ///
    /// ## Arguments
    ///
    /// * `value` - Value to compare against.
    ///
    /// ## Returns
    ///
    /// `true` if the values of `self` are ever greater than `value`, `false` otherwise.
    fn ever_greater_than_value(&self, value: Self::Type) -> Option<bool>;
}

/// A trait for simplifying temporal values in various ways.
pub trait SimplifiableTemporal: Temporal {
    /// Simplifies a temporal value ensuring that consecutive values are at least
    /// a certain distance apart.
    ///
    /// ## Arguments
    ///
    /// * `distance` - A `f64` representing the minimum distance between two points.
    ///
    /// ## Returns
    ///
    /// A simplified instance of the implementing type with the same subtype as the input.
    ///
    /// ## MEOS Functions
    #[doc(alias = "temporal_simplify_min_dist")]
    fn simplify_min_distance(&self, distance: f64) -> Self::Enum {
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_simplify_min_dist(self.inner(), distance)
        })
    }

    /// Simplifies a temporal value ensuring that consecutive values are at least
    /// a certain time apart.
    ///
    /// ## Arguments
    ///
    /// * `distance` - A `Duration` indicating the minimum time between two points.
    ///
    /// ## Returns
    ///
    /// A simplified instance of the implementing type with the same subtype as the input.
    #[doc(alias = "temporal_simplify_min_tdelta")]
    fn simplify_min_tdelta(&self, distance: TimeDelta) -> Self::Enum {
        let interval = create_interval(distance);
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_simplify_min_tdelta(self.inner(), ptr::addr_of!(interval))
        })
    }

    /// Simplifies a temporal value using the Douglas-Peucker line simplification algorithm.
    ///
    /// ## Arguments
    ///
    /// * `distance` - A `f64` representing the minimum distance between two points.
    /// * `synchronized` - A `bool` indicating if the Synchronized Distance should be used.
    ///   If `false`, the spatial-only distance will be used.
    ///
    /// ## Returns
    ///
    /// A simplified instance of the implementing type with the same subtype as the input.
    #[doc(alias = "temporal_simplify_dp")]
    fn simplify_douglas_peucker(&self, distance: f64, synchronized: bool) -> Self::Enum {
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_simplify_dp(self.inner(), distance, synchronized)
        })
    }

    /// Simplifies a temporal value using a single-pass Douglas-Peucker line simplification algorithm.
    ///
    /// ## Arguments
    ///
    /// * `distance` - A `f64` representing the minimum distance between two points.
    /// * `synchronized` - A `bool` indicating if the Synchronized Distance should be used.
    ///   If `false`, the spatial-only distance will be used.
    ///
    /// ## Returns
    ///
    /// A simplified instance of the implementing type with the same subtype as the input.
    #[doc(alias = "temporal_simplify_max_dist")]
    fn simplify_max_distance(&self, distance: f64, synchronized: bool) -> Self::Enum {
        factory::<Self::Enum>(unsafe {
            meos_sys::temporal_simplify_max_dist(self.inner(), distance, synchronized)
        })
    }
}

macro_rules! impl_simple_traits_for_temporal {
    ($type:ty) => {
        paste::paste! {
            impl AsRef<$type> for $type {
                fn as_ref(&self) -> &$type {
                    self
                }
            }

            impl Clone for $type {
                fn clone(&self) -> Self {
                    Temporal::from_inner_as_temporal(unsafe { meos_sys::temporal_copy(self.inner()) })
                }
            }
            impl PartialEq for $type {
                fn eq(&self, other: &Self) -> bool {
                    unsafe { meos_sys::temporal_eq(self.inner(), other.inner()) }
                }
            }

            impl Hash for $type {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    let hash = unsafe { meos_sys::temporal_hash(self.inner()) };
                    state.write_u32(hash);

                    let _ = state.finish();
                }
            }
        }
    };
    ($type:ty, with_drop) => {
        impl_simple_traits_for_temporal!($type);

        impl Drop for $type {
            fn drop(&mut self) {
                unsafe {
                    libc::free(self._inner.as_ptr() as *mut c_void);
                }
            }
        }
    }
}

macro_rules! impl_always_and_ever_value_equality_functions {
    ($type:ident, $transform_function:expr) => {
        paste::paste! {
            fn always_equal_than_value(&self, value: Self::Type) -> Option<bool> {
                let result = unsafe { meos_sys::[<always_eq_t $type _ $type>](self.inner(), $transform_function(&value)) };
                if result != -1 {
                    Some(result == 1)
                } else {
                    None
                }
            }
            fn always_not_equal_than_value(&self, value: Self::Type) -> Option<bool> {
                let result = unsafe { meos_sys::[<always_ne_t $type _ $type>](self.inner(), $transform_function(&value)) };
                if result != -1 {
                    Some(result == 1)
                } else {
                    None
                }
            }
            fn ever_equal_than_value(&self, value: Self::Type) -> Option<bool> {
                let result = unsafe { meos_sys::[<ever_eq_t $type _ $type>](self.inner(), $transform_function(&value)) };
                if result != -1 {
                    Some(result == 1)
                } else {
                    None
                }
            }
            fn ever_not_equal_than_value(&self, value: Self::Type) -> Option<bool> {
                let result = unsafe { meos_sys::[<ever_ne_t $type _ $type>](self.inner(), $transform_function(&value)) };
                if result != -1 {
                    Some(result == 1)
                } else {
                    None
                }

            }
        }
    };
    ($type:ident) => {
        impl_always_and_ever_value_equality_functions!($type, |&x| x);
    };
}

macro_rules! impl_ordered_temporal_functions {

    ($type:ident, $transform_function:expr) => {
        paste::paste! {
            fn always_less_than_value(&self, value: Self::Type) -> Option<bool> {
                let result = unsafe { meos_sys::[<always_lt_t $type _ $type>](self.inner(), $transform_function(&value)) };
                if result != -1 {
                    Some(result == 1)
                } else {
                    None
                }
            }
            fn always_less_or_equal_than_value(&self, value: Self::Type) -> Option<bool> {
                let result = unsafe { meos_sys::[<always_le_t $type _ $type>](self.inner(), $transform_function(&value)) };
                if result != -1 {
                    Some(result == 1)
                } else {
                    None
                }
            }
            fn always_greater_or_equal_than_value(&self, value: Self::Type) -> Option<bool> {
                let result = unsafe { meos_sys::[<always_ge_t $type _ $type>](self.inner(), $transform_function(&value)) };
                if result != -1 {
                    Some(result == 1)
                } else {
                    None
                }
            }
            fn always_greater_than_value(&self, value: Self::Type) -> Option<bool> {
                let result = unsafe { meos_sys::[<always_gt_t $type _ $type>](self.inner(), $transform_function(&value)) };
                if result != -1 {
                    Some(result == 1)
                } else {
                    None
                }
            }
            fn ever_less_than_value(&self, value: Self::Type) -> Option<bool> {
                let result = unsafe { meos_sys::[<ever_lt_t $type _ $type>](self.inner(), $transform_function(&value)) };
                if result != -1 {
                    Some(result == 1)
                } else {
                    None
                }
            }
            fn ever_less_or_equal_than_value(&self, value: Self::Type) -> Option<bool> {
                let result = unsafe { meos_sys::[<ever_le_t $type _ $type>](self.inner(), $transform_function(&value)) };
                if result != -1 {
                    Some(result == 1)
                } else {
                    None
                }
            }
            fn ever_greater_or_equal_than_value(&self, value: Self::Type) -> Option<bool> {
                let result = unsafe { meos_sys::[<ever_ge_t $type _ $type>](self.inner(), $transform_function(&value)) };
                if result != -1 {
                    Some(result == 1)
                } else {
                    None
                }
            }
            fn ever_greater_than_value(&self, value: Self::Type) -> Option<bool> {
                let result = unsafe { meos_sys::[<ever_gt_t $type _ $type>](self.inner(), $transform_function(&value)) };
                if result != -1 {
                    Some(result == 1)
                } else {
                    None
                }
            }

            fn temporal_greater_than_value(&self, other: &Self::Type) -> Self::TBoolType {
                Self::TBoolType::from_inner_as_temporal(unsafe {
                    meos_sys::[<tgt_t $type _ $type>](self.inner(), $transform_function(other))
                })
            }
            fn temporal_greater_or_equal_than_value(&self, other: &Self::Type) -> Self::TBoolType {
                Self::TBoolType::from_inner_as_temporal(unsafe {
                    meos_sys::[<tge_t $type _ $type>](self.inner(), $transform_function(other))
                })
            }
            fn temporal_lower_than_value(&self, other: &Self::Type) -> Self::TBoolType {
                Self::TBoolType::from_inner_as_temporal(unsafe {
                    meos_sys::[<tlt_t $type _ $type>](self.inner(), $transform_function(other))
                })
            }
            fn temporal_lower_or_equal_than_value(&self, other: &Self::Type) -> Self::TBoolType {
                Self::TBoolType::from_inner_as_temporal(unsafe {
                    meos_sys::[<tle_t $type _ $type>](self.inner(), $transform_function(other))
                })
            }
        }
    };
    ($type:ident) => {
        impl_ordered_temporal_functions!($type, |&x| x);
    };
}

pub(crate) use impl_ordered_temporal_functions;

pub(crate) use impl_always_and_ever_value_equality_functions;

pub(crate) use impl_simple_traits_for_temporal;
