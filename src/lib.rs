#![allow(refining_impl_trait)]
use std::{ffi::CString, sync::Once};

use bitmask_enum::bitmask;
use boxes::r#box::Box as MeosBox;
use collections::base::collection::Collection;
pub use meos_sys;

pub mod boxes;
pub mod collections;
pub mod errors;
pub mod temporal;
pub(crate) mod utils;

static START: Once = Once::new();

extern "C" fn finalize() {
    unsafe {
        meos_sys::meos_finalize();
    }
}

pub trait BoundingBox: Collection {}

impl<T> BoundingBox for T where T: MeosBox {}

pub fn init() {
    START.call_once(|| unsafe {
        let ptr = CString::new("UTC").unwrap();
        meos_sys::meos_initialize(ptr.as_ptr(), None);
        libc::atexit(finalize);
    });
}

#[bitmask(u8)]
pub enum WKBVariant {
    /// Little endian encoding
    NDR = meos_sys::WKB_NDR as u8,
    /// Big endian encoding
    XDR = meos_sys::WKB_XDR as u8,
    /// Extended variant
    Extended = meos_sys::WKB_EXTENDED as u8,
}
