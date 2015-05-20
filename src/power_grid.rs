use channel::*;
use dimensions::*;
use floorplan::*;
use heat_sink::*;
use stack_element_list::*;
use thermal_grid::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PowerGrid_t {
    pub NLayers: CellIndex_t,
    pub NCells: CellIndex_t,
    pub LayersProfile: *mut StackLayerType_t,
    pub FloorplansProfile: *mut *mut Floorplan_t,
    pub Sources: *mut Source_t,
    pub Channel: *mut Channel_t,
    pub HeatSink: *mut HeatSink_t,
}

extern "C" {
    pub fn power_grid_init(pgrid: *mut PowerGrid_t);

    pub fn power_grid_build(pgrid: *mut PowerGrid_t, nlayers: Quantity_t,
                            ncells: Quantity_t) -> Error_t;

    pub fn power_grid_destroy(pgrid: *mut PowerGrid_t);
    pub fn fill_power_grid(pgrid: *mut PowerGrid_t, list: *mut StackElementList_t);

    pub fn update_source_vector(pgrid: *mut PowerGrid_t, thermal_grid: *mut ThermalGrid_t,
                                dimensions: *mut Dimensions_t) -> Error_t;
}
