TODO
====
In order:
- [X] Support primitives
- [X] Support structs
- [X] Support enums
- [X] Support unions
- [-] Support `type` aliases
    - [ ] Need to make it so that in `type X = String`, X depends on String.
          This will require modification of the `Type` branch in `user_defined_types`
- [X] Support traits
    - [X] Want some sort of enum to distinguish between struct/enum/primitive/trait dependence
    - [N] Extra trait dependences (same as type aliases)
    - [ ] Support `where` clauses
    - [ ] Allow traits to depend on other traits `trait A: B {}`
- [X] Support generics / trait bounds
    - [X] Tricky since generics introduce scope. Two A's in diff structs are not the same A
- [X] Remove self-loops in example 7
- [ ] Improve `base_types` to support `Box<T>`, `Map<K,V>`, etc
- [ ] Add flag for showing builtin/primitive types (ex: Box/u8)
- [ ] Support multi-file projects
- [ ] Support modules
- [ ] For now, I am just using the type name as given by syn, which is not very robust.
