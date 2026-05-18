#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

/// Error returned when tissue names are empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TissueNameError {
    /// The supplied tissue name was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for TissueNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("tissue name cannot be empty"),
        }
    }
}

impl Error for TissueNameError {}

/// A non-empty tissue name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TissueName(String);

impl TissueName {
    /// Creates a tissue name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`TissueNameError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TissueNameError> {
        let trimmed = value.as_ref().trim();

        if trimmed.is_empty() {
            Err(TissueNameError::Empty)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    /// Returns the tissue name text.
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

impl AsRef<str> for TissueName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for TissueName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TissueName {
    type Err = TissueNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Animal and plant tissue vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TissueKind {
    /// Epithelial tissue.
    Epithelial,
    /// Connective tissue.
    Connective,
    /// Muscle tissue.
    Muscle,
    /// Nervous tissue.
    Nervous,
    /// Vascular tissue.
    Vascular,
    /// Dermal tissue.
    Dermal,
    /// Ground tissue.
    Ground,
    /// Meristematic tissue.
    Meristematic,
    /// Unknown tissue kind.
    Unknown,
    /// Caller-defined tissue kind text.
    Custom(String),
}

impl fmt::Display for TissueKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Epithelial => formatter.write_str("epithelial"),
            Self::Connective => formatter.write_str("connective"),
            Self::Muscle => formatter.write_str("muscle"),
            Self::Nervous => formatter.write_str("nervous"),
            Self::Vascular => formatter.write_str("vascular"),
            Self::Dermal => formatter.write_str("dermal"),
            Self::Ground => formatter.write_str("ground"),
            Self::Meristematic => formatter.write_str("meristematic"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for TissueKind {
    type Err = TissueKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(TissueKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "epithelial" => Ok(Self::Epithelial),
            "connective" => Ok(Self::Connective),
            "muscle" => Ok(Self::Muscle),
            "nervous" => Ok(Self::Nervous),
            "vascular" => Ok(Self::Vascular),
            "dermal" => Ok(Self::Dermal),
            "ground" => Ok(Self::Ground),
            "meristematic" => Ok(Self::Meristematic),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing tissue kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TissueKindParseError {
    /// The tissue kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for TissueKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("tissue kind cannot be empty"),
        }
    }
}

impl Error for TissueKindParseError {}

#[cfg(test)]
mod tests {
    use super::{TissueKind, TissueKindParseError, TissueName, TissueNameError};

    #[test]
    fn constructs_valid_tissue_name() -> Result<(), TissueNameError> {
        let name = TissueName::new("xylem")?;

        assert_eq!(name.as_str(), "xylem");
        Ok(())
    }

    #[test]
    fn rejects_empty_tissue_name() {
        assert_eq!(TissueName::new("  "), Err(TissueNameError::Empty));
    }

    #[test]
    fn displays_and_parses_tissue_kind() -> Result<(), TissueKindParseError> {
        assert_eq!(TissueKind::Epithelial.to_string(), "epithelial");
        assert_eq!("ground".parse::<TissueKind>()?, TissueKind::Ground);
        assert_eq!(
            "meristematic".parse::<TissueKind>()?,
            TissueKind::Meristematic
        );
        Ok(())
    }

    #[test]
    fn parses_custom_tissue_kind() -> Result<(), TissueKindParseError> {
        assert_eq!(
            "cartilaginous".parse::<TissueKind>()?,
            TissueKind::Custom("cartilaginous".to_string())
        );
        assert_eq!("".parse::<TissueKind>(), Err(TissueKindParseError::Empty));
        Ok(())
    }
}
