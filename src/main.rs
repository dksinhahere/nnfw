#![allow(unused_imports)]
mod matrix;
use matrix::_2d_array::NnfwMatrix;
use matrix::_type::def;

fn main() {
    let mut nmatrix: NnfwMatrix = NnfwMatrix::new(4, 5, 3.3);
    let mut mmatrix: NnfwMatrix = NnfwMatrix::new(4, 5, 5.7);

    println!("BEFORE ADD ----------");
    nmatrix.log_matrix();
    println!("AFTER ADDED ---------");
    mmatrix.log_matrix();
    println!("FINAL ---------------");
    nmatrix.add(&mmatrix);
    nmatrix.log_matrix();

    println!("BEFORE SUB ----------");
    nmatrix.log_matrix();
    println!("AFTER SUB ---------");
    mmatrix.log_matrix();
    nmatrix.subtract(&mmatrix);
    println!("FINAL -------------");
    nmatrix.log_matrix();

    println!("BEFORE MLT ----------");
    nmatrix.log_matrix();
    println!("AFTER MLT ---------");
    mmatrix = NnfwMatrix::new(5, 5, 4.5);
    mmatrix.log_matrix();
    println!("FINAL -------------");
    nmatrix.matmul(&mmatrix);
    nmatrix.log_matrix();

    println!("AFTER TRANSPOSE");
    nmatrix.transpose();
    nmatrix.log_matrix();

    println!("AFTER DIVIDE 4.4");
    nmatrix.divide_scalar(4.4);
    nmatrix.log_matrix();

    println!("DOT PRODUCT");
    nmatrix.log_matrix();
    mmatrix = NnfwMatrix::new(5, 4, 4.0);
    mmatrix.log_matrix();
    let value: f64 = nmatrix.dot_product(&mmatrix);
    println!("ANS --------------");
    println!("{}", value);
    
}
