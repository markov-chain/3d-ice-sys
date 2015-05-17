use superlu_sys::SuperMatrix;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FloorplanMatrix_t {
    pub ColumnPointers: *mut CellIndex_t,
    pub RowIndices: *mut CellIndex_t,
    pub Values: *mut Source_t,
    pub NRows: CellIndex_t,
    pub NColumns: CellIndex_t,
    pub NNz: CellIndex_t,
    pub SLUMatrix: SuperMatrix,
}
