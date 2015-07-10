use libc::c_void;
use std::mem;

use channel::*;
use die::*;
use heat_sink::*;
use layer::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StackElement_p(*mut c_void);

impl StackElement_p {
    #[inline]
    pub unsafe fn Layer(&self) -> *mut Layer_t {
        mem::transmute(self.0)
    }

    #[inline]
    pub unsafe fn Die(&self) -> *mut Die_t {
        mem::transmute(self.0)
    }

    #[inline]
    pub unsafe fn Channel(&self) -> *mut Channel_t {
        mem::transmute(self.0)
    }

    #[inline]
    pub unsafe fn HeatSink(&self) -> *mut HeatSink_t {
        mem::transmute(self.0)
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
