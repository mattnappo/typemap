enum A {
    B(B),
    C(C),
}
enum B {}
struct C {
    d: D,
}
union D {
    d1: i32,
    d2: usize,
}
