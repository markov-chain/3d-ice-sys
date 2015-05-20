use std::mem;

use raw::*;

#[test]
fn test_system_matrix() { ::setup_simulator(move |stkd, analysis, _| unsafe {
    let mut grid: ThermalGrid_t = mem::uninitialized();
    let mut matrix: SystemMatrix_t = mem::uninitialized();

    thermal_grid_init(&mut grid);
    success!(thermal_grid_build(&mut grid, get_number_of_layers(stkd.Dimensions)));
    fill_thermal_grid(&mut grid, &mut stkd.StackElements, stkd.Dimensions);

    system_matrix_init(&mut matrix);
    success!(system_matrix_build(&mut matrix, get_number_of_cells(stkd.Dimensions),
                                 get_number_of_connections(stkd.Dimensions)));
    fill_system_matrix(&mut matrix, &mut grid, analysis, stkd.Dimensions);

    thermal_grid_destroy(&mut grid);
    system_matrix_destroy(&mut matrix);
})}
