mod solutions;
use solutions::solution3::test as test3;
use solutions::solution33::test as test33;
use solutions::solution445::test as test445;

fn test(num: i32) {
    match num {
        3 => test3(),
        33 => test33(),
        445 => test445(),
        _ => println!("solution {0} not found", num),
    }
}

fn main() {
    test(33);
}

