#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_species::SpeciesName;
use use_taxonomy::Taxon;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, OrganismNameError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(OrganismNameError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

/// Error returned when organism labels are empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrganismNameError {
    /// The supplied value was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for OrganismNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("organism label cannot be empty"),
        }
    }
}

impl Error for OrganismNameError {}

/// A stable organism identifier string.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OrganismId(String);

impl OrganismId {
    /// Creates an organism identifier from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`OrganismNameError::Empty`] when the trimmed identifier is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, OrganismNameError> {
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

impl AsRef<str> for OrganismId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for OrganismId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for OrganismId {
    type Err = OrganismNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty organism name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OrganismName(String);

impl OrganismName {
    /// Creates an organism name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`OrganismNameError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, OrganismNameError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the organism name text.
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

impl AsRef<str> for OrganismName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for OrganismName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for OrganismName {
    type Err = OrganismNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Broad organism vocabulary. The `Virus` variant is descriptive and does not assert whether viruses are living organisms.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrganismKind {
    /// Animal organisms.
    Animal,
    /// Plant organisms.
    Plant,
    /// Fungi.
    Fungus,
    /// Bacteria.
    Bacterium,
    /// Archaea.
    Archaeon,
    /// Protists.
    Protist,
    /// Viruses, modeled descriptively.
    Virus,
    /// Unknown organism kind.
    Unknown,
    /// Caller-defined organism kind text.
    Custom(String),
}

impl fmt::Display for OrganismKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Animal => formatter.write_str("animal"),
            Self::Plant => formatter.write_str("plant"),
            Self::Fungus => formatter.write_str("fungus"),
            Self::Bacterium => formatter.write_str("bacterium"),
            Self::Archaeon => formatter.write_str("archaeon"),
            Self::Protist => formatter.write_str("protist"),
            Self::Virus => formatter.write_str("virus"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for OrganismKind {
    type Err = OrganismKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(OrganismKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "animal" | "animals" => Ok(Self::Animal),
            "plant" | "plants" => Ok(Self::Plant),
            "fungus" | "fungi" => Ok(Self::Fungus),
            "bacterium" | "bacteria" => Ok(Self::Bacterium),
            "archaeon" | "archaea" => Ok(Self::Archaeon),
            "protist" | "protists" => Ok(Self::Protist),
            "virus" | "viruses" => Ok(Self::Virus),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing organism kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrganismKindParseError {
    /// The organism kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for OrganismKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("organism kind cannot be empty"),
        }
    }
}

impl Error for OrganismKindParseError {}

/// A descriptive organism classification record.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OrganismClassification {
    kind: OrganismKind,
    taxon: Option<Taxon>,
    species: Option<SpeciesName>,
}

impl OrganismClassification {
    /// Creates a classification from a broad organism kind.
    #[must_use]
    pub const fn new(kind: OrganismKind) -> Self {
        Self {
            kind,
            taxon: None,
            species: None,
        }
    }

    /// Returns the broad organism kind.
    #[must_use]
    pub const fn kind(&self) -> &OrganismKind {
        &self.kind
    }

    /// Returns the optional taxonomy primitive.
    #[must_use]
    pub const fn taxon(&self) -> Option<&Taxon> {
        self.taxon.as_ref()
    }

    /// Returns the optional species primitive.
    #[must_use]
    pub const fn species(&self) -> Option<&SpeciesName> {
        self.species.as_ref()
    }

    /// Adds a taxonomy primitive.
    #[must_use]
    pub fn with_taxon(mut self, taxon: Taxon) -> Self {
        self.taxon = Some(taxon);
        self
    }

    /// Adds a species primitive.
    #[must_use]
    pub fn with_species(mut self, species: SpeciesName) -> Self {
        self.species = Some(species);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{
        OrganismClassification, OrganismKind, OrganismKindParseError, OrganismName,
        OrganismNameError,
    };
    use use_species::{BinomialName, GenusName, SpeciesName, SpecificEpithet};
    use use_taxonomy::{Taxon, TaxonName, TaxonomicRank};

    #[test]
    fn constructs_valid_organism_name() -> Result<(), OrganismNameError> {
        let name = OrganismName::new("Arabidopsis")?;

        assert_eq!(name.as_str(), "Arabidopsis");
        assert_eq!(name.to_string(), "Arabidopsis");
        Ok(())
    }

    #[test]
    fn rejects_empty_organism_name() {
        assert_eq!(OrganismName::new("  "), Err(OrganismNameError::Empty));
    }

    #[test]
    fn displays_and_parses_organism_kinds() -> Result<(), OrganismKindParseError> {
        assert_eq!(OrganismKind::Bacterium.to_string(), "bacterium");
        assert_eq!("plants".parse::<OrganismKind>()?, OrganismKind::Plant);
        assert_eq!("viruses".parse::<OrganismKind>()?, OrganismKind::Virus);
        Ok(())
    }

    #[test]
    fn parses_custom_organism_kind() -> Result<(), OrganismKindParseError> {
        assert_eq!(
            "lichen-forming association".parse::<OrganismKind>()?,
            OrganismKind::Custom("lichen-forming association".to_string())
        );
        assert_eq!(
            "".parse::<OrganismKind>(),
            Err(OrganismKindParseError::Empty)
        );
        Ok(())
    }

    #[test]
    fn constructs_organism_classification() {
        let taxon = Taxon::new(
            TaxonomicRank::Genus,
            TaxonName::new("Homo").expect("valid taxon"),
        );
        let species = SpeciesName::from(BinomialName::new(
            GenusName::new("Homo").expect("valid genus"),
            SpecificEpithet::new("sapiens").expect("valid epithet"),
        ));
        let classification = OrganismClassification::new(OrganismKind::Animal)
            .with_taxon(taxon.clone())
            .with_species(species.clone());

        assert_eq!(classification.kind(), &OrganismKind::Animal);
        assert_eq!(classification.taxon(), Some(&taxon));
        assert_eq!(classification.species(), Some(&species));
    }
}
