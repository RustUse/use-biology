#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

/// Life stage vocabulary for animals, plants, fungi, and microbes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LifeStage {
    /// Embryo stage.
    Embryo,
    /// Larval stage.
    Larva,
    /// Juvenile stage.
    Juvenile,
    /// Adult stage.
    Adult,
    /// Senescent stage.
    Senescent,
    /// Seed stage.
    Seed,
    /// Seedling stage.
    Seedling,
    /// Vegetative stage.
    Vegetative,
    /// Flowering stage.
    Flowering,
    /// Fruiting stage.
    Fruiting,
    /// Spore stage.
    Spore,
    /// Unknown stage.
    Unknown,
    /// Caller-defined life stage text.
    Custom(String),
}

impl fmt::Display for LifeStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Embryo => formatter.write_str("embryo"),
            Self::Larva => formatter.write_str("larva"),
            Self::Juvenile => formatter.write_str("juvenile"),
            Self::Adult => formatter.write_str("adult"),
            Self::Senescent => formatter.write_str("senescent"),
            Self::Seed => formatter.write_str("seed"),
            Self::Seedling => formatter.write_str("seedling"),
            Self::Vegetative => formatter.write_str("vegetative"),
            Self::Flowering => formatter.write_str("flowering"),
            Self::Fruiting => formatter.write_str("fruiting"),
            Self::Spore => formatter.write_str("spore"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for LifeStage {
    type Err = LifeStageParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(LifeStageParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "embryo" => Ok(Self::Embryo),
            "larva" | "larval" => Ok(Self::Larva),
            "juvenile" => Ok(Self::Juvenile),
            "adult" => Ok(Self::Adult),
            "senescent" => Ok(Self::Senescent),
            "seed" => Ok(Self::Seed),
            "seedling" => Ok(Self::Seedling),
            "vegetative" => Ok(Self::Vegetative),
            "flowering" => Ok(Self::Flowering),
            "fruiting" => Ok(Self::Fruiting),
            "spore" => Ok(Self::Spore),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing life stages fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifeStageParseError {
    /// The life stage was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for LifeStageParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("life stage cannot be empty"),
        }
    }
}

impl Error for LifeStageParseError {}

/// Error returned when development stage labels are empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DevelopmentStageError {
    /// The supplied label was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for DevelopmentStageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("development stage label cannot be empty"),
        }
    }
}

impl Error for DevelopmentStageError {}

/// A descriptive development stage record.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DevelopmentStage {
    stage: LifeStage,
    label: Option<String>,
}

impl DevelopmentStage {
    /// Creates a development stage from a life stage.
    #[must_use]
    pub const fn new(stage: LifeStage) -> Self {
        Self { stage, label: None }
    }

    /// Returns the life stage.
    #[must_use]
    pub const fn stage(&self) -> &LifeStage {
        &self.stage
    }

    /// Returns the optional descriptive label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Adds a descriptive non-empty label.
    ///
    /// # Errors
    ///
    /// Returns [`DevelopmentStageError::Empty`] when the trimmed label is empty.
    pub fn with_label(mut self, label: impl AsRef<str>) -> Result<Self, DevelopmentStageError> {
        let trimmed = label.as_ref().trim();

        if trimmed.is_empty() {
            return Err(DevelopmentStageError::Empty);
        }

        self.label = Some(trimmed.to_string());
        Ok(self)
    }
}

impl fmt::Display for DevelopmentStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.label() {
            Some(label) => write!(formatter, "{}: {label}", self.stage),
            None => self.stage.fmt(formatter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DevelopmentStage, DevelopmentStageError, LifeStage, LifeStageParseError};

    #[test]
    fn displays_and_parses_life_stage() -> Result<(), LifeStageParseError> {
        assert_eq!(LifeStage::Adult.to_string(), "adult");
        assert_eq!("larval".parse::<LifeStage>()?, LifeStage::Larva);
        assert_eq!("spore".parse::<LifeStage>()?, LifeStage::Spore);
        Ok(())
    }

    #[test]
    fn parses_custom_life_stage() -> Result<(), LifeStageParseError> {
        assert_eq!(
            "dormant".parse::<LifeStage>()?,
            LifeStage::Custom("dormant".to_string())
        );
        assert_eq!("".parse::<LifeStage>(), Err(LifeStageParseError::Empty));
        Ok(())
    }

    #[test]
    fn parses_plant_stage_variants() -> Result<(), LifeStageParseError> {
        assert_eq!("seed".parse::<LifeStage>()?, LifeStage::Seed);
        assert_eq!("seedling".parse::<LifeStage>()?, LifeStage::Seedling);
        assert_eq!("flowering".parse::<LifeStage>()?, LifeStage::Flowering);
        Ok(())
    }

    #[test]
    fn parses_animal_stage_variants() -> Result<(), LifeStageParseError> {
        assert_eq!("embryo".parse::<LifeStage>()?, LifeStage::Embryo);
        assert_eq!("juvenile".parse::<LifeStage>()?, LifeStage::Juvenile);
        assert_eq!("adult".parse::<LifeStage>()?, LifeStage::Adult);
        Ok(())
    }

    #[test]
    fn constructs_development_stage() -> Result<(), DevelopmentStageError> {
        let stage = DevelopmentStage::new(LifeStage::Vegetative).with_label("rosette")?;

        assert_eq!(stage.stage(), &LifeStage::Vegetative);
        assert_eq!(stage.label(), Some("rosette"));
        assert_eq!(stage.to_string(), "vegetative: rosette");
        Ok(())
    }
}
