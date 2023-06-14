fn quick_sort (input: &mut Vec<i32>, lo: usize, hi: usize) {

    let mut pivot_index = ((hi as f64/2.0)+(lo as f64/2.0)).floor() as usize;
    let pivot = input[pivot_index];
    let mut current_index = lo;
    for i in lo..hi {
        if input[i] <= pivot {
            let temp_element = input[current_index];
            input[current_index] = input[i];
            input[i] = temp_element;
            if input[current_index] == pivot {
                pivot_index = current_index;
            }
            current_index += 1;
        } 
    }

    if hi - lo <= 1{
        return;
    }
    quick_sort(input, lo, pivot_index);
    
    quick_sort(input, pivot_index+1, hi);

    return;
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
    }
}
