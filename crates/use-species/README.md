# use-species

Primitive species naming vocabulary.

`use-species` stores descriptive binomial, trinomial, and custom species names. It does not validate against taxonomy databases, resolve synonyms, fetch metadata, infer rank, or infer lineage.

```rust
use use_species::{BinomialName, GenusName, SpecificEpithet};

let name = BinomialName::new(
    GenusName::new("Escherichia").unwrap(),
    SpecificEpithet::new("coli").unwrap(),
);

assert_eq!(name.to_string(), "Escherichia coli");
```
