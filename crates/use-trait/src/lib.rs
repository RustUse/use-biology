#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, TraitNameError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(TraitNameError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned when trait labels are empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TraitNameError {
    /// The supplied value was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for TraitNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("trait text cannot be empty"),
        }
    }
}

impl Error for TraitNameError {}

/// A non-empty biological trait name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TraitName(String);

impl TraitName {
    /// Creates a trait name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`TraitNameError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TraitNameError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the trait name text.
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

impl AsRef<str> for TraitName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for TraitName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TraitName {
    type Err = TraitNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Broad biological trait families.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TraitKind {
    /// Shape, size, color, and other structural traits.
    Morphological,
    /// Functional traits.
    Physiological,
    /// Behavior-associated traits.
    Behavioral,
    /// Genetic trait labels.
    Genetic,
    /// Developmental traits.
    Developmental,
    /// Ecological trait labels.
    Ecological,
    /// Unknown trait kind.
    Unknown,
    /// Caller-defined trait kind text.
    Custom(String),
}

impl fmt::Display for TraitKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Morphological => formatter.write_str("morphological"),
            Self::Physiological => formatter.write_str("physiological"),
            Self::Behavioral => formatter.write_str("behavioral"),
            Self::Genetic => formatter.write_str("genetic"),
            Self::Developmental => formatter.write_str("developmental"),
            Self::Ecological => formatter.write_str("ecological"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for TraitKind {
    type Err = TraitKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(TraitKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "morphological" => Ok(Self::Morphological),
            "physiological" => Ok(Self::Physiological),
            "behavioral" | "behavioural" => Ok(Self::Behavioral),
            "genetic" => Ok(Self::Genetic),
            "developmental" => Ok(Self::Developmental),
            "ecological" => Ok(Self::Ecological),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing trait kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TraitKindParseError {
    /// The trait kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for TraitKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("trait kind cannot be empty"),
        }
    }
}

impl Error for TraitKindParseError {}

/// A descriptive trait value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TraitValue {
    name: TraitName,
    value: String,
    kind: Option<TraitKind>,
}

impl TraitValue {
    /// Creates a trait value from a trait name and non-empty descriptive value.
    ///
    /// # Errors
    ///
    /// Returns [`TraitNameError::Empty`] when the trimmed value is empty.
    pub fn new(name: TraitName, value: impl AsRef<str>) -> Result<Self, TraitNameError> {
        Ok(Self {
            name,
            value: non_empty_text(value)?,
            kind: None,
        })
    }

    /// Returns the trait name.
    #[must_use]
    pub const fn name(&self) -> &TraitName {
        &self.name
    }

    /// Returns the descriptive trait value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the optional trait kind.
    #[must_use]
    pub const fn kind(&self) -> Option<&TraitKind> {
        self.kind.as_ref()
    }

    /// Adds a trait kind.
    #[must_use]
    pub fn with_kind(mut self, kind: TraitKind) -> Self {
        self.kind = Some(kind);
        self
    }
}

impl fmt::Display for TraitValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.name, self.value)
    }
}

/// An ordered phenotype description.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Phenotype {
    traits: Vec<TraitValue>,
}

impl Phenotype {
    /// Creates a phenotype from caller-supplied trait order.
    #[must_use]
    pub const fn new(traits: Vec<TraitValue>) -> Self {
        Self { traits }
    }

    /// Returns trait values in caller-supplied order.
    #[must_use]
    pub fn traits(&self) -> &[TraitValue] {
        &self.traits
    }

    /// Returns the trait count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.traits.len()
    }

    /// Returns `true` when no trait values are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.traits.is_empty()
    }
}

impl From<Vec<TraitValue>> for Phenotype {
    fn from(traits: Vec<TraitValue>) -> Self {
        Self::new(traits)
    }
}

#[cfg(test)]
mod tests {
    use super::{Phenotype, TraitKind, TraitKindParseError, TraitName, TraitNameError, TraitValue};

    #[test]
    fn constructs_valid_trait_name() -> Result<(), TraitNameError> {
        let name = TraitName::new("leaf shape")?;

        assert_eq!(name.as_str(), "leaf shape");
        assert_eq!(name.to_string(), "leaf shape");
        Ok(())
    }

    #[test]
    fn rejects_empty_trait_name() {
        assert_eq!(TraitName::new("  "), Err(TraitNameError::Empty));
    }

    #[test]
    fn displays_and_parses_trait_kind() -> Result<(), TraitKindParseError> {
        assert_eq!(TraitKind::Morphological.to_string(), "morphological");
        assert_eq!("behavioural".parse::<TraitKind>()?, TraitKind::Behavioral);
        assert_eq!("genetic".parse::<TraitKind>()?, TraitKind::Genetic);
        Ok(())
    }

    #[test]
    fn parses_custom_trait_kind() -> Result<(), TraitKindParseError> {
        assert_eq!(
            "seasonal".parse::<TraitKind>()?,
            TraitKind::Custom("seasonal".to_string())
        );
        assert_eq!("".parse::<TraitKind>(), Err(TraitKindParseError::Empty));
        Ok(())
    }

    #[test]
    fn constructs_phenotype() -> Result<(), TraitNameError> {
        let value = TraitValue::new(TraitName::new("flower color")?, "white")?
            .with_kind(TraitKind::Morphological);
        let phenotype = Phenotype::new(vec![value.clone()]);

        assert_eq!(value.name().as_str(), "flower color");
        assert_eq!(value.value(), "white");
        assert_eq!(value.kind(), Some(&TraitKind::Morphological));
        assert_eq!(value.to_string(), "flower color: white");
        assert_eq!(phenotype.len(), 1);
        assert_eq!(phenotype.traits(), &[value]);
        Ok(())
    }
}
