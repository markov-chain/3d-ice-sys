use std::mem;

use channel::*;
use die::*;
use heat_sink::*;
use layer::*;
use types::*;

#[cfg(target_pointer_width = "32")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct StackElement_p {
    pointer: [u8; 4],
}

#[cfg(target_pointer_width = "64")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct StackElement_p {
    pointer: [u8; 8],
}

impl StackElement_p {
    #[inline]
    pub unsafe fn Layer(&mut self) -> *mut Layer_t {
        mem::transmute(&self.pointer)
    }

    #[inline]
    pub unsafe fn Die(&mut self) -> *mut Die_t {
        mem::transmute(&self.pointer)
    }

    #[inline]
    pub unsafe fn Channel(&mut self) -> *mut Channel_t {
        mem::transmute(&self.pointer)
    }

    #[inline]
    pub unsafe fn HeatSink(&mut self) -> *mut HeatSink_t {
        mem::transmute(&self.pointer)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StackElement_t {
    pub Id: String_t,
    pub Type: StackElementType_t,
    pub Pointer: StackElement_p,
    pub NLayers: CellIndex_t,
    pub Offset: CellIndex_t,
}
