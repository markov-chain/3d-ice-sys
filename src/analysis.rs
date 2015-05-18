use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Analysis_t {
    pub AnalysisType: AnalysisType_t,
    pub StepTime: Time_t,
    pub SlotTime: Time_t,
    pub SlotLength: Quantity_t,
    pub CurrentTime: Quantity_t,
    pub InitialTemperature: Temperature_t,
}

extern "C" {
    pub fn analysis_init(analysis: *mut Analysis_t);
    pub fn get_simulated_time(analysis: *mut Analysis_t) -> Time_t;
}
