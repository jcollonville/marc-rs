use crate::format::MarcFormat;

/// Note fields (5XX in MARC21, 3XX in UNIMARC)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Note {
    /// 500 - General note
    GeneralNote,
    /// 501 - With note
    WithNote,
    /// 502 - Dissertation note
    DissertationNote,
    /// 504 - Bibliography, etc. note
    BibliographyNote,
    /// 505 - Formatted contents note
    FormattedContentsNote,
    /// 506 - Restrictions on access note
    RestrictionsOnAccessNote,
    /// 507 - Scale note for graphic material
    ScaleNote,
    /// 508 - Creation/production credits note
    CreationProductionCreditsNote,
    /// 510 - Citation/references note
    CitationReferencesNote,
    /// 511 - Participant or performer note
    ParticipantOrPerformerNote,
    /// 513 - Type of report and period covered note
    TypeOfReportAndPeriodCoveredNote,
    /// 514 - Data quality note
    DataQualityNote,
    /// 515 - Numbering peculiarities note
    NumberingPeculiaritiesNote,
    /// 516 - Type of computer file or data note
    TypeOfComputerFileOrDataNote,
    /// 518 - Date/time and place of an event note
    DateTimeAndPlaceOfEventNote,
    /// 520 - Summary, etc.
    Summary,
    /// 521 - Target audience note
    TargetAudienceNote,
    /// 522 - Geographic coverage note
    GeographicCoverageNote,
    /// 524 - Preferred citation of described materials note
    PreferredCitationNote,
    /// 525 - Supplement note
    SupplementNote,
    /// 526 - Study program information note
    StudyProgramInformationNote,
    /// 530 - Additional physical form available note
    AdditionalPhysicalFormAvailableNote,
    /// 533 - Reproduction note
    ReproductionNote,
    /// 534 - Original version note
    OriginalVersionNote,
    /// 535 - Location of originals/duplicates note
    LocationOfOriginalsDuplicatesNote,
    /// 536 - Funding information note
    FundingInformationNote,
    /// 538 - System details note
    SystemDetailsNote,
    /// 540 - Terms governing use and reproduction note
    TermsGoverningUseAndReproductionNote,
    /// 541 - Immediate source of acquisition note
    ImmediateSourceOfAcquisitionNote,
    /// 542 - Information relating to copyright status
    InformationRelatingToCopyrightStatus,
    /// 544 - Location of other archival materials note
    LocationOfOtherArchivalMaterialsNote,
    /// 545 - Biographical or historical data
    BiographicalOrHistoricalData,
    /// 546 - Language note
    LanguageNote,
    /// 547 - Former title complexity note
    FormerTitleComplexityNote,
    /// 550 - Issuing body note
    IssuingBodyNote,
    /// 552 - Entity and attribute information note
    EntityAndAttributeInformationNote,
    /// 555 - Cumulative index/finding aids note
    CumulativeIndexFindingAidsNote,
    /// 556 - Information about documentation note
    InformationAboutDocumentationNote,
    /// 561 - Ownership and custodial history
    OwnershipAndCustodialHistory,
    /// 562 - Copy and version identification note
    CopyAndVersionIdentificationNote,
    /// 563 - Binding information
    BindingInformation,
    /// 565 - Case file characteristics note
    CaseFileCharacteristicsNote,
    /// 567 - Methodology note
    MethodologyNote,
    /// 580 - Linking entry complexity note
    LinkingEntryComplexityNote,
    /// 581 - Publications about described materials note
    PublicationsAboutDescribedMaterialsNote,
    /// 583 - Action note
    ActionNote,
    /// 584 - Accumulation and frequency of use note
    AccumulationAndFrequencyOfUseNote,
    /// 585 - Exhibitions note
    ExhibitionsNote,
    /// 586 - Awards note
    AwardsNote,
    /// 588 - Source of description note
    SourceOfDescriptionNote,
}

impl Note {
    /// Get the tag as string for the given format
    /// Note: Most note fields are similar between MARC21 and UNIMARC (3XX block in UNIMARC)
    pub fn tag(&self, format: MarcFormat) -> &'static str {
        // For most notes, tags are the same or very similar
        // UNIMARC uses 3XX block for notes, MARC21 uses 5XX
        let tag = match self {
            Note::GeneralNote => "500",
            Note::WithNote => "501",
            Note::DissertationNote => "502",
            Note::BibliographyNote => "504",
            Note::FormattedContentsNote => "505",
            Note::RestrictionsOnAccessNote => "506",
            Note::ScaleNote => "507",
            Note::CreationProductionCreditsNote => "508",
            Note::CitationReferencesNote => "510",
            Note::ParticipantOrPerformerNote => "511",
            Note::TypeOfReportAndPeriodCoveredNote => "513",
            Note::DataQualityNote => "514",
            Note::NumberingPeculiaritiesNote => "515",
            Note::TypeOfComputerFileOrDataNote => "516",
            Note::DateTimeAndPlaceOfEventNote => "518",
            Note::Summary => "520",
            Note::TargetAudienceNote => "521",
            Note::GeographicCoverageNote => "522",
            Note::PreferredCitationNote => "524",
            Note::SupplementNote => "525",
            Note::StudyProgramInformationNote => "526",
            Note::AdditionalPhysicalFormAvailableNote => "530",
            Note::ReproductionNote => "533",
            Note::OriginalVersionNote => "534",
            Note::LocationOfOriginalsDuplicatesNote => "535",
            Note::FundingInformationNote => "536",
            Note::SystemDetailsNote => "538",
            Note::TermsGoverningUseAndReproductionNote => "540",
            Note::ImmediateSourceOfAcquisitionNote => "541",
            Note::InformationRelatingToCopyrightStatus => "542",
            Note::LocationOfOtherArchivalMaterialsNote => "544",
            Note::BiographicalOrHistoricalData => "545",
            Note::LanguageNote => "546",
            Note::FormerTitleComplexityNote => "547",
            Note::IssuingBodyNote => "550",
            Note::EntityAndAttributeInformationNote => "552",
            Note::CumulativeIndexFindingAidsNote => "555",
            Note::InformationAboutDocumentationNote => "556",
            Note::OwnershipAndCustodialHistory => "561",
            Note::CopyAndVersionIdentificationNote => "562",
            Note::BindingInformation => "563",
            Note::CaseFileCharacteristicsNote => "565",
            Note::MethodologyNote => "567",
            Note::LinkingEntryComplexityNote => "580",
            Note::PublicationsAboutDescribedMaterialsNote => "581",
            Note::ActionNote => "583",
            Note::AccumulationAndFrequencyOfUseNote => "584",
            Note::ExhibitionsNote => "585",
            Note::AwardsNote => "586",
            Note::SourceOfDescriptionNote => "588",
        };

        // Most notes are the same in both formats
        // Some specific mappings could be added here if needed
        match format {
            MarcFormat::Marc21 | MarcFormat::MarcXml => tag,
            MarcFormat::Unimarc => {
                // In UNIMARC, notes are in 3XX block
                // Most correspond to similar tags, but we keep MARC21 tags for simplicity
                // A full mapping would require detailed UNIMARC specification
                tag
            }
        }
    }
}
