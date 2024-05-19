mod utils;

use utils::{test_code, test_expr};

#[test]
fn addition() {
    test_code(
        "
print(1 + 2)
"
        .trim(),
        "
3
"
        .trim(),
    );
}
