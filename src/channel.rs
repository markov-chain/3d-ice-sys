use material::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Channel_t {
    pub ChannelModel: ChannelModel_t,
    pub Height: CellDimension_t,
    pub Length: ChannelDimension_t,
    pub Pitch: ChannelDimension_t,
    pub Porosity: ChannelDimension_t,
    pub NChannels: Quantity_t,
    pub NLayers: CellIndex_t,
    pub SourceLayerOffset: CellIndex_t,
    pub Coolant: Coolant_t,
    pub WallMaterial: Material_t,
}
