use crate::{
    boxes::STBox,
    collections::base::{impl_collection, Collection},
    errors::ParseError,
    factory,
    temporal::{
        interpolation::TInterpolation,
        tbool::{TBool, TBoolInstant, TBoolSequence, TBoolSequenceSet},
        temporal::{
            impl_always_and_ever_value_equality_functions, impl_simple_traits_for_temporal,
            SimplifiableTemporal, Temporal,
        },
        tinstant::TInstant,
        tsequence::TSequence,
        tsequence_set::TSequenceSet,
    },
    utils::to_meos_timestamp,
    MeosEnum,
};
use chrono::{DateTime, TimeZone};
use geos::Geometry;
use std::{ffi::CString, hash::Hash, mem, ptr, str::FromStr};

use super::tgeo::{
    create_geomset, create_geogset, geometry_to_gserialized, gserialized_to_geometry, impl_tgeo_type,
    impl_tpoint_traits, geo_to_gserialized_geog, geo_to_gserialized_geom, Point, TGeoTrait,
};

impl_tgeo_type!(TGeomPoint, false, tpointinst_make, tgeompoint_in, tgeompoint_from_mfjson);

impl TGeomPointSequence {
    pub fn direction(&self) -> f64 {
        let mut result = 0.;
        unsafe { meos_sys::tpoint_direction(self.inner(), ptr::addr_of_mut!(result)) };
        result
    }
}

impl TGeomPointSequenceSet {
    pub fn direction(&self) -> f64 {
        let mut result = 0.;
        unsafe { meos_sys::tpoint_direction(self.inner(), ptr::addr_of_mut!(result)) };
        result
    }
}
