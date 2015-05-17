use inspection_point::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InspectionPointListNode_t {
    pub Data: InspectionPoint_t,
    pub Prev: *mut InspectionPointListNode_t,
    pub Next: *mut InspectionPointListNode_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InspectionPointList_t {
    pub Size: Quantity_t,
    pub First: *mut InspectionPointListNode_t,
    pub Last: *mut InspectionPointListNode_t,
}
