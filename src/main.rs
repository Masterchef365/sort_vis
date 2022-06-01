use std::collections::HashSet;

mod sort;

fn main() {
    let width = 100;
    let height = 50;

    let set: HashSet<i32> = //.
        (0..width)
        .map(|x| x * height / width)
        .collect();

    let mut array: Vec<i32> = set.into_iter().collect();

    dbg!(&array);

    let mut frames = vec![array.clone()];

    let record_frame = |array: &[i32]| {
        if array != frames.last().unwrap() {
            frames.push(array.to_vec());
        }
    };

    sort::quicksort(&mut array, |a, b| a < b, record_frame);

    dbg!(frames.len());
}
