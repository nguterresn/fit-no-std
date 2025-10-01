use std::{fs::File, io::Write};

use fit_no_std::{
    FitFile, FitFileType, FitGlobalMessageType, FitSportType, FitWorkoutFieldDefinition,
    FitWorkoutStepDuration, FitWorkoutStepFieldDefinition, FitWorkoutStepTarget,
};

fn main() {
    let ts: u32 = 1758989430;
    println!("Let's try to save a Workout .fit file! Timestamp: {}", ts);

    // An Workout file has:
    // - File Id [1]
    // - Workout [1]
    // - Woukout Step [1 .. N]
    let mut fit = FitFile::<128>::new(FitFileType::Workout, ts as u32).unwrap();

    // Define sport, num_valid_steps
    fit.define(
        FitGlobalMessageType::Workout,
        &[
            FitWorkoutFieldDefinition::Sport,
            FitWorkoutFieldDefinition::NumValidSteps,
        ],
    )
    .unwrap();

    // Push sport, num_valid_steps
    let mut data: [u8; 3] = [0; 3];
    data[0] = FitSportType::Basketball as u8;
    data[1..3].copy_from_slice(&(2u16).to_le_bytes()); // 2 steps?
    fit.push(&data).unwrap();

    // Define message_index, duration_type, duration_value, target_type
    fit.define(
        FitGlobalMessageType::WorkoutStep,
        &[
            FitWorkoutStepFieldDefinition::MessageIndex,
            FitWorkoutStepFieldDefinition::DurationType,
            FitWorkoutStepFieldDefinition::DurationValue,
            FitWorkoutStepFieldDefinition::TargetType,
        ],
    )
    .unwrap();

    // 2 Workout Steps
    let mut data: [u8; 7] = [0; 7];
    data[0] = 0;
    data[1] = FitWorkoutStepDuration::Time as u8;
    data[2..6].copy_from_slice(&(1u32).to_le_bytes()); // Scale is 1000, 1 * 1000?
    data[6] = FitWorkoutStepTarget::Speed as u8;
    fit.push(&data).unwrap();

    data[0] = 1;
    data[1] = FitWorkoutStepDuration::Time as u8;
    data[2..6].copy_from_slice(&(2u32).to_le_bytes()); // Scale is 1000, 2 * 1000?
    data[6] = FitWorkoutStepTarget::Speed as u8;
    fit.push(&data).unwrap();

    let buf = fit.done().unwrap();
    // println!("{:02X?}", buf);

    let mut file = File::create("workout.fit").unwrap();
    file.write_all(buf).unwrap();
}
