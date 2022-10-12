mod solutions;

use std::collections::HashMap;

struct Main {
    the_map: HashMap<isize, Box<dyn Fn()>>,
}

impl Main {

    pub fn new() -> Main {
        let mut r = Main {
            the_map: HashMap::with_capacity(100)
        };
        macro_rules! mm {
            ($map: ident, $mod_name: ident, $num: expr) => {
                use solutions::$mod_name::test as $mod_name;
                r.the_map.insert($num, Box::new(&$mod_name));
            };
        }
        mm!(the_map, solution3, 3);
        mm!(the_map, solution4, 4);
        mm!(the_map, solution10, 10);
        mm!(the_map, solution11, 11);
        mm!(the_map, solution15, 15);
        mm!(the_map, solution17, 17);
        mm!(the_map, solution33, 33);
        mm!(the_map, solution215, 215);
        mm!(the_map, solution445, 445);
        mm!(the_map, solution974, 974);
        r
    }

    pub fn test(&self, num: isize) {
        match self.the_map.get(&num) {
            Some(tst) => tst(),
            None => println!("solution {0} not found.", &num)
        }
    }
}

fn main() {
    Main::new().test(10);
}
