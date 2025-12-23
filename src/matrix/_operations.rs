use crate::matrix::NnfwMatrix;

#[allow(unused)]
impl NnfwMatrix {
    /* ---------- ELEMENT-WISE OPS (used in NN layers) ---------- */

    pub fn add(&mut self, another: &NnfwMatrix) {
        assert_eq!(self.rows, another.rows);
        assert_eq!(self.cols, another.cols);

        for i in 0..self.matrix.len() {
            self.matrix[i] += another.matrix[i];
        }
    }

    pub fn subtract(&mut self, another: &NnfwMatrix) {
        assert_eq!(self.rows, another.rows);
        assert_eq!(self.cols, another.cols);

        for i in 0..self.matrix.len() {
            self.matrix[i] -= another.matrix[i];
        }
    }

    /// Hadamard product (used in backprop)
    pub fn hadamard(&mut self, another: &NnfwMatrix) {
        assert_eq!(self.rows, another.rows);
        assert_eq!(self.cols, another.cols);

        for i in 0..self.matrix.len() {
            self.matrix[i] *= another.matrix[i];
        }
    }

    /// Scalar divide (VERY common in ML)
    pub fn divide_scalar(&mut self, value: f64) {
        assert_ne!(value, 0.0, "Division by zero");

        for v in self.matrix.iter_mut() {
            *v /= value;
        }
    }

    /* ---------- TRUE MATRIX MULTIPLICATION ---------- */
    /// (m×n) · (n×p) → (m×p)
    pub fn matmul(&mut self, another: &NnfwMatrix) {
        assert_eq!(
            self.cols, another.rows,
            "Matrix multiplication shape mismatch: {}×{} cannot multiply with {}×{}",
            self.rows, self.cols, another.rows, another.cols
        );
        assert_eq!(self.matrix.len(), self.rows * self.cols);
        assert_eq!(another.matrix.len(), another.rows * another.cols);

        // Result dimensions: self.rows × another.cols
        let result_rows = self.rows;
        let result_cols = another.cols;
        let mut result = vec![0.0; result_rows * result_cols];

        for r in 0..result_rows {
            for c in 0..result_cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.matrix[r * self.cols + k] * another.matrix[k * another.cols + c];
                }
                result[r * result_cols + c] = sum;
            }
        }

        self.matrix = result;
        self.rows = result_rows;
        self.cols = result_cols;
    }

    pub fn dot_product(&self, another: &NnfwMatrix) -> f64 {
        assert_eq!(
            self.rows, another.rows,
            "Dot product requires same rows: {} vs {}",
            self.rows, another.rows
        );
        assert_eq!(
            self.cols, another.cols,
            "Dot product requires same cols: {} vs {}",
            self.cols, another.cols
        );
        assert_eq!(self.matrix.len(), self.rows * self.cols);
        assert_eq!(another.matrix.len(), another.rows * another.cols);

        let mut sum = 0.0;
        for i in 0..self.matrix.len() {
            sum += self.matrix[i] * another.matrix[i];
        }
        sum
    }

    /* ---------- TRANSPOSE ---------- */
    pub fn transpose(&mut self) {
        let mut result = vec![0.0; self.rows * self.cols];

        for r in 0..self.rows {
            for c in 0..self.cols {
                result[c * self.rows + r] = self.matrix[r * self.cols + c];
            }
        }

        // 🔥 overwrite self
        self.matrix = result;
        std::mem::swap(&mut self.rows, &mut self.cols);
    }
}
