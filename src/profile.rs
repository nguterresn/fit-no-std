use crate::types::FitBaseType;

pub trait FitFieldDefinition {
    fn base_type(&self) -> FitBaseType;
    fn field_number(&self) -> u8;

    fn get(&self) -> [u8; 3] {
        [
            self.field_number(),
            self.base_type().size() as u8,
            self.base_type() as u8,
        ]
    }
}

///////////////////////////
/////  FILE SETTINGS  /////
///////////////////////////

pub enum FitFileIdFieldDefinition {
    Type,         // Required
    Manufacturer, // Required
    Product,
    SerialNumber,
    TimeCreated, // Required
    Number,
    ProductName,
}

impl FitFieldDefinition for FitFileIdFieldDefinition {
    fn base_type(&self) -> FitBaseType {
        match self {
            Self::Type => FitBaseType::Enum,
            Self::Manufacturer => FitBaseType::Uint16,
            Self::Product => FitBaseType::Uint16,
            Self::SerialNumber => FitBaseType::Uint32z,
            Self::TimeCreated => FitBaseType::Uint32,
            Self::Number => FitBaseType::Uint16,
            Self::ProductName => FitBaseType::String,
        }
    }

    fn field_number(&self) -> u8 {
        match self {
            Self::Type => 0,
            Self::Manufacturer => 1,
            Self::Product => 2,
            Self::SerialNumber => 3,
            Self::TimeCreated => 4,
            Self::Number => 5,
            Self::ProductName => 8,
        }
    }
}

////////////////////////////
// ACTIVITY FILE SETTINGS //
////////////////////////////

pub enum FitActivityFieldDefinition {
    Timestamp,
    TotalTimerTime,
    NumSessions,
    Type,
    Event,
    EventType,
    LocalTimestamp,
    EventGroup,
}

impl FitFieldDefinition for FitActivityFieldDefinition {
    fn base_type(&self) -> FitBaseType {
        match self {
            Self::Timestamp => FitBaseType::Uint32,
            Self::TotalTimerTime => FitBaseType::Uint32,
            Self::NumSessions => FitBaseType::Uint16,
            Self::Type => FitBaseType::Enum,
            Self::Event => FitBaseType::Enum,
            Self::EventType => FitBaseType::Enum,
            Self::LocalTimestamp => FitBaseType::Uint32,
            Self::EventGroup => FitBaseType::Uint8,
        }
    }

    fn field_number(&self) -> u8 {
        match self {
            Self::Timestamp => 253,
            Self::TotalTimerTime => 0,
            Self::NumSessions => 1,
            Self::Type => 2,
            Self::Event => 3,
            Self::EventType => 4,
            Self::LocalTimestamp => 5,
            Self::EventGroup => 6,
        }
    }
}

pub enum FitSessionFieldDefinition {
    Timestamp,
    StartTime,
    TotalElapsedTime,
    TotalTimerTime,
    TotalDistance,
    MessageIndex,
}

impl FitFieldDefinition for FitSessionFieldDefinition {
    fn base_type(&self) -> FitBaseType {
        match self {
            Self::Timestamp => FitBaseType::Uint32,
            Self::StartTime => FitBaseType::Uint32,
            Self::TotalElapsedTime => FitBaseType::Uint32,
            Self::TotalTimerTime => FitBaseType::Uint32,
            Self::TotalDistance => FitBaseType::Uint32,
            Self::MessageIndex => FitBaseType::Uint16,
        }
    }

    fn field_number(&self) -> u8 {
        match self {
            Self::StartTime => 2,
            Self::TotalElapsedTime => 7,
            Self::TotalTimerTime => 8,
            Self::TotalDistance => 9,
            Self::Timestamp => 253,
            Self::MessageIndex => 254,
        }
    }
}

// Start Time, Total Elapsed Time, Total Timer Time, and Timestamp fields are required for all Summary messages.
pub enum FitLapFieldDefinition {
    StartTime,
    StartPositionLat,
    StartPositionLong,
    EndPositionLat,
    EndPositionLong,
    TotalElapsedTime,
    TotalTimerTime,
    TotalDistance,
    TotalCalories,
    AverageSpeed,
    MaxSpeed,
    AverageHeartRate,
    MaxHeartRate,
    Sport,
    SubSport,
    Timestamp,
    MessageIndex,
}

impl FitFieldDefinition for FitLapFieldDefinition {
    fn base_type(&self) -> FitBaseType {
        match self {
            Self::StartTime => FitBaseType::Uint32,
            Self::StartPositionLat => FitBaseType::Sint32,
            Self::StartPositionLong => FitBaseType::Sint32,
            Self::EndPositionLat => FitBaseType::Sint32,
            Self::EndPositionLong => FitBaseType::Sint32,
            Self::TotalElapsedTime => FitBaseType::Uint32,
            Self::TotalTimerTime => FitBaseType::Uint32,
            Self::TotalDistance => FitBaseType::Uint32,
            Self::TotalCalories => FitBaseType::Uint16,
            Self::AverageSpeed => FitBaseType::Uint16,
            Self::MaxSpeed => FitBaseType::Uint16,
            Self::AverageHeartRate => FitBaseType::Uint8,
            Self::MaxHeartRate => FitBaseType::Uint8,
            Self::Sport => FitBaseType::Enum,
            Self::SubSport => FitBaseType::Enum,
            Self::Timestamp => FitBaseType::Uint32,
            Self::MessageIndex => FitBaseType::Uint16,
        }
    }

    fn field_number(&self) -> u8 {
        match self {
            Self::StartTime => 2,
            Self::StartPositionLat => 3,
            Self::StartPositionLong => 4,
            Self::EndPositionLat => 5,
            Self::EndPositionLong => 6,
            Self::TotalElapsedTime => 7,
            Self::TotalTimerTime => 8,
            Self::TotalDistance => 9,
            Self::TotalCalories => 11,
            Self::AverageSpeed => 13,
            Self::MaxSpeed => 14,
            Self::AverageHeartRate => 15,
            Self::MaxHeartRate => 16,
            Self::Sport => 25,
            Self::SubSport => 39,
            Self::Timestamp => 253,
            Self::MessageIndex => 254,
        }
    }
}

pub enum FitEventFieldDefinition {
    Timestamp,
    Event,
    EventType,
    Data16,
    Data,
}

impl FitFieldDefinition for FitEventFieldDefinition {
    fn base_type(&self) -> FitBaseType {
        match self {
            Self::Timestamp => FitBaseType::Uint32,
            Self::Event => FitBaseType::Enum,
            Self::EventType => FitBaseType::Enum,
            Self::Data16 => FitBaseType::Uint16,
            Self::Data => FitBaseType::Uint32,
        }
    }

    fn field_number(&self) -> u8 {
        match self {
            Self::Timestamp => 253,
            Self::Event => 0,
            Self::EventType => 1,
            Self::Data16 => 2,
            Self::Data => 3,
        }
    }
}

pub enum FitRecordFieldDefinition {
    PositionLat,
    PositionLong,
    Altitude,
    HeartRate,
    Speed,
    Timestamp,
}

impl FitFieldDefinition for FitRecordFieldDefinition {
    fn base_type(&self) -> FitBaseType {
        match self {
            Self::PositionLat => FitBaseType::Sint32,
            Self::PositionLong => FitBaseType::Sint32,
            Self::Altitude => FitBaseType::Uint16,
            Self::HeartRate => FitBaseType::Uint8,
            Self::Speed => FitBaseType::Uint16,
            Self::Timestamp => FitBaseType::Uint32,
        }
    }

    fn field_number(&self) -> u8 {
        match self {
            Self::PositionLat => 0,
            Self::PositionLong => 1,
            Self::Altitude => 2,
            Self::HeartRate => 3,
            Self::Speed => 6,
            Self::Timestamp => 253,
        }
    }
}

///////////////////////////
// WORKOUT FILE SETTINGS //
///////////////////////////

pub enum FitWorkoutFieldDefinition {
    MessageIndex,
    Sport,
    NumValidSteps, // uint16
    WorkoutName,
    SubSport,
    PoolLength, // in meters
    PoolLengthUnit,
    WorkoutDescription,
}

impl FitFieldDefinition for FitWorkoutFieldDefinition {
    fn base_type(&self) -> FitBaseType {
        match self {
            Self::MessageIndex => FitBaseType::Uint16,
            Self::Sport => FitBaseType::Enum,
            Self::NumValidSteps => FitBaseType::Uint16,
            Self::WorkoutName => FitBaseType::String,
            Self::SubSport => FitBaseType::Enum,
            Self::PoolLength => FitBaseType::Uint16,
            Self::PoolLengthUnit => FitBaseType::Enum,
            Self::WorkoutDescription => FitBaseType::String,
        }
    }

    fn field_number(&self) -> u8 {
        match self {
            Self::MessageIndex => 254,
            Self::Sport => 4,
            Self::NumValidSteps => 6,
            Self::WorkoutName => 8,
            Self::SubSport => 11,
            Self::PoolLength => 14,
            Self::PoolLengthUnit => 15,
            Self::WorkoutDescription => 17,
        }
    }
}

pub enum FitWorkoutStepFieldDefinition {
    MessageIndex,
    DurationType,
    DurationValue,
    DurationTime,
    DurationDistance,
    DurationCalories,
    DurationStep,
    TargetType,
    TargetValue,
}

impl FitFieldDefinition for FitWorkoutStepFieldDefinition {
    fn base_type(&self) -> FitBaseType {
        match self {
            Self::MessageIndex => FitBaseType::Uint16,
            Self::DurationType => FitBaseType::Enum,
            Self::DurationValue => FitBaseType::Uint32,
            Self::DurationTime => FitBaseType::Uint32,
            Self::DurationDistance => FitBaseType::Uint32,
            Self::DurationCalories => FitBaseType::Uint32,
            Self::DurationStep => FitBaseType::Uint32,
            Self::TargetType => FitBaseType::Enum,
            Self::TargetValue => FitBaseType::Uint32,
        }
    }

    fn field_number(&self) -> u8 {
        match self {
            Self::MessageIndex => 254,
            Self::DurationType => 1,
            Self::DurationValue => 2,
            Self::DurationTime => 2,
            Self::DurationDistance => 2,
            Self::DurationCalories => 2,
            Self::DurationStep => 2,
            Self::TargetType => 3,
            Self::TargetValue => 4,
        }
    }
}
