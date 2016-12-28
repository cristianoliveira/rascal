#[cfg(test)]
mod blocks {
    extern crate rascal;

    #[test]
    fn it_eval_retrieve_return_last_statement() {
        let source =
        "
           var x = 10;
           var y = x + 5;
           return y + 5
         end";

        let result = rascal::eval(String::from(source));
        assert_eq!("20", result);
    }

    #[test]
    fn it_eval_bolean_expressions() {
        let source =
        "begin
           let x = 1;
           let y = 2;
           return y == x
         end";
        let result = rascal::eval(String::from(source));
        assert_eq!("false", result);
    }

    #[test]
    fn it_eval_while_blocks() {
        let source =
        "begin
           var y = 0;
           while y < 4 begin
             y = y + 1
           end;
           return y == 4
        end";
        let result = rascal::eval(String::from(source));
        assert_eq!("true", result);
    }

    #[test]
    fn it_eval_if_blocks() {
        let source =
        "begin
           var y = 0;
           if y < 4 begin
             y = 4
           end;
           return y == 4
         end";
        let result = rascal::eval(String::from(source));
        assert_eq!("true", result);
    }

    #[test]
    fn it_eval_if_else_blocks() {
        let source =
        "begin
           var y = 0;
           if y > 4 begin
             y = 4
           else
             y = 10
           end;
           return y == 10
         end";
        let result = rascal::eval(String::from(source));
        assert_eq!("true", result);
    }

    #[test]
    fn it_validate_immutable_reassign() {
        let source =
        "begin
           let y = 0;
           y = 1;
           return y
         end";
        assert_eq!("Value error: imutable y was reassigned.",
                   rascal::eval(String::from(source)));
    }

    #[test]
    fn it_accepts_mutable() {
        let source =
        "begin
           var y = 0;
           y = 1;
           return y
        end";
        let result = rascal::eval(String::from(source));
        assert_eq!("1", result);
    }

    #[test]
    fn it_validate_not_declared_var() {
        let source =
        "begin
           let y = 0;
           x = 1;
           return x
         end";
        assert_eq!("Value error: variable x used before declared.",
                   rascal::eval(String::from(source)));
    }

    #[test]
    fn it_validates_block_context() {
        let source =
        "begin
           var x = 0;
           begin
             let y = 1
           end;
           return y
         end";
        assert_eq!("Variable y doesn't exist in this context",
                   rascal::eval(String::from(source)));
    }

    #[test]
    fn it_validates_nested_block_context() {
        let source =
        "
           var x = 0;
           begin
             var y = x;
             begin var z = 0 end;
             x = z
           end;
           return x
        ";
        assert_eq!("Variable z doesn't exist in this context",
                   rascal::eval(String::from(source)));
    }

    #[test]
    fn it_has_nested_block_context() {
        let source =
        "
           var x = 0;
           begin
             var x = 10;
             x = 15
           end;
           return x
        ";
        assert_eq!("Value error: variable x has already defined.",
                   rascal::eval(String::from(source)));
    }

    #[test]
    fn it_can_print_expression() {
        let source =
        "begin
           var x = 0;
           print (x)
         end";
        rascal::eval(String::from(source));
    }
}
