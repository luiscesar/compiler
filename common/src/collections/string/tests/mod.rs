use crate::pointer::Pointer;

#[test]
fn test_common_collections_string_ptr_case1() {
    let text = "uidauiuida".to_string();
    let text_ptr = Pointer::new_pointer(text);
    let text_ptr2 = Pointer::clone(&text_ptr);
    assert_eq!(*text_ptr, *text_ptr2);
    assert_eq!(*text_ptr, "uidauiuida".to_string());
    assert_eq!(*text_ptr2, "uidauiuida".to_string());
}
