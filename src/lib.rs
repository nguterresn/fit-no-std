#![no_std]

pub mod fit;

pub use fit::FitBaseType;
pub use fit::FitError;
pub use fit::FitFieldDefinitionContent;
pub use fit::FitFile;
pub use fit::FitFileType;
pub use fit::FitGlobalMessageNumber;
pub use fit::FitLapFieldDefinitionNumber;
pub use fit::FitMessageArchitecture;
pub use fit::FitMessageType;
pub use fit::FitProtocolVersion;

#[cfg(test)]
mod tests {
    use crate::{
        FitFileType,
        fit::{
            FitBaseType, FitFieldDefinitionContent, FitFile, FitGlobalMessageNumber,
            FitLapFieldDefinitionNumber, FitMessageArchitecture, FitProtocolVersion,
        },
    };

    #[test]
    fn create_min_file() {
        let fit = FitFile::<128>::new(
            FitProtocolVersion::Version2,
            21,
            171,
            FitFileType::Activity,
            0,
        );
        assert!(fit.is_ok());

        let mut fit_file = fit.unwrap();
        assert_eq!(fit_file.size(), 33);

        let buf = fit_file.done().unwrap();
        // Header 14 bytes. + File ID def 15 bytes + File Id Data 8 bytes + 2 bytes CRC  ✅
        assert_eq!(buf.len(), 39);
        // Data Length 15 + 8  bytes (File ID data). ✅
        assert_eq!(buf[5], 23);
    }

    #[test]
    fn create_def_activity_min_file() {
        let mut fit = FitFile::<128>::new(
            FitProtocolVersion::Version2,
            21,
            171,
            FitFileType::Activity,
            0,
        )
        .unwrap();
        let result = fit.define(
            FitMessageArchitecture::LSB,
            FitGlobalMessageNumber::Lap,
            &[FitFieldDefinitionContent {
                number: FitLapFieldDefinitionNumber::StartPositionLat as u8,
                size: 4,
                base_type: FitBaseType::Sint32,
            }],
        );
        assert!(result.is_ok());
        fit.push(&123u32.to_be_bytes()).unwrap(); // StartPositionLat is 123 LSB

        let buf = fit.done().unwrap();
        // Header 14 bytes
        // File ID Def 15 bytes
        // File ID Message 8 bytes
        // Data Definition 9 bytes
        // Data Message 4 bytes
        // CRC 2 bytes
        assert_eq!(buf.len(), 53);
    }
}
