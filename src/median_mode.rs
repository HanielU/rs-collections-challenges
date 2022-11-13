use std::collections::HashMap;
use MedianMode::{Median, Mode};

#[derive(Debug)]
enum MedianMode {
    Median(f32),
    Mode(HashMap<i32, i32>),
}
pub fn median_mode_main() {
    let returned = median_mode(&mut [1, 2, 4, 5, 6, 9, 3, 6, 7, 12]);
    println!("{:?}", returned);
}

fn median_mode(list: &mut [i32]) -> Vec<MedianMode> {
    list.sort();
    let list_len = list.len();
    let remainder = list_len % 2;
    let median = if remainder == 0 {
        let len_in_half = list_len / 2;
        let left_half = list[len_in_half - 1];
        let right_half = list[len_in_half];
        (left_half + right_half) as f32 / 2.0
    } else {
        list[((list_len - 1) / 2)] as f32
    };

    let mut mode: HashMap<i32, i32> = HashMap::new();
    for item in list.iter() {
        let count = mode.entry(item.clone()).or_insert(0);
        *count += 1;
    }

    println!("list = {:?}", list);
    return vec![Median(median), Mode(mode)];
}
