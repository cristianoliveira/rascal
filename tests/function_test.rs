#[cfg(test)]
mod functions {
    extern crate rascal;

    #[test]
    fn it_eval_function_without_params() {
        let source =
        "
           let add = fn [] { 2 + 2 };
           add()
         ";

        let result = rascal::eval(String::from(source));
        assert_eq!("4", result);
    }

    #[test]
    fn it_eval_function_with_params() {
        let source =
        "
           let add = fn [x] { x + 2 };
           add(2)
         ";

        let result = rascal::eval(String::from(source));
        assert_eq!("4", result);
    }

    #[test]
    fn it_eval_function_with_multiple_params() {
        let source =
        "
           let add = fn [x,y,z] { x + y + z + 1 };
           add(2,2,2)
         end";

        let result = rascal::eval(String::from(source));
        assert_eq!("7", result);
    }

    #[test]
    fn it_eval_complex_function() {
        let source =
        "
           var y = 0;

           let foo = fn [x] {
             if x < 10 {
                y = x + 1
             };

             y
           };

           foo(6)
        ";

        let result = rascal::eval(String::from(source));
        assert_eq!("7", result);
    }

    #[test]
    fn it_eval_recursive_functions() {
        let source =
        "
           let foo = fn [x] {
             if x < 10 {
                x = x + 1;
                foo(x)
             };

             x
           };

           foo(6)
        ";

        let result = rascal::eval(String::from(source));
        assert_eq!("10", result);
    }

    #[test]
    fn it_eval_expressions_as_args() {
        let source =
        "
           let foo = fn [x, y] {
             return x + y
           };

           foo((8 + 1), (1 + 1))
        ";

        let result = rascal::eval(String::from(source));
        assert_eq!("11", result);
    }

    #[test]
    fn it_eval_high_order_functions() {
        let source =
        "
           let foo = fn [x] { x + 1 };
           let other = foo;
           other(6)
        ";

        let result = rascal::eval(String::from(source));
        assert_eq!("7", result);
    }

    #[test]
    fn it_accepts_function_as_params() {
        let source =
        "
           let composed = fn [f] { f(10) };
           let foo = fn [x] { x + 1 };
           composed(foo)
        ";

        let result = rascal::eval(String::from(source));
        assert_eq!("11", result);
    }
}
