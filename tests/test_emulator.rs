use std::mem;
use threed_ice_sys::*;

use support::{setup_simulator, setup_ping};

// https://github.com/copies/3d-ice/blob/master/bin/3D-ICE-Emulator.c
#[test]
fn test_emulator() { setup_simulator(move |stkd, analysis, output| setup_ping(move || unsafe {
    let mut tdata: ThermalData_t = mem::uninitialized();

    thermal_data_init(&mut tdata);

    success!(thermal_data_build(&mut tdata, &mut stkd.StackElements,
                                stkd.Dimensions, analysis));

    success!(generate_output_headers(output, stkd.Dimensions,
                                     str_to_c_str!("% ").as_ptr() as String_t));

    loop {
        match emulate_step(&mut tdata, stkd.Dimensions, analysis) {
            SimResult_t::TDICE_STEP_DONE => {
                generate_output(output, stkd.Dimensions, tdata.Temperatures,
                                tdata.PowerGrid.Sources, get_simulated_time(analysis),
                                TDICE_OUTPUT_INSTANT_STEP);
            },
            SimResult_t::TDICE_SLOT_DONE => {
                generate_output(output, stkd.Dimensions, tdata.Temperatures,
                                tdata.PowerGrid.Sources, get_simulated_time(analysis),
                                TDICE_OUTPUT_INSTANT_STEP);
                generate_output(output, stkd.Dimensions, tdata.Temperatures,
                                tdata.PowerGrid.Sources, get_simulated_time(analysis),
                                TDICE_OUTPUT_INSTANT_SLOT);
            },
            SimResult_t::TDICE_END_OF_SIMULATION => break,
            _ => assert!(false),
        }
    }

    generate_output(output, stkd.Dimensions, tdata.Temperatures, tdata.PowerGrid.Sources,
                    get_simulated_time(analysis), TDICE_OUTPUT_INSTANT_FINAL);

    thermal_data_destroy(&mut tdata);
}))}
