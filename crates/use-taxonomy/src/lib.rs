#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, TaxonomyNameError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(TaxonomyNameError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

/// Error returned when taxonomy names are empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaxonomyNameError {
    /// The supplied name was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for TaxonomyNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("taxonomy name cannot be empty"),
        }
    }
}

impl Error for TaxonomyNameError {}

/// A non-empty taxon name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TaxonName(String);

impl TaxonName {
    /// Creates a taxon name from non-empty text.
    ///
    /// Surrounding whitespace is trimmed; the remaining text and casing are preserved.
    ///
    /// # Errors
    ///
    /// Returns [`TaxonomyNameError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TaxonomyNameError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the taxon name text.
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

impl AsRef<str> for TaxonName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for TaxonName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TaxonName {
    type Err = TaxonomyNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty scientific name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScientificName(String);

impl ScientificName {
    /// Creates a scientific name from non-empty text.
    ///
    /// Surrounding whitespace is trimmed; the remaining text and casing are preserved.
    ///
    /// # Errors
    ///
    /// Returns [`TaxonomyNameError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TaxonomyNameError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the scientific name text.
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

impl AsRef<str> for ScientificName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ScientificName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ScientificName {
    type Err = TaxonomyNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty common name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommonName(String);

impl CommonName {
    /// Creates a common name from non-empty text.
    ///
    /// Surrounding whitespace is trimmed; the remaining text and casing are preserved.
    ///
    /// # Errors
    ///
    /// Returns [`TaxonomyNameError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TaxonomyNameError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the common name text.
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

impl AsRef<str> for CommonName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CommonName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CommonName {
    type Err = TaxonomyNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Broad biological rank vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TaxonomicRank {
    /// Domain rank.
    Domain,
    /// Kingdom rank.
    Kingdom,
    /// Phylum rank.
    Phylum,
    /// Class rank.
    Class,
    /// Order rank.
    Order,
    /// Family rank.
    Family,
    /// Genus rank.
    Genus,
    /// Species rank.
    Species,
    /// Subspecies rank.
    Subspecies,
    /// Variety rank.
    Variety,
    /// Strain rank.
    Strain,
    /// Clade label.
    Clade,
    /// Explicitly unranked classification.
    Unranked,
    /// Unknown rank.
    Unknown,
    /// Caller-defined rank text.
    Custom(String),
}

impl fmt::Display for TaxonomicRank {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Domain => formatter.write_str("domain"),
            Self::Kingdom => formatter.write_str("kingdom"),
            Self::Phylum => formatter.write_str("phylum"),
            Self::Class => formatter.write_str("class"),
            Self::Order => formatter.write_str("order"),
            Self::Family => formatter.write_str("family"),
            Self::Genus => formatter.write_str("genus"),
            Self::Species => formatter.write_str("species"),
            Self::Subspecies => formatter.write_str("subspecies"),
            Self::Variety => formatter.write_str("variety"),
            Self::Strain => formatter.write_str("strain"),
            Self::Clade => formatter.write_str("clade"),
            Self::Unranked => formatter.write_str("unranked"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for TaxonomicRank {
    type Err = TaxonomicRankParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(TaxonomicRankParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "domain" => Ok(Self::Domain),
            "kingdom" => Ok(Self::Kingdom),
            "phylum" | "division" => Ok(Self::Phylum),
            "class" => Ok(Self::Class),
            "order" => Ok(Self::Order),
            "family" => Ok(Self::Family),
            "genus" => Ok(Self::Genus),
            "species" => Ok(Self::Species),
            "subspecies" | "sub-species" => Ok(Self::Subspecies),
            "variety" => Ok(Self::Variety),
            "strain" => Ok(Self::Strain),
            "clade" => Ok(Self::Clade),
            "unranked" | "un-ranked" => Ok(Self::Unranked),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing a taxonomic rank fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaxonomicRankParseError {
    /// The rank text was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for TaxonomicRankParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("taxonomic rank cannot be empty"),
        }
    }
}

impl Error for TaxonomicRankParseError {}

/// A taxon represented by rank and validated name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Taxon {
    rank: TaxonomicRank,
    name: TaxonName,
}

impl Taxon {
    /// Creates a taxon from rank and name.
    #[must_use]
    pub const fn new(rank: TaxonomicRank, name: TaxonName) -> Self {
        Self { rank, name }
    }

    /// Returns the rank.
    #[must_use]
    pub const fn rank(&self) -> &TaxonomicRank {
        &self.rank
    }

    /// Returns the name.
    #[must_use]
    pub const fn name(&self) -> &TaxonName {
        &self.name
    }
}

impl fmt::Display for Taxon {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.rank, self.name)
    }
}

/// An ordered taxonomic lineage.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TaxonomicLineage {
    taxa: Vec<Taxon>,
}

impl TaxonomicLineage {
    /// Creates a lineage from caller-supplied taxon order.
    #[must_use]
    pub const fn new(taxa: Vec<Taxon>) -> Self {
        Self { taxa }
    }

    /// Returns taxa in lineage order.
    #[must_use]
    pub fn taxa(&self) -> &[Taxon] {
        &self.taxa
    }

    /// Returns the number of taxa.
    #[must_use]
    pub fn len(&self) -> usize {
        self.taxa.len()
    }

    /// Returns `true` when the lineage has no taxa.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.taxa.is_empty()
    }
}

impl From<Vec<Taxon>> for TaxonomicLineage {
    fn from(taxa: Vec<Taxon>) -> Self {
        Self::new(taxa)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommonName, ScientificName, Taxon, TaxonName, TaxonomicLineage, TaxonomicRank,
        TaxonomicRankParseError, TaxonomyNameError,
    };

    #[test]
    fn constructs_valid_taxon_name() -> Result<(), TaxonomyNameError> {
        let name = TaxonName::new("  Animalia  ")?;

        assert_eq!(name.as_str(), "Animalia");
        assert_eq!(name.to_string(), "Animalia");
        Ok(())
    }

    #[test]
    fn rejects_empty_taxon_name() {
        assert_eq!(TaxonName::new("   "), Err(TaxonomyNameError::Empty));
    }

    #[test]
    fn displays_and_parses_ranks() -> Result<(), TaxonomicRankParseError> {
        assert_eq!(TaxonomicRank::Species.to_string(), "species");
        assert_eq!("kingdom".parse::<TaxonomicRank>()?, TaxonomicRank::Kingdom);
        assert_eq!(
            "sub species".parse::<TaxonomicRank>()?,
            TaxonomicRank::Subspecies
        );
        Ok(())
    }

    #[test]
    fn parses_custom_rank() -> Result<(), TaxonomicRankParseError> {
        assert_eq!(
            "section".parse::<TaxonomicRank>()?,
            TaxonomicRank::Custom("section".to_string())
        );
        assert_eq!(
            "  ".parse::<TaxonomicRank>(),
            Err(TaxonomicRankParseError::Empty)
        );
        Ok(())
    }

    #[test]
    fn lineage_preserves_order() -> Result<(), TaxonomyNameError> {
        let lineage = TaxonomicLineage::new(vec![
            Taxon::new(TaxonomicRank::Kingdom, TaxonName::new("Animalia")?),
            Taxon::new(TaxonomicRank::Phylum, TaxonName::new("Chordata")?),
            Taxon::new(TaxonomicRank::Genus, TaxonName::new("Homo")?),
        ]);

        assert_eq!(lineage.len(), 3);
        assert_eq!(lineage.taxa()[0].name().as_str(), "Animalia");
        assert_eq!(lineage.taxa()[2].rank(), &TaxonomicRank::Genus);
        assert!(!lineage.is_empty());
        Ok(())
    }

    #[test]
    fn constructs_scientific_and_common_names() -> Result<(), TaxonomyNameError> {
        let scientific = ScientificName::new("Homo sapiens")?;
        let common = CommonName::new("human")?;

        assert_eq!(scientific.to_string(), "Homo sapiens");
        assert_eq!(common.to_string(), "human");
        Ok(())
    }
}
