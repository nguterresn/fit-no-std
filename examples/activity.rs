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

    // Define timestamp, start_pos_lat
    fit.define(
        FitGlobalMessageNumber::Lap,
        &[
            FitFieldDefinitionContent {
                number: FitLapFieldDefinitionNumber::Timestamp as u8,
                size: 4,
                base_type: FitBaseType::Uint32,
            },
            FitFieldDefinitionContent {
                number: FitLapFieldDefinitionNumber::StartPositionLat as u8,
                size: 4,
                base_type: FitBaseType::Sint32,
            },
        ],
    )
    .unwrap();

    // Push timestamp, start_pos_lat
    let mut data: [u8; 8] = [0; 8];
    data[0..4].copy_from_slice(&(ts).to_le_bytes());
    data[4..8].copy_from_slice(&(123u32).to_le_bytes());
    fit.push(&data).unwrap();

    let buf = fit.done().unwrap();
    // println!("{:02X?}", buf);

    let mut file = File::create("activity.fit").unwrap();
    file.write_all(buf).unwrap();
}
