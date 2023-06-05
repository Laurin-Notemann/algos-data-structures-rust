fn linear_search(haystack: &Vec<i32>, needle: i32) -> bool {
    for i in 0..haystack.len() {
        if haystack [i] == needle {
            return true;
        }
    }
    return false;
}

pub fn test_linear_search(){
    let mut test_vec: Vec<i32> = Vec::new();

    test_vec.push(1);
    test_vec.push(5);
    test_vec.push(2);
    test_vec.push(3);
    test_vec.push(6);
    test_vec.push(4);

    println!("{}", linear_search(&test_vec, 6));
    println!("{}", linear_search(&test_vec, 3));
    println!("{}", linear_search(&test_vec, 7));
}
