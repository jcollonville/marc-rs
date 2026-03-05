use crate::formats::FormatDescriptor;
use crate::record::{ControlField, DataField};

/// Trait implemented by each block struct.
/// A block can receive parsed fields and emit raw MARC fields for writing.
pub trait MarcBlock: Default {
    /// Dispatch a parsed data field into the appropriate named field of this block.
    fn dispatch_data(
        &mut self,
        tag: &str,
        ind1: char,
        ind2: char,
        subfields: &[(char, String)],
        descriptor: &dyn FormatDescriptor,
    );

    /// Dispatch a parsed control field into the appropriate named field of this block.
    fn dispatch_control(&mut self, tag: &str, value: &str, descriptor: &dyn FormatDescriptor);

    /// Collect all data fields for writing.
    fn collect_data_fields(&self, descriptor: &dyn FormatDescriptor) -> Vec<DataField>;

    /// Collect all control fields for writing.
    fn collect_control_fields(&self, descriptor: &dyn FormatDescriptor) -> Vec<ControlField>;
}

pub mod identification;
pub mod coded_information;
pub mod description;
pub mod notes;
pub mod links;
pub mod associated_titles;
pub mod subject_analysis;
pub mod intellectual_responsibility;
pub mod international_use;
pub mod local_use;

pub use identification::IdentificationBlock;
pub use coded_information::CodedInformationBlock;
pub use description::DescriptionBlock;
pub use notes::NotesBlock;
pub use links::LinksBlock;
pub use associated_titles::AssociatedTitlesBlock;
pub use subject_analysis::SubjectAnalysisBlock;
pub use intellectual_responsibility::IntellectualResponsibilityBlock;
pub use international_use::InternationalUseBlock;
pub use local_use::LocalUseBlock;
