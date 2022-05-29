#[cfg(test)]
mod interpreter_tests {
    use interpreter::errors::InterpreterError;
    use interpreter::instructions::ByteCode::*;
    use interpreter::Interpreter;

    #[test]
    fn return_empty_stack() {
        let mut interpreter = Interpreter::new();
        let rslt = interpreter.run(vec![Return]).unwrap_err();

        assert_eq!(rslt, InterpreterError::StackUnderflow);
    }

    #[test]
    fn load_return_value() {
        let mut interpreter = Interpreter::new();
        let rslt = interpreter.run(vec![LoadVal(4), Return]).unwrap();

        assert_eq!(rslt, 4);
    }

    #[test]
    fn load_two_vals_addition() {
        let val1 = 2;
        let val2 = 4;

        let mut interpreter = Interpreter::new();
        let rslt = interpreter
            .run(vec![LoadVal(val1), LoadVal(val2), Add, Return])
            .unwrap();

        assert_eq!(rslt, val1 + val2);
    }

    #[test]
    fn load_two_vals_substraction() {
        let val1 = 2;
        let val2 = 4;

        let mut interpreter = Interpreter::new();
        let rslt = interpreter
            .run(vec![LoadVal(val1), LoadVal(val2), Sub, Return])
            .unwrap();

        assert_eq!(rslt, val2 - val1);
    }

    #[test]
    fn load_two_vals_multiplication() {
        let val1 = 2;
        let val2 = 4;

        let mut interpreter = Interpreter::new();
        let rslt = interpreter
            .run(vec![LoadVal(val1), LoadVal(val2), Mul, Return])
            .unwrap();

        assert_eq!(rslt, val1 * val2);
    }

    #[test]
    fn load_two_vals_division() {
        let val1 = 2;
        let val2 = 4;

        let mut interpreter = Interpreter::new();
        let rslt = interpreter
            .run(vec![LoadVal(val1), LoadVal(val2), Div, Return])
            .unwrap();

        assert_eq!(rslt, val2 / val1);
    }

    #[test]
    fn unknown_var() {
        let mut interpreter = Interpreter::new();
        let rslt = interpreter.run(vec![ReadVar('k'), Return]).unwrap_err();

        assert_eq!(rslt, InterpreterError::UnknownVariable { identifier: 'k' });
    }

    #[test]
    /// test loop: increment a number until reaches target value. Simple impl, does not work if nb_iter = 0
    fn test_increment_loop() {
        /*
            // Sample rust code would look like the following

            let mut v = 2;
            let nb_iter = 4;
            for i in 0..nb_iter {
                v += 1;
            }
            return v;

        */
        let initial_value = 2;
        let nb_iter = 4;

        let mut interpreter = Interpreter::new();
        let rslt = interpreter
            .run(vec![
                // initial setup: create variables
                LoadVal(initial_value),
                WriteVar('v'),
                LoadVal(nb_iter),
                WriteVar('n'),
                LoadVal(0),
                WriteVar('i'),
                // perform check to handle special case where nb_iter = 0
                ReadVar('n'),
                LoadVal(0),
                Eq,
                JumpI(23), // jump to readvar(v) before return
                // now incrementing both v and the counter i
                ReadVar('v'),
                LoadVal(1),
                Add,
                WriteVar('v'),
                ReadVar('i'),
                LoadVal(1),
                Add,
                WriteVar('i'),
                // check if i < n, ie true if we still have iterations to do
                ReadVar('n'),
                ReadVar('i'),
                Lower,
                // and jump back to increment instructions if still iterations needed
                JumpI(10), // loop instructions start at first ReadVar('v') which is instruction at index 10 of bytecode
                ReadVar('v'), // ensure v is at top of stack
                Return,
            ])
            .unwrap();

        assert_eq!(rslt, initial_value + nb_iter);
    }
}
