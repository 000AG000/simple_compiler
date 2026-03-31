mod common;
use common::*;

#[test]
fn exec_simple_assignment_and_print() {
    init();

    let input = "let x = 5; print x;";
    let output = run_program(input);

    assert_eq!(output.trim(), "5");
}

#[test]
fn exec_multiple_assignments() {
    init();

    let input = "let x = 1; x = x + 2; print x;";
    let output = run_program(input);

    assert_eq!(output.trim(), "3");
}

#[test]
fn exec_loop_counts_down() {
    init();

    let input = r#"
        let x = 3;
        LOOP x DO
            print x;
            x = x - 1;
        END
    "#;

    let output = run_program(input);

    assert_eq!(output.trim(), "3\n2\n1");
}

#[test]
fn exec_loop_zero_does_nothing() {
    init();

    let input = r#"
        let x = 0;
        LOOP x DO
            print x;
        END
    "#;

    let output = run_program(input);

    assert_eq!(output.trim(), "");
}

#[test]
fn exec_nested_loops() {
    init();

    let input = r#"
        let x = 2;
        let y;
        LOOP x DO
            y = 2
            LOOP y DO
                print y;
                y = y - 1;
            END
            x = x - 1;
        END
    "#;

    let output = run_program(input);

    assert_eq!(output.trim(), "2\n1\n2\n1");
}

#[test]
fn exec_subtraction_saturates() {
    init();

    let input = "let x = 1; x = x - 5; print x;";
    let output = run_program(input);

    assert_eq!(output.trim(), "0"); // document this behavior!
}

#[test]
fn exec_fails_on_undefined_variable() {
    init();

    run_program_expect_error("print x;");
}

#[test]
fn exec_fails_on_redefinition() {
    init();

    run_program_expect_error("let x = 1; let x = 2;");
}
