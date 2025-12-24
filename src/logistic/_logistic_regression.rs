
#[derive(Clone, Debug)]
pub struct Sample<const N: usize>
{
    pub x: [f64; N],
    pub y: f64
}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0+ (-z).exp())
}

pub struct LogisticModel<const N: usize> {
    weight: [f64; N],
    bias: f64,
}

impl<const N: usize> LogisticModel<N> {
    pub fn new() -> Self {
        Self { weight: [0.0; N], bias: 0.0 }
    }

    pub fn forward(&self, x: [f64; N]) -> f64 {
        let z = self.weight.iter().zip(x.iter()).map(|(weight, x)| weight * x).sum::<f64>() + self.bias;
        sigmoid(z)
    }

    pub fn train_with_validation(&mut self, train_data: &[Sample<N>], val_data: &[Sample<N>], lr: f64, epochs: usize, log_interval: usize) {
        
        let n = train_data.len() as f64;
        
        for epoch in 0..epochs {
            let mut dw = [0.0; N];
            let mut db = 0.0;

            // Forward + Backward pass
            for s in train_data {
                let p = self.forward(s.x);
                let dz = p - s.y;
                
                for i in 0..N {
                    dw[i] += dz * s.x[i];
                }
                db += dz;
            }

            // Weight update
            for i in 0..N {
                self.weight[i] -= lr * (dw[i] / n);
            }
            self.bias -= lr * (db / n);

            // Progress logging
            if epoch % log_interval == 0 {
                let val_acc = self.calculate_accuracy(val_data);
                println!("Epoch {:>5} | Val Accuracy: {:.2}%", epoch, val_acc * 100.0);
            }
        }
    }

    pub fn calculate_accuracy(&self, data: &[Sample<N>]) -> f64 {
        let mut correct = 0;
        for s in data {
            let pred = if self.forward(s.x) >= 0.5 { 1.0 } else { 0.0 };
            if (pred - s.y).abs() < 1e-9 {
                correct += 1;
            }
        }
        correct as f64 / data.len() as f64
    }

    pub fn predict(&self, x: [f64; N]) -> f64 {
        self.forward(x)
    }

    
}