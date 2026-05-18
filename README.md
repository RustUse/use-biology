# use-biology

RustUse is “Composable sets of primitive Rust utility crates for fellow crustaceans.”

`use-biology` is a primitive biology vocabulary set. It provides small, composable Rust primitives for describing organisms, taxonomy, species names, cells, genes, traits, tissues, organs, biological systems, life stages, and reproduction.

`use-biology` is not a bioinformatics framework, genome analysis engine, medical diagnosis library, ecology simulator, taxonomy database, lab information system, disease model, or biological simulation framework. It does not fetch biological data, infer classifications, resolve species, analyze sequences, simulate organisms, diagnose conditions, or manage lab workflows.

## Boundary With use-chemistry

`use-chemistry` owns atoms, molecules, compounds, reactions, and chemical primitives. `use-biology` complements `use-chemistry` by modeling biological organization rather than chemical structure.

A future `use-bioinformatics` set may own sequence analysis, alignment, motifs, annotations, and genomic data-processing primitives. Those concerns are intentionally outside this crate set.

## Crates

- `use-biology`: facade crate for the full primitive vocabulary set
- `use-taxonomy`: primitive biological classification vocabulary
- `use-species`: primitive species naming vocabulary
- `use-organism`: organism identity and classification vocabulary
- `use-cell`: primitive cell vocabulary
- `use-gene`: primitive gene identity vocabulary
- `use-trait`: biological trait and phenotype vocabulary
- `use-tissue`: primitive tissue vocabulary
- `use-organ`: primitive biological organ vocabulary
- `use-biological-system`: biological system vocabulary
- `use-life-stage`: life stage and development stage vocabulary
- `use-reproduction`: reproduction vocabulary

## Scope

- small focused crates
- primitives over frameworks
- few or no dependencies
- stable APIs
- Rust 2024 edition
- strong documentation
- meaningful tests
- composable exports
- dual MIT OR Apache-2.0 license
- no unnecessary macros
- no async
- no global runtime assumptions
- no unsafe code
- no competing with bioinformatics frameworks, medical libraries, taxonomy databases, or simulation systems

## Example

```rust
use use_biology::prelude::{
    BinomialName, CellKind, GeneSymbol, GenusName, LifeStage, OrganismKind, SpecificEpithet,
    TaxonomicRank, TraitKind, TraitName, TraitValue,
};

let rank = TaxonomicRank::Species;
let species = BinomialName::new(
    GenusName::new("Homo").unwrap(),
    SpecificEpithet::new("sapiens").unwrap(),
);
let organism_kind = OrganismKind::Animal;
let cell_kind = CellKind::Eukaryotic;
let gene_symbol = GeneSymbol::new("BRCA1").unwrap();
let trait_name = TraitName::new("eye color").unwrap();
let trait_value = TraitValue::new(trait_name, "brown").unwrap().with_kind(TraitKind::Morphological);
let life_stage = LifeStage::Adult;

assert_eq!(rank.to_string(), "species");
assert_eq!(species.to_string(), "Homo sapiens");
assert_eq!(organism_kind.to_string(), "animal");
assert_eq!(cell_kind.to_string(), "eukaryotic");
assert_eq!(gene_symbol.to_string(), "BRCA1");
assert_eq!(trait_value.value(), "brown");
assert_eq!(life_stage.to_string(), "adult");
```

This example describes biological concepts. It does not analyze, infer, simulate, diagnose, fetch, align, annotate, resolve, or call external services.

## Related Sets

- `use-chemistry`
- `use-data`
- `use-validate`
- `use-measure`
- `use-units`
- `use-ecology`
- `use-bioinformatics`

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
