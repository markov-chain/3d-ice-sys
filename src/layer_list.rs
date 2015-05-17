use layer::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LayerListNode_t {
    pub Data: Layer_t,
    pub Prev: *mut LayerListNode_t,
    pub Next: *mut LayerListNode_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LayerList_t {
    pub Size: Quantity_t,
    pub First: *mut LayerListNode_t,
    pub Last: *mut LayerListNode_t,
}
