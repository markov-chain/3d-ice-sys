use libc::{c_char, c_double, uint32_t};

pub type String_t = *mut c_char;

pub type Quantity_t = uint32_t;

pub type Time_t = c_double;

pub type Temperature_t = c_double;

pub type Source_t = c_double;

pub type Power_t = c_double;

pub type AmbientHTC_t = c_double;

pub type SolidVHC_t = c_double;

pub type SolidTC_t = c_double;

pub type SystemMatrixCoeff_t = c_double;

pub type CoolantHTC_t = c_double;

pub type CoolantVHC_t = c_double;

pub type CoolantFR_t = c_double;

pub type DarcyVelocity_t = c_double;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Coolant_t {
    pub HTCSide: CoolantHTC_t,
    pub HTCTop: CoolantHTC_t,
    pub HTCBottom: CoolantHTC_t,
    pub VHC: CoolantVHC_t,
    pub FlowRate: CoolantFR_t,
    pub DarcyVelocity: DarcyVelocity_t,
    pub TIn: Temperature_t,
}

pub type CellDimension_t = c_double;

pub type ChipDimension_t = c_double;

pub type ChannelDimension_t = c_double;

pub type CellIndex_t = uint32_t;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum HeatSinkModel_t {
    TDICE_HEATSINK_MODEL_NONE = 0,
    TDICE_HEATSINK_MODEL_CONNECTION_TO_AMBIENT,
    TDICE_HEATSINK_MODEL_TRADITIONAL,
}
pub use self::HeatSinkModel_t::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum StackLayerType_t {
    TDICE_LAYER_NONE = 0,
    TDICE_LAYER_SOLID,
    TDICE_LAYER_SOURCE,
    TDICE_LAYER_SOLID_CONNECTED_TO_AMBIENT,
    TDICE_LAYER_SOURCE_CONNECTED_TO_AMBIENT,
    TDICE_LAYER_SPREADER,
    TDICE_LAYER_SINK,
    TDICE_LAYER_CHANNEL_4RM,
    TDICE_LAYER_CHANNEL_2RM,
    TDICE_LAYER_PINFINS_INLINE,
    TDICE_LAYER_PINFINS_STAGGERED,
    TDICE_LAYER_VWALL_CHANNEL,
    TDICE_LAYER_VWALL_PINFINS,
    TDICE_LAYER_TOP_WALL,
    TDICE_LAYER_BOTTOM_WALL,
}
pub use self::StackLayerType_t::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum StackElementType_t {
    TDICE_STACK_ELEMENT_NONE = 0,
    TDICE_STACK_ELEMENT_LAYER,
    TDICE_STACK_ELEMENT_CHANNEL,
    TDICE_STACK_ELEMENT_DIE,
    TDICE_STACK_ELEMENT_HEATSINK,
}
pub use self::StackElementType_t::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum ChannelModel_t {
    TDICE_CHANNEL_MODEL_NONE = 0,
    TDICE_CHANNEL_MODEL_MC_4RM,
    TDICE_CHANNEL_MODEL_MC_2RM,
    TDICE_CHANNEL_MODEL_PF_INLINE,
    TDICE_CHANNEL_MODEL_PF_STAGGERED,
}
pub use self::ChannelModel_t::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum Error_t {
    TDICE_SUCCESS = 0,
    TDICE_FAILURE,
}
pub use self::Error_t::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum AnalysisType_t {
    TDICE_ANALYSIS_TYPE_NONE = 0,
    TDICE_ANALYSIS_TYPE_TRANSIENT,
    TDICE_ANALYSIS_TYPE_STEADY,
}
pub use self::AnalysisType_t::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum OutputQuantity_t {
    TDICE_OUTPUT_QUANTITY_NONE = 0,
    TDICE_OUTPUT_QUANTITY_AVERAGE,
    TDICE_OUTPUT_QUANTITY_MAXIMUM,
    TDICE_OUTPUT_QUANTITY_MINIMUM,
}
pub use self::OutputQuantity_t::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum OutputType_t {
    TDICE_OUTPUT_TYPE_NONE = 0,
    TDICE_OUTPUT_TYPE_TCELL,
    TDICE_OUTPUT_TYPE_TFLP,
    TDICE_OUTPUT_TYPE_TFLPEL,
    TDICE_OUTPUT_TYPE_TMAP,
    TDICE_OUTPUT_TYPE_PMAP,
    TDICE_OUTPUT_TYPE_TCOOLANT,
}
pub use self::OutputType_t::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum OutputInstant_t {
    TDICE_OUTPUT_INSTANT_NONE = 0,
    TDICE_OUTPUT_INSTANT_FINAL,
    TDICE_OUTPUT_INSTANT_SLOT,
    TDICE_OUTPUT_INSTANT_STEP,
}
pub use self::OutputInstant_t::*;
