pub fn binary_search<T: PartialEq + PartialOrd>(list: &[T], item: &T) -> Result<usize, usize> {
    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = &list[mid];
        if guess == item {
            return Ok(mid);
        }
        if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    return Err(0);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn binary_search_i32_matched() {
        let list = [1, 2, 3];
        assert_eq!(binary_search(&list, &1), Ok(0));
        assert_eq!(binary_search(&list, &2), Ok(1));
        assert_eq!(binary_search(&list, &3), Ok(2));
        assert_eq!(binary_search(&list, &4), Err(0));
    }

    #[test]
    fn binary_search_str_matched() {
        let list = ["a", "b", "c"];
        assert_eq!(binary_search(&list, &"a"), Ok(0));
        assert_eq!(binary_search(&list, &"b"), Ok(1));
        assert_eq!(binary_search(&list, &"c"), Ok(2));
        assert_eq!(binary_search(&list, &"d"), Err(0));
    }
}
