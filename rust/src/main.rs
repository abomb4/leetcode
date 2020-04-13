mod solutions;
use solutions::solution3::test as test3;
use solutions::solution4::test as test4;

fn test(num: i32) {
    match num {
        3 => test3(),
        4 => test4(),
        _ => println!("solution {0} not found", num),
    }
}

fn main() {
    test(3);
    println!("Hello, world!");
}

