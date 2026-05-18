# use-reproduction

Primitive reproduction vocabulary.

`use-reproduction` stores descriptive reproduction and fertilization terms. It does not implement genetics simulation, population dynamics, fertility modeling, or medical interpretation.

```rust
use use_reproduction::{FertilizationMode, ReproductionMode};

let reproduction: ReproductionMode = "spore formation".parse().unwrap();
let fertilization: FertilizationMode = "external".parse().unwrap();

assert_eq!(reproduction.to_string(), "spore-formation");
assert_eq!(fertilization.to_string(), "external");
```
