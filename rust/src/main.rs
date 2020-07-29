mod solutions;
use solutions::solution3::test as test3;
use solutions::solution4::test as test4;
use solutions::solution10::test as test10;
use solutions::solution11::test as test11;
use solutions::solution15::test as test15;
use solutions::solution17::test as test17;
use solutions::solution33::test as test33;
use solutions::solution215::test as test215;
use solutions::solution445::test as test445;
use solutions::solution542::test as test542;
use solutions::solution974::test as test974;

fn test(num: i32) {
    match num {
        3 => test3(),
        4 => test4(),
        10 => test10(),
        11 => test11(),
        15 => test15(),
        17 => test17(),
        33 => test33(),
        215 => test215(),
        445 => test445(),
        542 => test542(),
        974 => test974(),
        _ => println!("solution {0} not found", num),
    }
}

fn main() {
    test(17);
}
