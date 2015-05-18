use superlu_sys::SuperMatrix;

use power_grid::*;
use system_matrix::*;
use thermal_grid::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ThermalData_t {
    pub Temperatures: *mut Temperature_t,
    pub ThermalGrid: ThermalGrid_t,
    pub PowerGrid: PowerGrid_t,
    pub Size: CellIndex_t,
    pub SM_A: SystemMatrix_t,
    pub SLUMatrix_B: SuperMatrix,
}

extern "C" {
    pub fn thermal_data_init(tdata: *mut ThermalData_t);
    pub fn thermal_data_destroy(tdata: *mut ThermalData_t);
}
