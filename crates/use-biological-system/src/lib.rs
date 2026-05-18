#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

/// Error returned when biological system names are empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BiologicalSystemNameError {
    /// The supplied system name was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for BiologicalSystemNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("biological system name cannot be empty"),
        }
    }
}

impl Error for BiologicalSystemNameError {}

/// Biological system kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BiologicalSystemKind {
    /// Nervous system.
    Nervous,
    /// Circulatory system.
    Circulatory,
    /// Respiratory system.
    Respiratory,
    /// Digestive system.
    Digestive,
    /// Endocrine system.
    Endocrine,
    /// Immune system.
    Immune,
    /// Reproductive system.
    Reproductive,
    /// Musculoskeletal system.
    Musculoskeletal,
    /// Plant root system.
    RootSystem,
    /// Plant shoot system.
    ShootSystem,
    /// Vascular system.
    VascularSystem,
    /// Unknown biological system kind.
    Unknown,
    /// Caller-defined biological system kind text.
    Custom(String),
}

impl fmt::Display for BiologicalSystemKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nervous => formatter.write_str("nervous"),
            Self::Circulatory => formatter.write_str("circulatory"),
            Self::Respiratory => formatter.write_str("respiratory"),
            Self::Digestive => formatter.write_str("digestive"),
            Self::Endocrine => formatter.write_str("endocrine"),
            Self::Immune => formatter.write_str("immune"),
            Self::Reproductive => formatter.write_str("reproductive"),
            Self::Musculoskeletal => formatter.write_str("musculoskeletal"),
            Self::RootSystem => formatter.write_str("root-system"),
            Self::ShootSystem => formatter.write_str("shoot-system"),
            Self::VascularSystem => formatter.write_str("vascular-system"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for BiologicalSystemKind {
    type Err = BiologicalSystemKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(BiologicalSystemKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "nervous" | "nervous-system" => Ok(Self::Nervous),
            "circulatory" | "circulatory-system" => Ok(Self::Circulatory),
            "respiratory" | "respiratory-system" => Ok(Self::Respiratory),
            "digestive" | "digestive-system" => Ok(Self::Digestive),
            "endocrine" | "endocrine-system" => Ok(Self::Endocrine),
            "immune" | "immune-system" => Ok(Self::Immune),
            "reproductive" | "reproductive-system" => Ok(Self::Reproductive),
            "musculoskeletal" | "musculoskeletal-system" => Ok(Self::Musculoskeletal),
            "root-system" => Ok(Self::RootSystem),
            "shoot-system" => Ok(Self::ShootSystem),
            "vascular-system" => Ok(Self::VascularSystem),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing biological system kinds fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BiologicalSystemKindParseError {
    /// The biological system kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for BiologicalSystemKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("biological system kind cannot be empty"),
        }
    }
}

impl Error for BiologicalSystemKindParseError {}

/// A descriptive biological system record.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BiologicalSystem {
    name: String,
    kind: BiologicalSystemKind,
}

impl BiologicalSystem {
    /// Creates a biological system from non-empty name text and a kind.
    ///
    /// # Errors
    ///
    /// Returns [`BiologicalSystemNameError::Empty`] when the trimmed name is empty.
    pub fn new(
        name: impl AsRef<str>,
        kind: BiologicalSystemKind,
    ) -> Result<Self, BiologicalSystemNameError> {
        let trimmed = name.as_ref().trim();

        if trimmed.is_empty() {
            return Err(BiologicalSystemNameError::Empty);
        }

        Ok(Self {
            name: trimmed.to_string(),
            kind,
        })
    }

    /// Returns the system name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the system kind.
    #[must_use]
    pub const fn kind(&self) -> &BiologicalSystemKind {
        &self.kind
    }
}

impl fmt::Display for BiologicalSystem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BiologicalSystem, BiologicalSystemKind, BiologicalSystemKindParseError,
        BiologicalSystemNameError,
    };

    #[test]
    fn constructs_valid_biological_system_name() -> Result<(), BiologicalSystemNameError> {
        let system = BiologicalSystem::new("root system", BiologicalSystemKind::RootSystem)?;

        assert_eq!(system.name(), "root system");
        assert_eq!(system.kind(), &BiologicalSystemKind::RootSystem);
        Ok(())
    }

    #[test]
    fn rejects_empty_system_name() {
        assert_eq!(
            BiologicalSystem::new("   ", BiologicalSystemKind::Unknown),
            Err(BiologicalSystemNameError::Empty)
        );
    }

    #[test]
    fn displays_and_parses_system_kind() -> Result<(), BiologicalSystemKindParseError> {
        assert_eq!(BiologicalSystemKind::Nervous.to_string(), "nervous");
        assert_eq!(
            "respiratory system".parse::<BiologicalSystemKind>()?,
            BiologicalSystemKind::Respiratory
        );
        Ok(())
    }

    #[test]
    fn parses_plant_system_variants() -> Result<(), BiologicalSystemKindParseError> {
        assert_eq!(
            "root system".parse::<BiologicalSystemKind>()?,
            BiologicalSystemKind::RootSystem
        );
        assert_eq!(
            "shoot-system".parse::<BiologicalSystemKind>()?,
            BiologicalSystemKind::ShootSystem
        );
        assert_eq!(
            "vascular system".parse::<BiologicalSystemKind>()?,
            BiologicalSystemKind::VascularSystem
        );
        Ok(())
    }

    #[test]
    fn parses_custom_system_kind() -> Result<(), BiologicalSystemKindParseError> {
        assert_eq!(
            "water-vascular".parse::<BiologicalSystemKind>()?,
            BiologicalSystemKind::Custom("water-vascular".to_string())
        );
        assert_eq!(
            "".parse::<BiologicalSystemKind>(),
            Err(BiologicalSystemKindParseError::Empty)
        );
        Ok(())
    }
}
