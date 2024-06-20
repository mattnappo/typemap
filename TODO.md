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
- [ ] Support traits
    - [ ] Want some sort of enum to distinguish between struct/enum/primitive/trait dependence
    - [ ] Extra trait dependences (same as type aliases)
- [ ] Support generics / trait bounds
    - [ ] Tricky since generics introduce scope. Two A's in diff structs are not the same A
- [ ] Remove self-loops in example 7
- [ ] Add flag for showing builtin/primitive types (ex: Box/u8)
- [ ] Support multi-file projects
- [ ] Support modules
- [ ] For now, I am just using the type name as given by syn. This is really bad.

Questions
- [ ] What about Box<MyStruct> vs MyStruct? They will have different type ids
      but we want this to be recorded as the same dependence.
