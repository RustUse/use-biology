# use-biology

Facade crate for primitive `RustUse` biology vocabulary.

`use-biology` re-exports focused crates for taxonomy, species names, organisms, cells, genes, traits, tissues, organs, biological systems, life stages, and reproduction. It is not a bioinformatics framework, medical library, taxonomy database, ecology simulator, lab information system, disease model, or biological simulation framework.

```rust
use use_biology::prelude::{
    BinomialName, CellKind, GeneSymbol, GenusName, LifeStage, OrganismKind, SpecificEpithet,
    TaxonomicRank, TraitName, TraitValue,
};

let rank = TaxonomicRank::Species;
let species = BinomialName::new(
    GenusName::new("Arabidopsis").unwrap(),
    SpecificEpithet::new("thaliana").unwrap(),
);
let organism_kind = OrganismKind::Plant;
let cell_kind = CellKind::Eukaryotic;
let gene_symbol = GeneSymbol::new("AP1").unwrap();
let trait_value = TraitValue::new(TraitName::new("flower color").unwrap(), "white").unwrap();
let life_stage = LifeStage::Flowering;

assert_eq!(rank.to_string(), "species");
assert_eq!(species.to_string(), "Arabidopsis thaliana");
assert_eq!(organism_kind.to_string(), "plant");
assert_eq!(cell_kind.to_string(), "eukaryotic");
assert_eq!(gene_symbol.to_string(), "AP1");
assert_eq!(trait_value.value(), "white");
assert_eq!(life_stage.to_string(), "flowering");
```

This crate describes biological concepts. It does not analyze sequences, infer taxonomy, diagnose, simulate organisms, or fetch data.
