use dimensions::*;
use inspection_point_list::*;
use types::*;

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

    pub fn generate_output_headers(output: *mut Output_t,
                                   dimensions: *mut Dimensions_t,
                                   prefix: String_t) -> Error_t;

    pub fn generate_output(output: *mut Output_t,
                           dimensions: *mut Dimensions_t,
                           temperatures: *mut Temperature_t,
                           sources: *mut Source_t,
                           current_time: Time_t,
                           output_instant: OutputInstant_t) -> Error_t;
}
