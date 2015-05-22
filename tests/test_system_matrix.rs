use assert;
use std::mem;
use threed_ice_sys::*;

use support::setup_simulator;

#[test]
fn test_system_matrix() { setup_simulator(Some("hotspot"), move |stkd, analysis, _| unsafe {
    use std::slice::from_raw_parts;

    let mut grid: ThermalGrid_t = mem::uninitialized();
    let mut matrix: SystemMatrix_t = mem::uninitialized();

    let layers = get_number_of_layers(stkd.Dimensions);
    let cells = get_number_of_cells(stkd.Dimensions);
    let connections = get_number_of_connections(stkd.Dimensions);

    thermal_grid_init(&mut grid);
    success!(thermal_grid_build(&mut grid, layers));
    fill_thermal_grid(&mut grid, &mut stkd.StackElements, stkd.Dimensions);

    system_matrix_init(&mut matrix);
    success!(system_matrix_build(&mut matrix, cells, connections));
    fill_system_matrix(&mut matrix, &mut grid, analysis, stkd.Dimensions);

    assert::equal(matrix.Size, 16);
    assert::equal(matrix.NNz, 56);

    let actual = from_raw_parts(matrix.Values, 56);
    let expected = vec![
         2.080000000000000e+00, -1.500000000000000e-02, -1.500000000000000e-02,
        -1.000000000000000e+00, -1.500000000000000e-02,  2.080000000000000e+00,
        -1.500000000000000e-02, -1.000000000000000e+00, -1.500000000000000e-02,
         2.080000000000000e+00, -1.500000000000000e-02, -1.000000000000000e+00,
        -1.500000000000000e-02, -1.500000000000000e-02,  2.080000000000000e+00,
        -1.000000000000000e+00, -1.000000000000000e+00,  2.023681082934419e+00,
        -8.000000000000001e-05, -8.000000000000001e-05, -7.035210829344193e-01,
        -1.000000000000000e+00, -8.000000000000001e-05,  2.023681082934419e+00,
        -8.000000000000001e-05, -7.035210829344193e-01, -1.000000000000000e+00,
        -8.000000000000001e-05,  2.023681082934419e+00, -8.000000000000001e-05,
        -7.035210829344193e-01, -1.000000000000000e+00, -8.000000000000001e-05,
        -8.000000000000001e-05,  2.023681082934419e+00, -7.035210829344193e-01,
        -7.035210829344193e-01,  8.006550829678521e+02, -1.201561884917640e+00,
        -7.035210829344193e-01,  8.006550829678521e+02, -1.201561884917640e+00,
        -7.035210829344193e-01,  8.006550829678521e+02, -1.201561884917640e+00,
        -7.035210829344193e-01,  8.006550829678521e+02, -1.201561884917640e+00,
        -1.201561884917640e+00,  2.206800548917464e+04, -1.201561884917640e+00,
         2.206800548917464e+04, -1.201561884917640e+00,  2.206800548917464e+04,
        -1.201561884917640e+00,  2.206800548917464e+04,
    ];
    assert::within(actual, &expected[..], 1e-10);

    let actual = from_raw_parts(matrix.RowIndices, 56);
    let expected: Vec<u32> = vec![
        0, 1, 2, 4, 0, 1, 3, 5, 0, 2, 3, 6, 1, 2, 3, 7, 0, 4, 5, 6, 8, 1, 4, 5, 7, 9, 2, 4, 6,
        7, 10, 3, 5, 6, 7, 11, 4, 8, 12, 5, 9, 13, 6, 10, 14, 7, 11, 15, 8, 12, 9, 13, 10, 14,
        11, 15,
    ];
    assert::equal(actual, &expected[..]);

    let actual = from_raw_parts(matrix.ColumnPointers, 16 + 1);
    let expected: Vec<u32> = vec![0, 4, 8, 12, 16, 21, 26, 31, 36, 39, 42, 45, 48, 50, 52, 54, 56];
    assert::equal(actual, &expected[..]);

    thermal_grid_destroy(&mut grid);
    system_matrix_destroy(&mut matrix);
})}
