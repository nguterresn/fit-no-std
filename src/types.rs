#![allow(dead_code)]

#[derive(Clone, Copy)]
pub enum FitBaseType {
    Enum,
    Sint8,
    Uint8,
    Sint16 = 131,
    Uint16 = 132,
    Sint32 = 133,
    Uint32 = 134,
    String = 7,
    Float32 = 136,
    Float64 = 137,
    Uint8z = 10,
    Uint16z = 139,
    Uint32z = 140,
    Byte = 13,
    Sint64 = 142,
    Uint64 = 143,
    Uint64z = 144,
}

impl FitBaseType {
    pub fn size(&self) -> usize {
        match self {
            Self::Sint8 | Self::Uint8 | Self::Uint8z | Self::Byte | Self::Enum => 1,
            Self::Sint16 | Self::Uint16 | Self::Uint16z => 2,
            Self::Sint32 | Self::Uint32 | Self::Float32 | Self::Uint32z => 4,
            Self::Float64 | Self::Sint64 | Self::Uint64 | Self::Uint64z => 8,
            Self::String => 1, // Variable.
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
    Event = 21,
    Workout = 26,
    WorkoutStep = 27,
    Activity = 34,
}

pub enum FitActivityType {
    Manual,
    AutoMultiSport,
}

pub enum FitEventType {
    Timer,
    Workout = 3,
    WorkoutStep,
    PowerDown,
    PowerUp,
    OffCourse,
    Session,
    Lap,
    CoursePoint,
    Battery,
    Activity = 26,
    Length = 28,
}

pub enum FitEventTypeType {
    Start,
    Stop,
    ConsecutiveDepreciated,
    Marker,
    StopAll,
    BeginDepreciated,
    EndDepreciated,
    EndAllDepreciated,
    StopDisable,
    StopDisableAll,
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
