# use-biological-cell

Primitive cell vocabulary.

`use-biological-cell` models descriptive cell kinds, organelles, cell types, and cell structures. It does not model cell dynamics, simulate metabolism, model molecular biology pathways, or implement microscopy/image analysis.

```rust
use use_cell::{CellKind, CellOrganelle, CellStructure, CellType};

let cell_type = CellType::new("guard cell", CellKind::Plant).unwrap();
let structure = CellStructure::from(CellOrganelle::Chloroplast);

assert_eq!(cell_type.name(), "guard cell");
assert_eq!(structure.to_string(), "chloroplast");
```
