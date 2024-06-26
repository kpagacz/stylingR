use unsully::{config::Config, format};

fn log_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

macro_rules! comparison_test {
    ($name: ident, $file_number: literal) => {
        #[test]
        fn $name() {
            log_init();
            let input = include_str!(concat!("test_cases/", $file_number, ".R"));
            let expected = include_str!(concat!("test_cases/", $file_number, ".expected"));
            assert_eq!(
                format(
                    input,
                    Some(Config {
                        indent: 0,
                        line_length: 120
                    })
                )
                .unwrap(),
                expected
            );
        }
    };
    ($name: ident, $file_number: literal, $config: ident) => {
        #[test]
        fn $name() {
            log_init();
            let input = include_str!(concat!("test_cases/", $file_number, ".R"));
            let expected = include_str!(concat!("test_cases/", $file_number, ".expected"));
            assert_eq!(format(input, Some($config())).unwrap(), expected);
        }
    };
}

comparison_test!(adds_a_newline_at_the_end, "001");
comparison_test!(adds_a_newline_at_the_end2, "002");
comparison_test!(simple_bops, "003");

#[test]
fn simple_bops_indents_and_new_lines() {
    log_init();
    let input = include_str!(concat!("./test_cases/003.R"));
    let expected = include_str!(concat!("./test_cases/003-0-line-length.expected"));
    let config = Config::new(0, 0);
    assert_eq!(format(input, Some(config)).unwrap(), expected);

    let config = Config::new(0, 4);
    let input = include_str!(concat!("./test_cases/003.R"));
    let expected = include_str!(concat!("./test_cases/003-3-line-length.expected"));
    assert_eq!(format(input, Some(config)).unwrap(), expected);
}

fn short_line_config() -> Config {
    Config::new(0, 4)
}

fn short_line_plus_indent() -> Config {
    Config {
        indent: 2,
        line_length: 0,
    }
}

comparison_test!(simple_bop_with_parenthesis, "004");
comparison_test!(
    simple_bop_with_parentheses_forced_to_break_line,
    "005",
    short_line_config
);
comparison_test!(
    simple_term_with_parentheses_forced_to_break_line,
    "006",
    short_line_config
);
comparison_test!(
    simple_bop_forced_to_break_and_indent,
    "007",
    short_line_config
);
comparison_test!(range_bop_one_line, "008");
comparison_test!(parenthesized_bop_one_line, "009");
comparison_test!(simple_function_definition, "010");
comparison_test!(function_definition_no_args_one_expression, "011");
comparison_test!(function_definition_no_args_two_expressions, "012");
comparison_test!(function_definition_one_arg_no_body, "013");
comparison_test!(function_definition_tw0_arg_no_body, "014");
comparison_test!(function_definition_one_default_arg_no_body, "015");
comparison_test!(function_definition_three_args_multiline_body, "016");
comparison_test!(simple_conditional, "017");
comparison_test!(conditional_with_one_expression_in_body, "018");
comparison_test!(conditional_with_two_expression_in_body, "019");
comparison_test!(conditional_with_empty_trailing_else, "020");
comparison_test!(conditional_with_one_expr_trailing_else, "021");
comparison_test!(conditional_with_one_expr_and_one_expr_trailing_else, "022");
comparison_test!(conditional_with_if_else, "023");
comparison_test!(conditional_with_if_if_else_and_trailing_else, "024");
comparison_test!(term_with_braces, "025");
comparison_test!(
    conditional_with_if_if_else_and_trailing_else_short_lines,
    "026",
    short_line_config
);
comparison_test!(while_empty_loop, "027");
comparison_test!(while_single_expression_loop, "028");
comparison_test!(while_two_expressions_additional_line_breaks, "029");
comparison_test!(repeat_loop, "030");
comparison_test!(function_call_no_args, "031");
comparison_test!(function_call_one_arg, "032");
comparison_test!(function_call_multiple_args, "033");
comparison_test!(function_call_named_args, "034");
comparison_test!(function_call_multiple_calls, "035");
comparison_test!(empty_subset, "036");
comparison_test!(subset_with_three_args, "037");
comparison_test!(multiple_subset, "038");
comparison_test!(function_call_plus_subset, "039");
comparison_test!(simple_for_loop, "040");
comparison_test!(for_loop_with_multiline_body, "041");
comparison_test!(break_continue, "042");
comparison_test!(lambda_function_test, "043");
comparison_test!(indent_bop, "044", short_line_plus_indent);
comparison_test!(indent_multiline_bop, "045", short_line_plus_indent);
comparison_test!(
    indent_multiline_bop_parenthesized,
    "046",
    short_line_plus_indent
);
comparison_test!(indent_function_def, "047", short_line_plus_indent);
comparison_test!(indent_multiline_term, "048", short_line_plus_indent);
comparison_test!(indent_conditional_no_brace, "049", short_line_plus_indent);
comparison_test!(indent_conditional_with_brace, "050", short_line_plus_indent);
comparison_test!(indent_while_multiline_body, "051", short_line_plus_indent);
comparison_test!(indent_for_loop_complex, "052", short_line_plus_indent);
comparison_test!(
    indent_bop_multiline_many_new_lines,
    "053",
    short_line_plus_indent
);
comparison_test!(longer_example, "054");
