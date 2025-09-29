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

    let mut fit = FitFile::<1024>::new(
        FitProtocolVersion::Version2,
        21,
        171,
        FitFileType::Activity,
        ts as u32,
    )
    .unwrap();

    // Define an Activity [required]
    fit.define(
        FitGlobalMessageType::Activity,
        &[
            FitActivityFieldDefinition::Timestamp,
            FitActivityFieldDefinition::TotalTimerTime,
            FitActivityFieldDefinition::NumSessions,
            FitActivityFieldDefinition::LocalTimestamp,
        ],
    )
    .unwrap();
    // Define an Session [required]
    fit.define(
        FitGlobalMessageType::Session,
        &[
            FitActivityFieldDefinition::Timestamp,
            FitActivityFieldDefinition::TotalTimerTime,
            FitActivityFieldDefinition::NumSessions,
            FitActivityFieldDefinition::LocalTimestamp,
        ],
    )
    .unwrap();

    // Start Time, Total Elapsed Time, Total Timer Time, and Timestamp fields are required for all Summary messages.
    fit.define(
        FitGlobalMessageType::Lap,
        &[
            FitLapFieldDefinition::Timestamp,
            FitLapFieldDefinition::StartTime,
            FitLapFieldDefinition::TotalElapsedTime,
            FitLapFieldDefinition::TotalTimerTime,
            FitLapFieldDefinition::TotalDistance,
        ],
    )
    .unwrap();

    // 1ST LAP (start at +1, takes 200s, 150s of running)
    let mut data: [u8; 20] = [0; 20];
    data[0..4].copy_from_slice(&(ts).to_le_bytes()); // TimeStamp
    data[4..8].copy_from_slice(&(ts).to_le_bytes()); // Start time
    data[8..12].copy_from_slice(&(300u32 * 1000).to_le_bytes()); // Elapsed Time
    data[12..16].copy_from_slice(&(300u32 * 1000).to_le_bytes()); // Timer Time
    data[16..20].copy_from_slice(&(1000u32 * 100).to_le_bytes()); // Distance
    fit.push(&data).unwrap();
    // 2ND LAP
    data[0..4].copy_from_slice(&(ts + 300).to_le_bytes()); // TimeStamp
    data[4..8].copy_from_slice(&(ts + 300).to_le_bytes()); // Start time
    data[8..12].copy_from_slice(&(300u32 * 1000).to_le_bytes()); // Elapsed Time
    data[12..16].copy_from_slice(&(300u32 * 1000).to_le_bytes()); // Timer Time
    data[16..20].copy_from_slice(&(1100u32 * 100).to_le_bytes()); // Distance
    fit.push(&data).unwrap();

    // Wrap up Activity
    let mut data: [u8; 14] = [0; 14];
    data[0..4].copy_from_slice(&(ts).to_le_bytes());
    data[4] = FitEventType::Activity as u8;
    data[5] = FitEventTypeType::Start as u8;
    fit.push(&data).unwrap();

    let buf = fit.done().unwrap();
    // println!("{:02X?}", buf);

    let mut file = File::create("activity.fit").unwrap();
    file.write_all(buf).unwrap();
}
