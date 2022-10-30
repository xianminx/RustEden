use bench_test::fibonacci;

#[test]
fn test_fibonacci() {
    assert_eq!(55, fibonacci(10));
}