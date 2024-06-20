trait A {}
struct B<T: A> {
    t: T,
}
struct C<T>(T);
