use crate::types::{
    FitBaseType, FitDisplayMeasureType, FitFileManufacturerType, FitFileType, FitSportType,
    FitSubSportType, FitWorkoutStepDuration, FitWorkoutStepTarget,
};
use core::mem::size_of;

pub trait FitFieldDefinition {
    fn base_type(&self) -> FitBaseType;
    fn size(&self) -> usize;
    fn field_number(&self) -> u8;

    fn get(&self) -> [u8; 3] {
        [
            self.field_number(),
            self.size() as u8,
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
            Self::Manufacturer => FitBaseType::Enum,
            Self::Product => FitBaseType::Uint16,
            Self::SerialNumber => FitBaseType::Uint32z,
            Self::TimeCreated => FitBaseType::Uint32,
            Self::Number => FitBaseType::Uint16,
            Self::ProductName => FitBaseType::String,
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::Type => size_of::<FitFileType>(),
            Self::Manufacturer => size_of::<FitFileManufacturerType>(),
            _ => self.base_type().size(),
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

pub enum FitLapFieldDefinition {
    StartPositionLat,
    StartPositionLong,
    EndPositionLat,
    EndPositionLong,
    Timestamp,
}

impl FitFieldDefinition for FitLapFieldDefinition {
    fn base_type(&self) -> FitBaseType {
        match self {
            Self::StartPositionLat => FitBaseType::Uint32,
            Self::StartPositionLong => FitBaseType::Uint32,
            Self::EndPositionLat => FitBaseType::Uint32,
            Self::EndPositionLong => FitBaseType::Uint32,
            Self::Timestamp => FitBaseType::Uint32,
        }
    }

    fn size(&self) -> usize {
        match self {
            _ => self.base_type().size(),
        }
    }

    fn field_number(&self) -> u8 {
        match self {
            Self::StartPositionLat => 3,
            Self::StartPositionLong => 4,
            Self::EndPositionLat => 5,
            Self::EndPositionLong => 6,
            Self::Timestamp => 254,
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
            Self::MessageIndex => FitBaseType::Uint8,
            Self::Sport => FitBaseType::Enum,
            Self::NumValidSteps => FitBaseType::Uint16,
            Self::WorkoutName => FitBaseType::String,
            Self::SubSport => FitBaseType::Enum,
            Self::PoolLength => FitBaseType::Uint16,
            Self::PoolLengthUnit => FitBaseType::Enum,
            Self::WorkoutDescription => FitBaseType::String,
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::Sport => size_of::<FitSportType>(),
            Self::SubSport => size_of::<FitSubSportType>(),
            Self::PoolLengthUnit => size_of::<FitDisplayMeasureType>(),
            _ => self.base_type().size(),
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
            Self::MessageIndex => FitBaseType::Uint8,
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

    fn size(&self) -> usize {
        match self {
            Self::DurationType => size_of::<FitWorkoutStepDuration>(),
            Self::TargetType => size_of::<FitWorkoutStepTarget>(),
            _ => self.base_type().size(),
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
