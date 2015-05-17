use inspection_point_list::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Output_t {
    pub InspectionPointListFinal: InspectionPointList_t,
    pub InspectionPointListSlot: InspectionPointList_t,
    pub InspectionPointListStep: InspectionPointList_t,
}

extern "C" {
    pub fn output_init(output: *mut Output_t);
    pub fn output_destroy(output: *mut Output_t);
}
