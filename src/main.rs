use std::io;

fn main() {
    let q = get_line().trim().parse::<i32>().unwrap();
    // println!("closet pow2 to {} is {}", 33, closest_pow2(33));
    for _ in 0..q {
        let index = get_line().trim().parse::<i32>().unwrap();
        // println!("processing {}", index);
        println!("{}", get_val(index));
    }
}

fn get_val(index: i32) -> &'static str {
    if index == 0 {
        return "0"
    }

    let sub_index = index - closest_pow2(index);
    return comp(get_val(sub_index));
}

fn comp(x: &str) -> &str {
    if x == "0" {
        "1"
    } else {
        "0"
    }
}

/// Closest power of 2 that is less than or equal to x
fn closest_pow2(x: i32) -> i32 {
    let exp = (x as f64).log2().floor() as i32;
    2f64.powi(exp) as i32
}

fn get_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
