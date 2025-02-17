use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let mut count = 0;
    let char_arr = input_str.split(",").collect::<Vec<&str>>();
    let mut set = HashSet::new();
    for char in char_arr {
        if !set.contains(char) {
            set.insert(char);
            count += 1;
        }
    }

    count
}
