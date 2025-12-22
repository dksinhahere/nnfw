use crate::matrix::_type::def;
#[allow(unused)]
use std::io::{self, Write, stdout};

#[derive(Debug)]
pub struct NnfwMatrix {
    matrix: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl NnfwMatrix {
    pub fn new(rows: usize, cols: usize, fill: f64) -> Self {
        Self {
            matrix: vec![fill; rows * cols],
            rows,
            cols,
        }
    }

    #[allow(clippy::unused_unit)]
    #[allow(clippy::print_with_newline)]
    pub fn log_matrix(&self) -> () {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{} ", self.matrix[i * self.cols + j]);
            }
            print!("\n");
        }
        print!("\n")
    }

    pub fn merge_ethernal(matrix: Vec<f64>, rows: usize, cols: usize) -> Self {
        assert_eq!(matrix.len(), rows * cols);
        Self { matrix, rows, cols }
    }

    #[allow(dead_code)]
    pub fn input_matrix() -> Vec<f64> {
        let mut user = String::new();
        io::stdin().read_line(&mut user).expect("Input error");

        let mut tmp: Vec<&str> = user.split_whitespace().collect();

        if tmp.len() < 3 {
            panic!("Input length error: need row col and at least 1 value");
        }

        let row: usize = tmp.remove(0).parse().unwrap();
        let col: usize = tmp.remove(0).parse().unwrap();

        if tmp.len() != row * col {
            panic!(
                "Matrix data mismatch: expected {}, got {}",
                row * col,
                tmp.len()
            );
        }

        let mut matrix = vec![0.0; row * col];
        for i in 0..matrix.len() {
            matrix[i] = tmp[i].parse::<f64>().unwrap();
        }
        matrix
    }

    pub fn retrive(&self, rows: usize, cols: usize) -> f64 {
        self.matrix[rows * self.cols + cols]
    }

    #[allow(clippy::unused_unit)]
    pub fn reclaim(&mut self, rows: usize, cols: usize, value: def) -> () {
        self.matrix[rows * self.cols + cols] = NnfwMatrix::compact(value);
    }

    #[allow(clippy::unused_unit)]
    #[allow(clippy::print_with_newline)]
    pub fn log(matrix: Vec<f64>, rows: usize, cols: usize) -> () {
        for r in 0..rows {
            for c in 0..cols {
                print!("{} ", matrix[r * cols + c]);
            }
            print!("\n");
        }
        print!("\n");
    }

    #[allow(dead_code)]
    pub fn zero_mat(rows: usize, cols: usize) -> Vec<f64> {
        vec![0.0; rows * cols]
    }

    #[allow(dead_code)]
    pub fn ones_mat(rows: usize, cols: usize) -> Vec<f64> {
        vec![1.0; rows * cols]
    }

    #[allow(clippy::unnecessary_cast)]
    pub fn compact(value: def) -> f64 {
        match value {
            def::I1(x) => x as f64,
            def::I2(x) => x as f64,
            def::I4(x) => x as f64,
            def::I8(x) => x as f64,
            def::I16(x) => x as f64,
            def::U1(x) => x as f64,
            def::U2(x) => x as f64,
            def::U4(x) => x as f64,
            def::U8(x) => x as f64,
            def::U16(x) => x as f64,
            def::I(x) => x as f64,
            def::U(x) => x as f64,
            def::F4(y) => y as f64,
            def::F8(y) => y as f64,
        }
    }

    #[allow(clippy::match_ref_pats)]
    #[allow(clippy::unused_unit)]
    #[allow(clippy::unnecessary_cast)]
    pub fn claim_matrix(&mut self, data: Vec<def>) -> () {
        assert_eq!(data.len(), self.rows * self.cols);
        for (index, item) in data.iter().enumerate() {
            self.matrix[index] = match item {
                &def::I1(x) => x as f64,
                &def::I2(x) => x as f64,
                &def::I4(x) => x as f64,
                &def::I8(x) => x as f64,
                &def::I16(x) => x as f64,
                &def::U1(x) => x as f64,
                &def::U2(x) => x as f64,
                &def::U4(x) => x as f64,
                &def::U8(x) => x as f64,
                &def::U16(x) => x as f64,
                &def::I(x) => x as f64,
                &def::U(x) => x as f64,
                &def::F4(y) => y as f64,
                &def::F8(y) => y as f64,
            }
        }
    }
}
