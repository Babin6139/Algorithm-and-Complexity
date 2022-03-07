use std::cmp::PartialOrd;
use std::marker::Copy;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_linear() {
        let value = vec![5, 9, 2, 3, 7, 15];
        assert_eq!(linear_search(&value, 5), 0);
        assert_eq!(linear_search(&value, 2), 2);
        assert_eq!(linear_search(&value, 10), -1);
        assert_eq!(linear_search(&value, 15), 5);
    }
    #[test]
    fn search_binary() {
        let value = vec![1, 3, 5, 8, 10, 15];
        assert_eq!(binary_search(&value, 3), 1);
        assert_eq!(binary_search(&value, 1), 0);
        assert_eq!(binary_search(&value, 15), 5);
        assert_eq!(binary_search(&value, 13), -1);
        assert_eq!(binary_search(&value, 10), 4);
    }
}

pub fn linear_search<T: PartialOrd + Copy>(value: &[T], target: T) -> i32 {
    for (index, &data) in value.iter().enumerate() {
        if data == target {
            return index as i32;
        }
    }
    -1
}

pub fn binary_search<T: PartialOrd>(value: &[T], target: T) -> i32 {
    let mut low: i32 = 0;
    let mut high: i32 = value.len() as i32 - 1;
    while high >= low {
        let middle: i32 = (high + low) / 2;
        if value[middle as usize] == target {
            return middle;
        } else if value[middle as usize] > target {
            high = middle - 1;
        } else if value[middle as usize] < target {
            low = middle + 1;
        }
    }
    -1
}
