pub fn part_1(_: &Vec<String>) {
    //                       0   1   2    3   4   5    6   7   8   9  10  11  12  13
    let model_number = vec![ 9,  9,  9,   1,  9,  7,   6,  5,  9,  4,  9,  4,  9,  8];
    check_model_number(&model_number);
}

pub fn part_2(_: &Vec<String>) {
    //                       0   1   2    3   4   5    6   7   8   9  10  11  12  13
    let model_number = vec![ 2,  4,  9,   1,  3,  1,   1,  1,  6,  1,  6,  1,  5,  1];
    check_model_number(&model_number);
}


fn check_model_number(model_number: &Vec<i64>) {
    //                   0   1   2    3   4   5    6   7   8   9  10  11  12  13
    let a_lookup = vec![14, 15, 13, -10, 14, -3, -14, 12, 14, 12, -6, -6, -2, -9];
    let b_lookup = vec![ 8, 11,  2,  11,  1,  5,  10,  6,  1, 11,  9, 14, 11,  2];
    let k_lookup = vec![ 1,  1,  1,  26,  1, 26,  26,  1,  1,  1, 26, 26, 26, 26];
    let mut z = 0;
    for (i, digit) in model_number.iter().enumerate() {
        let a = a_lookup[i];
        let b = b_lookup[i];
        println!("Current: {} {} {} {} {} {}", i, digit, a, b, z, z % 26);
        let cond = (z % 26) + a == *digit;
        z /= k_lookup[i];
        if !cond {
            z = (z * 26) + digit + b;
            println!("Adding layer {} with {}, new z {}", i, digit + b, z);
        }
    }
    println!("Number: {} z: {}", model_number.iter().map(|i| i.to_string()).collect::<String>(), z);
}
