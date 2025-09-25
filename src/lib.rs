#![no_std]

pub mod fit_file;

#[cfg(test)]
mod tests {
    use crate::fit_file::{
        FitBaseType, FitFieldDefinitionContent, FitFile, FitGlobalMessageNumber,
        FitLapFieldDefinitionNumber, FitMessageArchitecture, FitMessageType,
    };

    #[test]
    fn create_min_file() {
        let fit = FitFile::<128>::new();
        assert!(fit.is_ok());

        let mut fit_file = fit.unwrap();
        assert_eq!(fit_file.size(), 10);

        let buf = fit_file.done().unwrap();
        // Header 14 bytes. + 2 bytes CRC  ✅
        assert_eq!(buf.len(), 16);
        // Data Length 0 bytes. ✅
        assert_eq!(buf[5], 0);
    }

    #[test]
    fn create_def_min_file() {
        let mut fit = FitFile::<128>::new().unwrap();
        let defs: [FitFieldDefinitionContent; 1] = [FitFieldDefinitionContent {
            number: FitLapFieldDefinitionNumber::StartPositionLat as u8,
            size: 4,
            base_type: FitBaseType::Sint32,
        }];
        let result = fit.define(
            FitMessageType::DefinitionMessage,
            FitMessageArchitecture::LSB,
            FitGlobalMessageNumber::Lap,
            &defs,
        );
        assert!(result.is_ok());
        fit.push(&123u32.to_be_bytes()).unwrap(); // StartPositionLat is 123 LSB

        let buf = fit.done().unwrap();
        // Header 14 bytes
        // Data Definition 9 bytes
        // Data Message 4 bytes
        // CRC 2 bytes
        assert_eq!(buf.len(), 30);
    }
}
