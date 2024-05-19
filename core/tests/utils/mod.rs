use sciscript::run_code;

pub fn test_expr(expr: &str, expected: &str) {
    let code = format!("print({})", expr);
    test_code(&code, expected);
}

pub fn test_code(code: &str, expected: &str) {
    let result = run_code(code);
    assert_eq!(result, expected);
}
