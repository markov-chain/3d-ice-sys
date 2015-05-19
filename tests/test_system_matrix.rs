use std::mem;

use raw::*;

#[test]
fn test_system_matrix() { ::setup_simulator(move |stkd, analysis, _| unsafe {
    let mut tdata: ThermalData_t = mem::uninitialized();

    thermal_data_init(&mut tdata);
    tdata.Size = get_number_of_cells(stkd.Dimensions);

    success!(thermal_grid_build(&mut tdata.ThermalGrid,
                                get_number_of_layers(stkd.Dimensions)));


    fill_thermal_grid(&mut tdata.ThermalGrid, &mut stkd.StackElements, stkd.Dimensions);

    success!(system_matrix_build(&mut tdata.SM_A, tdata.Size,
                                 get_number_of_connections(stkd.Dimensions)));

    fill_system_matrix(&mut tdata.SM_A, &mut tdata.ThermalGrid, analysis, stkd.Dimensions);

    thermal_data_destroy(&mut tdata);
})}
