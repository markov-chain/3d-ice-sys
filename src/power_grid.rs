use channel::*;
use floorplan::*;
use heat_sink::*;
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
