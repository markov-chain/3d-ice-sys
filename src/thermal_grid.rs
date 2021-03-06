use channel::*;
use dimensions::*;
use heat_sink::*;
use stack_element_list::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ThermalGrid_t {
    pub Size: CellIndex_t,
    pub LayersProfile: *mut StackLayerType_t,
    pub VHCProfile: *mut SolidVHC_t,
    pub TCProfile: *mut SolidTC_t,
    pub Channel: *mut Channel_t,
    pub HeatSink: *mut HeatSink_t,
}

extern "C" {
    pub fn thermal_grid_init(tgrid: *mut ThermalGrid_t);
    pub fn thermal_grid_build(tgrid: *mut ThermalGrid_t, size: Quantity_t) -> Error_t;
    pub fn thermal_grid_destroy(tgrid: *mut ThermalGrid_t);

    pub fn fill_thermal_grid(tgrid: *mut ThermalGrid_t, list: *mut StackElementList_t,
                             dimensions: *mut Dimensions_t);

    pub fn get_capacity(tgrid: *mut ThermalGrid_t, dimensions: *mut Dimensions_t,
                        layer_index: CellIndex_t, row_index: CellIndex_t,
                        column_index: CellIndex_t) -> Capacity_t;
}
