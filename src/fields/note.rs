use serde::{Deserialize, Serialize};

use crate::fields::common::*;
use crate::format::MarcFormat;
use crate::record::DataField;

/// Note fields (5XX in MARC21, 3XX in UNIMARC)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Note {
    GeneralNote(NoteData),
    WithNote(NoteData),
    DissertationNote(NoteData),
    BibliographyNote(NoteData),
    FormattedContentsNote(NoteData),
    RestrictionsOnAccessNote(NoteData),
    ScaleNote(NoteData),
    CreationProductionCreditsNote(NoteData),
    CitationReferencesNote(NoteData),
    ParticipantOrPerformerNote(NoteData),
    TypeOfReportAndPeriodCoveredNote(NoteData),
    DataQualityNote(NoteData),
    NumberingPeculiaritiesNote(NoteData),
    TypeOfComputerFileOrDataNote(NoteData),
    DateTimeAndPlaceOfEventNote(NoteData),
    Summary(NoteData),
    TargetAudienceNote(NoteData),
    GeographicCoverageNote(NoteData),
    PreferredCitationNote(NoteData),
    SupplementNote(NoteData),
    StudyProgramInformationNote(NoteData),
    AdditionalPhysicalFormAvailableNote(NoteData),
    ReproductionNote(NoteData),
    OriginalVersionNote(NoteData),
    LocationOfOriginalsDuplicatesNote(NoteData),
    FundingInformationNote(NoteData),
    SystemDetailsNote(NoteData),
    TermsGoverningUseAndReproductionNote(NoteData),
    ImmediateSourceOfAcquisitionNote(NoteData),
    InformationRelatingToCopyrightStatus(NoteData),
    LocationOfOtherArchivalMaterialsNote(NoteData),
    BiographicalOrHistoricalData(NoteData),
    LanguageNote(NoteData),
    FormerTitleComplexityNote(NoteData),
    IssuingBodyNote(NoteData),
    EntityAndAttributeInformationNote(NoteData),
    CumulativeIndexFindingAidsNote(NoteData),
    InformationAboutDocumentationNote(NoteData),
    OwnershipAndCustodialHistory(NoteData),
    CopyAndVersionIdentificationNote(NoteData),
    BindingInformation(NoteData),
    CaseFileCharacteristicsNote(NoteData),
    MethodologyNote(NoteData),
    LinkingEntryComplexityNote(NoteData),
    PublicationsAboutDescribedMaterialsNote(NoteData),
    ActionNote(NoteData),
    AccumulationAndFrequencyOfUseNote(NoteData),
    ExhibitionsNote(NoteData),
    AwardsNote(NoteData),
    SourceOfDescriptionNote(NoteData),
}

impl Note {
    pub fn tag(&self, format: MarcFormat) -> &'static str {
        let tag = match self {
            Note::GeneralNote(_) => "500",
            Note::WithNote(_) => "501",
            Note::DissertationNote(_) => "502",
            Note::BibliographyNote(_) => "504",
            Note::FormattedContentsNote(_) => "505",
            Note::RestrictionsOnAccessNote(_) => "506",
            Note::ScaleNote(_) => "507",
            Note::CreationProductionCreditsNote(_) => "508",
            Note::CitationReferencesNote(_) => "510",
            Note::ParticipantOrPerformerNote(_) => "511",
            Note::TypeOfReportAndPeriodCoveredNote(_) => "513",
            Note::DataQualityNote(_) => "514",
            Note::NumberingPeculiaritiesNote(_) => "515",
            Note::TypeOfComputerFileOrDataNote(_) => "516",
            Note::DateTimeAndPlaceOfEventNote(_) => "518",
            Note::Summary(_) => "520",
            Note::TargetAudienceNote(_) => "521",
            Note::GeographicCoverageNote(_) => "522",
            Note::PreferredCitationNote(_) => "524",
            Note::SupplementNote(_) => "525",
            Note::StudyProgramInformationNote(_) => "526",
            Note::AdditionalPhysicalFormAvailableNote(_) => "530",
            Note::ReproductionNote(_) => "533",
            Note::OriginalVersionNote(_) => "534",
            Note::LocationOfOriginalsDuplicatesNote(_) => "535",
            Note::FundingInformationNote(_) => "536",
            Note::SystemDetailsNote(_) => "538",
            Note::TermsGoverningUseAndReproductionNote(_) => "540",
            Note::ImmediateSourceOfAcquisitionNote(_) => "541",
            Note::InformationRelatingToCopyrightStatus(_) => "542",
            Note::LocationOfOtherArchivalMaterialsNote(_) => "544",
            Note::BiographicalOrHistoricalData(_) => "545",
            Note::LanguageNote(_) => "546",
            Note::FormerTitleComplexityNote(_) => "547",
            Note::IssuingBodyNote(_) => "550",
            Note::EntityAndAttributeInformationNote(_) => "552",
            Note::CumulativeIndexFindingAidsNote(_) => "555",
            Note::InformationAboutDocumentationNote(_) => "556",
            Note::OwnershipAndCustodialHistory(_) => "561",
            Note::CopyAndVersionIdentificationNote(_) => "562",
            Note::BindingInformation(_) => "563",
            Note::CaseFileCharacteristicsNote(_) => "565",
            Note::MethodologyNote(_) => "567",
            Note::LinkingEntryComplexityNote(_) => "580",
            Note::PublicationsAboutDescribedMaterialsNote(_) => "581",
            Note::ActionNote(_) => "583",
            Note::AccumulationAndFrequencyOfUseNote(_) => "584",
            Note::ExhibitionsNote(_) => "585",
            Note::AwardsNote(_) => "586",
            Note::SourceOfDescriptionNote(_) => "588",
        };
        match format {
            MarcFormat::Marc21 | MarcFormat::MarcXml | MarcFormat::Unimarc => tag,
        }
    }

    fn data(&self) -> &NoteData {
        match self {
            Note::GeneralNote(d)
            | Note::WithNote(d)
            | Note::DissertationNote(d)
            | Note::BibliographyNote(d)
            | Note::FormattedContentsNote(d)
            | Note::RestrictionsOnAccessNote(d)
            | Note::ScaleNote(d)
            | Note::CreationProductionCreditsNote(d)
            | Note::CitationReferencesNote(d)
            | Note::ParticipantOrPerformerNote(d)
            | Note::TypeOfReportAndPeriodCoveredNote(d)
            | Note::DataQualityNote(d)
            | Note::NumberingPeculiaritiesNote(d)
            | Note::TypeOfComputerFileOrDataNote(d)
            | Note::DateTimeAndPlaceOfEventNote(d)
            | Note::Summary(d)
            | Note::TargetAudienceNote(d)
            | Note::GeographicCoverageNote(d)
            | Note::PreferredCitationNote(d)
            | Note::SupplementNote(d)
            | Note::StudyProgramInformationNote(d)
            | Note::AdditionalPhysicalFormAvailableNote(d)
            | Note::ReproductionNote(d)
            | Note::OriginalVersionNote(d)
            | Note::LocationOfOriginalsDuplicatesNote(d)
            | Note::FundingInformationNote(d)
            | Note::SystemDetailsNote(d)
            | Note::TermsGoverningUseAndReproductionNote(d)
            | Note::ImmediateSourceOfAcquisitionNote(d)
            | Note::InformationRelatingToCopyrightStatus(d)
            | Note::LocationOfOtherArchivalMaterialsNote(d)
            | Note::BiographicalOrHistoricalData(d)
            | Note::LanguageNote(d)
            | Note::FormerTitleComplexityNote(d)
            | Note::IssuingBodyNote(d)
            | Note::EntityAndAttributeInformationNote(d)
            | Note::CumulativeIndexFindingAidsNote(d)
            | Note::InformationAboutDocumentationNote(d)
            | Note::OwnershipAndCustodialHistory(d)
            | Note::CopyAndVersionIdentificationNote(d)
            | Note::BindingInformation(d)
            | Note::CaseFileCharacteristicsNote(d)
            | Note::MethodologyNote(d)
            | Note::LinkingEntryComplexityNote(d)
            | Note::PublicationsAboutDescribedMaterialsNote(d)
            | Note::ActionNote(d)
            | Note::AccumulationAndFrequencyOfUseNote(d)
            | Note::ExhibitionsNote(d)
            | Note::AwardsNote(d)
            | Note::SourceOfDescriptionNote(d) => d,
        }
    }

    pub fn try_parse(
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        format: MarcFormat,
    ) -> Option<Self> {
        let d = NoteData::from_subfields(ind1, ind2, subfields, format)?;
        let note = match tag {
            "500" => Note::GeneralNote(d),
            "501" => Note::WithNote(d),
            "502" => Note::DissertationNote(d),
            "504" => Note::BibliographyNote(d),
            "505" => Note::FormattedContentsNote(d),
            "506" => Note::RestrictionsOnAccessNote(d),
            "507" => Note::ScaleNote(d),
            "508" => Note::CreationProductionCreditsNote(d),
            "510" => Note::CitationReferencesNote(d),
            "511" => Note::ParticipantOrPerformerNote(d),
            "513" => Note::TypeOfReportAndPeriodCoveredNote(d),
            "514" => Note::DataQualityNote(d),
            "515" => Note::NumberingPeculiaritiesNote(d),
            "516" => Note::TypeOfComputerFileOrDataNote(d),
            "518" => Note::DateTimeAndPlaceOfEventNote(d),
            "520" => Note::Summary(d),
            "521" => Note::TargetAudienceNote(d),
            "522" => Note::GeographicCoverageNote(d),
            "524" => Note::PreferredCitationNote(d),
            "525" => Note::SupplementNote(d),
            "526" => Note::StudyProgramInformationNote(d),
            "530" => Note::AdditionalPhysicalFormAvailableNote(d),
            "533" => Note::ReproductionNote(d),
            "534" => Note::OriginalVersionNote(d),
            "535" => Note::LocationOfOriginalsDuplicatesNote(d),
            "536" => Note::FundingInformationNote(d),
            "538" => Note::SystemDetailsNote(d),
            "540" => Note::TermsGoverningUseAndReproductionNote(d),
            "541" => Note::ImmediateSourceOfAcquisitionNote(d),
            "542" => Note::InformationRelatingToCopyrightStatus(d),
            "544" => Note::LocationOfOtherArchivalMaterialsNote(d),
            "545" => Note::BiographicalOrHistoricalData(d),
            "546" => Note::LanguageNote(d),
            "547" => Note::FormerTitleComplexityNote(d),
            "550" => Note::IssuingBodyNote(d),
            "552" => Note::EntityAndAttributeInformationNote(d),
            "555" => Note::CumulativeIndexFindingAidsNote(d),
            "556" => Note::InformationAboutDocumentationNote(d),
            "561" => Note::OwnershipAndCustodialHistory(d),
            "562" => Note::CopyAndVersionIdentificationNote(d),
            "563" => Note::BindingInformation(d),
            "565" => Note::CaseFileCharacteristicsNote(d),
            "567" => Note::MethodologyNote(d),
            "580" => Note::LinkingEntryComplexityNote(d),
            "581" => Note::PublicationsAboutDescribedMaterialsNote(d),
            "583" => Note::ActionNote(d),
            "584" => Note::AccumulationAndFrequencyOfUseNote(d),
            "585" => Note::ExhibitionsNote(d),
            "586" => Note::AwardsNote(d),
            "588" => Note::SourceOfDescriptionNote(d),
            _ => return None,
        };
        Some(note)
    }

    pub fn to_raw(&self, format: MarcFormat) -> DataField {
        let tag = self.tag(format);
        let d = self.data();
        to_data_field(tag, ' ', ' ', d.to_subfields())
    }
}
