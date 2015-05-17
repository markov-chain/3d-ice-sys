use inspection_point_list::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Output_t {
    pub InspectionPointListFinal: InspectionPointList_t,
    pub InspectionPointListSlot: InspectionPointList_t,
    pub InspectionPointListStep: InspectionPointList_t,
}
