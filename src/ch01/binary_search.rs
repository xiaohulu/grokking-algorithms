pub fn binary_search<T: PartialEq + PartialOrd>(list: &[T], item: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = &list[mid];
        if guess == item {
            return Some(mid);
        }
        if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    return None;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn binary_search_i32_matched() {
        let list = [1, 2, 3];
        assert_eq!(binary_search(&list, &1), Some(0));
        assert_eq!(binary_search(&list, &2), Some(1));
        assert_eq!(binary_search(&list, &3), Some(2));
        assert_eq!(binary_search(&list, &4), None);
    }

    #[test]
    fn binary_search_str_matched() {
        let list = ["a", "b", "c"];
        assert_eq!(binary_search(&list, &"a"), Some(0));
        assert_eq!(binary_search(&list, &"b"), Some(1));
        assert_eq!(binary_search(&list, &"c"), Some(2));
        assert_eq!(binary_search(&list, &"d"), None);
    }
}
