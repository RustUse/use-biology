# use-organism

Primitive organism identity and classification vocabulary.

`use-organism` models organism identifiers, names, broad kinds, and descriptive links to taxonomy or species primitives. The `Virus` kind is descriptive and does not assert whether viruses are living organisms. This crate does not create organism databases, infer taxonomy, model ecology, or model medical/pathogen behavior.

```rust
use use_organism::{OrganismClassification, OrganismKind};

let classification = OrganismClassification::new(OrganismKind::Virus);

assert_eq!(classification.kind(), &OrganismKind::Virus);
assert!(classification.taxon().is_none());
```
