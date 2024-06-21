struct A<'a> {
    a: (B, C),
    b: &'a D,
    c: [E; 3],
    d: fn(usize, isize) -> (bool, f64),
    e: (F),
    f: (F, G),
    g: std::collections::HashMap<String, i32>,
    h: [H],
    i: fn(impl X) -> impl Y,
}
struct B;
struct C;
struct D;
struct E;
struct F;
struct G;
struct H;
trait T {}
trait V {}
trait X {}
trait Y {}
