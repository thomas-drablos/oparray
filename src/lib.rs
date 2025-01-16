use rand::Rng;

pub use array::Array;
pub use dimension::Dimension;
pub use numeric::Numeric;

mod array;
mod dimension;
mod numeric;

pub fn matmul<N: Numeric>(
    a: Array<[usize; 2], N>,
    b: Array<[usize; 2], N>,
) -> Array<[usize; 2], N> {
    assert!(
        a.shape()[1] == b.shape()[0],
        "incompatible matrices for multiplication: {:?} x {:?}",
        a.shape(),
        b.shape(),
    );

    let mut c: Array<_, N> = Array::from_shape([a.shape()[0], b.shape()[1]]);

    for i in 0..a.shape()[0] {
        for j in 0..b.shape()[1] {
            c[[i, j]] = (0..a.shape()[1]).map(|k| a[[i, k]] * b[[k, j]]).sum();
        }
    }
    c
}

pub fn randn<D: Dimension>(shape: D) -> Array<D, f64> {
    let mut rng = rand::thread_rng();
    let data = (0..shape.size())
        .map(|_| rng.sample(rand_distr::StandardNormal))
        .collect();
    Array::new(data, shape)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matmul() {
        let a = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], [5, 2]);
        let b = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], [2, 4]);
        assert_eq!(
            matmul(a, b).data(),
            &vec![11, 14, 17, 20, 23, 30, 37, 44, 35, 46, 57, 68, 47, 62, 77, 92, 59, 78, 97, 116,]
        );
    }
}
