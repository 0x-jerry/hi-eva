#![allow(non_upper_case_globals)]
use core_foundation::base::TCFType;
use core_foundation::{declare_TCFType, impl_CFTypeDescription, impl_TCFType};

use accessibility_sys::{
    AXTextMarkerRangeCopyEndMarker, AXTextMarkerRangeCopyStartMarker, AXTextMarkerRangeGetTypeID,
    AXTextMarkerRangeRef,
};

use crate::text_marker::AXTextMarker;

declare_TCFType!(AXTextMarkerRange, AXTextMarkerRangeRef);
impl_TCFType!(
    AXTextMarkerRange,
    AXTextMarkerRangeRef,
    AXTextMarkerRangeGetTypeID
);
impl_CFTypeDescription!(AXTextMarkerRange);

impl AXTextMarkerRange {
    pub fn start_marker(&self) -> AXTextMarker {
        unsafe {
            let start_ref = AXTextMarkerRangeCopyStartMarker(self.0);
            AXTextMarker::new(start_ref)
        }
    }

    pub fn end_marker(&self) -> AXTextMarker {
        unsafe {
            let start_ref = AXTextMarkerRangeCopyEndMarker(self.0);
            AXTextMarker::new(start_ref)
        }
    }

    pub fn is_the_same_marker(&self) -> bool {
        self.start_marker().bytes() == self.end_marker().bytes()
    }
}
