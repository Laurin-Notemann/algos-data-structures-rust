pub fn bubbleee_sort(arr: &mut Vec<isize>) {
    for i in 0..arr.len(){
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j+1] {
                let smaller_element = arr[j+1];
                arr[j+1] = arr[j];
                arr[j] = smaller_element;
            }
        }
    }

}

#[cfg(test)]
mod test {
    use crate::algos::sorting::bubble_sort::bubbleee_sort;

    #[test]
    pub fn bubble_sort() {
        let mut test_vec: Vec<isize> = vec![1,5,3,7,2,8,4];
        let sorted_vec: Vec<isize> = vec![1,2,3,4,5,7,8];
        bubbleee_sort(&mut test_vec);
        assert_eq!(sorted_vec, test_vec);

        let mut test_vec: Vec<isize> = vec![9,8,7,6,5,4,3,2,1];
        let sorted_vec: Vec<isize> = vec![1,2,3,4,5,6,7,8,9];
        bubbleee_sort(&mut test_vec);
        assert_eq!(sorted_vec, test_vec);
    }
}

