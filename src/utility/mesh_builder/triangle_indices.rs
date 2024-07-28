#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TriangleIndices(pub u32, pub u32, pub u32);

impl TriangleIndices {
    /// Returns itself with the indices offsetted by the given value
    #[allow(unused)]
    pub fn offsetted(&self, value: u32) -> Self {
        Self(self.0 + value, self.1 + value, self.2 + value)
    }

    /// Returns itself with switched indices, this flips the triangle
    ///
    /// So turns 1 2 3 into 3 2 1
    pub fn reversed(&self) -> Self {
        Self(self.2, self.1, self.0)
    }

    pub const fn to_array(&self) -> [u32; 3] {
        [self.0, self.1, self.2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_indices_to_array() {
        let indices = TriangleIndices(0, 1, 2);

        assert_eq!(indices.to_array(), [0, 1, 2]);
    }

    #[test]
    fn can_offset_indices() {
        let indices = TriangleIndices(0, 1, 2);

        assert_eq!(indices.offsetted(10), TriangleIndices(10, 11, 12));
    }

    #[test]
    fn can_reverse_indices() {
        let indices = TriangleIndices(0, 1, 2);

        assert_eq!(indices.reversed(), TriangleIndices(2, 1, 0));
    }
}
