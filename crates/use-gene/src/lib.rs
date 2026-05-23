#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, GeneValueError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(GeneValueError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned when gene vocabulary values are empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeneValueError {
    /// The supplied value was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for GeneValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("gene value cannot be empty"),
        }
    }
}

impl Error for GeneValueError {}

/// A stable gene identifier string.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GeneId(String);

impl GeneId {
    /// Creates a gene identifier from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`GeneValueError::Empty`] when the trimmed identifier is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, GeneValueError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for GeneId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for GeneId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GeneId {
    type Err = GeneValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty gene symbol that preserves caller casing.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GeneSymbol(String);

impl GeneSymbol {
    /// Creates a gene symbol from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`GeneValueError::Empty`] when the trimmed symbol is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, GeneValueError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the symbol text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the symbol and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for GeneSymbol {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for GeneSymbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GeneSymbol {
    type Err = GeneValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty descriptive gene name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GeneName(String);

impl GeneName {
    /// Creates a gene name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`GeneValueError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, GeneValueError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the gene name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the name and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for GeneName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for GeneName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GeneName {
    type Err = GeneValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty descriptive locus identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Locus(String);

impl Locus {
    /// Creates a locus from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`GeneValueError::Empty`] when the trimmed locus is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, GeneValueError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the locus text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the locus and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Locus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Locus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Locus {
    type Err = GeneValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty descriptive allele identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Allele(String);

impl Allele {
    /// Creates an allele from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`GeneValueError::Empty`] when the trimmed allele is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, GeneValueError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the allele text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the allele and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Allele {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Allele {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Allele {
    type Err = GeneValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// An ordered descriptive genotype value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Genotype {
    alleles: Vec<Allele>,
}

impl Genotype {
    /// Creates a genotype from caller-supplied allele order.
    #[must_use]
    pub const fn new(alleles: Vec<Allele>) -> Self {
        Self { alleles }
    }

    /// Returns alleles in caller-supplied order.
    #[must_use]
    pub fn alleles(&self) -> &[Allele] {
        &self.alleles
    }

    /// Returns the allele count.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.alleles.len()
    }

    /// Returns `true` when no alleles are present.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.alleles.is_empty()
    }
}

impl From<Vec<Allele>> for Genotype {
    fn from(alleles: Vec<Allele>) -> Self {
        Self::new(alleles)
    }
}

impl fmt::Display for Genotype {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, allele) in self.alleles.iter().enumerate() {
            if index > 0 {
                formatter.write_str("/")?;
            }
            write!(formatter, "{allele}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Allele, GeneSymbol, GeneValueError, Genotype, Locus};

    #[test]
    fn constructs_valid_gene_symbol() -> Result<(), GeneValueError> {
        let symbol = GeneSymbol::new("BRCA1")?;

        assert_eq!(symbol.as_str(), "BRCA1");
        assert_eq!(symbol.to_string(), "BRCA1");
        Ok(())
    }

    #[test]
    fn rejects_empty_gene_symbol() {
        assert_eq!(GeneSymbol::new("   "), Err(GeneValueError::Empty));
    }

    #[test]
    fn constructs_valid_locus() -> Result<(), GeneValueError> {
        let locus = Locus::new("17q21.31")?;

        assert_eq!(locus.to_string(), "17q21.31");
        Ok(())
    }

    #[test]
    fn constructs_valid_allele() -> Result<(), GeneValueError> {
        let allele = Allele::new("A")?;

        assert_eq!(allele.as_str(), "A");
        Ok(())
    }

    #[test]
    fn constructs_genotype() -> Result<(), GeneValueError> {
        let genotype = Genotype::new(vec![Allele::new("A")?, Allele::new("a")?]);

        assert_eq!(genotype.len(), 2);
        assert_eq!(genotype.alleles()[0].as_str(), "A");
        assert!(!genotype.is_empty());
        Ok(())
    }

    #[test]
    fn displays_genotype() -> Result<(), GeneValueError> {
        let genotype = Genotype::new(vec![Allele::new("A")?, Allele::new("a")?]);

        assert_eq!(genotype.to_string(), "A/a");
        Ok(())
    }
}
