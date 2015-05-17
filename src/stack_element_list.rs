use stack_element::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StackElementListNode_t {
    pub Data: StackElement_t,
    pub Prev: *mut StackElementListNode_t,
    pub Next: *mut StackElementListNode_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StackElementList_t {
    pub Size: Quantity_t,
    pub First: *mut StackElementListNode_t,
    pub Last: *mut StackElementListNode_t,
}
