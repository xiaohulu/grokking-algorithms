pub fn sum(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    return arr[0] + sum(&arr[1..]);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sum_success() {
        assert_eq!(sum(&[1, 2, 3, 4]), 10);
    }

}