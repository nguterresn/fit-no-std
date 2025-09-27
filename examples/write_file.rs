use std::{fs::File, io::Write};

use fit_no_std::{
    FitBaseType, FitFieldDefinitionContent, FitFile, FitFileType, FitGlobalMessageNumber,
    FitLapFieldDefinitionNumber, FitProtocolVersion,
};

fn main() {
    let ts: u32 = 1758989430;
    println!("Let's try to save a .fit file! Timestamp: {}", ts);
    let mut fit = FitFile::<128>::new(
        FitProtocolVersion::Version2,
        21,
        171,
        FitFileType::Activity,
        ts as u32,
    )
    .unwrap();
    fit.define(
        FitGlobalMessageNumber::Lap,
        &[FitFieldDefinitionContent {
            number: FitLapFieldDefinitionNumber::StartPositionLat as u8,
            size: 4,
            base_type: FitBaseType::Sint32,
        }],
    )
    .unwrap();
    fit.push(&123u32.to_be_bytes()).unwrap(); // StartPositionLat is 123 LSB

    let buf = fit.done().unwrap();
    println!("{:02X?}", buf);

    {
        let mut file = File::create("example.fit").unwrap();
        file.write_all(buf).unwrap();
    }
}

// [
// 14, 16, 179, 82, 46, 37, 18, 9, 4, 70, 73, 84, 0, 0, 64, 0, 0, 0, 0, 3, 0, 1, 2, 1, 2, 4, 4, 4, 6, 0, 4, 255, 0, 118, 12, 216, 104, 64, 0, 0, 19, 0, 1, 3, 4, 5, 0, 0, 0, 0, 123, 51, 245]
