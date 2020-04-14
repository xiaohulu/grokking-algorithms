pub fn count(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    return 1 + count(&arr[1..]);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn count_success() {
        assert_eq!(count(&[0, 1, 2, 3, 4, 5]), 6);
    }

}