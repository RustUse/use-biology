#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

/// Reproduction mode vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReproductionMode {
    /// Asexual reproduction.
    Asexual,
    /// Sexual reproduction.
    Sexual,
    /// Budding.
    Budding,
    /// Fragmentation.
    Fragmentation,
    /// Spore formation.
    SporeFormation,
    /// Binary fission.
    BinaryFission,
    /// Pollination.
    Pollination,
    /// Unknown reproduction mode.
    Unknown,
    /// Caller-defined reproduction mode text.
    Custom(String),
}

impl fmt::Display for ReproductionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Asexual => formatter.write_str("asexual"),
            Self::Sexual => formatter.write_str("sexual"),
            Self::Budding => formatter.write_str("budding"),
            Self::Fragmentation => formatter.write_str("fragmentation"),
            Self::SporeFormation => formatter.write_str("spore-formation"),
            Self::BinaryFission => formatter.write_str("binary-fission"),
            Self::Pollination => formatter.write_str("pollination"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for ReproductionMode {
    type Err = ReproductionModeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ReproductionModeParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "asexual" => Ok(Self::Asexual),
            "sexual" => Ok(Self::Sexual),
            "budding" => Ok(Self::Budding),
            "fragmentation" => Ok(Self::Fragmentation),
            "spore-formation" => Ok(Self::SporeFormation),
            "binary-fission" => Ok(Self::BinaryFission),
            "pollination" => Ok(Self::Pollination),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing reproduction modes fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReproductionModeParseError {
    /// The reproduction mode was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ReproductionModeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("reproduction mode cannot be empty"),
        }
    }
}

impl Error for ReproductionModeParseError {}

/// Fertilization mode vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FertilizationMode {
    /// Internal fertilization.
    Internal,
    /// External fertilization.
    External,
    /// Self-fertilization.
    SelfFertilization,
    /// Cross-fertilization.
    CrossFertilization,
    /// Unknown fertilization mode.
    Unknown,
    /// Caller-defined fertilization mode text.
    Custom(String),
}

impl fmt::Display for FertilizationMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal => formatter.write_str("internal"),
            Self::External => formatter.write_str("external"),
            Self::SelfFertilization => formatter.write_str("self-fertilization"),
            Self::CrossFertilization => formatter.write_str("cross-fertilization"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for FertilizationMode {
    type Err = FertilizationModeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(FertilizationModeParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "internal" => Ok(Self::Internal),
            "external" => Ok(Self::External),
            "self-fertilization" | "selfing" => Ok(Self::SelfFertilization),
            "cross-fertilization" => Ok(Self::CrossFertilization),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing fertilization modes fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FertilizationModeParseError {
    /// The fertilization mode was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for FertilizationModeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("fertilization mode cannot be empty"),
        }
    }
}

impl Error for FertilizationModeParseError {}

#[cfg(test)]
mod tests {
    use super::{
        FertilizationMode, FertilizationModeParseError, ReproductionMode,
        ReproductionModeParseError,
    };

    #[test]
    fn displays_and_parses_reproduction_mode() -> Result<(), ReproductionModeParseError> {
        assert_eq!(
            ReproductionMode::BinaryFission.to_string(),
            "binary-fission"
        );
        assert_eq!(
            "spore formation".parse::<ReproductionMode>()?,
            ReproductionMode::SporeFormation
        );
        assert_eq!(
            "pollination".parse::<ReproductionMode>()?,
            ReproductionMode::Pollination
        );
        Ok(())
    }

    #[test]
    fn displays_and_parses_fertilization_mode() -> Result<(), FertilizationModeParseError> {
        assert_eq!(
            FertilizationMode::SelfFertilization.to_string(),
            "self-fertilization"
        );
        assert_eq!(
            "external".parse::<FertilizationMode>()?,
            FertilizationMode::External
        );
        assert_eq!(
            "selfing".parse::<FertilizationMode>()?,
            FertilizationMode::SelfFertilization
        );
        Ok(())
    }

    #[test]
    fn parses_custom_reproduction_mode() -> Result<(), ReproductionModeParseError> {
        assert_eq!(
            "vegetative propagation".parse::<ReproductionMode>()?,
            ReproductionMode::Custom("vegetative propagation".to_string())
        );
        assert_eq!(
            "".parse::<ReproductionMode>(),
            Err(ReproductionModeParseError::Empty)
        );
        Ok(())
    }

    #[test]
    fn parses_custom_fertilization_mode() -> Result<(), FertilizationModeParseError> {
        assert_eq!(
            "broadcast spawning".parse::<FertilizationMode>()?,
            FertilizationMode::Custom("broadcast spawning".to_string())
        );
        assert_eq!(
            " ".parse::<FertilizationMode>(),
            Err(FertilizationModeParseError::Empty)
        );
        Ok(())
    }
}
