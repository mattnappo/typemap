TODO
====
In order:
- [ ] Support structs
- [ ] Support enums
- [ ] Support unions
- [ ] Support `type` aliases
- [ ] Support traits
- [ ] Support generics / trait bounds
- [ ] Add flag for showing builtin/primitive types (ex: Box/u8)
- [ ] Support multi-file projects
- [ ] Support modules
- [ ] For now, I am just using the type name as given by syn. This is really bad.

Questions
- [ ] What about Box<MyStruct> vs MyStruct? They will have different type ids
      but we want this to be recorded as the same dependence.
