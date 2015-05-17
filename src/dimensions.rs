use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CellDimensions_t {
    pub FirstWallLength: CellDimension_t,
    pub WallLength: CellDimension_t,
    pub ChannelLength: CellDimension_t,
    pub LastWallLength: CellDimension_t,
    pub Width: CellDimension_t,
    pub NHeights: Quantity_t,
    pub Heights: *mut CellDimension_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GridDimensions_t {
    pub NLayers: CellIndex_t,
    pub NRows: CellIndex_t,
    pub NColumns: CellIndex_t,
    pub NCells: CellIndex_t,
    pub NConnections: CellIndex_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ChipDimensions_t {
    pub Length: ChipDimension_t,
    pub Width: ChipDimension_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Dimensions_t {
    pub Cell: CellDimensions_t,
    pub Grid: GridDimensions_t,
    pub Chip: ChipDimensions_t,
}
