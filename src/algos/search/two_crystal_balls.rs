fn two_crystal_balls(breaks: Vec<bool>) -> usize {
    let length = breaks.len() as f64;
    let jump_amount = length.sqrt().floor() as usize;
    let mut first_ball = false;
    let mut second_ball = false;
    let mut changing_jump_amount = jump_amount;
    loop {

        if breaks[changing_jump_amount] == true && first_ball == false {
            first_ball = true;
            changing_jump_amount -= jump_amount; 
        } else if breaks[changing_jump_amount] == true && first_ball == true && second_ball == false {
            second_ball = true;
        } else if first_ball == true {
            changing_jump_amount += 1;
        } else {
            changing_jump_amount += jump_amount;
            if changing_jump_amount >=  length as usize {
                changing_jump_amount = (length as usize)-1;
            }
        }

        if changing_jump_amount > length as usize || (first_ball == true && second_ball == true) {
            return changing_jump_amount;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algos::search::two_crystal_balls::two_crystal_balls;

    #[test]
    fn crystal_balls() {
        let mut test_vec = Vec::new();
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);
        test_vec.push(false);

        test_vec.push(true);
        test_vec.push(true);
        test_vec.push(true);
        test_vec.push(true);
        test_vec.push(true);
        test_vec.push(true);
        test_vec.push(true);
        test_vec.push(true);
        test_vec.push(true);
        test_vec.push(true);
        test_vec.push(true);

        assert_eq!(two_crystal_balls(test_vec), 35);
    }
}
