use assert;
use libc::c_double;
use std::{iter, mem};
use threed_ice_sys::*;

use support::setup_simulator;

#[test]
fn test_system_vector() { setup_simulator(Some("hotspot"), move |stkd, analysis, _| unsafe {
    let mut tgrid: ThermalGrid_t = mem::uninitialized();
    let mut pgrid: PowerGrid_t = mem::uninitialized();

    let layers = get_number_of_layers(stkd.Dimensions);
    let cells = get_number_of_cells(stkd.Dimensions);

    thermal_grid_init(&mut tgrid);
    success!(thermal_grid_build(&mut tgrid, layers));
    fill_thermal_grid(&mut tgrid, &mut stkd.StackElements, stkd.Dimensions);

    power_grid_init(&mut pgrid);
    success!(power_grid_build(&mut pgrid, layers, cells));
    fill_power_grid(&mut pgrid, &mut stkd.StackElements);

    let mut actual = iter::repeat(analysis.InitialTemperature).take(cells as usize)
                                                              .collect::<Vec<_>>();

    let temperatures = actual.as_mut_ptr() as *mut _;
    success!(update_source_vector(&mut pgrid, &mut tgrid, stkd.Dimensions));
    fill_system_vector(stkd.Dimensions, temperatures, pgrid.Sources, &mut tgrid, temperatures,
                       analysis.StepTime);

    let expected = vec![
        3.440574999999999e+02, 3.440574999999999e+02, 3.440574999999999e+02, 3.440574999999999e+02,
        1.018080000000000e+02, 1.018080000000000e+02, 1.018080000000000e+02, 1.018080000000000e+02,
        2.541223125000000e+05, 2.541223125000000e+05, 2.541223125000000e+05, 2.541223125000000e+05,
        7.020553669467223e+06, 7.020553669467223e+06, 7.020553669467223e+06, 7.020553669467223e+06,
    ];

    assert::within(&actual, &expected, 1e-10);

    thermal_grid_destroy(&mut tgrid);
    power_grid_destroy(&mut pgrid);
})}

unsafe fn fill_system_vector(dimensions: *mut Dimensions_t, vector: *mut c_double,
                             sources: *mut Source_t, thermal_grid: *mut ThermalGrid_t,
                             temperatures: *mut Temperature_t, step_time: Time_t) {

    let mut k = 0;
    for layer in 0..get_number_of_layers(dimensions) {
        for row in 0..get_number_of_rows(dimensions) {
            for column in 0..get_number_of_columns(dimensions) {
                let capacitance = get_capacity(thermal_grid, dimensions, layer, row, column);
                *vector.offset(k) = *sources.offset(k) + (capacitance / step_time) *
                                                         *temperatures.offset(k);
                k += 1;
            }
        }
    }
}
