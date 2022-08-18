use super::{label, new_label};

#[test]
fn test_parser_stmt_new_label_case1() {
    let expected = label();
    let l = new_label();
    assert_eq!(l, expected);

    let expected = label();
    let l = new_label();
    assert_eq!(l, expected);
}
