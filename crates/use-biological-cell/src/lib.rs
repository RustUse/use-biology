#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn normalized_key(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, CellNameError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(CellNameError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned when cell labels are empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CellNameError {
    /// The supplied value was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for CellNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("cell label cannot be empty"),
        }
    }
}

impl Error for CellNameError {}

/// Broad cell kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CellKind {
    /// Prokaryotic cells.
    Prokaryotic,
    /// Eukaryotic cells.
    Eukaryotic,
    /// Animal cells.
    Animal,
    /// Plant cells.
    Plant,
    /// Fungal cells.
    Fungal,
    /// Bacterial cells.
    Bacterial,
    /// Archaeal cells.
    Archaeal,
    /// Stem cells.
    Stem,
    /// Somatic cells.
    Somatic,
    /// Germ cells.
    Germ,
    /// Unknown cell kind.
    Unknown,
    /// Caller-defined cell kind text.
    Custom(String),
}

impl fmt::Display for CellKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Prokaryotic => formatter.write_str("prokaryotic"),
            Self::Eukaryotic => formatter.write_str("eukaryotic"),
            Self::Animal => formatter.write_str("animal"),
            Self::Plant => formatter.write_str("plant"),
            Self::Fungal => formatter.write_str("fungal"),
            Self::Bacterial => formatter.write_str("bacterial"),
            Self::Archaeal => formatter.write_str("archaeal"),
            Self::Stem => formatter.write_str("stem"),
            Self::Somatic => formatter.write_str("somatic"),
            Self::Germ => formatter.write_str("germ"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for CellKind {
    type Err = CellKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CellKindParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "prokaryotic" => Ok(Self::Prokaryotic),
            "eukaryotic" => Ok(Self::Eukaryotic),
            "animal" => Ok(Self::Animal),
            "plant" => Ok(Self::Plant),
            "fungal" => Ok(Self::Fungal),
            "bacterial" => Ok(Self::Bacterial),
            "archaeal" => Ok(Self::Archaeal),
            "stem" => Ok(Self::Stem),
            "somatic" => Ok(Self::Somatic),
            "germ" => Ok(Self::Germ),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing a cell kind fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CellKindParseError {
    /// The cell kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for CellKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("cell kind cannot be empty"),
        }
    }
}

impl Error for CellKindParseError {}

/// Primitive cell organelle vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CellOrganelle {
    /// Nucleus.
    Nucleus,
    /// Mitochondrion.
    Mitochondrion,
    /// Chloroplast.
    Chloroplast,
    /// Ribosome.
    Ribosome,
    /// Cell membrane.
    CellMembrane,
    /// Cell wall.
    CellWall,
    /// Cytoplasm.
    Cytoplasm,
    /// Golgi apparatus.
    GolgiApparatus,
    /// Endoplasmic reticulum.
    EndoplasmicReticulum,
    /// Vacuole.
    Vacuole,
    /// Unknown organelle.
    Unknown,
    /// Caller-defined organelle text.
    Custom(String),
}

impl fmt::Display for CellOrganelle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nucleus => formatter.write_str("nucleus"),
            Self::Mitochondrion => formatter.write_str("mitochondrion"),
            Self::Chloroplast => formatter.write_str("chloroplast"),
            Self::Ribosome => formatter.write_str("ribosome"),
            Self::CellMembrane => formatter.write_str("cell-membrane"),
            Self::CellWall => formatter.write_str("cell-wall"),
            Self::Cytoplasm => formatter.write_str("cytoplasm"),
            Self::GolgiApparatus => formatter.write_str("golgi-apparatus"),
            Self::EndoplasmicReticulum => formatter.write_str("endoplasmic-reticulum"),
            Self::Vacuole => formatter.write_str("vacuole"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for CellOrganelle {
    type Err = CellOrganelleParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(CellOrganelleParseError::Empty);
        }

        match normalized_key(trimmed).as_str() {
            "nucleus" => Ok(Self::Nucleus),
            "mitochondrion" | "mitochondria" => Ok(Self::Mitochondrion),
            "chloroplast" => Ok(Self::Chloroplast),
            "ribosome" => Ok(Self::Ribosome),
            "cell-membrane" => Ok(Self::CellMembrane),
            "cell-wall" => Ok(Self::CellWall),
            "cytoplasm" => Ok(Self::Cytoplasm),
            "golgi-apparatus" | "golgi" => Ok(Self::GolgiApparatus),
            "endoplasmic-reticulum" => Ok(Self::EndoplasmicReticulum),
            "vacuole" => Ok(Self::Vacuole),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing a cell organelle fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CellOrganelleParseError {
    /// The organelle text was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for CellOrganelleParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("cell organelle cannot be empty"),
        }
    }
}

impl Error for CellOrganelleParseError {}

/// A descriptive cell type label with a broad cell kind.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CellType {
    name: String,
    kind: CellKind,
}

impl CellType {
    /// Creates a cell type from non-empty text and a cell kind.
    ///
    /// # Errors
    ///
    /// Returns [`CellNameError::Empty`] when the trimmed name is empty.
    pub fn new(name: impl AsRef<str>, kind: CellKind) -> Result<Self, CellNameError> {
        Ok(Self {
            name: non_empty_text(name)?,
            kind,
        })
    }

    /// Returns the cell type label.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the broad cell kind.
    #[must_use]
    pub const fn kind(&self) -> &CellKind {
        &self.kind
    }
}

impl fmt::Display for CellType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A primitive cell structure reference.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CellStructure {
    /// A named organelle.
    Organelle(CellOrganelle),
    /// Caller-defined structure label.
    Custom(String),
}

impl CellStructure {
    /// Creates a custom structure label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`CellNameError::Empty`] when the trimmed label is empty.
    pub fn custom(value: impl AsRef<str>) -> Result<Self, CellNameError> {
        non_empty_text(value).map(Self::Custom)
    }
}

impl From<CellOrganelle> for CellStructure {
    fn from(organelle: CellOrganelle) -> Self {
        Self::Organelle(organelle)
    }
}

impl fmt::Display for CellStructure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Organelle(organelle) => organelle.fmt(formatter),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CellKind, CellKindParseError, CellNameError, CellOrganelle, CellOrganelleParseError,
        CellStructure, CellType,
    };

    #[test]
    fn displays_and_parses_cell_kind() -> Result<(), CellKindParseError> {
        assert_eq!(CellKind::Eukaryotic.to_string(), "eukaryotic");
        assert_eq!("plant".parse::<CellKind>()?, CellKind::Plant);
        assert_eq!("stem".parse::<CellKind>()?, CellKind::Stem);
        Ok(())
    }

    #[test]
    fn displays_and_parses_organelles() -> Result<(), CellOrganelleParseError> {
        assert_eq!(CellOrganelle::CellWall.to_string(), "cell-wall");
        assert_eq!(
            "cell membrane".parse::<CellOrganelle>()?,
            CellOrganelle::CellMembrane
        );
        assert_eq!(
            "golgi".parse::<CellOrganelle>()?,
            CellOrganelle::GolgiApparatus
        );
        Ok(())
    }

    #[test]
    fn parses_custom_cell_kind() -> Result<(), CellKindParseError> {
        assert_eq!(
            "guard cell".parse::<CellKind>()?,
            CellKind::Custom("guard cell".to_string())
        );
        assert_eq!(" ".parse::<CellKind>(), Err(CellKindParseError::Empty));
        Ok(())
    }

    #[test]
    fn parses_custom_organelle() -> Result<(), CellOrganelleParseError> {
        assert_eq!(
            "eyespot".parse::<CellOrganelle>()?,
            CellOrganelle::Custom("eyespot".to_string())
        );
        assert_eq!(
            "".parse::<CellOrganelle>(),
            Err(CellOrganelleParseError::Empty)
        );
        Ok(())
    }

    #[test]
    fn cell_type_and_structure_display_stably() -> Result<(), CellNameError> {
        let cell_type = CellType::new("neuron", CellKind::Animal)?;
        let structure = CellStructure::from(CellOrganelle::Nucleus);
        let custom = CellStructure::custom("contractile vacuole")?;

        assert_eq!(cell_type.name(), "neuron");
        assert_eq!(cell_type.kind(), &CellKind::Animal);
        assert_eq!(cell_type.to_string(), "neuron");
        assert_eq!(structure.to_string(), "nucleus");
        assert_eq!(custom.to_string(), "contractile vacuole");
        Ok(())
    }
}
