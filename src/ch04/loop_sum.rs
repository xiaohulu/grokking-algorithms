pub fn sum(arr: &[i32]) -> i32 {
    let mut total = 0;
    for value in arr.iter() {
        total += value;
    }
    total
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sum_success() {
        assert_eq!(sum(&[1, 2, 3, 4]), 10);
    }

}