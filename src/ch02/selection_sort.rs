fn find_smallest<T: PartialOrd>(arr:  &Vec<T>) -> usize {
    // 存储最小的值
    let mut smallest = &arr[0];
    // 存储最小元素的索引
    let mut smallest_index = 0;
    for (index, value) in arr.iter().enumerate() {
        if value < smallest {
            smallest = value;
            smallest_index = index;
        }
    }
    smallest_index
}

/// 对数组进行排序
pub fn selection_sort<T: PartialOrd + Copy>(arr: &mut Vec<T>) -> Vec<T> {
    let mut new_arr: Vec<T> = Vec::new();

    for _ in 0..arr.len() {
        // 找出数组中最小的元素，并将其加入到新数组中
        let smallest = find_smallest(arr);
        new_arr.push(arr.swap_remove(smallest));
    }

    return new_arr;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn selection_sort_success() {
        let mut arr = vec![5, 3, 6, 2, 10];
        assert_eq!(selection_sort(&mut arr), [2, 3, 5, 6, 10]);
    }
}