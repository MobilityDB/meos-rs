use chrono::{DateTime, TimeZone};
use geos::Geometry;

use crate::{
    boxes::stbox::STBox,
    temporal::number::tfloat::{TFloat, TFloatSequenceSet},
};

pub struct Point(isize, isize, isize);
pub trait TPoint {
    /// Returns the string representation of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `String` representing the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_out
    fn to_string(&self) -> String;

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
    /// tpoint_out
    fn as_wkt(&self, precision: i32) -> String;

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
    /// tpoint_as_ewkt
    fn as_ewkt(&self, precision: i32) -> String;

    /// Returns the trajectory of the temporal point as a GeoJSON string.
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
    /// gserialized_as_geojson
    fn as_geojson(&self, option: i32, precision: i32, srs: Option<String>) -> String;

    #[cfg(feature = "geos")]
    /// Returns the trajectory of the temporal point as a geos geometry.
    ///
    /// ## Arguments
    ///
    /// * `precision` - The precision of the returned geometry.
    ///
    /// ## Returns
    ///
    /// A `BaseGeometry` representing the trajectory.
    ///
    /// ## MEOS Functions
    ///
    /// gserialized_to_geos_geometry
    fn to_geos_geometry(&self, precision: i32) -> Geometry;

    /// Returns the bounding box of the temporal point.
    ///
    /// ## Returns
    ///
    /// An `STBox` representing the bounding box.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_to_stbox
    fn bounding_box(&self) -> STBox;

    /// Returns the values of the temporal point.
    ///
    /// ## Arguments
    ///
    /// * `precision` - The precision of the returned values.
    ///
    /// ## Returns
    ///
    /// A `Vec<Point>` with the values.
    ///
    /// ## MEOS Functions
    ///
    /// temporal_instants
    fn values(&self, precision: i32) -> Vec<Point>;

    /// Returns the start value of the temporal point.
    ///
    /// ## Arguments
    ///
    /// * `precision` - The precision of the returned value.
    ///
    /// ## Returns
    ///
    /// A `Point` with the start value.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_start_value
    fn start_value(&self, precision: i32) -> Point;

    /// Returns the end value of the temporal point.
    ///
    /// ## Arguments
    ///
    /// * `precision` - The precision of the returned value.
    ///
    /// ## Returns
    ///
    /// A `Point` with the end value.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_end_value
    fn end_value(&self, precision: i32) -> Point;

    /// Returns the set of values of the temporal point.
    ///
    /// ## Arguments
    ///
    /// * `precision` - The precision of the returned values.
    ///
    /// ## Returns
    ///
    /// A `HashSet<Point>` with the values.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_values
    //fn value_set(&self, precision: i32) -> HashSet<Point>;

    /// Returns the value of the temporal point at the given timestamp.
    ///
    /// ## Arguments
    ///
    /// * `timestamp` - A `DateTime` representing the timestamp.
    /// * `precision` - The precision of the returned value.
    ///
    /// ## Returns
    ///
    /// A `Point` with the value.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_value_at_timestamp
    fn value_at_timestamp<Tz: TimeZone>(&self, timestamp: DateTime<Tz>, precision: i32) -> Point;

    /// Returns the length of the trajectory.
    ///
    /// ## Returns
    ///
    /// A `f64` with the length of the trajectory.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_length
    fn length(&self) -> f64;

    /// Returns the cumulative length of the trajectory.
    ///
    /// ## Returns
    ///
    /// A `TFloat` with the cumulative length of the trajectory.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_cumulative_length
    fn cumulative_length(&self) -> TFloat;

    /// Returns the speed of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `TFloat` with the speed of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_speed
    fn speed(&self) -> TFloat;

    /// Returns the x coordinate of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `TFloat` with the x coordinate of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_get_x
    fn x(&self) -> TFloat;

    /// Returns the y coordinate of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `TFloat` with the y coordinate of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_get_y
    fn y(&self) -> TFloat;

    /// Returns the z coordinate of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `TFloat` with the z coordinate of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_get_z
    fn z(&self) -> TFloat;

    /// Returns whether the temporal point has a z coordinate.
    ///
    /// ## Returns
    ///
    /// A `bool` indicating whether the temporal point has a z coordinate.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_start_value
    fn has_z(&self) -> bool;

    /// Returns a collection of bounding boxes representing the segments of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `Vec<STBox>` with the bounding boxes.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_stboxes
    fn stboxes(&self) -> Vec<STBox>;

    /// Returns whether the temporal point is simple (i.e., does not self-intersect).
    ///
    /// ## Returns
    ///
    /// A `bool` indicating whether the temporal point is simple.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_is_simple
    fn is_simple(&self) -> bool;

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
    /// bearing_tpoint_point, bearing_tpoint_tpoint
    fn bearing(&self, other: &dyn Any) -> TFloat;

    /// Returns the azimuth of the temporal point between the start and end locations.
    ///
    /// ## Returns
    ///
    /// A `f64` indicating the direction of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_direction
    fn direction(&self) -> f64;

    /// Returns the temporal azimuth of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `TFloatSequenceSet` indicating the temporal azimuth of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_azimuth
    fn azimuth(&self) -> TFloatSequenceSet;

    /// Returns the angular difference of the temporal point.
    ///
    /// ## Returns
    ///
    /// A `TFloatSequenceSet` indicating the temporal angular difference of the temporal point.
    ///
    /// ## MEOS Functions
    ///
    /// tpoint_angular_difference
    fn angular_difference(&self) -> TFloatSequenceSet;

    #[cfg(feature = "geos")]
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
    /// tpoint_twcentroid
    fn time_weighted_centroid(&self, precision: i32) -> Geometry;

    // ------------------------- Spatial Reference System ----------------------

    /// Returns the SRID.
    ///
    /// MEOS Functions:
    ///     tpoint_srid
    fn srid(&self) -> i32 {
        // Function implementation
    }

    /// Returns a new TPoint with the given SRID.
    ///
    /// MEOS Functions:
    ///     tpoint_set_srid
    fn set_srid(&self, srid: i32) -> Self {
        // Function implementation
    }

    // ------------------------- Transformations -------------------------------
    /// Round the coordinate values to a number of decimal places.
    ///
    /// Returns:
    ///     A new `TGeomPoint` object.
    ///
    /// MEOS Functions:
    ///     tpoint_round
    fn round(&self, max_decimals: u32) -> TPoint {
        // Function implementation
    }

    /// Split the temporal point into a collection of simple temporal points.
    ///
    /// Returns:
    ///     A `Vec<TPoint>`.
    ///
    /// MEOS Functions:
    ///     tpoint_make_simple
    fn make_simple(&self) -> Vec<TPoint> {
        // Function implementation
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
    ///     tpoint_expand_space
    fn expand(&self, other: f64) -> STBox {
        // Function implementation
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
    ///     tpoint_transform
    fn transform(&self, srid: i32) -> Self {
        // Function implementation
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
    ///     tpoint_at_value, tpoint_at_stbox, temporal_at_values,
    ///     temporal_at_timestamp, temporal_at_tstzset, temporal_at_tstzspan, temporal_at_tstzspanset
    fn at(&self, other: impl Into<RestrictionType>) -> TPoint {
        // Function implementation
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
    ///     tpoint_minus_value, tpoint_minus_stbox, temporal_minus_values,
    ///     temporal_minus_timestamp, temporal_minus_tstzset, temporal_minus_tstzspan, temporal_minus_tstzspanset
    fn minus(&self, other: impl Into<RestrictionType>) -> TPoint {
        // Function implementation
    }

    // ------------------------- Position Operations ---------------------------
    /// Returns whether the bounding box of `self` is left to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if left, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_before`
    fn is_left(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
    }

    /// Returns whether the bounding box of `self` is over or left to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if over or left, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_over_or_before`
    fn is_over_or_left(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
    }

    /// Returns whether the bounding box of `self` is right to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if right, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_after`
    fn is_right(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
    }

    /// Returns whether the bounding box of `self` is over or right to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if over or right, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_over_or_before`
    fn is_over_or_right(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
    }

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
    fn is_below(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
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
    fn is_over_or_below(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
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
    fn is_above(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
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
    fn is_over_or_above(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
    }

    /// Returns whether the bounding box of `self` is front to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if front, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_before`
    fn is_front(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
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
    fn is_over_or_front(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
    }

    /// Returns whether the bounding box of `self` is behind to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if behind, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_after`
    fn is_behind(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
    }

    /// Returns whether the bounding box of `self` is over or behind to the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if over or behind, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_over_or_before`
    fn is_over_or_behind(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
    }

    /// Returns whether the bounding box of `self` is the same as the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if the same, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_over_or_before`
    fn is_same(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
    }

    /// Returns whether the bounding box of `self` is over the bounding box of `other`.
    ///
    /// Args:
    ///     other: A box or a temporal object to compare to `self`.
    ///
    /// Returns:
    ///     True if over, False otherwise.
    ///
    /// See Also:
    ///     `TsTzSpan::is_over_or_before`
    fn is_over(&self, other: impl Into<BoxType>) -> bool {
        // Function implementation
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
    /// * `tcontains_geo_tpoint`
    pub fn is_spatially_contained_in(&self, container: &impl Geometry) -> TBool {
        let gs = geo_to_gserialized(container, self.is_geog_point());
        let result = tcontains_geo_tpoint(gs, self.inner(), false, false);
        TBool::new(result)
    }

    /// Returns a new temporal boolean indicating whether the temporal point intersects `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - An object to check for intersection with.
    ///
    /// # Returns
    ///
    /// A new `TBool` indicating whether the temporal point intersects `other`.
    ///
    /// # MEOS Functions
    ///
    /// * `tintersects_tpoint_geo`
    pub fn disjoint(&self, other: &impl Geometry) -> TBool {
        let gs = geo_to_gserialized(other, self.is_geog_point());
        let result = tdisjoint_tpoint_geo(self.inner(), gs, false, false);
        TBool::new(result)
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
    /// * `tdwithin_tpoint_geo`, `tdwithin_tpoint_tpoint`
    pub fn within_distance(&self, other: &impl Geometry, distance: f64) -> TBool {
        let gs = geo_to_gserialized(other, self.is_geog_point());
        let result = tdwithin_tpoint_geo(self.inner(), gs, distance, false, false);
        TBool::new(result)
    }

    /// Returns a new temporal boolean indicating whether the temporal point intersects `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - An object to check for intersection with.
    ///
    /// # Returns
    ///
    /// A new `TBool` indicating whether the temporal point intersects `other`.
    ///
    /// # MEOS Functions
    ///
    /// * `tintersects_tpoint_geo`
    pub fn intersects(&self, other: &impl Geometry) -> TBool {
        let gs = geo_to_gserialized(other, self.is_geog_point());
        let result = tintersects_tpoint_geo(self.inner(), gs, false, false);
        TBool::new(result)
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
    /// * `ttouches_tpoint_geo`
    pub fn touches(&self, other: &impl Geometry) -> TBool {
        let gs = geo_to_gserialized(other, self.is_geog_point());
        let result = ttouches_tpoint_geo(self.inner(), gs, false, false);
        TBool::new(result)
    }
}

impl TPoint {
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
    /// * `distance_tpoint_point`, `distance_tpoint_tpoint`
    pub fn distance(&self, other: &impl Geometry) -> TFloat {
        let gs = geo_to_gserialized(other, self.is_geog_point());
        let result = distance_tpoint_point(self.inner(), gs);
        TFloat::new(result)
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
    /// * `nad_tpoint_geo`, `nad_tpoint_stbox`, `nad_tpoint_tpoint`
    pub fn nearest_approach_distance(&self, other: &impl Geometry) -> f64 {
        let gs = geo_to_gserialized(other, self.is_geog_point());
        nad_tpoint_geo(self.inner(), gs)
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
    /// * `nai_tpoint_geo`, `nai_tpoint_tpoint`
    pub fn nearest_approach_instant(&self, other: &impl Geometry) -> TI {
        let gs = geo_to_gserialized(other, self.is_geog_point());
        let result = nai_tpoint_geo(self.inner(), gs);
        TI::new(result)
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
    /// * `shortestline_tpoint_geo`, `shortestline_tpoint_tpoint`
    pub fn shortest_line(&self, other: &impl Geometry) -> BaseGeometry {
        let gs = geo_to_gserialized(other, self.is_geog_point());
        let result = shortestline_tpoint_geo(self.inner(), gs);
        gserialized_to_shapely_geometry(result, 10)
    }
}

impl TPoint {
    /// Split the temporal point into segments following the tiling of the bounding box.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the spatial tiles. If `self` has a spatial dimension and this argument is not provided, the tiling will be only temporal.
    /// * `duration` - The duration of the temporal tiles. If `self` has a time dimension and this argument is not provided, the tiling will be only spatial.
    /// * `origin` - The origin of the spatial tiling. If not provided, the origin will be (0, 0, 0).
    /// * `start` - The start time of the temporal tiling. If not provided, the start time used by default is Monday, January 3, 2000.
    /// * `remove_empty` - If True, remove the tiles that are empty.
    ///
    /// # Returns
    ///
    /// A list of `TPoint` objects.
    ///
    /// # See Also
    ///
    /// `STBox::tile`
    pub fn tile(
        &self,
        size: f64,
        duration: Option<&str>,
        origin: Option<&impl Geometry>,
        start: Option<&str>,
        remove_empty: bool,
    ) -> Vec<TPoint> {
        let bbox = STBox::from_tpoint(self);
        let tiles = bbox.tile(size, duration, origin, start);
        if remove_empty {
            tiles.iter().filter_map(|tile| self.at(tile)).collect()
        } else {
            tiles.iter().map(|tile| self.at(tile)).collect()
        }
    }

    /// Splits `self` into fragments with respect to space buckets.
    ///
    /// # Arguments
    ///
    /// * `xsize` - Size of the x dimension.
    /// * `ysize` - Size of the y dimension.
    /// * `zsize` - Size of the z dimension.
    /// * `origin` - The origin of the spatial tiling. If not provided, the origin will be (0, 0, 0).
    /// * `bitmatrix` - If True, use a bitmatrix to speed up the process.
    /// * `include_border` - If True, include the upper border in the box.
    ///
    /// # Returns
    ///
    /// A list of temporal points.
    ///
    /// # MEOS Functions
    ///
    /// * `tpoint_value_split`
    pub fn space_split(
        &self,
        xsize: f64,
        ysize: Option<f64>,
        zsize: Option<f64>,
        origin: Option<&impl Geometry>,
        bitmatrix: bool,
        include_border: bool,
    ) -> Vec<Temporal> {
        let ysz = ysize.unwrap_or(xsize);
        let zsz = zsize.unwrap_or(xsize);
        let gs = match origin {
            Some(geo) => geo_to_gserialized(geo, self.is_geog_point()),
            None => {
                if self.is_geog_point() {
                    pgis_geography_in("Point(0 0 0)", -1)
                } else {
                    pgis_geometry_in("Point(0 0 0)", -1)
                }
            }
        };
        let (fragments, values, count) =
            tpoint_space_split(self.inner(), xsize, ysz, zsz, gs, bitmatrix, include_border);
        (0..count).map(|i| Temporal::new(fragments[i])).collect()
    }

    /// Splits `self` into fragments with respect to space and tstzspan buckets.
    ///
    /// # Arguments
    ///
    /// * `xsize` - Size of the x dimension.
    /// * `duration` - Duration of the tstzspan buckets.
    /// * `ysize` - Size of the y dimension.
    /// * `zsize` - Size of the z dimension.
    /// * `origin` - The origin of the spatial tiling. If not provided, the origin will be (0, 0, 0).
    /// * `time_start` - Start time of the first tstzspan bucket. If None, the start time used by default is Monday, January 3, 2000.
    /// * `bitmatrix` - If True, use a bitmatrix to speed up the process.
    /// * `include_border` - If True, include the upper border in the box.
    ///
    /// # Returns
    ///
    /// A list of temporal floats.
    ///
    /// # MEOS Functions
    ///
    /// * `tfloat_value_time_split`
    pub fn space_time_split(
        &self,
        xsize: f64,
        duration: &str,
        ysize: Option<f64>,
        zsize: Option<f64>,
        origin: Option<&impl Geometry>,
        time_start: Option<&str>,
        bitmatrix: bool,
        include_border: bool,
    ) -> Vec<Temporal> {
        let ysz = ysize.unwrap_or(xsize);
        let zsz = zsize.unwrap_or(xsize);
        let dt = pg_interval_in(duration, -1);
        let gs = match origin {
            Some(geo) => geo_to_gserialized(geo, self.is_geog_point()),
            None => {
                if self.is_geog_point() {
                    pgis_geography_in("Point(0 0 0)", -1)
                } else {
                    pgis_geometry_in("Point(0 0 0)", -1)
                }
            }
        };
        let st = match time_start {
            Some(start) => pg_timestamptz_in(start, -1),
            None => pg_timestamptz_in("2000-01-03", -1),
        };
        let (fragments, points, times, count) = tpoint_space_time_split(
            self.inner(),
            xsize,
            ysz,
            zsz,
            dt,
            gs,
            st,
            bitmatrix,
            include_border,
        );
        (0..count).map(|i| Temporal::new(fragments[i])).collect()
    }
}
