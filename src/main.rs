use std::io;

fn main() {
    println!("Please input two strings with the same size: ");

    let mut s1 = String::new();
    let mut s2 = String::new();

    io::stdin()
        .read_line(&mut s1)
        .expect("Failed to read line");

    io::stdin()
        .read_line(&mut s2)
        .expect("Failed to read line");

    let s1 = s1.trim();
    let s2 = s2.trim();

    if s1.len() != s2.len() {
        println!("The two strings must have the same size!");
        return;
    }

    let result = check_if_can_break(s1, s2);

    println!("Result: {}", result);
}

fn check_if_can_break(s1: &str, s2: &str) -> bool {
    let mut s1_chars: Vec<char> = s1.chars().collect();
    let mut s2_chars: Vec<char> = s2.chars().collect();

    s1_chars.sort();
    s2_chars.sort();

    let mut flag = true;
    for i in 0..s1_chars.len() {
        if s1_chars[i] >= s2_chars[i] {
            continue;
        } else {
            flag = false;
            break;
        }
    }
    if flag == true {
        return true;
    }

    flag = true;
    for i in 0..s2_chars.len() {
        if s2_chars[i] >= s1_chars[i] {
            continue;
        } else {
            flag = false;
            break;
        }
    }
    if flag == true {
        return true;
    }

    return false;
}
