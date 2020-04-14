pub fn quick_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {
        return arr;
    }
    let pivot = arr[0];
    let (less, greater): (Vec<i32>, Vec<i32>) = arr[1..].iter().partition(|&&el| el <= pivot);
    [quick_sort(less), quick_sort(greater)].join(&pivot)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn quick_sort_success() {
        assert_eq!(quick_sort(vec![10, 5, 2, 3]), [2, 3, 5, 10]);
    }
}
