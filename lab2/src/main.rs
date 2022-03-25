use lab2::{insertion_sort, merge_sort};
use std::time::Instant;

fn main() {
    let mut datas = vec![5, 6, 3, 2, 9];
    let duration = Instant::now();
    insertion_sort(&mut datas);
    let mut datas = vec![5, 6, 3, 2, 9, 1];
    let length = datas.len();
    merge_sort(&mut datas, 0, length - 1);
}
