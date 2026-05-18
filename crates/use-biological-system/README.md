# use-biological-system

Primitive biological system vocabulary.

`use-biological-system` models descriptive animal and plant biological systems without using the generic crate name `use-system`. It does not implement physiology simulation, disease modeling, or human-only system assumptions.

```rust
use use_biological_system::{BiologicalSystem, BiologicalSystemKind};

let system = BiologicalSystem::new("root system", BiologicalSystemKind::RootSystem).unwrap();

assert_eq!(system.name(), "root system");
assert_eq!(system.kind(), &BiologicalSystemKind::RootSystem);
```
