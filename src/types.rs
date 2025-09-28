#![allow(dead_code)]

#[derive(Clone, Copy)]
pub enum FitBaseType {
    Enum,
    Sint8,
    Uint8,
    Sint16,
    Uint16,
    Sint32,
    Uint32,
    String,
    Float32,
    Float64,
    Uint8z,
    Uint16z,
    Uint32z,
    Byte,
    Sint64,
    Uint64,
    Uint64z,
}

impl FitBaseType {
    pub fn size(&self) -> usize {
        match self {
            Self::Sint8 | Self::Uint8 | Self::Uint8z | Self::Byte => 1,
            Self::Sint16 | Self::Uint16 | Self::Uint16z => 2,
            Self::Sint32 | Self::Uint32 | Self::Float32 | Self::Uint32z => 4,
            Self::Float64 | Self::Sint64 | Self::Uint64 | Self::Uint64z => 8,
            Self::String | Self::Enum => 1, // Variable.
        }
    }
}

#[derive(Debug)]
pub enum FitFileType {
    Device = 1,
    Settings,
    Sport,
    Activity,
    Workout,
    Course,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FitFileManufacturerType {
    Garmin = 1,
    Zephyr = 3,
    Development = 255,
    Decathlon = 310,
}

#[derive(Debug)]
pub enum FitGlobalMessageType {
    FileId,
    Capabilities,
    Lap = 19,
    Record = 20,
    Workout = 26,
    WorkoutStep = 27,
}

#[derive(Debug)]
pub enum FitSportType {
    Generic,
    Running,
    Cycling,
    Transition,
    FitnessEquipment,
    Swimming,
    Basketball,
    Soccer,
    Tennis,
    AmericanFootball,
    Training,
    Walking,
    CrossCountrySkiing,
    AlpineSkiing,
    Snowboarding,
    Rowing,
    Mountaineering,
    Hiking,
    Multisport,
    Paddling,
    Flying,
    EBiking,
    Motorcycling,
    Boating,
    Driving,
    Golf,
    HangGliding,
    HorsebackRiding,
    Hunting,
    Fishing,
    InlineSkating,
    RockClimbing,
    Sailing,
    IceSkating,
    SkyDiving,
    Snowshoeing,
    Snowmobiling,
    StandUpPaddleboarding,
    Surfing,
    Wakeboarding,
    WaterSkiing,
    Kayaking,
    Rafting,
    Windsurfing,
    Kitesurfing,
    Tactical,
    Jumpmaster,
    Boxing,
    All = 254,
}

#[derive(Debug)]
pub enum FitSubSportType {
    Generic,
    Treadmill,
    Street,
    Trail,
    Track,
    Spin,
    IndoorCycling,
    Road,
    Mountain,
    Downhill,
    Recumbent,
    Cyclocross,
    HandCycling,
    TrackCycling,
    IndoorRowing,
    Elliptical,
    StairClimbing,
    LapSwimming,
    OpenWater,
    FlexibilityTraining,
    StrengthTraining,
    WarmUp,
    Match,
    Exercise,
    Challenge,
    IndoorSkiing,
    CardioTraining,
    IndoorWalking,
    EBikeFitness,
    Bmx,
    CasualWalking,
    SpeedWalking,
    BikeToRunTransition,
    RunToBikeTransition,
    SwimToBikeTransition,
    Atv,
    Motocross,
    Backcountry,
    Resort,
    RcDrone,
    Wingsuit,
    Whitewater,
    SkateSkiing,
    Yoga,
    Pilates,
    IndoorRunning,
    GravelCycling,
    EBikeMountain,
    Commuting,
    MixedSurface,
    Navigate,
    TrackMe,
    Map,
    SingleGasDiving,
    MultiGasDiving,
    GaugeDiving,
    ApneaDiving,
    ApneaHunting,
    VirtualActivity,
    Obstacle,
}

#[derive(Debug)]
pub enum FitWorkoutStepDuration {
    // wkt_step_duration
    Time,
    Distance,
    HrLessThan,
    HrGreaterThan,
    Calories,
    Open,
}

#[derive(Debug)]
pub enum FitWorkoutStepTarget {
    // wkt_step_target
    Speed,
    HeartRate,
    Open,
    Cadence,
    Power,
    Grade,
    Resistance,
    Power3s,
    Power10s,
    Power30s,
    PowerLap,
    SwimStroke,
    SpeedLap,
    HeartRateLap,
}

#[derive(Debug)]
pub enum FitDisplayMeasureType {
    Metric,
    Statue,
    Nautical,
}
