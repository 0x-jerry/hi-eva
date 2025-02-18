#![allow(non_upper_case_globals)]
use core::slice;

use core_foundation::base::{CFIndex, TCFType};
use core_foundation::{declare_TCFType, impl_CFTypeDescription, impl_TCFType};

use accessibility_sys::{
    AXTextMarkerGetBytePtr, AXTextMarkerGetLength, AXTextMarkerGetTypeID, AXTextMarkerRef,
};

declare_TCFType!(AXTextMarker, AXTextMarkerRef);
impl_TCFType!(AXTextMarker, AXTextMarkerRef, AXTextMarkerGetTypeID);
impl_CFTypeDescription!(AXTextMarker);

impl AXTextMarker {
    pub fn new(ptr: AXTextMarkerRef) -> AXTextMarker {
        AXTextMarker(ptr)
    }

    pub fn length(&self) -> CFIndex {
        unsafe { AXTextMarkerGetLength(self.0) }
    }

    pub fn bytes(&self) -> &[u8] {
        unsafe {
            let ptr = AXTextMarkerGetBytePtr(self.0);

            if ptr.is_null() {
                return &[];
            }

            slice::from_raw_parts(ptr, self.length() as usize)
        }
    }
}
