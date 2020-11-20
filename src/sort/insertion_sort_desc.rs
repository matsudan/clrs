pub fn insertion_sort_desc(slice: &mut [i32]) {
    for i in 1..slice.len() {
        let mut j = i;
        while j > 0 && slice[j - 1] < slice[j] {
            slice.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut v1 = vec![];
        insertion_sort_desc(&mut v1);
        assert_eq!(v1, []);
    }

    #[test]
    fn one_element() {
        let mut v1 = vec![3];
        insertion_sort_desc(&mut v1);
        assert_eq!(v1, [3]);
    }

    #[test]
    fn standard_case() {
        let mut v1 = vec![10, 30, 15, 5];
        insertion_sort_desc(&mut v1);
        assert_eq!(v1, [30, 15, 10, 5]);
    }
}