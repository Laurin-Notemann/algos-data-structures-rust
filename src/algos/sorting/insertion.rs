pub fn insertion_sort() -> Vec<u8> {
    let mut test_array: Vec<u8> = vec![9,5,7,3,6,2,1,4,8];
    let length_of_array = test_array.len();
    for i in 1..length_of_array {
        let key = test_array[i];
        let mut j = i;
        while j > 0 && test_array[j-1] > key {
            test_array[j] = test_array[j-1];
            j = j - 1;
        }
        test_array[j] = key;
    }
    println!("{:?}", test_array);
    return test_array;
}
