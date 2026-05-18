#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, OrganNameError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(OrganNameError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned when organ labels are empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrganNameError {
    /// The supplied value was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for OrganNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("organ label cannot be empty"),
        }
    }
}

impl Error for OrganNameError {}

/// A non-empty organ name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OrganName(String);

impl OrganName {
    /// Creates an organ name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`OrganNameError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, OrganNameError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the organ name text.
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

impl AsRef<str> for OrganName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for OrganName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for OrganName {
    type Err = OrganNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A simple organ-system reference label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OrganSystemRef(String);

impl OrganSystemRef {
    /// Creates an organ-system reference from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`OrganNameError::Empty`] when the trimmed label is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, OrganNameError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the reference text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the reference and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for OrganSystemRef {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for OrganSystemRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for OrganSystemRef {
    type Err = OrganNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Broad animal and plant organ vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrganKind {
    /// Heart.
    Heart,
    /// Lung.
    Lung,
    /// Brain.
    Brain,
    /// Liver.
    Liver,
    /// Kidney.
    Kidney,
    /// Leaf.
    Leaf,
    /// Root.
    Root,
    /// Stem.
    Stem,
    /// Flower.
    Flower,
    /// Seed.
    Seed,
    /// Unknown organ kind.
    Unknown,
    /// Caller-defined organ kind text.
    Custom(String),
}

impl fmt::Display for OrganKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Heart => formatter.write_str("heart"),
            Self::Lung => formatter.write_str("lung"),
            Self::Brain => formatter.write_str("brain"),
            Self::Liver => formatter.write_str("liver"),
            Self::Kidney => formatter.write_str("kidney"),
            Self::Leaf => formatter.write_str("leaf"),
            Self::Root => formatter.write_str("root"),
            Self::Stem => formatter.write_str("stem"),
            Self::Flower => formatter.write_str("flower"),
            Self::Seed => formatter.write_str("seed"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for OrganKind {
    type Err = OrganKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(OrganKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "heart" => Ok(Self::Heart),
            "lung" | "lungs" => Ok(Self::Lung),
            "brain" => Ok(Self::Brain),
            "liver" => Ok(Self::Liver),
            "kidney" => Ok(Self::Kidney),
            "leaf" | "leaves" => Ok(Self::Leaf),
            "root" => Ok(Self::Root),
            "stem" => Ok(Self::Stem),
            "flower" => Ok(Self::Flower),
            "seed" => Ok(Self::Seed),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing organ kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrganKindParseError {
    /// The organ kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for OrganKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("organ kind cannot be empty"),
        }
    }
}

impl Error for OrganKindParseError {}

#[cfg(test)]
mod tests {
    use super::{OrganKind, OrganKindParseError, OrganName, OrganNameError};

    #[test]
    fn constructs_valid_organ_name() -> Result<(), OrganNameError> {
        let name = OrganName::new("leaf")?;

        assert_eq!(name.as_str(), "leaf");
        Ok(())
    }

    #[test]
    fn rejects_empty_organ_name() {
        assert_eq!(OrganName::new(""), Err(OrganNameError::Empty));
    }

    #[test]
    fn displays_and_parses_organ_kind() -> Result<(), OrganKindParseError> {
        assert_eq!(OrganKind::Heart.to_string(), "heart");
        assert_eq!("kidney".parse::<OrganKind>()?, OrganKind::Kidney);
        Ok(())
    }

    #[test]
    fn parses_plant_organ_variants() -> Result<(), OrganKindParseError> {
        assert_eq!("leaf".parse::<OrganKind>()?, OrganKind::Leaf);
        assert_eq!("root".parse::<OrganKind>()?, OrganKind::Root);
        assert_eq!("flower".parse::<OrganKind>()?, OrganKind::Flower);
        Ok(())
    }

    #[test]
    fn parses_custom_organ_kind() -> Result<(), OrganKindParseError> {
        assert_eq!(
            "hypha".parse::<OrganKind>()?,
            OrganKind::Custom("hypha".to_string())
        );
        assert_eq!(" ".parse::<OrganKind>(), Err(OrganKindParseError::Empty));
        Ok(())
    }
}
