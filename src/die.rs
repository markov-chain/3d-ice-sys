use floorplan::*;
use layer_list::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Die_t {
    pub Id: String_t,
    pub NLayers: CellIndex_t,
    pub SourceLayerOffset: CellIndex_t,
    pub Layers: LayerList_t,
    pub Floorplan: Floorplan_t,
}
