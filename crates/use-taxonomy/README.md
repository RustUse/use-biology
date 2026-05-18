# use-taxonomy

Primitive biological classification vocabulary.

`use-taxonomy` models taxon names, ranks, taxa, scientific/common names, and ordered lineages. It does not maintain an authoritative taxonomy database, fetch taxonomy data, resolve synonyms, infer classification, or model phylogenetic trees.

```rust
use use_taxonomy::{Taxon, TaxonName, TaxonomicLineage, TaxonomicRank};

let kingdom = Taxon::new(TaxonomicRank::Kingdom, TaxonName::new("Animalia").unwrap());
let genus = Taxon::new(TaxonomicRank::Genus, TaxonName::new("Homo").unwrap());
let lineage = TaxonomicLineage::new(vec![kingdom, genus]);

assert_eq!(lineage.len(), 2);
assert_eq!(lineage.taxa()[1].to_string(), "genus: Homo");
```
