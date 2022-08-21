use std::fmt::Display;

fn main() {
    // example
    let array = vec![0, 1, 2, 3, -1, -6, -7];
    let sorted_array = merge_sort(&array);
    print_array(&sorted_array);
}

fn merge_sort<T: Ord + Clone + Copy>(array: &Vec<T>) -> Vec<T> {
    if array.len() < 2 {
        return array.to_vec();
    }
    // split the array into 2
    let middle = array.len() / 2;
    let left = merge_sort(&array[0..middle].to_vec());
    let right = merge_sort(&array[middle..].to_vec());
    let merged = merge(&left, &right);

    merged
}

fn merge<T: Ord + Clone + Copy>(left: &Vec<T>, right: &Vec<T>) -> Vec<T> {
    let mut l = 0 as usize;
    let mut r = 0 as usize;

    let mut temp_array: Vec<T> = Vec::new();
    while l < left.len() && r < right.len() {
        if left.get(l) < right.get(r) {
            temp_array.push(left.get(l).unwrap().clone());
            l = l + 1;
        } else {
            temp_array.push(right.get(r).unwrap().clone());
            r = r + 1;
        }
    }
    if l < left.len() {
        while l < left.len() {
            temp_array.push(left[l]);
            l = l + 1;
        }
    }

    if r < right.len() {
        while r < right.len() {
            temp_array.push(right[r]);
            r = r + 1;
        }
    }

    temp_array
}

fn print_array<T: Display>(printable_array: &Vec<T>) {
    print!("[");
    for i in printable_array {
        print!("{}, ", i);
    }
    println!("]");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn array_have_same_length() {
        let array = vec![0, 1, 2, 3, -1, -6, -7];
        let sorted_array = merge_sort(&array);
        assert_eq!(array.len(), sorted_array.len())
    }

    #[test]
    fn array_is_sorted_1() {
        let array = vec![0, 1, 2, 3, -1, -6, -7];
        let sorted_array = merge_sort(&array);
        assert_eq!(sorted_array, vec![-7, -6, -1, 0, 1, 2, 3])
    }

    #[test]
    fn array_is_sorted_2() {
        let array = vec![0, 0, 0, 1, 0];
        let sorted_array = merge_sort(&array);
        assert_eq!(sorted_array, vec![0, 0, 0, 0, 1])
    }
}
