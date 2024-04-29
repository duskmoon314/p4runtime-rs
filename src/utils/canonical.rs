pub trait Canonical {
    fn canonical(self) -> Self;
}

impl Canonical for Vec<u8> {
    fn canonical(self) -> Self {
        // remove redundant leading zeros
        // If all bytes are zero, return only one zero
        if self.is_empty() {
            return self;
        }
        let mut i = 0;
        while i < self.len() - 1 && self[i] == 0 {
            i += 1;
        }
        self[i..].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical() {
        assert_eq!(vec![0, 0, 0, 0].canonical(), vec![0]);
        assert_eq!(vec![0, 0, 0, 1].canonical(), vec![1]);
        assert_eq!(vec![0, 0, 1, 0].canonical(), vec![1, 0]);
        assert_eq!(vec![0, 1, 0, 0].canonical(), vec![1, 0, 0]);
    }
}
