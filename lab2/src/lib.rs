use std::cmp::PartialOrd;
use std::marker::Copy;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion_sort_test() {
        let mut data: Vec<i32> = vec![5, 6, 3, 9, 2];
        let mut data1: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut data2: Vec<f32> = vec![1.5, 1.2, 2.3, 2.1, 6.3];

        insertion_sort(&mut data);
        insertion_sort(&mut data1);
        insertion_sort(&mut data2);
        assert_eq!(data1, [1, 2, 3, 4, 5]);
        assert_eq!(data, [2, 3, 5, 6, 9]);
        assert_eq!(data2, [1.2, 1.5, 2.1, 2.3, 6.3]);
    }

    #[test]
    fn merge_test() {
        let mut data: Vec<i32> = vec![5, 6, 3, 9, 2];
        let mut data1: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut data2: Vec<f32> = vec![1.5, 2.3, 0.5];
        merge(&mut data, 0, 1, 2);
        merge(&mut data1, 0, 1, 2);
        merge(&mut data2, 0, 1, 2);
        assert_eq!(data, [3, 5, 6, 9, 2]);
        assert_eq!(data1, [1, 2, 3, 4, 5]);
        assert_eq!(data2, [0.5, 1.5, 2.3]);
    }

    #[test]
    fn merge_sort_test() {
        let mut data: Vec<i32> = vec![5, 6, 3, 9, 2];
        let mut data1: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut data2: Vec<f32> = vec![1.5, 1.3, 0.5, 2.5, 3.3];
        let length = data.len() - 1;
        let length1 = data1.len() - 1;
        let length2 = data2.len() - 1;
        merge_sort(&mut data, 0, length);
        merge_sort(&mut data1, 0, length1);
        merge_sort(&mut data2, 0, length2);
        assert_eq!(data, [2, 3, 5, 6, 9]);
        assert_eq!(data1, [1, 2, 3, 4, 5]);
        assert_eq!(data2, [0.5, 1.3, 1.5, 2.5, 3.3]);
    }
}

pub fn insertion_sort<T: PartialOrd + Copy>(unsorted_data: &mut [T]) {
    for j in 1..unsorted_data.len() {
        let data = unsorted_data[j];
        let mut i: i32 = (j - 1) as i32;
        while i >= 0 && unsorted_data[i as usize] > data {
            unsorted_data[(i + 1) as usize] = unsorted_data[i as usize];
            i = i - 1;
        }
        unsorted_data[(i + 1) as usize] = data;
    }
}

// pub fn reverse_insertion_sort<T: PartialOrd + Copy>(unsorted_data: &mut [T]) {
//     for j in 1..unsorted_data.len() {
//         let data = unsorted_data[j];
//         let mut i: i32 = (j - 1) as i32;
//         while i >= 0 && unsorted_data[i as usize] < data {
//             unsorted_data[(i + 1) as usize] = unsorted_data[i as usize];
//             i = i - 1;
//         }
//         unsorted_data[(i + 1) as usize] = data;
//     }
// }

pub fn merge_sort<T: PartialOrd + Copy>(unsorted_data: &mut [T], p: usize, r: usize) {
    if p < r {
        let q = (p + r) / 2;
        merge_sort(unsorted_data, p, q);
        merge_sort(unsorted_data, q + 1, r);
        merge(unsorted_data, p, q, r);
    }
}

pub fn merge<T: PartialOrd + Copy>(unsorted_data: &mut [T], p: usize, q: usize, r: usize) -> () {
    let L1 = unsorted_data[p..q + 1].to_vec();
    let R1 = unsorted_data[q + 1..r + 1].to_vec();
    let (mut i, mut j) = (0, 0);
    for k in p..r + 1 {
        if i >= L1.len() {
            unsorted_data[k] = R1[j];
            j = j + 1;
        } else if j >= R1.len() {
            unsorted_data[k] = L1[i];
            i = i + 1;
        } else {
            if L1[i] <= R1[j] {
                unsorted_data[k] = L1[i];
                i = i + 1;
            } else {
                unsorted_data[k] = R1[j];
                j = j + 1;
            }
        }
    }
    ()
}
