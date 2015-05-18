use channel::*;
use heat_sink::*;
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
