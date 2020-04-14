pub fn max(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    if arr.len() == 1 {
        return Some(arr[0]);
    }
    let sub_max_option = max(&arr[1..]);
    if let Some(sub_max) = sub_max_option {
        if arr[0] > sub_max {
            return Some(arr[0]);
        }
        return Some(sub_max);
    }
    return None;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_success() {
        assert_eq!(max(&[]), None);
        assert_eq!(max(&[1, 5, 10, 25, 16, 1]), Some(25));
    }

}