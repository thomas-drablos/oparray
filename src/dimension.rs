pub trait Dimension: Clone {
    fn offset(&self, other: &Self) -> usize {
        let sslice = self.slice();
        let oslice = other.slice();

        let mut offset: usize = (0..oslice.len() - 1)
            .map(|i| oslice[i] * (i + 1..sslice.len()).map(|j| sslice[j]).product::<usize>())
            .sum();
        offset += oslice[oslice.len() - 1];

        offset
    }

    fn slice(&self) -> &[usize];
    fn size(&self) -> usize;
}

impl Dimension for [usize; 1] {
    fn slice(&self) -> &[usize] {
        self
    }

    fn size(&self) -> usize {
        self[0]
    }
}

impl Dimension for [usize; 2] {
    fn slice(&self) -> &[usize] {
        self
    }

    fn size(&self) -> usize {
        self[0] * self[1]
    }
}

impl Dimension for [usize; 3] {
    fn slice(&self) -> &[usize] {
        self
    }

    fn size(&self) -> usize {
        self[0] * self[1] * self[2]
    }
}
