# use-life-stage

Primitive life stage and development stage vocabulary.

`use-life-stage` supports descriptive animal, plant, fungal, and microbial stages. It does not model developmental biology processes, infer age, or implement lifecycle simulation.

```rust
use use_life_stage::{DevelopmentStage, LifeStage};

let stage = DevelopmentStage::new(LifeStage::Flowering)
    .with_label("anthesis")
    .unwrap();

assert_eq!(stage.stage(), &LifeStage::Flowering);
assert_eq!(stage.label(), Some("anthesis"));
```
