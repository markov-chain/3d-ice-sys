use channel::*;
use die_list::*;
use dimensions::*;
use heat_sink::*;
use layer_list::*;
use material_list::*;
use stack_element_list::*;
use types::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StackDescription_t {
    pub FileName: String_t,
    pub Materials: MaterialList_t,
    pub HeatSink: *mut HeatSink_t,
    pub Channel: *mut Channel_t,
    pub Layers: LayerList_t,
    pub Dies: DieList_t,
    pub Dimensions: *mut Dimensions_t,
    pub StackElements: StackElementList_t,
}

extern "C" {
    pub fn stack_description_init(stkd: *mut StackDescription_t);
    pub fn stack_description_destroy(stkd: *mut StackDescription_t);
}
