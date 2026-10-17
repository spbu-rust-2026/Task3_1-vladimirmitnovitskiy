pub fn mirror_double(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().rev().map(|&x| x * 2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(mirror_double(&[]), vec![]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(mirror_double(&[5]), vec![10]);
        assert_eq!(mirror_double(&[-3]), vec![-6]);
    }

    #[test]
    fn test_multiple_elements() {
        assert_eq!(mirror_double(&[1, 2, 3]), vec![6, 4, 2]);
        assert_eq!(mirror_double(&[0, -1, 4, 2]), vec![4, 8, -2, 0]);
    }
}
