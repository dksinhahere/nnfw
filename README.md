# NNFW API Reference

## Matrix Module

### NnfwMatrix

```rust
pub struct NnfwMatrix
```

A matrix structure for neural network operations with flexible numeric type support.

**Attributes:**
- `matrix` : Vec<f64> - Internal storage in row-major order
- `rows` : usize - Number of rows
- `cols` : usize - Number of columns

---

## Constructor Functions

### NnfwMatrix::new

```rust
pub fn new(rows: usize, cols: usize, fill: f64) -> Self
```

Create a new matrix filled with a constant value.

**Parameters:**

**rows** : usize
    Number of rows in the matrix.

**cols** : usize
    Number of columns in the matrix.

**fill** : f64
    Value to fill the matrix with.

**Returns:**

**out** : NnfwMatrix
    A new matrix instance of shape (rows, cols) filled with `fill`.

**Examples:**

```rust
// Create a 3x4 matrix filled with zeros
let matrix = NnfwMatrix::new(3, 4, 0.0);

// Create a 2x2 matrix filled with ones
let identity_start = NnfwMatrix::new(2, 2, 1.0);
```

---

### NnfwMatrix::merge_ethernal

```rust
pub fn merge_ethernal(matrix: Vec<f64>, rows: usize, cols: usize) -> Self
```

Construct a matrix from an existing vector.

**Parameters:**

**matrix** : Vec<f64>
    Input vector containing matrix elements in row-major order.

**rows** : usize
    Number of rows.

**cols** : usize
    Number of columns.

**Returns:**

**out** : NnfwMatrix
    Matrix constructed from the input vector.

**Raises:**

Panics if `matrix.len() != rows * cols`.

**Examples:**

```rust
let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
let matrix = NnfwMatrix::merge_ethernal(data, 2, 3);
// Creates:
// [[1.0, 2.0, 3.0],
//  [4.0, 5.0, 6.0]]
```

---

## Matrix Creation Functions

### NnfwMatrix::zero_mat

```rust
pub fn zero_mat(rows: usize, cols: usize) -> Vec<f64>
```

Return a vector filled with zeros.

**Parameters:**

**rows** : usize
    Number of rows.

**cols** : usize
    Number of columns.

**Returns:**

**out** : Vec<f64>
    Vector of zeros with length `rows * cols`.

**See Also:**

`ones_mat` : Return a vector filled with ones.

**Examples:**

```rust
let zeros = NnfwMatrix::zero_mat(2, 3);
// Returns: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]

NnfwMatrix::log(zeros, 2, 3);
// Output:
// 0 0 0
// 0 0 0
```

---

### NnfwMatrix::ones_mat

```rust
pub fn ones_mat(rows: usize, cols: usize) -> Vec<f64>
```

Return a vector filled with ones.

**Parameters:**

**rows** : usize
    Number of rows.

**cols** : usize
    Number of columns.

**Returns:**

**out** : Vec<f64>
    Vector of ones with length `rows * cols`.

**See Also:**

`zero_mat` : Return a vector filled with zeros.

**Examples:**

```rust
let ones = NnfwMatrix::ones_mat(3, 2);
// Returns: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0]

let matrix = NnfwMatrix::merge_ethernal(ones, 3, 2);
matrix.log_matrix();
// Output:
// 1 1
// 1 1
// 1 1
```

---

### NnfwMatrix::input_matrix

```rust
pub fn input_matrix() -> Vec<f64>
```

Read matrix data from standard input.

**Returns:**

**out** : Vec<f64>
    Vector containing parsed matrix values.

**Raises:**

Panics if:
- Input contains fewer than 3 elements
- Number of values doesn't match specified dimensions

**Notes:**

Input format: `<rows> <cols> <val1> <val2> ... <valN>`

All values must be space-separated on a single line.

**Examples:**

```rust
// User inputs: "2 3 1.0 2.0 3.0 4.0 5.0 6.0"
let data = NnfwMatrix::input_matrix();
// Returns: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]

let matrix = NnfwMatrix::merge_ethernal(data, 2, 3);
```

---

## Array Manipulation

### NnfwMatrix::claim_matrix

```rust
pub fn claim_matrix(&mut self, data: Vec<def>)
```

Fill the matrix with values from a vector.

**Parameters:**

**data** : Vec<def>
    Vector of def enum values to populate the matrix.

**Raises:**

Panics if `data.len() != rows * cols`.

**Notes:**

All input values are automatically converted to f64 regardless of their original type.

**Examples:**

```rust
let mut matrix = NnfwMatrix::new(2, 3, 0.0);
let data = vec![
    def::I2(1), def::I2(2), def::I2(3),
    def::F4(4.5), def::F4(5.5), def::F4(6.5),
];
matrix.claim_matrix(data);
matrix.log_matrix();
// Output:
// 1 2 3
// 4.5 5.5 6.5
```

---

### NnfwMatrix::reclaim

```rust
pub fn reclaim(&mut self, rows: usize, cols: usize, value: def)
```

Set a value at a specific position.

**Parameters:**

**rows** : usize
    Row index (0-based).

**cols** : usize
    Column index (0-based).

**value** : def
    New value to set at the specified position.

**See Also:**

`retrive` : Get value at a specific position.

**Examples:**

```rust
let mut matrix = NnfwMatrix::new(3, 3, 0.0);
matrix.reclaim(1, 2, def::F4(42.0));
matrix.reclaim(0, 0, def::I4(10));

println!("{}", matrix.retrive(1, 2)); // Output: 42
```

---

## Indexing and Retrieval

### NnfwMatrix::retrive

```rust
pub fn retrive(&self, rows: usize, cols: usize) -> f64
```

Get the value at a specific position.

**Parameters:**

**rows** : usize
    Row index (0-based).

**cols** : usize
    Column index (0-based).

**Returns:**

**out** : f64
    Value at position (rows, cols).

**See Also:**

`reclaim` : Set value at a specific position.

**Examples:**

```rust
let matrix = NnfwMatrix::new(3, 3, 5.0);
let value = matrix.retrive(1, 1);
println!("{}", value); // Output: 5

matrix.reclaim(1, 1, def::F4(10.0));
let new_value = matrix.retrive(1, 1);
println!("{}", new_value); // Output: 10
```

---

## Input/Output

### NnfwMatrix::log_matrix

```rust
pub fn log_matrix(&self)
```

Print the matrix to standard output.

**Examples:**

```rust
let matrix = NnfwMatrix::new(2, 3, 1.0);
matrix.log_matrix();
// Output:
// 1 1 1
// 1 1 1
```

---

### NnfwMatrix::log

```rust
pub fn log(matrix: Vec<f64>, rows: usize, cols: usize)
```

Print a vector as a matrix with specified dimensions.

**Parameters:**

**matrix** : Vec<f64>
    Vector containing matrix data in row-major order.

**rows** : usize
    Number of rows.

**cols** : usize
    Number of columns.

**Examples:**

```rust
let data = vec![1.0, 2.0, 3.0, 4.0];
NnfwMatrix::log(data, 2, 2);
// Output:
// 1 2
// 3 4
```

---

## Type Conversion

### NnfwMatrix::compact

```rust
pub fn compact(value: def) -> f64
```

Convert a def enum value to f64.

**Parameters:**

**value** : def
    Input value in def enum format.

**Returns:**

**out** : f64
    Converted floating-point value.

**Examples:**

```rust
let result1 = NnfwMatrix::compact(def::I4(42));
println!("{}", result1); // Output: 42.0

let result2 = NnfwMatrix::compact(def::F4(3.14));
println!("{}", result2); // Output: 3.14

let result3 = NnfwMatrix::compact(def::U16(1000));
println!("{}", result3); // Output: 1000.0
```

---

## Type System

### def

```rust
pub enum def
```

Flexible numeric type enumeration for matrix operations.

**Variants:**

| Variant | Type | Description |
|---------|------|-------------|
| `I1(i8)` | i8 | 8-bit signed integer |
| `I2(i16)` | i16 | 16-bit signed integer |
| `I4(i32)` | i32 | 32-bit signed integer |
| `I8(i64)` | i64 | 64-bit signed integer |
| `I16(i128)` | i128 | 128-bit signed integer |
| `U1(u8)` | u8 | 8-bit unsigned integer |
| `U2(u16)` | u16 | 16-bit unsigned integer |
| `U4(u32)` | u32 | 32-bit unsigned integer |
| `U8(u64)` | u64 | 64-bit unsigned integer |
| `U16(u128)` | u128 | 128-bit unsigned integer |
| `U(usize)` | usize | Platform-dependent unsigned integer |
| `I(isize)` | isize | Platform-dependent signed integer |
| `F4(f32)` | f32 | 32-bit floating point |
| `F8(f64)` | f64 | 64-bit floating point |

**Examples:**

```rust
use matrix::_type::def;

let int_val = def::I4(100);
let float_val = def::F4(3.14159);
let large_int = def::I16(999999999999);

// All types can be converted to f64
let converted = NnfwMatrix::compact(int_val);
```

---

## Complete Example

```rust
use matrix::_2d_array::NnfwMatrix;
use matrix::_type::def;

fn main() {
    // Create a 3x3 matrix filled with 1.0
    let mut matrix = NnfwMatrix::new(3, 3, 1.0);
    println!("Initial matrix:");
    matrix.log_matrix();
    
    // Fill with sequential values
    let mut data = Vec::new();
    for i in 0..9 {
        data.push(def::I2(i));
    }
    matrix.claim_matrix(data);
    println!("After filling:");
    matrix.log_matrix();
    
    // Access and modify elements
    let value = matrix.retrive(1, 1);
    println!("Value at (1,1): {}", value);
    
    matrix.reclaim(1, 1, def::F4(99.9));
    println!("After modification:");
    matrix.log_matrix();
    
    // Create matrices using static methods
    let zeros = NnfwMatrix::zero_mat(2, 2);
    let ones = NnfwMatrix::ones_mat(2, 2);
    
    println!("Zeros matrix:");
    NnfwMatrix::log(zeros, 2, 2);
    
    println!("Ones matrix:");
    NnfwMatrix::log(ones, 2, 2);
}
```

**Output:**
```
Initial matrix:
1 1 1
1 1 1
1 1 1

After filling:
0 1 2
3 4 5
6 7 8

Value at (1,1): 4
After modification:
0 1 2
3 99.9 5
6 7 8

Zeros matrix:
0 0
0 0

Ones matrix:
1 1
1 1
```