#![allow(unused_imports)]
mod logistic;
mod matrix;

use logistic::_logistic_regression::LogisticModel;
use logistic::_logistic_regression::Sample;
use matrix::_2d_array::NnfwMatrix;
use matrix::_type::def;

use rand::Rng;
use rand::prelude::SliceRandom;

const FEATURE_COUNT: usize = 2;

// Feature normalization (User apne features ke hisaab se change kare)
fn normalize_features(cgpa: f64, iq: f64) -> [f64; FEATURE_COUNT] {
    let norm_cgpa = (cgpa - 4.0) / 6.0;
    let norm_iq = (iq - 80.0) / 70.0;
    [norm_cgpa, norm_iq]
}

// Data generation (User apna logic yahaan implement kare)
fn generate_data(n: usize) -> Vec<Sample<FEATURE_COUNT>> {
    let mut rng = rand::rng();
    let mut data = Vec::new();

    for _ in 0..n {
        let cgpa = rng.random_range(4.0..10.0);
        let iq = rng.random_range(80.0..150.0);

        // Target logic (User apne classification rule define kare)
        let placed = if cgpa > 7.5 || (cgpa > 6.5 && iq > 115.0) {
            1.0
        } else {
            0.0
        };

        data.push(Sample {
            x: normalize_features(cgpa, iq),
            y: placed,
        });
    }

    data.shuffle(&mut rng);
    data
}

fn main() {
    // let mut nmatrix: NnfwMatrix = NnfwMatrix::new(4, 5, 3.3);
    // let mut mmatrix: NnfwMatrix = NnfwMatrix::new(4, 5, 5.7);

    // println!("BEFORE ADD ----------");
    // nmatrix.log_matrix();
    // println!("AFTER ADDED ---------");
    // mmatrix.log_matrix();
    // println!("FINAL ---------------");
    // nmatrix.add(&mmatrix);
    // nmatrix.log_matrix();

    // println!("BEFORE SUB ----------");
    // nmatrix.log_matrix();
    // println!("AFTER SUB ---------");
    // mmatrix.log_matrix();
    // nmatrix.subtract(&mmatrix);
    // println!("FINAL -------------");
    // nmatrix.log_matrix();

    // println!("BEFORE MLT ----------");
    // nmatrix.log_matrix();
    // println!("AFTER MLT ---------");
    // mmatrix = NnfwMatrix::new(5, 5, 4.5);
    // mmatrix.log_matrix();
    // println!("FINAL -------------");
    // nmatrix.matmul(&mmatrix);
    // nmatrix.log_matrix();

    // println!("AFTER TRANSPOSE");
    // nmatrix.transpose();
    // nmatrix.log_matrix();

    // println!("AFTER DIVIDE 4.4");
    // nmatrix.divide_scalar(4.4);
    // nmatrix.log_matrix();

    // println!("DOT PRODUCT");
    // nmatrix.log_matrix();
    // mmatrix = NnfwMatrix::new(5, 4, 4.0);
    // mmatrix.log_matrix();
    // let value: f64 = nmatrix.dot_product(&mmatrix);
    // println!("ANS --------------");
    // println!("{}", value);

    println!("🔧 Generating dataset...");
    let total_data = generate_data(1000);

    // 80-20 train-validation split
    let split_idx = (total_data.len() as f64 * 0.8) as usize;
    let (train_set, val_set) = total_data.split_at(split_idx);

    println!("📊 Training model...\n");
    let mut model = LogisticModel::<FEATURE_COUNT>::new();
    model.train_with_validation(train_set, val_set, 0.1, 10_001, 2000);

    let mut test_cgpa = 8.5;
    let mut test_iq = 120.0;
    let mut features = normalize_features(test_cgpa, test_iq);
    let mut prob = model.predict(features);

    println!("\n✅ Final Result:");
    println!("   CGPA: {}, IQ: {}", test_cgpa, test_iq);
    println!("   Placement Probability: {:.2}%", prob * 100.0);

    test_cgpa = 3.4;
    test_iq = 100.0;
    features = normalize_features(test_cgpa, test_iq);
    prob = model.predict(features);

    println!("\n✅ Final Result:");
    println!("   CGPA: {}, IQ: {}", test_cgpa, test_iq);
    println!("   Placement Probability: {:.2}%", prob * 100.0);
}
