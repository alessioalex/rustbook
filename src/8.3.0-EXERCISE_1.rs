use std::collections::HashMap;

/*
    Given a list of integers, use a vector and return the mean (the average value),
    median (when sorted, the value in the middle position), and mode
    (the value that occurs most often; a hash map will be helpful here) of the list.
*/
fn main() {
    let mut numbers = vec![1, 3, 41, 5, 8, 0, 22, 3, 3, 5];
    numbers.sort();
    println!("sorted array: {:?}", numbers);

    let length = numbers.len();

    let median: f64;
    let middle = length / 2;

    if length % 2 == 0 {
        median = (numbers[middle] + numbers[middle - 1]) as f64 / 2.0;
    } else {
        median = numbers[middle] as f64;
    }

    let mut frequency = HashMap::new();
    // mode keeps track of the integer that
    // occurs most often in the array: (integer, occurences)
    let mut mode = (numbers[0], 0);

    // will probably be learning about this in a later chapter ->
    // let mode = counts.iter().max_by_key(|(_k, v)| *v).unwrap();

    // alternative ->
    // let sum: i64 = numbers.iter().sum();
    let mut sum = 0;

    // for (index, value) in numbers.iter().enumerate() {
    for value in &numbers {
        sum += *value;
        let count = frequency.entry(*value).or_insert(0);
        *count += 1;

        let (_, mode_count) = mode;
        if mode_count < *count {
            mode = (*value, *count);
        }
    }

    // need to explicitely cast each value to floating
    // otherwise mean is integer
    let mean = sum as f64 / length as f64;

    println!(
        "sum = {:?}, length = {:?}, mean = {:?}, median = {:?}, mode = {:?}",
        sum, length, mean, median, mode.0
    );
}
