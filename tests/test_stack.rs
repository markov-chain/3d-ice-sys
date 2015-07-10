use ffi::*;

macro_rules! c_str_to_str(
    ($string:expr) => (
        ::std::str::from_utf8(::std::ffi::CStr::from_ptr($string).to_bytes()).unwrap()
    );
);

#[test]
fn test_stack() { ::support::setup(Some("hotspot"), move |stkd, _, _| unsafe {
    assert_eq!(stkd.StackElements.Size, 2);

    let element = &(&*stkd.StackElements.First).Data;
    assert!(element.Type == TDICE_STACK_ELEMENT_HEATSINK);

    let element = &(&*stkd.StackElements.Last).Data;
    assert!(element.Type == TDICE_STACK_ELEMENT_DIE);

    let die = &*element.Pointer.Die();
    let floorplan = die.Floorplan;
    assert_eq!(floorplan.NElements, 4);
    assert_eq!(floorplan.ElementsList.Size, 4);

    let element = &(&*floorplan.ElementsList.First).Data;
    assert_eq!(c_str_to_str!(element.Id), "Core0");
})}
