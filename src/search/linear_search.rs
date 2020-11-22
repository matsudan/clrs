pub fn linear_search<T: Ord>(slice: &[T], target: &T) -> Result<usize, usize> {
    for (i, e) in slice.iter().enumerate() {
        if e == target {
            return Ok(i)
        }
    }
    Err(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_arr() {
        let mut v1 = vec![];
        let res = linear_search(&mut v1, &5);
        assert_eq!(res, Err(0));
    }

    #[test]
    fn one_element_int() {
        let mut v1 = vec![3];
        let res = linear_search(&mut v1, &3);
        assert_eq!(res, Ok(0));
    }

    #[test]
    fn standard_case_int() {
        let mut v1 = vec![10, 30, 15, 5];
        let res = linear_search(&mut v1, &15);
        assert_eq!(res, Ok(2));
    }

    #[test]
    fn standard_case_str() {
        let mut v1 = vec!["a", "b", "C", "d"];
        let res = linear_search(&mut v1, &"d");
        assert_eq!(res, Ok(3));
    }

    #[test]
    fn not_found_case() {
        let mut v1 = vec![10, 30, 15, 5];
        let res = linear_search(&mut v1, &20);
        assert_eq!(res, Err(0));
    }
}