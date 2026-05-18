# use-gene

Primitive gene identity vocabulary.

`use-gene` stores descriptive gene identifiers, symbols, names, loci, alleles, and genotypes. It does not implement sequence alignment, genome annotation, FASTA/FASTQ parsing, variant calling, mutation-effect prediction, or medical interpretation.

```rust
use use_gene::{Allele, GeneSymbol, Genotype, Locus};

let symbol = GeneSymbol::new("BRCA1").unwrap();
let locus = Locus::new("17q21.31").unwrap();
let genotype = Genotype::new(vec![Allele::new("A").unwrap(), Allele::new("a").unwrap()]);

assert_eq!(symbol.to_string(), "BRCA1");
assert_eq!(locus.to_string(), "17q21.31");
assert_eq!(genotype.to_string(), "A/a");
```
