use std::time::Instant;
use std::{fs::File, io::Write};

use fit_no_std::{
    FitBaseType, FitFieldDefinitionContent, FitFile, FitFileType, FitGlobalMessageNumber,
    FitLapFieldDefinitionNumber, FitMessageArchitecture, FitProtocolVersion,
};

fn main() {
    println!("Let's try to save a .fit file!");
    let mut fit = FitFile::<128>::new(
        FitProtocolVersion::Version2,
        21,
        171,
        FitFileType::Activity,
        Instant::now().elapsed().as_secs() as u32,
    )
    .unwrap();
    fit.define(
        FitMessageArchitecture::LSB,
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

    {
        let mut file = File::create("example.fit").unwrap();
        file.write_all(buf).unwrap();
    }
}
