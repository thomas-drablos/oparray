use std::{fmt, ops};

use crate::{Dimension, Numeric};

pub struct Array<D, N> {
    data: Vec<N>,
    dimension: D,
}

impl<D: Dimension, N: Numeric> Array<D, N> {
    pub fn new(data: Vec<N>, dimension: D) -> Array<D, N> {
        assert_eq!(
            dimension.size(),
            data.len(),
            "mismatch on dimension size and Vec length: {} != {}",
            dimension.size(),
            data.len(),
        );
        Self { data, dimension }
    }

    pub fn from_shape(dimension: D) -> Array<D, N> {
        Self {
            data: vec![N::zero(); dimension.size()],
            dimension,
        }
    }

    pub fn data(&self) -> &[N] {
        &self.data
    }

    pub fn shape(&self) -> &[usize] {
        self.dimension.slice()
    }

    pub fn sum_interior(&self) -> Array<[usize; 1], N> {
        let shape = self.shape();
        if shape.len() == 1 {
            Array::new(self.data.clone(), [shape[0]])
        } else {
            let step: usize = shape.iter().skip(1).product();
            let data = (0..shape[0])
                .map(|i| {
                    let start = i * step;
                    self.data[start..start + step].iter().copied().sum()
                })
                .collect();
            Array::new(data, [shape[0]])
        }
    }
}

impl<D: Dimension, N: Numeric> ops::Index<D> for Array<D, N> {
    type Output = N;

    fn index(&self, index: D) -> &Self::Output {
        &self.data[self.dimension.offset(&index)]
    }
}

impl<D: Dimension, N: Numeric> ops::IndexMut<D> for Array<D, N> {
    fn index_mut(&mut self, index: D) -> &mut Self::Output {
        &mut self.data[self.dimension.offset(&index)]
    }
}

impl<D: Dimension, N: Numeric> fmt::Debug for Array<D, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", &self.data)?;
        write!(f, "shape: {:?}", self.shape())
    }
}

impl<D, N: Numeric> ops::DivAssign<N> for Array<D, N> {
    fn div_assign(&mut self, rhs: N) {
        for value in self.data.iter_mut() {
            *value /= rhs;
        }
    }
}

impl<D: Dimension, N: Numeric> ops::Mul<Array<D, N>> for Array<D, N> {
    type Output = Array<D, N>;

    fn mul(self, rhs: Array<D, N>) -> Self::Output {
        &self * &rhs
    }
}

impl<'a, D: Dimension, N: Numeric> ops::Mul<&'a Array<D, N>> for &'a Array<D, N> {
    type Output = Array<D, N>;

    fn mul(self, rhs: &'a Array<D, N>) -> Self::Output {
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a * b)
            .collect();
        Array::new(data, self.dimension.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divassign() {
        let mut array = Array::new(vec![2., 4., 6., 8., 10., 12.], [3, 2]);
        array /= 2.;
        assert_eq!(array.data(), &vec![1., 2., 3., 4., 5., 6.]);
    }

    #[test]
    fn test_mul() {
        let a = Array::new(vec![1., 2., 3., 4., 5., 6., 7., 8.], [2, 4]);
        let b = Array::new(vec![2., 2., 2., 2., 2., 2., 2., 2.], [2, 4]);
        assert_eq!((a * b).data(), &vec![2., 4., 6., 8., 10., 12., 14., 16.]);
    }

    #[test]
    fn test_sum_interior() {
        let a = Array::new(
            vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 3, 5, 1, 3, 5, 1, 3, 5],
            [2, 3, 3],
        );
        assert_eq!(a.sum_interior().data(), &vec![18, 27]);
    }
}
