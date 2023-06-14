fn quick_sort (input: &mut Vec<i32>, lo: usize, hi: usize) {
    let mut pivot_index = ((hi as f64/2.0).floor()+(lo as f64/2.0).floor()).floor() as i32;

    if pivot_index == input.len() as i32 {
        pivot_index = input.len() as i32 - 1;
    }

    let pivot = input[pivot_index as usize];

    let mut current_index: i32 = lo as i32 - 1;
    for i in lo..hi {
        if input[i] <= pivot {
            current_index += 1;
            let temp_element = input[current_index as usize];
            input[current_index as usize] = input[i];
            input[i] = temp_element;
            if input[current_index as usize] == pivot {
                pivot_index = current_index;
            }
        } 
    }
    if current_index <= lo as i32 {
    } else {
        input[pivot_index as usize] = input[current_index as usize];
        input[current_index as usize] = pivot;
        pivot_index = current_index;
    }
    if hi - lo <= 1{
        return;
    }
    quick_sort(input, lo, pivot_index as usize);
    
    quick_sort(input, pivot_index as usize+1, hi);

    return;
}

pub fn test_quick() {
    let mut test_array: Vec<i32> = vec![9,4,69,420,42,87,345, 512, 1, 6,98,346];
    let length = test_array.len();
    quick_sort(&mut test_array, 0, length);

    println!("{:?}", test_array);
}


#[cfg(test)]
mod test {
    use super::quick_sort;

    #[test]
    fn test_quick_sort() {
        let mut test_array: Vec<i32> = vec![8,4,1,3,2,5,7,6,9];
        let length = test_array.len();
        let sorted_array = vec![1,2,3,4,5,6,7,8,9];
        quick_sort(&mut test_array, 0, length);

        assert_eq!(test_array, sorted_array);

        let mut test_array: Vec<i32> = vec![9,3,7,4,69,420,42,87,345, 512, 1, 6,98,346];
        let sorted_array: Vec<i32> = vec![1,3,4,6,7,9,42,69,87,98,345,346,420, 512];
        let length = test_array.len();
        quick_sort(&mut test_array, 0, length);

        assert_eq!(test_array, sorted_array);

        let mut test_array: Vec<i32> = vec![9,8,7,6,5,4,3,2,1];
        let sorted_array: Vec<i32> = vec![1,2,3,4,5,6,7,8,9];
        let length = test_array.len();
        quick_sort(&mut test_array, 0, length);

        assert_eq!(test_array, sorted_array);

        let mut test_array: Vec<i32> = vec![1,1,6,6,7,5,4,2,9,8,8,3];
        let sorted_array: Vec<i32> = vec![1,1,2,3,4,5,6,6,7,8,8,9];
        let length = test_array.len();
        quick_sort(&mut test_array, 0, length);

        assert_eq!(test_array, sorted_array);
    }
}
