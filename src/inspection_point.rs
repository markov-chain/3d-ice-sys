use floorplan_element::*;
use stack_element::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InspectionPoint_t {
    pub FileName: String_t,
    pub Instant: OutputInstant_t,
    pub Type: OutputType_t,
    pub Quantity: OutputQuantity_t,
    pub Xval: ChipDimension_t,
    pub ActualXval: ChipDimension_t,
    pub Yval: ChipDimension_t,
    pub ActualYval: ChipDimension_t,
    pub RowIndex: CellIndex_t,
    pub ColumnIndex: CellIndex_t,
    pub StackElement: *mut StackElement_t,
    pub FloorplanElement: *mut FloorplanElement_t,
}
