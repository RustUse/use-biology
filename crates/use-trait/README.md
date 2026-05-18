# use-trait

Primitive biological trait and phenotype vocabulary.

`use-trait` stores descriptive trait names, values, broad trait kinds, and ordered phenotype records. It does not infer heredity, model dominance/recessiveness, implement genetics calculators, or predict traits.

```rust
use use_trait::{Phenotype, TraitKind, TraitName, TraitValue};

let value = TraitValue::new(TraitName::new("leaf shape").unwrap(), "lobed")
    .unwrap()
    .with_kind(TraitKind::Morphological);
let phenotype = Phenotype::new(vec![value]);

assert_eq!(phenotype.len(), 1);
assert_eq!(phenotype.traits()[0].value(), "lobed");
```
