fn main() {
    println!("Hello, World!");
}

fn f(x: i64) -> i64 {
    2 * x + 1
}
#[test]
fn prop_x() {
    assert!(f(2) == 5);
}
