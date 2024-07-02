use num_traits::Num;

#[allow(dead_code)]
pub fn sqr<T: Copy + std::ops::Mul<Output = T>>(a: T) -> T {
    a * a
}

#[allow(dead_code)]
pub fn max<T: Copy + PartialOrd>(a: T, b: T) -> T {
    if b > a {
        b
    } else {
        a
    }
}

#[allow(dead_code)]
pub fn min<T: Copy + PartialOrd>(a: T, b: T) -> T {
    if b < a {
        b
    } else {
        a
    }
}

#[allow(dead_code)]
pub fn sign<T: Copy + PartialOrd + std::ops::Neg<Output = T> + Num>(a: T, b: T) -> T {
    if b >= T::zero() {
        if a >= T::zero() {
            a
        } else {
            -a
        }
    } else {
        if a >= T::zero() {
            -a
        } else {
            a
        }
    }
}

#[allow(dead_code)]
pub fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

#[derive(Clone)]
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(size: usize) -> Vector<T>
    where
        T: Default + Clone,
    {
        Vector {
            data: vec![T::default(); size],
        }
    }

    #[allow(dead_code)]
    pub fn with_value(size: usize, value: T) -> Vector<T>
    where
        T: Clone,
    {
        Vector {
            data: vec![value; size],
        }
    }

    pub fn from_slice(slice: &[T]) -> Vector<T>
    where
        T: Clone,
    {
        Vector {
            data: slice.to_vec(),
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn resize(&mut self, new_size: usize)
    where
        T: Default + Clone,
    {
        self.data.resize(new_size, T::default());
    }

    #[allow(dead_code)]
    pub fn assign(&mut self, new_size: usize, value: T)
    where
        T: Clone,
    {
        self.data = vec![value; new_size];
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }
}

impl<T> std::ops::Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[derive(Clone)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Default + Clone,
{
    pub fn new(rows: usize, cols: usize) -> Matrix<T> {
        Matrix {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    #[allow(dead_code)]
    pub fn with_value(rows: usize, cols: usize, value: T) -> Matrix<T> {
        Matrix {
            rows,
            cols,
            data: vec![value; rows * cols],
        }
    }

    pub fn from_slice(rows: usize, cols: usize, slice: &[T]) -> Matrix<T>
    where
        T: Clone,
    {
        Matrix {
            rows,
            cols,
            data: slice.to_vec(),
        }
    }

    pub fn resize(&mut self, new_rows: usize, new_cols: usize) {
        self.rows = new_rows;
        self.cols = new_cols;
        self.data.resize(new_rows * new_cols, T::default());
    }

    #[allow(dead_code)]
    pub fn assign(&mut self, new_rows: usize, new_cols: usize, value: T)
    where
        T: Clone,
    {
        self.rows = new_rows;
        self.cols = new_cols;
        self.data = vec![value; new_rows * new_cols];
    }

    pub fn nrows(&self) -> usize {
        self.rows
    }

    pub fn ncols(&self) -> usize {
        self.cols
    }

    pub fn swap_elements(&mut self, row1: usize, col1: usize, row2: usize, col2: usize) {
        self.data
            .swap(row1 * self.cols + col1, row2 * self.cols + col2);
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }
}

impl<T> std::ops::Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.cols..(index + 1) * self.cols]
    }
}

impl<T> std::ops::IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.cols..(index + 1) * self.cols]
    }
}

pub type VectorInt = Vector<i32>;
pub type VectorDouble = Vector<f64>;
pub type MatrixDouble = Matrix<f64>;

