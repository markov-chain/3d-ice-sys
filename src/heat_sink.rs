use material::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HeatSink_t {
    pub SinkModel: HeatSinkModel_t,
    pub NLayers: CellIndex_t,
    pub SourceLayerOffset: CellIndex_t,
    pub AmbientHTC: AmbientHTC_t,
    pub AmbientTemperature: Temperature_t,
    pub SinkHeight: CellDimension_t,
    pub SinkArea: CellDimension_t,
    pub SpreaderHeight: CellDimension_t,
    pub SpreaderArea: CellDimension_t,
    pub SinkMaterial: Material_t,
    pub SpreaderMaterial: Material_t,
}
