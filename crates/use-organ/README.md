# use-organ

Primitive biological organ vocabulary.

`use-organ` supports descriptive organ names, broad organ kinds, and simple organ-system reference labels for animals, plants, and other organisms where reasonable. It is not human-only, and it does not implement anatomy atlas behavior, medical diagnosis, or physiology modeling.

```rust
use use_organ::{OrganKind, OrganName, OrganSystemRef};

let name = OrganName::new("leaf").unwrap();
let kind: OrganKind = "leaf".parse().unwrap();
let system = OrganSystemRef::new("shoot system").unwrap();

assert_eq!(name.to_string(), "leaf");
assert_eq!(kind, OrganKind::Leaf);
assert_eq!(system.to_string(), "shoot system");
```
