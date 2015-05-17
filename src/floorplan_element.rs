use ic_element_list::*;
use powers_queue::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FloorplanElement_t {
    pub Id: String_t,
    pub NICElements: Quantity_t,
    pub ICElements: ICElementList_t,
    pub Area: ChipDimension_t,
    pub PowerValues: *mut PowersQueue_t,
}
