#![no_std]

pub mod fit;
pub mod profile;
pub mod types;

pub use fit::{FitError, FitFile, FitMessageArchitecture, FitProtocolVersion};
pub use profile::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use crate::{
        FitFile, FitFileType, FitGlobalMessageType, FitLapFieldDefinition, FitProtocolVersion,
    };

    #[test]
    fn create_min_file() {
        let fit = FitFile::<128>::new(FitFileType::Activity, 0);
        assert!(fit.is_ok());

        let mut fit_file = fit.unwrap();
        assert_eq!(fit_file.size(), 33);

        let buf = fit_file.done().unwrap();
        // Header 14 bytes. + File ID def 15 bytes + File Id Data 8 bytes + 2 bytes CRC  ✅
        assert_eq!(buf.len(), 39);
        // Data Length 15 + 8  bytes (File ID data). ✅
        assert_eq!(buf[4], 23);
    }

    #[test]
    fn create_def_activity_min_file() {
        let mut fit = FitFile::<128>::new(FitFileType::Activity, 0).unwrap();
        let result = fit.define(
            FitGlobalMessageType::Lap,
            &[FitLapFieldDefinition::StartPositionLat],
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
