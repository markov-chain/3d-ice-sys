use floorplan_element::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FloorplanElementListNode_t {
    pub Data: FloorplanElement_t,
    pub Prev: *mut FloorplanElementListNode_t,
    pub Next: *mut FloorplanElementListNode_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FloorplanElementList_t {
    pub Size: Quantity_t,
    pub First: *mut FloorplanElementListNode_t,
    pub Last: *mut FloorplanElementListNode_t,
}
