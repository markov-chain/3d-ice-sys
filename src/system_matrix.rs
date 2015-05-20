use libc::c_int;
use superlu_sys::{SuperLUStat_t, SuperMatrix, superlu_options_t};

use analysis::*;
use dimensions::*;
use thermal_grid::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SystemMatrix_t {
    pub ColumnPointers: *mut CellIndex_t,
    pub RowIndices: *mut CellIndex_t,
    pub Values: *mut SystemMatrixCoeff_t,
    pub Size: CellIndex_t,
    pub NNz: CellIndex_t,
    pub SLUMatrix_A: SuperMatrix,
    pub SLUMatrix_A_Permuted: SuperMatrix,
    pub SLUMatrix_L: SuperMatrix,
    pub SLUMatrix_U: SuperMatrix,
    pub SLU_Stat: SuperLUStat_t,
    pub SLU_Options: superlu_options_t,
    pub SLU_Info: c_int,
    pub SLU_PermutationMatrixR: *mut c_int,
    pub SLU_PermutationMatrixC: *mut c_int,
    pub SLU_Etree: *mut c_int,
}

extern "C" {
    pub fn system_matrix_init(sysmatrix: *mut SystemMatrix_t);

    pub fn system_matrix_build(sysmatrix: *mut SystemMatrix_t, size: CellIndex_t,
                               nnz: CellIndex_t) -> Error_t;

    pub fn system_matrix_destroy(sysmatrix: *mut SystemMatrix_t);

    pub fn fill_system_matrix(sysmatrix: *mut SystemMatrix_t, thermal_grid: *mut ThermalGrid_t,
                              analysis: *mut Analysis_t, dimensions: *mut Dimensions_t);
}
