extern crate assert;
extern crate threed_ice_sys as ffi;

use ffi::*;
use std::{mem, slice};

#[macro_use]
mod support;

// https://github.com/copies/3d-ice/blob/master/bin/3D-ICE-Emulator.c
#[test]
fn test_emulator() { support::setup(move |stkd, analysis, output| unsafe {
    assert!(analysis.AnalysisType == TDICE_ANALYSIS_TYPE_TRANSIENT);

    let mut tdata: ThermalData_t = mem::uninitialized();

    thermal_data_init(&mut tdata);

    success!(thermal_data_build(&mut tdata, &mut stkd.StackElements,
                                stkd.Dimensions, analysis));

    success!(generate_output_headers(output, stkd.Dimensions,
                                     str_to_cstr!("% ").as_ptr() as String_t));

    let mut k = 0;
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
            _ => unreachable!(),
        }

        let slice = slice::from_raw_parts(tdata.Temperatures as *const f64, 4);
        assert::close(slice, &T[(k * 4)..(k * 4 + 4)], 1e-12);
        k += 1;
    }
    assert_eq!(k, 20);

    generate_output(output, stkd.Dimensions, tdata.Temperatures, tdata.PowerGrid.Sources,
                    get_simulated_time(analysis), TDICE_OUTPUT_INSTANT_FINAL);

    thermal_data_destroy(&mut tdata);
})}

const T: [f64; 20 * 4] = [
    3.246999090668178e+02, 3.373126353184268e+02, 3.246999090668178e+02, 3.373126353184269e+02,
    3.295373788723945e+02, 3.511249469518436e+02, 3.295373788723944e+02, 3.511249469518436e+02,
    3.331616718826381e+02, 3.612354156729658e+02, 3.331616718826381e+02, 3.612354156729657e+02,
    3.358840893932381e+02, 3.686615425506865e+02, 3.358840893932381e+02, 3.686615425506864e+02,
    3.410830940741837e+02, 3.709689943478313e+02, 3.410830940741837e+02, 3.709689943478314e+02,
    3.448644047631342e+02, 3.727439029585881e+02, 3.448644047631343e+02, 3.727439029585882e+02,
    3.476418916675181e+02, 3.740825393531209e+02, 3.476418916675180e+02, 3.740825393531209e+02,
    3.496872965177109e+02, 3.750897778932595e+02, 3.496872965177109e+02, 3.750897778932596e+02,
    3.543487568092195e+02, 3.726955876625655e+02, 3.543487568092194e+02, 3.726955876625656e+02,
    3.577061660907752e+02, 3.710254484187042e+02, 3.577061660907752e+02, 3.710254484187042e+02,
    3.601514565805920e+02, 3.698390913217752e+02, 3.601514565805920e+02, 3.698390913217754e+02,
    3.619380365575412e+02, 3.689951081552742e+02, 3.619380365575413e+02, 3.689951081552744e+02,
    3.663989508576396e+02, 3.652431636976722e+02, 3.663989508576396e+02, 3.652431636976723e+02,
    3.696018200411830e+02, 3.625767166553152e+02, 3.696018200411830e+02, 3.625767166553153e+02,
    3.719286956656231e+02, 3.606588212003851e+02, 3.719286956656229e+02, 3.606588212003853e+02,
    3.736250969144198e+02, 3.592772677721312e+02, 3.736250969144197e+02, 3.592772677721314e+02,
    3.780178132778689e+02, 3.551298653364983e+02, 3.780178132778688e+02, 3.551298653364985e+02,
    3.811695346247659e+02, 3.521720894753647e+02, 3.811695346247658e+02, 3.521720894753649e+02,
    3.834584456993621e+02, 3.500391702614812e+02, 3.834584456993620e+02, 3.500391702614813e+02,
    3.851270466043140e+02, 3.484985141702951e+02, 3.851270466043139e+02, 3.484985141702953e+02,
];
