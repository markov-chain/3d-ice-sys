use die::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct DieListNode_t {
    pub Data: Die_t,
    pub Prev: *mut DieListNode_t,
    pub Next: *mut DieListNode_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct DieList_t {
    pub Size: Quantity_t,
    pub First: *mut DieListNode_t,
    pub Last: *mut DieListNode_t,
}
