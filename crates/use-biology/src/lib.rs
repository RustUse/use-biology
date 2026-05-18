#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_biological_system as biological_system;
pub use use_cell as cell;
pub use use_gene as gene;
pub use use_life_stage as life_stage;
pub use use_organ as organ;
pub use use_organism as organism;
pub use use_reproduction as reproduction;
pub use use_species as species;
pub use use_taxonomy as taxonomy;
pub use use_tissue as tissue;
pub use use_trait as trait_;

/// Common primitive biology vocabulary reexports.
pub mod prelude {
    pub use use_biological_system::{
        BiologicalSystem, BiologicalSystemKind, BiologicalSystemKindParseError,
        BiologicalSystemNameError,
    };
    pub use use_cell::{
        CellKind, CellKindParseError, CellNameError, CellOrganelle, CellOrganelleParseError,
        CellStructure, CellType,
    };
    pub use use_gene::{Allele, GeneId, GeneName, GeneSymbol, GeneValueError, Genotype, Locus};
    pub use use_life_stage::{
        DevelopmentStage, DevelopmentStageError, LifeStage, LifeStageParseError,
    };
    pub use use_organ::{
        OrganKind, OrganKindParseError, OrganName, OrganNameError, OrganSystemRef,
    };
    pub use use_organism::{
        OrganismClassification, OrganismId, OrganismKind, OrganismKindParseError, OrganismName,
        OrganismNameError,
    };
    pub use use_reproduction::{
        FertilizationMode, FertilizationModeParseError, ReproductionMode,
        ReproductionModeParseError,
    };
    pub use use_species::{
        BinomialName, GenusName, SpeciesName, SpeciesNameError, SpecificEpithet,
        SubspecificEpithet, TrinomialName,
    };
    pub use use_taxonomy::{
        CommonName, ScientificName, Taxon, TaxonName, TaxonomicLineage, TaxonomicRank,
        TaxonomicRankParseError, TaxonomyNameError,
    };
    pub use use_tissue::{TissueKind, TissueKindParseError, TissueName, TissueNameError};
    pub use use_trait::{
        Phenotype, TraitKind, TraitKindParseError, TraitName, TraitNameError, TraitValue,
    };
}

#[cfg(test)]
mod tests {
    use super::prelude::{
        BinomialName, CellKind, GeneSymbol, GenusName, LifeStage, OrganismKind, SpecificEpithet,
        TaxonomicRank, TraitKind, TraitName, TraitValue,
    };

    #[test]
    fn facade_composes_biology_primitives_without_analysis() {
        let rank = TaxonomicRank::Species;
        let species = BinomialName::new(
            GenusName::new("Homo").expect("valid genus"),
            SpecificEpithet::new("sapiens").expect("valid epithet"),
        );
        let organism_kind = OrganismKind::Animal;
        let cell_kind = CellKind::Eukaryotic;
        let gene_symbol = GeneSymbol::new("BRCA1").expect("valid symbol");
        let trait_value =
            TraitValue::new(TraitName::new("eye color").expect("valid trait"), "brown")
                .expect("valid trait value")
                .with_kind(TraitKind::Morphological);
        let life_stage = LifeStage::Adult;

        assert_eq!(rank.to_string(), "species");
        assert_eq!(species.to_string(), "Homo sapiens");
        assert_eq!(organism_kind.to_string(), "animal");
        assert_eq!(cell_kind.to_string(), "eukaryotic");
        assert_eq!(gene_symbol.to_string(), "BRCA1");
        assert_eq!(trait_value.value(), "brown");
        assert_eq!(trait_value.kind(), Some(&TraitKind::Morphological));
        assert_eq!(life_stage.to_string(), "adult");
    }
}
