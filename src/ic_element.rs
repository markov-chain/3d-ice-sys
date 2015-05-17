use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ICElement_t {
    pub SW_X: ChipDimension_t,
    pub SW_Y: ChipDimension_t,
    pub Length: ChipDimension_t,
    pub Width: ChipDimension_t,
    pub SW_Row: CellIndex_t,
    pub SW_Column: CellIndex_t,
    pub NE_Row: CellIndex_t,
    pub NE_Column: CellIndex_t,
}
