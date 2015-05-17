use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Material_t {
    pub Id: String_t,
    pub VolumetricHeatCapacity: SolidVHC_t,
    pub ThermalConductivity: SolidTC_t,
}
