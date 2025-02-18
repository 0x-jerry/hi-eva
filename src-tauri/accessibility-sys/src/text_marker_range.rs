#![allow(non_upper_case_globals)]

use core_foundation_sys::base::{CFAllocatorRef, CFIndex, CFTypeID, UInt8};

use crate::text_marker::AXTextMarkerRef;

pub enum __AXTextMarkerRange {}
pub type AXTextMarkerRangeRef = *mut __AXTextMarkerRange;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    pub fn AXTextMarkerRangeGetTypeID() -> CFTypeID;
    pub fn AXTextMarkerRangeCreate(
        allocator: CFAllocatorRef,
        startMarker: AXTextMarkerRef,
        endMarker: AXTextMarkerRef,
    ) -> AXTextMarkerRangeRef;
    pub fn AXTextMarkerRangeCreateWithBytes(
        allocator: CFAllocatorRef,
        startMarkerBytes: *const UInt8,
        startMarkerLength: CFIndex,
        endMarkerBytes: *const UInt8,
    ) -> AXTextMarkerRangeRef;
    pub fn AXTextMarkerRangeCopyStartMarker(value: AXTextMarkerRangeRef) -> AXTextMarkerRef;
    pub fn AXTextMarkerRangeCopyEndMarker(value: AXTextMarkerRangeRef) -> AXTextMarkerRef;
}
