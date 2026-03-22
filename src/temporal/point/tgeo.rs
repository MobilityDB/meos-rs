use crate::temporal::tinstant::TInstant;
use crate::temporal::JSONCVariant;
use crate::{
    boxes::STBox,
    factory,
    temporal::{number::tfloat::TFloat, temporal::Temporal},
};
use core::fmt;
use geos::{CoordDimensions, Geom, Geometry, WKBWriter};
use meos_sys::GSERIALIZED;
use std::{
    ffi::{c_void, CStr, CString},
    ptr, slice,
};
#[derive(Clone, Copy)]
pub struct Point(f64, f64, Option<f64>);

pub(super) fn point_to_gserialized_geom(point: Point) -> *mut meos_sys::GSERIALIZED {
    let cstring = CString::new(point.to_string()).unwrap();
    unsafe { meos_sys::geom_in(cstring.as_ptr().cast_mut(), -1) }
}

pub(super) fn point_to_gserialized_geog(point: Point) -> *mut meos_sys::GSERIALIZED {
    let cstring = CString::new(point.to_string()).unwrap();
    unsafe { meos_sys::geog_in(cstring.as_ptr().cast_mut(), -1) }
}
pub(super) fn geometry_to_gserialized(geometry: &Geometry) -> *mut GSERIALIZED {
    let mut writer = WKBWriter::new().expect("Failed to create WKBWriter");
    writer.set_output_dimension(CoordDimensions::ThreeD);
    let wkb: Vec<u8> = writer.write_wkb(geometry).unwrap();
    let wkb_len = wkb.len();

    unsafe {
        meos_sys::geo_from_ewkb(
            wkb.as_ptr(),
            wkb_len,
            geometry.get_srid().unwrap_or_default(),
        )
    }
}

pub(super) fn gserialized_to_geometry(
    gs: *mut meos_sys::GSERIALIZED,
) -> Result<Geometry, geos::Error> {
    let mut size = 0;
    let endian = CString::new("xdr").unwrap();
    let bytes = unsafe { meos_sys::geo_as_ewkb(gs, endian.as_ptr(), ptr::addr_of_mut!(size)) };

    Geometry::new_from_wkb(unsafe { slice::from_raw_parts(bytes, size) })
}

pub(super) fn create_set_of_geometries(values: &[Geometry]) -> *mut meos_sys::Set {
    let mut cgeos: Vec<_> = values.iter().map(geometry_to_gserialized).collect();
    unsafe { meos_sys::geoset_make(cgeos.as_mut_ptr(), values.len() as i32) }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(z) = self.2 {
            f.write_fmt(format_args!("Point({}, {}, {})", self.0, self.1, z))
        } else {
            f.write_fmt(format_args!("Point({}, {})", self.0, self.1))
        }
    }
}

pub trait TGeoTrait: Temporal {
    /// The only geodetic-specific method — each concrete type implements this once.
    fn point_to_gserialized(point: Point) -> *mut meos_sys::GSERIALIZED;
    /// Returns the temporal point as a WKT string.
    ///
    /// ## Arguments
    ///
    /// * `precision` - The precision of the returned geometry.
    ///
    /// ## Returns
    ///
    /// A `String` representing the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_out`
    fn as_wkt(&self, precision: i32) -> String {
        let out_str = unsafe { meos_sys::tspatial_as_text(self.inner(), precision) };
        let c_str = unsafe { CStr::from_ptr(out_str) };
        let str = c_str.to_str().unwrap().to_owned();
        unsafe { libc::free(out_str.cast::<c_void>()) };
        str
    }

    /// Returns the temporal point as an EWKT string.
    ///
    /// ## Arguments
    ///
    /// * `precision` - The precision of the returned geometry.
    ///
    /// ## Returns
    ///
    /// A `String` representing the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_as_ewkt`
    fn as_ewkt(&self, precision: i32) -> String {
        let out_str = unsafe { meos_sys::tspatial_as_ewkt(self.inner(), precision) };
        let c_str = unsafe { CStr::from_ptr(out_str) };
        let str = c_str.to_str().unwrap().to_owned();
        unsafe { libc::free(out_str.cast::<c_void>()) };
        str
    }

    /// Returns the trajectory of the temporal point as a `GeoJSON` string.
    ///
    /// ## Arguments
    ///
    /// * `option` - The option to use when serializing the trajectory.
    /// * `precision` - The precision of the returned geometry.
    /// * `srs` - The spatial reference system of the returned geometry.
    ///
    /// ## Returns
    ///
    /// A `String` representing the trajectory of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// `gserialized_as_geojson`
    fn as_geojson(&self, variant: JSONCVariant, srs: &str) -> Option<String> {
        let cstring = CString::new(srs).unwrap();
        let trajectory = unsafe { meos_sys::tpoint_trajectory(self.inner(), false) };
        let out_str =
            unsafe { meos_sys::geo_as_geojson(trajectory, variant as i32, 5, cstring.as_ptr()) };
        let c_str = unsafe { CStr::from_ptr(out_str) };
        let str = c_str.to_str().map_err(|_| std::fmt::Error).ok()?;
        let result = str.to_owned();
        unsafe { libc::free(out_str.cast::<c_void>()) };
        Some(result)
    }

    /// Returns the length of the trajectory.
    ///
    /// ## Returns
    ///
    /// A `f64` with the length of the trajectory.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_length`
    fn length(&self) -> f64 {
        unsafe { meos_sys::tpoint_length(self.inner()) }
    }

    /// Returns the cumulative length of the trajectory.
    ///
    /// ## Returns
    ///
    /// A `TFloat` with the cumulative length of the trajectory.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_cumulative_length`
    fn cumulative_length(&self) -> TFloat {
        factory::<TFloat>(unsafe { meos_sys::tpoint_cumulative_length(self.inner()) })
    }

    /// Returns the speed of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `TFloat` with the speed of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_speed`
    fn speed(&self) -> TFloat {
        factory::<TFloat>(unsafe { meos_sys::tpoint_speed(self.inner()) })
    }

    /// Returns the x coordinate of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `TFloat` with the x coordinate of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_get_x`
    fn x(&self) -> TFloat {
        factory::<TFloat>(unsafe { meos_sys::tpoint_get_x(self.inner()) })
    }

    /// Returns the y coordinate of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `TFloat` with the y coordinate of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_get_y`
    fn y(&self) -> TFloat {
        factory::<TFloat>(unsafe { meos_sys::tpoint_get_y(self.inner()) })
    }

    /// Returns the z coordinate of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `TFloat` with the z coordinate of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_get_z`
    fn z(&self) -> Option<TFloat> {
        if self.has_z() {
            Some(factory::<TFloat>(unsafe {
                meos_sys::tpoint_get_z(self.inner())
            }))
        } else {
            None
        }
    }

    /// Returns whether the temporal point has a z coordinate.
    ///
    /// ## Returns
    ///
    /// A `bool` indicating whether the temporal point has a z coordinate.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_start_value`
    fn has_z(&self) -> bool {
        let ptr = unsafe { meos_sys::tpoint_get_z(self.inner()) };
        !ptr.is_null()
    }

    /// Returns a `STBox` representing the bounding box of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `STBox` with the bounding box.
    ///
    /// ## MEOS Functions
    ///
    /// `tspatial_to_stbox`
    fn stbox(&self) -> STBox {
        STBox::from_inner(unsafe { meos_sys::tspatial_to_stbox(self.inner()) })
    }

    /// Returns a collection of bounding boxes representing the segments of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `Vec<STBox>` with the bounding boxes.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_stboxes`
    fn stboxes(&self) -> Vec<STBox> {
        let mut count = 0;
        let result = unsafe { meos_sys::tgeo_stboxes(self.inner(), ptr::addr_of_mut!(count)) };

        unsafe {
            std::slice::from_raw_parts(result, count as usize)
                .iter()
                .map(|&stbox| {
                    let mut boxed_stbox = Box::new(stbox);
                    let ptr: *mut meos_sys::STBox = &raw mut *boxed_stbox;
                    STBox::from_inner(ptr)
                })
                .collect()
        }
    }

    /// Returns whether the temporal point is simple (i.e., does not self-intersect).
    ///
    /// ## Returns
    ///
    /// A `bool` indicating whether the temporal point is simple.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_is_simple`
    fn is_simple(&self) -> bool {
        unsafe { meos_sys::tpoint_is_simple(self.inner()) }
    }

    /// Returns the temporal bearing between the temporal point and another point.
    ///
    /// ## Arguments
    ///
    /// * `other` - A `BaseGeometry` or `TPoint` to check the bearing to.
    ///
    /// ## Returns
    ///
    /// A `TFloat` indicating the temporal bearing between the temporal point and `other`.
    ///
    /// ## MEOS Functions
    ///
    /// `bearing_tpoint_point`, `bearing_tpoint_tpoint`
    fn bearing(&self, other: &Self::Enum) -> TFloat {
        factory::<TFloat>(unsafe { meos_sys::bearing_tpoint_tpoint(self.inner(), other.inner()) })
    }

    /// Returns the temporal bearing between the temporal point and another point.
    ///
    /// ## Arguments
    ///
    /// * `other` - A `BaseGeometry` or `TPoint` to check the bearing to.
    ///
    /// ## Returns
    ///
    /// A `TFloat` indicating the temporal bearing between the temporal point and `other`.
    ///
    /// ## MEOS Functions
    ///
    /// `bearing_tpoint_point`, `bearing_tpoint_tpoint`
    fn bearing_geometry(&self, geometry: &Geometry) -> TFloat {
        let geo = geometry_to_gserialized(geometry);
        factory::<TFloat>(unsafe { meos_sys::bearing_tpoint_point(self.inner(), geo, false) })
    }

    /// Returns the temporal azimuth of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `TFloatSequenceSet` indicating the temporal azimuth of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_azimuth`
    fn azimuth(&self) -> Option<TFloat> {
        let result = unsafe { meos_sys::tpoint_azimuth(self.inner()) };
        if result.is_null() {
            None
        } else {
            Some(factory::<TFloat>(result))
        }
    }

    /// Returns the angular difference of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `TFloatSequenceSet` indicating the temporal angular difference of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_angular_difference`
    fn angular_difference(&self) -> Option<TFloat> {
        let result = unsafe { meos_sys::tpoint_angular_difference(self.inner()) };
        if result.is_null() {
            None
        } else {
            Some(factory::<TFloat>(result))
        }
    }

    /// Returns the time-weighted centroid of the temporal point.
    ///
    /// ## Arguments
    ///
    /// * `precision` - The precision of the returned geometry.
    ///
    /// ## Returns
    ///
    /// A `BaseGeometry` indicating the time-weighted centroid of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// `tpoint_twcentroid`
    fn time_weighted_centroid(&self) -> Result<Geometry, geos::Error> {
        let gs = unsafe { meos_sys::tpoint_twcentroid(self.inner()) };
        gserialized_to_geometry(gs)
    }

    /// Returns the trajectory of the temporal point as a geos geometry.
    ///
    /// ## Arguments
    ///
    /// * `unary_union` - True when the `ST_UnaryUnion` function is applied to
    ///   the result to remove redundant geometry components. Note that applying the
    ///   `ST_UnaryUnion` function is EXTREMELY slow as reported by Issue MobilityDB#679.
    ///
    /// ## Returns
    ///
    /// A `Geometry` representing the trajectory.
    ///
    /// ## MEOS Functions
    ///
    /// `gserialized_to_geos_geometry`
    fn trajectory(&self, unary_union: bool) -> Result<Geometry, geos::Error> {
        let gs = unsafe { meos_sys::tpoint_trajectory(self.inner(), unary_union) };

        gserialized_to_geometry(gs)
    }

    // ------------------------- Spatial Reference System ----------------------

    /// Returns the SRID.
    ///
    /// MEOS Functions:
    ///     `tpoint_srid`
    fn srid(&self) -> i32 {
        unsafe { meos_sys::tspatial_srid(self.inner()) }
    }

    /// Returns a new `TPoint` with the given SRID.
    ///
    /// MEOS Functions:
    ///     `tpoint_set_srid`
    fn with_srid(&self, srid: i32) -> Self {
        Self::from_inner_as_temporal(unsafe { meos_sys::tspatial_set_srid(self.inner(), srid) })
    }

    // ------------------------- Transformations -------------------------------
    /// Round the coordinate values to a number of decimal places.
    ///
    /// Returns:
    ///     A new `TGeomPoint` object.
    ///
    /// MEOS Functions:
    ///     `tpoint_round`
    fn round(&self, max_decimals: i32) -> Self {
        Self::from_inner_as_temporal(unsafe {
            meos_sys::temporal_round(self.inner(), max_decimals)
        })
    }

    /// Split the temporal point into a collection of simple temporal points.
    ///
    /// Returns:
    ///     A `Vec<Self::Enum>`.
    ///
    /// MEOS Functions:
    ///     `tpoint_make_simple`
    fn make_simple(&self) -> Vec<Self::Enum> {
        let mut count = 0;
        let result =
            unsafe { meos_sys::tpoint_make_simple(self.inner(), ptr::addr_of_mut!(count)) };
        unsafe {
            std::slice::from_raw_parts(result, count as usize)
                .iter()
                .map(|&temporal| factory::<Self::Enum>(temporal))
                .collect()
        }
    }

    /// Expands `self` with `other`.
    /// The result is equal to `self` but with the spatial dimensions
    /// expanded by `other` in all directions.
    ///
    /// Args:
    ///     other: The object to expand `self` with.
    ///
    /// Returns:
    ///     A new `STBox` instance.
    ///
    /// MEOS Functions:
    ///     `tpoint_expand_space`
    fn expand(&self, distance: f64) -> STBox {
        STBox::from_inner(unsafe { meos_sys::stbox_expand_space(self.stbox().inner(), distance) })
    }

    /// Returns a new `TPoint` of the same subclass of `self` transformed to another SRID.
    ///
    /// Args:
    ///     srid: The desired SRID
    ///
    /// Returns:
    ///      A new `TPoint` instance.
    ///
    /// MEOS Functions:
    ///     `tpoint_transform`
    fn transform(&self, srid: i32) -> Self {
        Self::from_inner_as_temporal(unsafe { meos_sys::tspatial_transform(self.inner(), srid) })
    }

    // ------------------------- Restrictions ----------------------------------
    /// Returns a new temporal object with the values of `self` restricted to `other`.
    ///
    /// Args:
    ///     other: An object to restrict the values of `self` to.
    ///
    /// Returns:
    ///     A new `TPoint` with the values of `self` restricted to `other`.
    ///
    /// MEOS Functions:
    ///     `tpoint_at_value`, `tpoint_at_stbox`, `temporal_at_values`,
    ///     `temporal_at_timestamp`, `temporal_at_tstzset`, `temporal_at_tstzspan`, `temporal_at_tstzspanset`
    fn at_point(&self, point: Point) -> Self::Enum {
        let geo = Self::point_to_gserialized(point);
        factory::<Self::Enum>(unsafe { meos_sys::tpoint_at_value(self.inner(), geo) })
    }

    /// Returns a new temporal object with the values of `self` restricted to `other`.
    ///
    /// Args:
    ///     other: An object to restrict the values of `self` to.
    ///
    /// Returns:
    ///     A new `TPoint` with the values of `self` restricted to `other`.
    ///
    /// MEOS Functions:
    ///     `tpoint_at_value`, `tpoint_at_stbox`, `temporal_at_values`,
    ///     `temporal_at_timestamp`, `temporal_at_tstzset`, `temporal_at_tstzspan`, `temporal_at_tstzspanset`
    fn at_geometry(&self, geometry: &Geometry) -> Self::Enum {
        let geo = geometry_to_gserialized(geometry);
        factory::<Self::Enum>(unsafe { meos_sys::tpoint_at_value(self.inner(), geo) })
    }

    /// Returns a new temporal object with the values of `self` restricted to `other`.
    ///
    /// Args:
    ///     other: An object to restrict the values of `self` to.
    ///
    /// Returns:
    ///     A new `TPoint` with the values of `self` restricted to `other`.
    ///
    /// MEOS Functions:
    ///     `tpoint_at_value`, `tpoint_at_stbox`, `temporal_at_values`,
    ///     `temporal_at_timestamp`, `temporal_at_tstzset`, `temporal_at_tstzspan`, `temporal_at_tstzspanset`
    fn at_geometries(&self, geometries: &[Geometry]) -> Self::Enum {
        let mut pointers: Vec<_> = geometries
            .iter()
            .map(|g| {
                let bytes = g.to_wkb().unwrap();
                let bytes_len = bytes.len();

                unsafe {
                    meos_sys::geo_from_ewkb(
                        bytes.as_ptr(),
                        bytes_len,
                        g.get_srid().expect("No SRID"),
                    )
                }
            })
            .collect();
        let geoset = unsafe { meos_sys::geoset_make(pointers.as_mut_ptr(), pointers.len() as i32) };
        factory::<Self::Enum>(unsafe { meos_sys::temporal_at_values(self.inner(), geoset) })
    }

    /// Returns a new temporal object with the values of `self` restricted to the complement of `other`.
    ///
    /// Args:
    ///     other: An object to restrict the values of `self` to the complement of.
    ///
    /// Returns:
    ///     A new `TPoint` with the values of `self` restricted to the complement of `other`.
    ///
    /// MEOS Functions:
    ///     `tpoint_minus_value`, `tpoint_minus_stbox`, `temporal_minus_values`,
    ///     `temporal_minus_timestamp`, `temporal_minus_tstzset`, `temporal_minus_tstzspan`, `temporal_minus_tstzspanset`
    fn minus_point(&self, point: Point) -> Self::Enum {
        let geo = Self::point_to_gserialized(point);
        factory::<Self::Enum>(unsafe { meos_sys::tpoint_minus_value(self.inner(), geo) })
    }

    /// Returns a new temporal object with the values of `self` restricted to the complement of `other`.
    ///
    /// Args:
    ///     other: An object to restrict the values of `self` to the complement of.
    ///
    /// Returns:
    ///     A new `TPoint` with the values of `self` restricted to the complement of `other`.
    ///
    /// MEOS Functions:
    ///     `tpoint_minus_value`, `tpoint_minus_stbox`, `temporal_minus_values`,
    ///     `temporal_minus_timestamp`, `temporal_minus_tstzset`, `temporal_minus_tstzspan`, `temporal_minus_tstzspanset`
    fn minus_geometry(&self, geometry: &Geometry) -> Self::Enum {
        let geo = geometry_to_gserialized(geometry);
        factory::<Self::Enum>(unsafe { meos_sys::tpoint_minus_value(self.inner(), geo) })
    }

    /// Returns a new temporal object with the values of `self` restricted to the complement of `other`.
    ///
    /// Args:
    ///     other: An object to restrict the values of `self` to the complement of.
    ///
    /// Returns:
    ///     A new `TPoint` with the values of `self` restricted to the complement of `other`.
    ///
    /// MEOS Functions:
    ///     `tpoint_minus_value`, `tpoint_minus_stbox`, `temporal_minus_values`,
    ///     `temporal_minus_timestamp`, `temporal_minus_tstzset`, `temporal_minus_tstzspan`, `temporal_minus_tstzspanset`
    fn minus_geometries(&self, geometries: &[Geometry]) -> Self::Enum {
        let mut pointers: Vec<_> = geometries
            .iter()
            .map(|g| {
                let bytes = g.to_wkb().unwrap();
                let bytes_len = bytes.len();

                unsafe {
                    meos_sys::geo_from_ewkb(
                        bytes.as_ptr(),
                        bytes_len,
                        g.get_srid().expect("No SRID"),
                    )
                }
            })
            .collect();
        let geoset = unsafe { meos_sys::geoset_make(pointers.as_mut_ptr(), pointers.len() as i32) };
        factory::<Self::Enum>(unsafe { meos_sys::temporal_minus_values(self.inner(), geoset) })
    }

    // ------------------------- Position Operations ---------------------------

    /// Returns whether the bounding box of `self` is below to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if below, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_before`
    fn is_below(&self, other: &Self::Enum) -> bool {
        unsafe { meos_sys::below_tspatial_tspatial(self.inner(), other.inner()) }
    }

    /// Returns whether the bounding box of `self` is over or below to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if over or below, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_over_or_before`
    fn is_over_or_below(&self, other: &Self::Enum) -> bool {
        unsafe { meos_sys::overbelow_tspatial_tspatial(self.inner(), other.inner()) }
    }

    /// Returns whether the bounding box of `self` is above to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if above, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_after`
    fn is_above(&self, other: &Self::Enum) -> bool {
        unsafe { meos_sys::above_tspatial_tspatial(self.inner(), other.inner()) }
    }

    /// Returns whether the bounding box of `self` is over or above to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if over or above, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_over_or_before`
    fn is_over_or_above(&self, other: &Self::Enum) -> bool {
        unsafe { meos_sys::overabove_tspatial_tspatial(self.inner(), other.inner()) }
    }

    /// Returns whether the bounding box of `self` is front to the bounding box of `other`. Both must have 3rd dimension
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if front, False otherwise.
    fn is_front(&self, other: &Self::Enum) -> Option<bool> {
        if self.has_z() {
            Some(unsafe { meos_sys::front_tspatial_tspatial(self.inner(), other.inner()) })
        } else {
            None
        }
    }

    /// Returns whether the bounding box of `self` is over or front to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if over or front, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_over_or_before`
    fn is_over_or_front(&self, other: &Self::Enum) -> Option<bool> {
        if self.has_z() {
            Some(unsafe { meos_sys::overfront_tspatial_tspatial(self.inner(), other.inner()) })
        } else {
            None
        }
    }

    /// Returns whether the bounding box of `self` is behind to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if behind, False otherwise.
    fn is_behind(&self, other: &Self::Enum) -> Option<bool> {
        if self.has_z() {
            Some(unsafe { meos_sys::back_tspatial_tspatial(self.inner(), other.inner()) })
        } else {
            None
        }
    }

    /// Returns whether the bounding box of `self` is over or behind to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if over or behind, False otherwise.
    fn is_over_or_behind(&self, other: &Self::Enum) -> Option<bool> {
        if self.has_z() {
            Some(unsafe { meos_sys::overback_tspatial_tspatial(self.inner(), other.inner()) })
        } else {
            None
        }
    }

    /// Returns a new temporal boolean indicating whether the temporal point is contained by `container`.
    ///
    /// # Arguments
    ///
    /// * `container` - An object to check for containing `self`.
    ///
    /// # Returns
    ///
    /// A new `TBool` indicating whether the temporal point is contained by `container`.
    ///
    /// # MEOS Functions
    ///
    /// * `tcontains_geo_tgeo`
    fn is_spatially_contained_in_geometry(&self, container: &Geometry) -> Self::TBoolType {
        let geo = geometry_to_gserialized(container);
        Self::TBoolType::from_inner_as_temporal(unsafe {
            meos_sys::tcontains_geo_tgeo(geo, self.inner(), false, false)
        })
    }

    /// Returns a new temporal boolean indicating whether the temporal point intersects `geometry`.
    ///
    /// # Arguments
    ///
    /// * `geometry` - An object to check for intersection with.
    ///
    /// # Returns
    ///
    /// A new `TBool` indicating whether the temporal point intersects `geometry`.
    ///
    /// # MEOS Functions
    ///
    /// * `tintersects_tgeo_geo`
    fn is_disjoint_to_geometry(&self, geometry: &Geometry) -> Self::TBoolType {
        let geo = geometry_to_gserialized(geometry);
        Self::TBoolType::from_inner_as_temporal(unsafe {
            meos_sys::tdisjoint_tgeo_geo(self.inner(), geo, false, false)
        })
    }

    /// Returns a new temporal boolean indicating whether the temporal point is within `distance` of `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - An object to check the distance to.
    /// * `distance` - The distance to check in units of the spatial reference system.
    ///
    /// # Returns
    ///
    /// A new `TBool` indicating whether the temporal point is within `distance` of `other`.
    ///
    /// # MEOS Functions
    ///
    /// * `tdwithin_tgeo_geo`, `tdwithin_tgeo_tgeo`
    fn is_within_distance(&self, other: &Self::Enum, distance: f64) -> Self::TBoolType {
        Self::TBoolType::from_inner_as_temporal(unsafe {
            meos_sys::tdwithin_tgeo_tgeo(self.inner(), other.inner(), distance, false, false)
        })
    }

    /// Returns a new temporal boolean indicating whether the temporal point is within `distance` of `geometry`.
    ///
    /// # Arguments
    ///
    /// * `geometry` - An object to check the distance to.
    /// * `distance` - The distance to check in units of the spatial reference system.
    ///
    /// # Returns
    ///
    /// A new `TBool` indicating whether the temporal point is within `distance` of `geometry`.
    ///
    /// # MEOS Functions
    ///
    /// * `tdwithin_tgeo_geo`, `tdwithin_tgeo_tgeo`
    fn within_distance_of_geometry(&self, geometry: &Geometry, distance: f64) -> Self::TBoolType {
        let geo = geometry_to_gserialized(geometry);
        Self::TBoolType::from_inner_as_temporal(unsafe {
            meos_sys::tdwithin_tgeo_geo(self.inner(), geo, distance, false, false)
        })
    }

    /// Returns a new temporal boolean indicating whether the temporal point intersects `geometry`.
    ///
    /// # Arguments
    ///
    /// * `geometry` - An object to check for intersection with.
    ///
    /// # Returns
    ///
    /// A new `TBool` indicating whether the temporal point intersects `geometry`.
    ///
    /// # MEOS Functions
    ///
    /// * `tintersects_tgeo_geo`
    fn intersects_geometry(&self, geometry: &Geometry) -> Self::TBoolType {
        let geo = geometry_to_gserialized(geometry);
        Self::TBoolType::from_inner_as_temporal(unsafe {
            meos_sys::tintersects_tgeo_geo(self.inner(), geo, false, false)
        })
    }

    /// Returns a new temporal boolean indicating whether the temporal point touches `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - An object to check for touching with.
    ///
    /// # Returns
    ///
    /// A new `TBool` indicating whether the temporal point touches `other`.
    ///
    /// # MEOS Functions
    ///
    /// * `ttouches_tgeo_geo`
    fn touches_geometry(&self, geometry: &Geometry) -> Self::TBoolType {
        let geo = geometry_to_gserialized(geometry);
        Self::TBoolType::from_inner_as_temporal(unsafe {
            meos_sys::ttouches_tgeo_geo(self.inner(), geo, false, false)
        })
    }

    /// Returns the temporal distance between the temporal point and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - An object to check the distance to.
    ///
    /// # Returns
    ///
    /// A new `TFloat` indicating the temporal distance between the temporal point and `other`.
    ///
    /// # MEOS Functions
    ///
    /// * `distance_tgeo_point`, `distance_tgeo_tgeo`
    fn distance(&self, other: &Self::Enum) -> TFloat {
        factory::<TFloat>(unsafe { meos_sys::tdistance_tgeo_tgeo(self.inner(), other.inner()) })
    }

    /// Returns the temporal distance between the temporal point and `other`.
    ///
    /// # Arguments
    ///
    /// * `geometry` - An object to check the distance to.
    ///
    /// # Returns
    ///
    /// A new `TFloat` indicating the temporal distance between the temporal point and `geometry`.
    ///
    /// # MEOS Functions
    ///
    /// * `distance_tgeo_point`, `distance_tgeo_tgeo`
    fn distance_to_point(&self, point: Point) -> TFloat {
        let point = Self::point_to_gserialized(point);
        factory::<TFloat>(unsafe { meos_sys::tdistance_tgeo_geo(self.inner(), point) })
    }

    /// Returns the nearest approach distance between the temporal point and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - An object to check the nearest approach distance to.
    ///
    /// # Returns
    ///
    /// A `f64` indicating the nearest approach distance between the temporal point and `other`.
    ///
    /// # MEOS Functions
    ///
    /// * `nad_tgeo_geo`, `nad_tgeo_stbox`, `nad_tgeo_tgeo`
    fn nearest_approach_distance(&self, other: &Self::Enum) -> f64 {
        unsafe { meos_sys::nad_tgeo_tgeo(self.inner(), other.inner()) }
    }

    /// Returns the nearest approach distance between the temporal point and `other`.
    ///
    /// # Arguments
    ///
    /// * `geometry` - An object to check the nearest approach distance to.
    ///
    /// # Returns
    ///
    /// A `f64` indicating the nearest approach distance between the temporal point and `geometry`.
    ///
    /// # MEOS Functions
    ///
    /// * `nad_tgeo_geo`, `nad_tgeo_stbox`, `nad_tgeo_tgeo`
    fn nearest_approach_distance_to_geometry(&self, geometry: &Geometry) -> f64 {
        let geo = geometry_to_gserialized(geometry);
        unsafe { meos_sys::nad_tgeo_geo(self.inner(), geo) }
    }

    /// Returns the nearest approach instant between the temporal point and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - An object to check the nearest approach instant to.
    ///
    /// # Returns
    ///
    /// A new temporal instant indicating the nearest approach instant between the temporal point and `other`.
    ///
    /// # MEOS Functions
    ///
    /// * `nai_tgeo_geo`, `nai_tgeo_tgeo`
    fn nearest_approach_instant(&self, other: &Self::Enum) -> Self::TI {
        Self::TI::from_inner(unsafe { meos_sys::nai_tgeo_tgeo(self.inner(), other.inner()) })
    }

    /// Returns the nearest approach instant between the temporal point and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - An object to check the nearest approach instant to.
    ///
    /// # Returns
    ///
    /// A new temporal instant indicating the nearest approach instant between the temporal point and `other`.
    ///
    /// # MEOS Functions
    ///
    /// * `nai_tgeo_geo`
    fn nearest_approach_instant_to_geometry(&self, geometry: &Geometry) -> Self::TI {
        let geo = geometry_to_gserialized(geometry);
        Self::TI::from_inner(unsafe { meos_sys::nai_tgeo_geo(self.inner(), geo) })
    }

    /// Returns the shortest line between the temporal point and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - An object to check the shortest line to.
    ///
    /// # Returns
    ///
    /// A new `BaseGeometry` indicating the shortest line between the temporal point and `other`.
    ///
    /// # MEOS Functions
    ///
    /// * `shortestline_tgeo_geo`, `shortestline_tgeo_tgeo`
    fn shortest_line(&self, other: &Self::Enum) -> Result<Geometry, geos::Error> {
        let gs = unsafe { meos_sys::shortestline_tgeo_tgeo(self.inner(), other.inner()) };
        gserialized_to_geometry(gs)
    }

    /// Returns the shortest line between the temporal point and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - An object to check the shortest line to.
    ///
    /// # Returns
    ///
    /// A new `BaseGeometry` indicating the shortest line between the temporal point and `other`.
    ///
    /// # MEOS Functions
    ///
    /// * `shortestline_tgeo_geo`, `shortestline_tgeo_tgeo`
    fn shortest_line_to_geometry(&self, geometry: &Geometry) -> Result<Geometry, geos::Error> {
        let geo = geometry_to_gserialized(geometry);
        let gs = unsafe { meos_sys::shortestline_tgeo_geo(self.inner(), geo) };
        gserialized_to_geometry(gs)
    }

    // /// Split the temporal point into segments following the tiling of the bounding box.
    // ///
    // /// # Arguments
    // ///
    // /// * `size` - The size of the spatial tiles. If `self` has a spatial dimension and this argument is not provided, the tiling will be only temporal.
    // /// * `duration` - The duration of the temporal tiles. If `self` has a time dimension and this argument is not provided, the tiling will be only spatial.
    // /// * `origin` - The origin of the spatial tiling. If not provided, the origin will be (0, 0, 0).
    // /// * `start` - The start time of the temporal tiling. If not provided, the start time used by default is Monday, January 3, 2000.
    // /// * `remove_empty` - If True, remove the tiles that are empty.
    // ///
    // /// # Returns
    // ///
    // /// A list of `TPoint` objects.
    // ///
    // /// # See Also
    // ///
    // /// `STBox::tile`
    // fn tile(
    //     &self,
    //     size: f64,
    //     duration: Option<&str>,
    //     origin: Option<&impl Geometry>,
    //     start: Option<&str>,
    //     remove_empty: bool,
    // ) -> Vec<Self::Enum> {
    //     let bbox = STBox::from_tpoint(self);
    //     let tiles = bbox.tile(size, duration, origin, start);
    //     if remove_empty {
    //         tiles.iter().filter_map(|tile| self.at(tile)).collect()
    //     } else {
    //         tiles.iter().map(|tile| self.at(tile)).collect()
    //     }
    // }

    // /// Splits `self` into fragments with respect to space buckets.
    // ///
    // /// # Arguments
    // ///
    // /// * `xsize` - Size of the x dimension.
    // /// * `ysize` - Size of the y dimension.
    // /// * `zsize` - Size of the z dimension.
    // /// * `origin` - The origin of the spatial tiling. If not provided, the origin will be (0, 0, 0).
    // /// * `bitmatrix` - If True, use a bitmatrix to speed up the process.
    // /// * `include_border` - If True, include the upper border in the box.
    // ///
    // /// # Returns
    // ///
    // /// A list of temporal points.
    // ///
    // /// # MEOS Functions
    // ///
    // /// * `tpoint_value_split`
    // fn space_split(
    //     &self,
    //     xsize: f64,
    //     ysize: Option<f64>,
    //     zsize: Option<f64>,
    //     origin: Option<&impl Geometry>,
    //     bitmatrix: bool,
    //     include_border: bool,
    // ) -> Vec<Temporal> {
    //     let ysz = ysize.unwrap_or(xsize);
    //     let zsz = zsize.unwrap_or(xsize);
    //     let gs = match origin {
    //         Some(geo) => geo_to_gserialized(geo, self.is_geog_point()),
    //         None => {
    //             if self.is_geog_point() {
    //                 pgis_geography_in("Point(0 0 0)", -1)
    //             } else {
    //                 pgis_geometry_in("Point(0 0 0)", -1)
    //             }
    //         }
    //     };
    //     let (fragments, values, count) =
    //         tpoint_space_split(self.inner(), xsize, ysz, zsz, gs, bitmatrix, include_border);
    //     (0..count).map(|i| Temporal::new(fragments[i])).collect()
    // }

    // /// Splits `self` into fragments with respect to space and tstzspan buckets.
    // ///
    // /// # Arguments
    // ///
    // /// * `xsize` - Size of the x dimension.
    // /// * `duration` - Duration of the tstzspan buckets.
    // /// * `ysize` - Size of the y dimension.
    // /// * `zsize` - Size of the z dimension.
    // /// * `origin` - The origin of the spatial tiling. If not provided, the origin will be (0, 0, 0).
    // /// * `time_start` - Start time of the first tstzspan bucket. If None, the start time used by default is Monday, January 3, 2000.
    // /// * `bitmatrix` - If True, use a bitmatrix to speed up the process.
    // /// * `include_border` - If True, include the upper border in the box.
    // ///
    // /// # Returns
    // ///
    // /// A list of temporal floats.
    // ///
    // /// # MEOS Functions
    // ///
    // /// * `tfloat_value_time_split`
    // fn space_time_split(
    //     &self,
    //     xsize: f64,
    //     duration: &str,
    //     ysize: Option<f64>,
    //     zsize: Option<f64>,
    //     origin: Option<&impl Geometry>,
    //     time_start: Option<&str>,
    //     bitmatrix: bool,
    //     include_border: bool,
    // ) -> Vec<Temporal> {
    //     let ysz = ysize.unwrap_or(xsize);
    //     let zsz = zsize.unwrap_or(xsize);
    //     let dt = pg_interval_in(duration, -1);
    //     let gs = match origin {
    //         Some(geo) => geo_to_gserialized(geo, self.is_geog_point()),
    //         None => {
    //             if self.is_geog_point() {
    //                 pgis_geography_in("Point(0 0 0)", -1)
    //             } else {
    //                 pgis_geometry_in("Point(0 0 0)", -1)
    //             }
    //         }
    //     };
    //     let st = match time_start {
    //         Some(start) => pg_timestamptz_in(start, -1),
    //         None => pg_timestamptz_in("2000-01-03", -1),
    //     };
    //     let (fragments, points, times, count) = tpoint_space_time_split(
    //         self.inner(),
    //         xsize,
    //         ysz,
    //         zsz,
    //         dt,
    //         gs,
    //         st,
    //         bitmatrix,
    //         include_border,
    //     );
    //     (0..count).map(|i| Temporal::new(fragments[i])).collect()
    // }
}

macro_rules! impl_tpoint_traits {
    ($type:ty, $temporal_type:ident, $geodetic:expr, $enum_prefix:ident) => {
        paste::paste! {
            impl TGeoTrait for $type {
                fn point_to_gserialized(point: Point) -> *mut meos_sys::GSERIALIZED {
                    if $geodetic {
                        point_to_gserialized_geog(point)
                    } else {
                        point_to_gserialized_geom(point)
                    }
                }
            }
            impl Collection for $type {
                impl_collection!(tspatial, Geometry);
                fn contains(&self, element: &Self::Type) -> bool {
                    unsafe { meos_sys::contains_tspatial_stbox(self.inner(), meos_sys::geo_to_stbox(geometry_to_gserialized(element))) }
                }
            }

            impl_simple_traits_for_temporal!($type, with_drop);
            impl core::fmt::Debug for $type {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_str(&self.as_wkt(5))
                }
            }
            impl SimplifiableTemporal for $type {}

            impl From<$type> for [<$enum_prefix>] {
                fn from(value: $type) -> Self {
                    Self::$temporal_type(value)
                }
            }

            impl Temporal for $type {
                type TI = [<$enum_prefix Instant>];
                type TS = [<$enum_prefix Sequence>];
                type TSS = [<$enum_prefix SequenceSet>];
                type TBB = STBox;
                type Enum = [<$enum_prefix>];
                type TBoolType = [<TBool $temporal_type>];

                impl_always_and_ever_value_equality_functions!(geo, geometry_to_gserialized);
                fn from_inner_as_temporal(inner: *mut meos_sys::Temporal) -> Self {
                    Self {
                        #[allow(clippy::cast_ptr_alignment)]
                        _inner: ptr::NonNull::new(inner.cast::<meos_sys::[<T $temporal_type>]>()).expect("Null pointers not allowed"),
                    }
                }

                fn inner(&self) -> *const meos_sys::Temporal {
                    self._inner.as_ptr() as *const meos_sys::Temporal
                }

                fn bounding_box(&self) -> Self::TBB {
                    STBox::from_inner(unsafe { meos_sys::tspatial_to_stbox(self.inner()) })
                }

                fn values(&self) -> Vec<Self::Type> {
                    let mut count = 0;
                    unsafe {
                        let values = meos_sys::tgeo_values(self.inner(), ptr::addr_of_mut!(count));

                        std::slice::from_raw_parts(values, count as usize)
                            .into_iter()
                            .map(|&gs| gserialized_to_geometry(gs))
                            .map(Result::unwrap)
                            .collect()
                    }
                }

                fn start_value(&self) -> Self::Type {
                    gserialized_to_geometry(unsafe { meos_sys::tgeo_start_value(self.inner()) }).unwrap()
                }

                fn end_value(&self) -> Self::Type {
                    gserialized_to_geometry(unsafe { meos_sys::tgeo_end_value(self.inner()) }).unwrap()
                }

                fn value_at_timestamp<Tz: TimeZone>(
                    &self,
                    timestamp: DateTime<Tz>,
                ) -> Option<Self::Type> {
                    let mut result: mem::MaybeUninit<*mut meos_sys::GSERIALIZED> = mem::MaybeUninit::uninit();
                    unsafe {
                        let success = meos_sys::tgeo_value_at_timestamptz(
                            self.inner(),
                            to_meos_timestamp(&timestamp),
                            true,
                            result.as_mut_ptr(),
                        );
                        if success {
                            Some(gserialized_to_geometry(result.assume_init()).unwrap())
                        } else {
                            None
                        }
                    }
                }

                fn at_value(&self, value: &Self::Type) -> Option<Self::Enum> {
                    let result = unsafe { meos_sys::tpoint_at_value(self.inner(), geometry_to_gserialized(value)) };
                    if !result.is_null() {
                        Some(factory::<Self::Enum>(result))
                    } else {
                        None
                    }
                }
                fn at_values(&self, values: &[Self::Type]) -> Option<Self::Enum> {
                    unsafe {
                        let mut cgeos: Vec<_> = values.into_iter().map(|geo| geometry_to_gserialized(&geo)).collect();
                        let set = meos_sys::geoset_make(cgeos.as_mut_ptr(), values.len() as i32);
                        let result = meos_sys::temporal_at_values(self.inner(), set);
                        if !result.is_null() {
                            Some(factory::<Self::Enum>(result))
                        } else {
                            None
                        }
                    }
                }

                fn minus_value(&self, value: Self::Type) -> Self::Enum {
                    factory::<Self::Enum>(unsafe {
                        meos_sys::tpoint_minus_value(self.inner(), geometry_to_gserialized(&value))
                    })
                }

                fn minus_values(&self, values: &[Self::Type]) -> Self::Enum {
                    factory::<Self::Enum>(unsafe {
                        let mut cgeos: Vec<_> = values.into_iter().map(|geo| geometry_to_gserialized(&geo)).collect();
                        let set = meos_sys::geoset_make(cgeos.as_mut_ptr(), values.len() as i32);
                        meos_sys::temporal_minus_values(self.inner(), set)
                    })
                }

                fn temporal_equal_value(&self, value: &Self::Type) -> Self::TBoolType {
                    Self::TBoolType::from_inner_as_temporal(unsafe {
                        meos_sys::teq_tgeo_geo(self.inner(), geometry_to_gserialized(value))
                    })
                }

                fn temporal_not_equal_value(&self, value: &Self::Type) -> Self::TBoolType {
                    Self::TBoolType::from_inner_as_temporal(unsafe {
                        meos_sys::tne_tgeo_geo(self.inner(), geometry_to_gserialized(value))
                    })
                }
            }
        }
    };
}

pub(super) use impl_tpoint_traits;

/// Generates all concrete structs, enum, and trait impls for a geo temporal type.
///
/// Parameters:
/// - `$prefix`:      e.g. `TGeomPoint`, `TGeometry`
/// - `$geodetic`:    `true` / `false`
/// - `$inst_make`:   C function to create an instant, e.g. `tpointinst_make` or `tgeoinst_make`
/// - `$in_fn`:       C parse function, e.g. `tgeompoint_in`
/// - `$mfjson_fn`:   C mfjson function, e.g. `tgeompoint_from_mfjson`
macro_rules! impl_tgeo_type {
    ($prefix:ident, $geodetic:expr, $inst_make:ident, $in_fn:ident, $mfjson_fn:ident) => {
        paste::paste! {
            pub struct [<$prefix Instant>] {
                _inner: ptr::NonNull<meos_sys::TInstant>,
            }
            impl_tpoint_traits!([<$prefix Instant>], Instant, $geodetic, $prefix);
            impl TInstant for [<$prefix Instant>] {
                fn from_inner(inner: *mut meos_sys::TInstant) -> Self {
                    Self { _inner: ptr::NonNull::new(inner).expect("Null pointers not allowed") }
                }
                fn inner_as_tinstant(&self) -> *const meos_sys::TInstant { self._inner.as_ptr() }
                fn from_value_and_timestamp<Tz: TimeZone>(value: Self::Type, timestamp: DateTime<Tz>) -> Self {
                    Self::from_inner(unsafe {
                        meos_sys::$inst_make(geometry_to_gserialized(&value), to_meos_timestamp(&timestamp))
                    })
                }
            }
            impl<Tz: TimeZone> From<(Geometry, DateTime<Tz>)> for [<$prefix Instant>] {
                fn from((value, timestamp): (Geometry, DateTime<Tz>)) -> Self {
                    Self::from_value_and_timestamp(value, timestamp)
                }
            }

            pub struct [<$prefix Sequence>] {
                _inner: ptr::NonNull<meos_sys::TSequence>,
            }
            impl_tpoint_traits!([<$prefix Sequence>], Sequence, $geodetic, $prefix);
            impl TSequence for [<$prefix Sequence>] {
                fn from_inner(inner: *mut meos_sys::TSequence) -> Self {
                    Self { _inner: ptr::NonNull::new(inner).expect("Null pointers not allowed") }
                }
                fn inner_mut_as_tsequence(&self) -> *mut meos_sys::TSequence { self._inner.as_ptr() }
            }
            impl FromIterator<[<$prefix Instant>]> for [<$prefix Sequence>] {
                fn from_iter<T: IntoIterator<Item = [<$prefix Instant>]>>(iter: T) -> Self {
                    let vec: Vec<_> = iter.into_iter().collect();
                    Self::new(&vec, TInterpolation::Linear)
                }
            }
            impl<'a> FromIterator<&'a [<$prefix Instant>]> for [<$prefix Sequence>] {
                fn from_iter<T: IntoIterator<Item = &'a [<$prefix Instant>]>>(iter: T) -> Self {
                    let vec: Vec<_> = iter.into_iter().collect();
                    Self::new(&vec, TInterpolation::Linear)
                }
            }
            impl<Tz: TimeZone> FromIterator<(Geometry, DateTime<Tz>)> for [<$prefix Sequence>] {
                fn from_iter<T: IntoIterator<Item = (Geometry, DateTime<Tz>)>>(iter: T) -> Self {
                    iter.into_iter().map(Into::<[<$prefix Instant>]>::into).collect()
                }
            }

            pub struct [<$prefix SequenceSet>] {
                _inner: ptr::NonNull<meos_sys::TSequenceSet>,
            }
            impl_tpoint_traits!([<$prefix SequenceSet>], SequenceSet, $geodetic, $prefix);
            impl TSequenceSet for [<$prefix SequenceSet>] {
                fn from_inner(inner: *mut meos_sys::TSequenceSet) -> Self {
                    Self { _inner: ptr::NonNull::new(inner).expect("Null pointers not allowed") }
                }
            }

            #[derive(Debug)]
            pub enum $prefix {
                Instant([<$prefix Instant>]),
                Sequence([<$prefix Sequence>]),
                SequenceSet([<$prefix SequenceSet>]),
            }
            impl TGeoTrait for $prefix {
                fn point_to_gserialized(point: Point) -> *mut meos_sys::GSERIALIZED {
                    if $geodetic { point_to_gserialized_geog(point) } else { point_to_gserialized_geom(point) }
                }
            }
            impl MeosEnum for $prefix {
                fn from_instant(inner: *mut meos_sys::TInstant) -> Self {
                    Self::Instant([<$prefix Instant>]::from_inner(inner))
                }
                fn from_sequence(inner: *mut meos_sys::TSequence) -> Self {
                    Self::Sequence([<$prefix Sequence>]::from_inner(inner))
                }
                fn from_sequence_set(inner: *mut meos_sys::TSequenceSet) -> Self {
                    Self::SequenceSet([<$prefix SequenceSet>]::from_inner(inner))
                }
                fn from_mfjson(mfjson: &str) -> Self {
                    let cstr = CString::new(mfjson).unwrap();
                    factory::<Self>(unsafe { meos_sys::$mfjson_fn(cstr.as_ptr()) })
                }
            }
            impl FromStr for $prefix {
                type Err = ParseError;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    CString::new(s).map_err(|_| ParseError).map(|string| {
                        factory::<Self>(unsafe { meos_sys::$in_fn(string.as_ptr()) })
                    })
                }
            }
            impl Collection for $prefix {
                impl_collection!(tspatial, Geometry);
                fn contains(&self, element: &Self::Type) -> bool {
                    unsafe { meos_sys::contains_tspatial_stbox(self.inner(), meos_sys::geo_to_stbox(geometry_to_gserialized(element))) }
                }
            }
            impl_simple_traits_for_temporal!($prefix);
            impl SimplifiableTemporal for $prefix {}
            impl Temporal for $prefix {
                type TI = [<$prefix Instant>];
                type TS = [<$prefix Sequence>];
                type TSS = [<$prefix SequenceSet>];
                type TBB = STBox;
                type Enum = $prefix;
                type TBoolType = TBool;
                impl_always_and_ever_value_equality_functions!(geo, geometry_to_gserialized);
                fn from_inner_as_temporal(inner: *mut meos_sys::Temporal) -> Self { factory::<Self>(inner) }
                fn inner(&self) -> *const meos_sys::Temporal {
                    match self {
                        $prefix::Instant(v) => v.inner(),
                        $prefix::Sequence(v) => v.inner(),
                        $prefix::SequenceSet(v) => v.inner(),
                    }
                }
                fn bounding_box(&self) -> Self::TBB {
                    STBox::from_inner(unsafe { meos_sys::tspatial_to_stbox(self.inner()) })
                }
                fn values(&self) -> Vec<Self::Type> {
                    let mut count = 0;
                    unsafe {
                        let values = meos_sys::tgeo_values(self.inner(), ptr::addr_of_mut!(count));
                        std::slice::from_raw_parts(values, count as usize)
                            .iter().map(|&gs| gserialized_to_geometry(gs).unwrap()).collect()
                    }
                }
                fn start_value(&self) -> Self::Type {
                    gserialized_to_geometry(unsafe { meos_sys::tgeo_start_value(self.inner()) }).unwrap()
                }
                fn end_value(&self) -> Self::Type {
                    gserialized_to_geometry(unsafe { meos_sys::tgeo_end_value(self.inner()) }).unwrap()
                }
                fn value_at_timestamp<Tz: TimeZone>(&self, timestamp: DateTime<Tz>) -> Option<Self::Type> {
                    let mut result: mem::MaybeUninit<*mut meos_sys::GSERIALIZED> = mem::MaybeUninit::uninit();
                    unsafe {
                        let success = meos_sys::tgeo_value_at_timestamptz(self.inner(), to_meos_timestamp(&timestamp), true, result.as_mut_ptr());
                        if success { Some(gserialized_to_geometry(result.assume_init()).unwrap()) } else { None }
                    }
                }
                fn at_value(&self, value: &Self::Type) -> Option<Self::Enum> {
                    let result = unsafe { meos_sys::tpoint_at_value(self.inner(), geometry_to_gserialized(value)) };
                    if result.is_null() { None } else { Some(factory::<Self::Enum>(result)) }
                }
                fn at_values(&self, values: &[Self::Type]) -> Option<Self::Enum> {
                    unsafe {
                        let set = create_set_of_geometries(values);
                        let result = meos_sys::temporal_at_values(self.inner(), set);
                        if result.is_null() { None } else { Some(factory::<Self::Enum>(result)) }
                    }
                }
                fn minus_value(&self, value: Self::Type) -> Self::Enum {
                    factory::<Self::Enum>(unsafe { meos_sys::tpoint_minus_value(self.inner(), geometry_to_gserialized(&value)) })
                }
                fn minus_values(&self, values: &[Self::Type]) -> Self::Enum {
                    factory::<Self::Enum>(unsafe {
                        let set = create_set_of_geometries(values);
                        meos_sys::temporal_minus_values(self.inner(), set)
                    })
                }
                fn temporal_equal_value(&self, value: &Self::Type) -> Self::TBoolType {
                    Self::TBoolType::from_inner_as_temporal(unsafe { meos_sys::teq_tgeo_geo(self.inner(), geometry_to_gserialized(value)) })
                }
                fn temporal_not_equal_value(&self, value: &Self::Type) -> Self::TBoolType {
                    Self::TBoolType::from_inner_as_temporal(unsafe { meos_sys::tne_tgeo_geo(self.inner(), geometry_to_gserialized(value)) })
                }
            }
        }
    }
}
pub(super) use impl_tgeo_type;
