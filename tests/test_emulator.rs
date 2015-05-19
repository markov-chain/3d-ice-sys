use std::mem;
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};

use raw::*;

// https://github.com/copies/3d-ice/blob/master/bin/3D-ICE-Emulator.c
#[test]
fn emulator() { ::setup(|stack| unsafe {
    let (tx, rx) = mpsc::channel();
    let handle = ping(rx);

    let mut stkd: StackDescription_t = mem::uninitialized();
    let mut analysis: Analysis_t = mem::uninitialized();
    let mut output: Output_t = mem::uninitialized();
    let mut tdata: ThermalData_t = mem::uninitialized();

    stack_description_init(&mut stkd);
    analysis_init(&mut analysis);
    output_init(&mut output);

    let mut error = parse_stack_description_file(
        path_to_c_str!(stack).as_ptr() as *mut _,
        &mut stkd, &mut analysis, &mut output,
    );
    assert!(error == TDICE_SUCCESS);

    assert!(analysis.AnalysisType == TDICE_ANALYSIS_TYPE_TRANSIENT);

    error = generate_output_headers(&mut output, stkd.Dimensions,
                                    str_to_c_str!("% ").as_ptr() as String_t);
    assert!(error == TDICE_SUCCESS);

    thermal_data_init(&mut tdata);

    error = thermal_data_build(&mut tdata, &mut stkd.StackElements, stkd.Dimensions,
                               &mut analysis);
    assert!(error == TDICE_SUCCESS);

    loop {
        match emulate_step(&mut tdata, stkd.Dimensions, &mut analysis) {
            SimResult_t::TDICE_STEP_DONE => {
                generate_output(&mut output, stkd.Dimensions, tdata.Temperatures,
                                tdata.PowerGrid.Sources, get_simulated_time(&mut analysis),
                                TDICE_OUTPUT_INSTANT_STEP);
            },
            SimResult_t::TDICE_SLOT_DONE => {
                generate_output(&mut output, stkd.Dimensions, tdata.Temperatures,
                                tdata.PowerGrid.Sources, get_simulated_time(&mut analysis),
                                TDICE_OUTPUT_INSTANT_STEP);
                generate_output(&mut output, stkd.Dimensions, tdata.Temperatures,
                                tdata.PowerGrid.Sources, get_simulated_time(&mut analysis),
                                TDICE_OUTPUT_INSTANT_SLOT);
            },
            SimResult_t::TDICE_END_OF_SIMULATION => break,
            _ => assert!(false),
        }
    }

    generate_output(&mut output, stkd.Dimensions, tdata.Temperatures, tdata.PowerGrid.Sources,
                    get_simulated_time(&mut analysis), TDICE_OUTPUT_INSTANT_FINAL);

    thermal_data_destroy(&mut tdata);
    stack_description_destroy(&mut stkd);
    output_destroy(&mut output);

    ok!(tx.send(true));
    ok!(handle.join());
})}

fn ping(done: Receiver<bool>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            println!("Working...");
            match done.try_recv() {
                Err(mpsc::TryRecvError::Empty) => {},
                _ => break,
            }
            thread::sleep_ms(10 * 1000);
        }
    })
}
