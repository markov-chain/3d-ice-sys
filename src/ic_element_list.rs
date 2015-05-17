use ic_element::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ICElementListNode_t {
    pub Data: ICElement_t,
    pub Prev: *mut ICElementListNode_t,
    pub Next: *mut ICElementListNode_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ICElementList_t {
    pub Size: Quantity_t,
    pub First: *mut ICElementListNode_t,
    pub Last: *mut ICElementListNode_t,
}
