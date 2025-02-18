#![allow(non_upper_case_globals)]

use core_foundation_sys::base::{CFAllocatorRef, CFIndex, CFTypeID, UInt8};

pub enum __AXTextMarker {}
pub type AXTextMarkerRef = *mut __AXTextMarker;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    pub fn AXTextMarkerGetTypeID() -> CFTypeID;
    pub fn AXTextMarkerCreate(
        allocator: CFAllocatorRef,
        bytes: *const UInt8,
        index: CFIndex,
    ) -> AXTextMarkerRef;
    pub fn AXTextMarkerGetLength(value: AXTextMarkerRef) -> CFIndex;
    pub fn AXTextMarkerGetBytePtr(value: AXTextMarkerRef) -> *const UInt8;
}
