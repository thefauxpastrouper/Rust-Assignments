use ex064_function_like_macros::make_answer;

make_answer!();

#[test]
fn test() {
    assert_eq!(answer(),42);
}
