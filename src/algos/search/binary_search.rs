fn binary_search_ordered_list(haystack: &Vec<i32>, needle: i32) -> i32 {
    let mut high: i32 = haystack.len() as i32;
    let mut low: i32 = 0;
    loop {
        let mid: i32 = low + ((high - 1) - low)/ 2;
        let current_value: i32 = haystack[mid as usize];
        if needle == current_value {
            return mid;
        }else if needle > current_value {
            low = mid + 1;
        } else {
            high = mid;
        }
        if low >= high {
            return -1;
        }

    }
}

pub fn test_binary_search(){
    let mut test_vec: Vec<i32> = Vec::new();

//     test_vec.push(1);
//     test_vec.push(2);
//     test_vec.push(3);
//     test_vec.push(4);
//     test_vec.push(5);
//     test_vec.push(6);

    test_vec.push(55);
    test_vec.push(56);
    test_vec.push(57);
    test_vec.push(58);
    test_vec.push(59);
    test_vec.push(60);

    println!("{}", binary_search_ordered_list(&test_vec, 58));
    println!("{}", binary_search_ordered_list(&test_vec, 56));
    println!("{}", binary_search_ordered_list(&test_vec, 61));
}
