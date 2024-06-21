type A = String;
struct B {
    a: A,
}
type C<T: D> = String;
trait D<T: E + F, U: G> {}
trait E {}
trait F {}
trait G {}
