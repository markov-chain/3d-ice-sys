use floorplan_element_list::*;
use floorplan_matrix::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Floorplan_t {
    pub FileName: String_t,
    pub NElements: Quantity_t,
    pub ElementsList: FloorplanElementList_t,
    pub SurfaceCoefficients: FloorplanMatrix_t,
    pub Bpowers: *mut Power_t,
}

extern "C" {
    pub fn fill_sources_floorplan(floorplan: *mut Floorplan_t, sources: *mut Source_t) -> Error_t;
}
