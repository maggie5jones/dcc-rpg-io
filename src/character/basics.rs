use rand::Rng;
use std::io;

pub fn roll(num_dice: i32, num_sides: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let mut total = 0;

    for _ in 0..num_dice {
        let roll_result = rng.gen_range(1..=num_sides);
        total += roll_result;
    }

    if total == 0 {
        total = roll(num_dice, num_sides);
    }

    total
}

pub fn modifier(score: i32) -> i32 {
    if score <= 3 {
        return -3;
    } else if score <= 5 {
        return -2;
    } else if score <= 8 {
        return -1;
    } else if score <= 12 {
        return 0;
    } else if score <= 15 {
        return 1;
    } else if score <= 17 {
        return 2;
    } else if score <= 19 {
        return 3;
    } else if score <= 21 {
        return 4;
    } else if score <= 23 {
        return 5;
    } else {
        return 6;
    }
}

pub fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read user input");

    input.trim().to_string()
}
