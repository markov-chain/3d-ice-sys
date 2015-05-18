use superlu_sys::SuperMatrix;

use analysis::*;
use dimensions::*;
use power_grid::*;
use stack_element_list::*;
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

    pub fn thermal_data_build(tdata: *mut ThermalData_t,
                              list: *mut StackElementList_t,
                              dimensions: *mut Dimensions_t,
                              analysis: *mut Analysis_t) -> Error_t;

    pub fn thermal_data_destroy(tdata: *mut ThermalData_t);

    pub fn emulate_step(tdata: *mut ThermalData_t,
                        dimensions: *mut Dimensions_t,
                        analysis: *mut Analysis_t) -> SimResult_t;
}
