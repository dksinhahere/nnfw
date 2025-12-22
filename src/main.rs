mod matrix;
use matrix::_2d_array::NnfwMatrix;
use matrix::_type::def;

fn main() {
    let mut value: NnfwMatrix = NnfwMatrix::new(1, 3, 1.0);
    let other: Vec<def> = vec![def::I2(1), def::I4(2), def::U16(3)];
    value.claim_matrix(other);
    value.log_matrix();

    let mut value1: NnfwMatrix = NnfwMatrix::new(3, 6, 1.0);
    let mut other: Vec<def> = Vec::new();
    for i in 0..=2 {
        for j in 0..=5 {
            let val: i16 = (i + j) as i16;
            other.push(def::I2(val));
        }
    }
    value1.claim_matrix(other);
    value1.log_matrix();

    let res: f64 = value1.retrive(2, 5);
    println!("{:?}", res);

    value1.reclaim(2, 5, def::F4(44.99));
    value1.log_matrix();

    let matrix1: Vec<f64> = NnfwMatrix::zero_mat(1, 1);
    NnfwMatrix::log(matrix1, 1, 1);

    let matrix2: Vec<f64> = NnfwMatrix::ones_mat(3, 3);
    NnfwMatrix::log(matrix2.clone(), 3, 3);

    let imat: Vec<f64> = NnfwMatrix::input_matrix();
    NnfwMatrix::log(imat, 1, 1);

    let mat: NnfwMatrix = NnfwMatrix::merge_ethernal(matrix2.clone(), 3, 3);
    mat.log_matrix();
}
