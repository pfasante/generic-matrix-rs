//! Manipulations and data types that represent 2d matrix.

#![warn(bad_style, missing_docs,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate num;

use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Index, Mul, Sub};
use num::{One, Zero};

/// 2D matrix.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Matrix<T> {
    row: usize,
    column: usize,
    data: Vec<T>
}

impl<T> Matrix<T> {
    /// Creates a new `Matrix`.
    #[inline]
    pub fn from_fn<F>(row: usize, column: usize, f: F) -> Matrix<T>
        where F:Fn(usize, usize) -> T
    {
        Matrix {
            row: row,
            column: column,
            data: (0 .. row * column).map(|i| f(i / column, i % column)).collect()
        }
    }

    /// Creates a new `Matrix` from vector.
    #[inline]
    pub fn from_vec(row: usize, column: usize, data: Vec<T>) -> Matrix<T> {
        assert_eq!(row * column, data.len());
        Matrix { row: row, column: column, data: data }
    }

    /// Returns the matrix's row and column.
    #[inline]
    pub fn size(&self) -> (usize, usize) { (self.row(), self.column()) }
    /// Returns the matrix's row.
    #[inline]
    pub fn row(&self) -> usize { self.row }
    /// Returns the matrix's column.
    #[inline]
    pub fn column(&self) -> usize { self.column }
}

impl<T: Zero> Matrix<T> {
    /// Creates a matrix whose elements are all zero.
    #[inline]
    pub fn zero(row: usize, column: usize) -> Matrix<T> {
        Matrix::from_fn(row, column, |_, _| Zero::zero())
    }
}

impl<T: One + Zero> Matrix<T> {
    /// Creates a identity matrix.
    #[inline]
    pub fn one(row: usize, column: usize) -> Matrix<T> {
        Matrix::from_fn(row, column, |i, j| {
            if i == j {
                One::one()
            } else {
                Zero::zero()
            }
        })
    }
}

impl<T: Clone> Matrix<T> {
    #[inline]
    /// Returns transpose of the matrix.
    pub fn trans(&self) -> Matrix<T> {
        Matrix::from_fn(self.column(), self.row(), |i, j| self[(j, i)].clone())
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut res = write!(f, "");
        let (row, col) = self.size();

        for i in 0..row {
            // write matrix content
            for j in 0..col {
                res = res.and(write!(f, "{} ", self[(i, j)]));
            }
            res = res.and(write!(f, "\n"));
        }

        res
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, (i, j): (usize, usize)) -> &T {
        assert!(i < self.row() && j < self.column());
        &self.data[i * self.column() + j]
    }
}

macro_rules! forward_val_val_binop {
    (impl $imp:ident, $method:ident) => {
        impl<Lhs, Rhs> $imp<Matrix<Rhs>> for Matrix<Lhs>
            where Lhs: $imp<Rhs> + Clone, Rhs: Clone
        {
            type Output = Matrix<<Lhs as $imp<Rhs>>::Output>;

            #[inline]
            fn $method(self, other: Matrix<Rhs>) -> Matrix<<Lhs as $imp<Rhs>>::Output> {
                (&self).$method(&other)
            }
        }
    }
}

macro_rules! forward_ref_val_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, Lhs, Rhs> $imp<Matrix<Rhs>> for &'a Matrix<Lhs>
            where Lhs: $imp<Rhs> + Clone, Rhs: Clone
        {
            type Output = Matrix<<Lhs as $imp<Rhs>>::Output>;

            #[inline]
            fn $method(self, other: Matrix<Rhs>) -> Matrix<<Lhs as $imp<Rhs>>::Output> {
                self.$method(&other)
            }
        }
    }
}

macro_rules! forward_val_ref_binop {
    (impl $imp:ident, $method:ident) => {
        impl<'a, Lhs, Rhs> $imp<&'a Matrix<Rhs>> for Matrix<Lhs>
            where Lhs: $imp<Rhs> + Clone, Rhs: Clone
        {
            type Output = Matrix<<Lhs as $imp<Rhs>>::Output>;

            #[inline]
            fn $method(self, other: &Matrix<Rhs>) -> Matrix<<Lhs as $imp<Rhs>>::Output> {
                (&self).$method(other)
            }
        }
    }
}

macro_rules! forward_all_binop {
    (impl $imp:ident, $method:ident) => {
        forward_val_val_binop!(impl $imp, $method);
        forward_ref_val_binop!(impl $imp, $method);
        forward_val_ref_binop!(impl $imp, $method);
    };
}

forward_all_binop!(impl Add, add);

impl<'a, 'b, Lhs, Rhs> Add<&'b Matrix<Rhs>> for &'a Matrix<Lhs>
    where Lhs: Add<Rhs> + Clone, Rhs: Clone
{
    type Output = Matrix<<Lhs as Add<Rhs>>::Output>;

    #[inline]
    fn add(self, other: &Matrix<Rhs>) -> Matrix<<Lhs as Add<Rhs>>::Output> {
        assert_eq!(self.size(), other.size());
        Matrix::from_fn(self.row(), self.column(), |i, j| self[(i, j)].clone() + other[(i, j)].clone())
    }
}

forward_all_binop!(impl Sub, sub);

impl<'a, 'b, Lhs, Rhs> Sub<&'b Matrix<Rhs>> for &'a Matrix<Lhs>
    where Lhs: Sub<Rhs> + Clone, Rhs: Clone
{
    type Output = Matrix<<Lhs as Sub<Rhs>>::Output>;

    #[inline]
    fn sub(self, other: &Matrix<Rhs>) -> Matrix<<Lhs as Sub<Rhs>>::Output> {
        assert_eq!(self.size(), other.size());
        Matrix::from_fn(self.row(), self.column(), |i, j| self[(i, j)].clone() - other[(i, j)].clone())
    }
}

impl<Lhs, Rhs> Mul<Matrix<Rhs>> for Matrix<Lhs>
    where Lhs: Mul<Rhs> + Clone, Rhs: Clone,
          <Lhs as Mul<Rhs>>::Output: Add<Output = <Lhs as Mul<Rhs>>::Output>
{
    type Output = Matrix<<Lhs as Mul<Rhs>>::Output>;

    #[inline]
    fn mul(self, other: Matrix<Rhs>) -> Matrix<<Lhs as Mul<Rhs>>::Output> {
        (&self).mul(&other)
    }
}

impl<'a, Lhs, Rhs> Mul<Matrix<Rhs>> for &'a Matrix<Lhs>
    where Lhs: Mul<Rhs> + Clone, Rhs: Clone,
          <Lhs as Mul<Rhs>>::Output: Add<Output = <Lhs as Mul<Rhs>>::Output>
{
    type Output = Matrix<<Lhs as Mul<Rhs>>::Output>;

    #[inline]
    fn mul(self, other: Matrix<Rhs>) ->  Matrix<<Lhs as Mul<Rhs>>::Output> {
        self.mul(&other)
    }
}

impl<'a, Lhs, Rhs> Mul<&'a Matrix<Rhs>> for Matrix<Lhs>
    where Lhs: Mul<Rhs> + Clone, Rhs: Clone,
          <Lhs as Mul<Rhs>>::Output: Add<Output = <Lhs as Mul<Rhs>>::Output>
{
    type Output = Matrix<<Lhs as Mul<Rhs>>::Output>;

    #[inline]
    fn mul(self, other: &Matrix<Rhs>) -> Matrix<<Lhs as Mul<Rhs>>::Output> {
        self.mul(other)
    }
}

impl<'a, 'b, Lhs, Rhs> Mul<&'b Matrix<Rhs>> for &'a Matrix<Lhs>
    where Lhs: Mul<Rhs> + Clone, Rhs: Clone,
          <Lhs as Mul<Rhs>>::Output: Add<Output = <Lhs as Mul<Rhs>>::Output>
{
    type Output = Matrix<<Lhs as Mul<Rhs>>::Output>;

    #[inline]
    fn mul(self, other: &Matrix<Rhs>) -> Matrix<<Lhs as Mul<Rhs>>::Output> {
        assert_eq!(self.column(), other.row());
        Matrix::from_fn(self.row(), other.column(), |i, j| {
            let mut sum = self[(i, 0)].clone() * other[(0, j)].clone();
            for k in 1 .. self.column() {
                sum = sum + self[(i, k)].clone() * other[(k, j)].clone();
            }
            sum
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn from_vec() {
        let mat = Matrix::from_vec(2, 3, vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]);
        for i in 0 .. mat.row() {
            for j in 0 .. mat.column() {
                assert_eq!((i, j), mat[(i, j)]);
            }
        }
    }

    #[test]
    fn index() {
        let mat = Matrix::from_fn(3, 5, |i, j| (i, j));
        for i in 0 .. mat.row() {
            for j in 0 .. mat.column() {
                assert_eq!((i, j), mat[(i, j)]);
            }
        }
    }


    #[test]
    fn mul() {
        let m1 = Matrix::from_vec(1, 3, vec![1.0f64, 2.0, 3.0]);
        let m2 = Matrix::from_vec(3, 1, vec![1.0, 2.0, 3.0]);
        assert_eq!(Matrix::from_vec(1, 1, vec![14.0]), m1 * m2);
        assert_eq!(Matrix::from_vec(3, 1, vec![1.0f64, 4.0, 7.0]),
                   Matrix::from_vec(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]) *
                   Matrix::from_vec(3, 1, vec![1.0, 0.0, 0.0]));
        assert_eq!(Matrix::from_vec(3, 2, vec![1.0f64, 3.0, 4.0, 6.0, 7.0, 9.0]),
                   Matrix::from_vec(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]) *
                   Matrix::from_vec(3, 2, vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0]));
    }
}

