use material::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MaterialListNode_t {
    pub Data: Material_t,
    pub Prev: *mut MaterialListNode_t,
    pub Next: *mut MaterialListNode_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MaterialList_t {
    pub Size: Quantity_t,
    pub First: *mut MaterialListNode_t,
    pub Last: *mut MaterialListNode_t,
}
