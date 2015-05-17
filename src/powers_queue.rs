use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PowersQueue_t {
    pub Capacity: Quantity_t,
    pub Memory: *mut Power_t,
    pub Size: Quantity_t,
    pub End: Quantity_t,
    pub Start: Quantity_t,
}
