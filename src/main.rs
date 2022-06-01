mod sort;

fn main() {
    let mut array = [1, 4, 8, 9, 10, 34802];
    sort::quicksort(&mut array, |a, b| a < b);
    dbg!(array);
}
