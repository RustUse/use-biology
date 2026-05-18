# use-tissue

Primitive tissue vocabulary.

`use-tissue` supports descriptive animal and plant tissue names and kinds. It is not human-only, and it does not implement histology analysis, medical diagnosis, or tissue behavior simulation.

```rust
use use_tissue::{TissueKind, TissueName};

let name = TissueName::new("xylem").unwrap();
let kind: TissueKind = "vascular".parse().unwrap();

assert_eq!(name.to_string(), "xylem");
assert_eq!(kind.to_string(), "vascular");
```
