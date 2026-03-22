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
    create_set_of_geometries, geometry_to_gserialized, gserialized_to_geometry, impl_tgeo_type,
    impl_tpoint_traits, point_to_gserialized_geog, point_to_gserialized_geom, Point, TGeoTrait,
};

impl_tgeo_type!(
    TGeography,
    true,
    tgeoinst_make,
    tgeography_in,
    tgeography_from_mfjson
);
