mod solutions;
use solutions::solution3::test as test3;

fn test(num: i32) {
    match num {
        3 => test3(),
        _ => println!("solution {0} not found", num),
    }
}

fn main() {
    test(3);
}

