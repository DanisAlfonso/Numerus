use crate::math_utilities::{MatrixDouble, VectorDouble, VectorInt};

pub struct LU {
    n: usize,
    lu: MatrixDouble,
    index: VectorInt,
    d: f64,
}

impl LU {
    pub fn new(a: &MatrixDouble) -> LU {
        let n = a.nrows();
        let mut lu = a.clone();
        let mut index = VectorInt::new(n);
        let mut d = 1.0;
        let mut vv = VectorDouble::new(n);
        let tiny = 1.0e-40;

        for i in 0..n {
            let mut big = 0.0;
            for j in 0..n {
                let temp = lu[i][j].abs();
                if temp > big {
                    big = temp;
                }
            }
            if big == 0.0 {
                panic!("Singular matrix in LU decomposition");
            }
            vv[i] = 1.0 / big;
        }

        for k in 0..n {
            let mut big = 0.0;
            let mut imax = k;
            for i in k..n {
                let temp = vv[i] * lu[i][k].abs();
                if temp > big {
                    big = temp;
                    imax = i;
                }
            }
            if k != imax {
                for j in 0..n {
                    lu.swap_elements(imax, j, k, j);
                }
                d = -d;
                vv[imax] = vv[k];
            }
            index[k] = imax as i32;
            if lu[k][k] == 0.0 {
                lu[k][k] = tiny;
            }
            for i in (k + 1)..n {
                lu[i][k] /= lu[k][k];
                let temp = lu[i][k];
                for j in (k + 1)..n {
                    lu[i][j] -= temp * lu[k][j];
                }
            }
        }

        LU { n, lu, index, d }
    }

    pub fn solve(&self, b: &VectorDouble, x: &mut VectorDouble) {
        if b.size() != self.n || x.size() != self.n {
            panic!("LU::solve bad sizes");
        }

        for i in 0..self.n {
            x[i] = b[i];
        }

        let mut sum;
        let mut ii = 0;

        for i in 0..self.n {
            let ip = self.index[i] as usize;
            sum = x[ip];
            x[ip] = x[i];
            if ii != 0 {
                for j in (ii - 1)..i {
                    sum -= self.lu[i][j] * x[j];
                }
            } else if sum != 0.0 {
                ii = i + 1;
            }
            x[i] = sum;
        }

        for i in (0..self.n).rev() {
            sum = x[i];
            for j in (i + 1)..self.n {
                sum -= self.lu[i][j] * x[j];
            }
            x[i] = sum / self.lu[i][i];
        }
    }

    pub fn solve_matrix(&self, b: &MatrixDouble, x: &mut MatrixDouble) {
        if b.nrows() != self.n || x.nrows() != self.n || b.ncols() != x.ncols() {
            panic!("LU::solve bad sizes");
        }

        let mut xx = VectorDouble::new(self.n);

        for j in 0..b.ncols() {
            for i in 0..self.n {
                xx[i] = b[i][j];
            }
            {
                let mut xx_mut = xx.clone();
                self.solve(&xx, &mut xx_mut);
                for i in 0..self.n {
                    x[i][j] = xx_mut[i];
                }
            }
        }
    }

    pub fn inverse(&self, ainv: &mut MatrixDouble) {
        ainv.resize(self.n, self.n);
        for i in 0..self.n {
            for j in 0..self.n {
                ainv[i][j] = 0.0;
            }
            ainv[i][i] = 1.0;
        }
        let mut ainv_mut = ainv.clone();
        self.solve_matrix(&ainv, &mut ainv_mut);
        *ainv = ainv_mut;
    }

    pub fn det(&self) -> f64 {
        let mut dd = self.d;
        for i in 0..self.n {
            dd *= self.lu[i][i];
        }
        dd
    }

    pub fn lu_decomposition(&self, _b: &MatrixDouble, x: &mut MatrixDouble) {
        for i in 0..self.n {
            for j in 0..self.n {
                x[i][j] = self.lu[i][j];
            }
        }
    }
}

