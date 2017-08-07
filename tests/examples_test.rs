#[cfg(test)]
mod examples {
    use std::io::{self};
    use std::io::prelude::*;
    use std::fs::File;
    extern crate rascal;

    fn load_file(path: &'static str) -> String {
        let mut f = File::open(&path).unwrap();
        let mut source_code = String::new();
        let _ = f.read_to_string(&mut source_code);
        source_code
    }

    #[test]
    fn prooject_euler_1() {
        let source = load_file("./examples/projecteuler1.rl");
        assert_eq!("23", rascal::eval(String::from(source)));
    }

    #[test]
    fn first_class_functions_and_closure() {
        let source = load_file("./examples/firstclassfunc.rl");
        assert_eq!("40", rascal::eval(String::from(source)));
    }
}
