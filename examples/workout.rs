use std::{fs::File, io::Write};

use fit_no_std::{
    FitBaseType, FitFieldDefinitionContent, FitFile, FitFileType, FitGlobalMessageNumber,
    FitProtocolVersion,
    fit::{
        FitSportType, FitWorkoutFieldDefinitionNumber, FitWorkoutStepDuration,
        FitWorkoutStepFieldDefinitionNumber, FitWorkoutStepTarget,
    },
};

fn main() {
    let ts: u32 = 1758989430;
    println!("Let's try to save a Workout .fit file! Timestamp: {}", ts);

    // An Workout file has:
    // - File Id [1]
    // - Workout [1]
    // - Woukout Step [1 .. N]
    let mut fit = FitFile::<128>::new(
        FitProtocolVersion::Version2,
        21,
        171,
        FitFileType::Workout,
        ts as u32,
    )
    .unwrap();

    // Define sport, num_valid_steps
    fit.define(
        FitGlobalMessageNumber::Workout,
        &[
            FitFieldDefinitionContent {
                number: FitWorkoutFieldDefinitionNumber::Sport as u8,
                size: 1,
                base_type: FitBaseType::Enum,
            },
            FitFieldDefinitionContent {
                number: FitWorkoutFieldDefinitionNumber::NumValidSteps as u8,
                size: 2,
                base_type: FitBaseType::Uint16,
            },
        ],
    )
    .unwrap();

    // Push sport, num_valid_steps
    let mut data: [u8; 3] = [0; 3];
    data[0] = FitSportType::Basketball as u8;
    data[1..3].copy_from_slice(&(1u16).to_le_bytes()); // 1 step?
    fit.push(&data).unwrap();

    // Define message_index, duration_type, duration_value, target_type
    fit.define(
        FitGlobalMessageNumber::WorkoutStep,
        &[
            FitFieldDefinitionContent {
                number: FitWorkoutStepFieldDefinitionNumber::MessageIndex as u8,
                size: 1,
                base_type: FitBaseType::Uint8,
            },
            FitFieldDefinitionContent {
                number: FitWorkoutStepFieldDefinitionNumber::DurationType as u8,
                size: 1,
                base_type: FitBaseType::Enum, // wkt_step_duration
            },
            FitFieldDefinitionContent {
                number: FitWorkoutStepFieldDefinitionNumber::DurationValue as u8,
                size: 4,
                base_type: FitBaseType::Uint32,
            },
            FitFieldDefinitionContent {
                number: FitWorkoutStepFieldDefinitionNumber::TargetType as u8,
                size: 1,
                base_type: FitBaseType::Enum, // wkt_step_target
            },
        ],
    )
    .unwrap();

    let mut data: [u8; 7] = [0; 7];
    data[0] = 0;
    data[1] = FitWorkoutStepDuration::Time as u8;
    data[2..6].copy_from_slice(&(1u32).to_le_bytes()); // 1 step? // Scale is 1000, 1 * 1000?
    data[6] = FitWorkoutStepTarget::Speed as u8;

    fit.push(&data).unwrap();
    let buf = fit.done().unwrap();
    // println!("{:02X?}", buf);

    let mut file = File::create("workout.fit").unwrap();
    file.write_all(buf).unwrap();
}
