#![allow(dead_code)]

use std::{fs::File, io::Write};

use fit_no_std::{
    FitActivityFieldDefinition, FitEventType, FitEventTypeType, FitFile, FitFileIdFieldDefinition,
    FitFileType, FitGlobalMessageType, FitLapFieldDefinition, FitProtocolVersion,
};

// > The required message types for an Activity file are File Id, Activity,
// > Session, Lap, and Record messages. Device Info and Event messages are not
// > required, but it is considered a best practice to include them.

fn main() {
    let ts: u32 = 1759162622;
    println!("Let's try to save a .fit file! Timestamp: {}", ts);

    let mut fit = FitFile::<128>::new(
        FitProtocolVersion::Version2,
        21,
        171,
        FitFileType::Activity,
        ts as u32,
    )
    .unwrap();

    // Define an Session [required]
    // Define an Activity [required]
    fit.define(
        FitGlobalMessageType::Activity,
        &[
            FitActivityFieldDefinition::Timestamp,
            FitActivityFieldDefinition::Event,
            FitActivityFieldDefinition::EventType,
        ],
    )
    .unwrap();

    let mut data: [u8; 6] = [0; 6];
    // Event Start
    data[0..4].copy_from_slice(&(ts).to_le_bytes());
    data[4] = FitEventType::Activity as u8;
    data[5] = FitEventTypeType::Start as u8;
    fit.push(&data).unwrap();

    // Start Time, Total Elapsed Time, Total Timer Time, and Timestamp fields are required for all Summary messages.
    fit.define(
        FitGlobalMessageType::Lap,
        &[
            FitLapFieldDefinition::Timestamp,
            FitLapFieldDefinition::StartTime,
            FitLapFieldDefinition::TotalElapsedTime,
            FitLapFieldDefinition::TotalTimerTime,
        ],
    )
    .unwrap();

    // Lap (start at +1, takes 200s, 150s of running)
    let mut data: [u8; 16] = [0; 16];
    data[0..4].copy_from_slice(&(ts + 1).to_le_bytes());
    data[4..8].copy_from_slice(&(ts + 1).to_le_bytes());
    data[8..12].copy_from_slice(&(200u32).to_le_bytes());
    data[12..16].copy_from_slice(&(150u32).to_le_bytes());
    fit.push(&data).unwrap();

    let buf = fit.done().unwrap();
    // println!("{:02X?}", buf);

    let mut file = File::create("activity.fit").unwrap();
    file.write_all(buf).unwrap();
}
