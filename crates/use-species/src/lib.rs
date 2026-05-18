#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, SpeciesNameError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(SpeciesNameError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned when species name components are empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpeciesNameError {
    /// The supplied value was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for SpeciesNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("species name component cannot be empty"),
        }
    }
}

impl Error for SpeciesNameError {}

/// A non-empty genus name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GenusName(String);

impl GenusName {
    /// Creates a genus name from non-empty text.
    ///
    /// Surrounding whitespace is trimmed; the remaining text and casing are preserved.
    ///
    /// # Errors
    ///
    /// Returns [`SpeciesNameError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SpeciesNameError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the genus text.
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

impl AsRef<str> for GenusName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for GenusName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GenusName {
    type Err = SpeciesNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty specific epithet.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SpecificEpithet(String);

impl SpecificEpithet {
    /// Creates a specific epithet from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`SpeciesNameError::Empty`] when the trimmed epithet is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SpeciesNameError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the epithet text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the epithet and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for SpecificEpithet {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SpecificEpithet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SpecificEpithet {
    type Err = SpeciesNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty subspecific epithet.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SubspecificEpithet(String);

impl SubspecificEpithet {
    /// Creates a subspecific epithet from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`SpeciesNameError::Empty`] when the trimmed epithet is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SpeciesNameError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the epithet text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the epithet and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for SubspecificEpithet {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SubspecificEpithet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SubspecificEpithet {
    type Err = SpeciesNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A descriptive binomial species name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BinomialName {
    genus: GenusName,
    specific_epithet: SpecificEpithet,
}

impl BinomialName {
    /// Creates a binomial name from validated components.
    #[must_use]
    pub const fn new(genus: GenusName, specific_epithet: SpecificEpithet) -> Self {
        Self {
            genus,
            specific_epithet,
        }
    }

    /// Returns the genus component.
    #[must_use]
    pub const fn genus(&self) -> &GenusName {
        &self.genus
    }

    /// Returns the specific epithet component.
    #[must_use]
    pub const fn specific_epithet(&self) -> &SpecificEpithet {
        &self.specific_epithet
    }
}

impl fmt::Display for BinomialName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.genus, self.specific_epithet)
    }
}

/// A descriptive trinomial species name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TrinomialName {
    binomial: BinomialName,
    subspecific_epithet: SubspecificEpithet,
}

impl TrinomialName {
    /// Creates a trinomial name from validated components.
    #[must_use]
    pub const fn new(binomial: BinomialName, subspecific_epithet: SubspecificEpithet) -> Self {
        Self {
            binomial,
            subspecific_epithet,
        }
    }

    /// Returns the binomial portion.
    #[must_use]
    pub const fn binomial(&self) -> &BinomialName {
        &self.binomial
    }

    /// Returns the subspecific epithet.
    #[must_use]
    pub const fn subspecific_epithet(&self) -> &SubspecificEpithet {
        &self.subspecific_epithet
    }
}

impl fmt::Display for TrinomialName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.binomial, self.subspecific_epithet)
    }
}

/// A species name represented as binomial, trinomial, or caller-defined descriptive text.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SpeciesName {
    /// A binomial species name.
    Binomial(BinomialName),
    /// A trinomial species name.
    Trinomial(TrinomialName),
    /// Caller-defined descriptive species text.
    Custom(String),
}

impl SpeciesName {
    /// Creates a custom descriptive species name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`SpeciesNameError::Empty`] when the trimmed text is empty.
    pub fn custom(value: impl AsRef<str>) -> Result<Self, SpeciesNameError> {
        non_empty_text(value).map(Self::Custom)
    }
}

impl From<BinomialName> for SpeciesName {
    fn from(name: BinomialName) -> Self {
        Self::Binomial(name)
    }
}

impl From<TrinomialName> for SpeciesName {
    fn from(name: TrinomialName) -> Self {
        Self::Trinomial(name)
    }
}

impl fmt::Display for SpeciesName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binomial(name) => name.fmt(formatter),
            Self::Trinomial(name) => name.fmt(formatter),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BinomialName, GenusName, SpeciesName, SpeciesNameError, SpecificEpithet,
        SubspecificEpithet, TrinomialName,
    };

    #[test]
    fn constructs_valid_binomial_name() -> Result<(), SpeciesNameError> {
        let name = BinomialName::new(GenusName::new("Homo")?, SpecificEpithet::new("sapiens")?);

        assert_eq!(name.genus().as_str(), "Homo");
        assert_eq!(name.specific_epithet().as_str(), "sapiens");
        assert_eq!(name.to_string(), "Homo sapiens");
        Ok(())
    }

    #[test]
    fn constructs_valid_trinomial_name() -> Result<(), SpeciesNameError> {
        let binomial = BinomialName::new(GenusName::new("Canis")?, SpecificEpithet::new("lupus")?);
        let trinomial = TrinomialName::new(binomial, SubspecificEpithet::new("familiaris")?);

        assert_eq!(trinomial.to_string(), "Canis lupus familiaris");
        Ok(())
    }

    #[test]
    fn rejects_empty_genus() {
        assert_eq!(GenusName::new("  "), Err(SpeciesNameError::Empty));
    }

    #[test]
    fn rejects_empty_specific_epithet() {
        assert_eq!(SpecificEpithet::new(""), Err(SpeciesNameError::Empty));
    }

    #[test]
    fn displays_species_names() -> Result<(), SpeciesNameError> {
        let binomial = BinomialName::new(
            GenusName::new("Escherichia")?,
            SpecificEpithet::new("coli")?,
        );
        let species = SpeciesName::from(binomial);

        assert_eq!(species.to_string(), "Escherichia coli");
        Ok(())
    }

    #[test]
    fn constructs_custom_species_name() -> Result<(), SpeciesNameError> {
        let species = SpeciesName::custom("unresolved species descriptor")?;

        assert_eq!(species.to_string(), "unresolved species descriptor");
        assert_eq!(SpeciesName::custom("   "), Err(SpeciesNameError::Empty));
        Ok(())
    }
}
