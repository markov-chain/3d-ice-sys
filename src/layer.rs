use material::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Layer_t {
    pub Height: CellDimension_t,
    pub Material: Material_t,
    pub Id: String_t,
}
