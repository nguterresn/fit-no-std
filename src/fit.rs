use heapless::Vec;

#[derive(Debug, PartialEq)]
pub enum FitError {
    Failed(u8),
}

#[derive(Debug)]
pub enum FitMessageType {
    DataMessage,
    DefinitionMessage = 64,
}

#[derive(Debug)]
pub enum FitGlobalMessageNumber {
    FileId,
    Capabilities,
    Lap = 19,
    Record = 20,
    Workout = 26,
    WorkoutStep = 27,
}

pub enum FitFileIdFieldDefinitionNumber {
    Type,         // Required
    Manufacturer, // Required
    Product,
    SerialNumber,
    TimeCreated, // Required
    Number,
    ProductName = 8,
}

#[derive(Debug)]
pub enum FitLapFieldDefinitionNumber {
    StartPositionLat = 3,
    StartPositionLong,
    EndPositionLat,
    EndPositionLong,
    Timestamp = 253,
}

#[derive(Debug)]
pub enum FitWorkoutFieldDefinitionNumber {
    Sport = 4,         // sport
    Capabilities = 5,  // workout_capabilities
    NumValidSteps = 6, // uint16
}

#[derive(Debug)]
pub enum FitWorkoutStepFieldDefinitionNumber {
    DurationType = 1,
    DurationValue,
    TargetType,
    TargetValue,
    Intensity = 7,
    MessageIndex = 254,
}

#[derive(Debug)]
pub enum FitWorkoutStepDuration {
    Time,
    Distance,
    HrLessThan,
    HrGreaterThan,
    Calories,
    Open,
}

#[derive(Debug)]
pub enum FitWorkoutStepTarget {
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FitMessageArchitecture {
    LSB,
    MSB,
}

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

pub enum FitProtocolVersion {
    Version1 = 1,
    Version2 = 2,
}

pub enum FitFileType {
    Device = 1,
    Settings,
    Sport,
    Activity,
    Workout,
    Course,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FitFileManufacturer {
    Garmin = 1,
    Zephyr = 3,
    Development = 255,
}

// The number is very well connected to the size and the base_type.
// For example, a start_lat will have a size of sint32 and thus 4 bytes of size.
pub struct FitFieldDefinitionContent {
    pub number: u8, // Varies per 'FitGlobalMessageNumber'
    pub size: u8,
    pub base_type: FitBaseType,
}

#[derive(Debug, PartialEq)]
pub struct FitFile<const N: usize> {
    stream: Vec<u8, N>,
    arch: FitMessageArchitecture,
    manufacturer: FitFileManufacturer,
}

impl<const N: usize> FitFile<N> {
    pub fn new(
        protocol_version: FitProtocolVersion,
        major: u8,
        minor: u8,
        file_type: FitFileType,
        ts: u32,
    ) -> Result<Self, FitError> {
        let mut fit_file = Self {
            stream: Vec::new(),
            arch: FitMessageArchitecture::LSB,
            manufacturer: FitFileManufacturer::Development,
        };
        fit_file
            .build_header(protocol_version, major, minor)
            .map_err(|e| FitError::Failed(e))?;
        fit_file
            .build_file_id(file_type, ts)
            .map_err(|e| FitError::Failed(e))?;
        Ok(fit_file)
    }

    fn build_header(
        &mut self,
        protocol_version: FitProtocolVersion,
        major: u8,
        minor: u8,
    ) -> Result<(), u8> {
        // Header Size
        self.stream.push(14)?;

        // Protocol Version
        self.stream.push(4 << protocol_version as u8)?;

        let profile_version: u16 = major as u16 * 1000 + minor as u16;
        // Profile Version
        self.stream
            .extend_from_slice(&profile_version.to_le_bytes())
            .map_err(|_e| 0)?;

        // Data size (4 bytes)
        // Added from `done()`

        // Data Type (.FIT)
        self.stream.extend_from_slice(b".FIT").map_err(|_e| 0)?;

        // CRC size (2 bytes)
        self.stream.push(0)?;
        self.stream.push(0)?;

        Ok(())
    }

    /// All FIT files must contain a single File Id message. The File Id
    /// message identifies the intent of the FIT file through the Type field.
    /// The File Id message should be the first message in the file.
    fn build_file_id(
        &mut self,
        file_type: FitFileType,
        // The FIT Profile defines the date_time type as an uint32 that
        // represents the number of seconds since midnight on December 31, 1989 UTC*.
        // This date is often referred to as the FIT Epoch.
        ts: u32,
    ) -> Result<(), u8> {
        self.define(
            FitGlobalMessageNumber::FileId,
            &[
                FitFieldDefinitionContent {
                    number: FitFileIdFieldDefinitionNumber::Type as u8,
                    size: 1,
                    base_type: FitBaseType::Uint8,
                },
                FitFieldDefinitionContent {
                    number: FitFileIdFieldDefinitionNumber::Manufacturer as u8,
                    size: 2,
                    base_type: FitBaseType::Uint16,
                },
                FitFieldDefinitionContent {
                    number: FitFileIdFieldDefinitionNumber::TimeCreated as u8,
                    size: 4,
                    base_type: FitBaseType::Uint32,
                },
            ],
        )?;

        let mut buffer = [0u8; 7]; // 1 + 2 + 4 bytes
        buffer[0] = file_type as u8;
        buffer[1..3].copy_from_slice(&(self.manufacturer as u16).to_le_bytes());
        buffer[3..7].copy_from_slice(&(ts).to_le_bytes());
        self.push(&buffer)?;

        Ok(())
    }

    fn build_record_header(&mut self, msg_type: FitMessageType) -> Result<(), u8> {
        self.stream.push(msg_type as u8)?;
        Ok(())
    }

    fn build_message_definition_content(
        &mut self,
        gmsg_num: FitGlobalMessageNumber,
        fields_def: &[FitFieldDefinitionContent],
    ) -> Result<(), u8> {
        // [0] Reserved
        self.stream.push(0)?;
        // [1] Architecture LSB (0) or MSB (1)
        // Note: LSB only.
        self.stream.push(self.arch as u8)?;

        // [2:4] Global Message Number (0:65535 Unique)
        self.stream
            .extend_from_slice(&(gmsg_num as u16).to_le_bytes())
            .map_err(|_e| 0)?;

        // [4] Number of fields in the Data Message
        self.stream.push(fields_def.len() as u8)?;

        // [5:N] Field Definition
        for def in fields_def {
            self.build_field_definition_content(def.number, def.size, def.base_type as u8)?;
        }

        Ok(())
    }

    fn build_field_definition_content(
        &mut self,
        field_def_number: u8,
        size: u8,
        base_type: u8,
    ) -> Result<(), u8> {
        // Field Definition Number
        self.stream.push(field_def_number)?;
        // Size
        self.stream.push(size)?;
        // Base Type
        self.stream.push(base_type)?;
        Ok(())
    }

    fn crc_get16(&self, crc: u16, byte: u8) -> u16 {
        const CRC_TABLE: [u16; 16] = [
            0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401, 0xA001, 0x6C00, 0x7800,
            0xB401, 0x5000, 0x9C01, 0x8801, 0x4400,
        ];

        let tmp = CRC_TABLE[(crc & 0xF) as usize];
        let mut crc = (crc >> 4) & 0x0FFF;
        crc = crc ^ tmp ^ CRC_TABLE[(byte & 0xF) as usize];

        let tmp = CRC_TABLE[(crc & 0xF) as usize];
        crc = (crc >> 4) & 0x0FFF;
        crc ^ tmp ^ CRC_TABLE[((byte >> 4) & 0xF) as usize]
    }

    pub fn done(&mut self) -> Result<&[u8], FitError> {
        let size = self.size() - 10;
        for n in 0..4 {
            self.stream
                .insert(4, ((size >> (3 - n) * 8) & 0xff) as u8)
                .map_err(|e| FitError::Failed(e))?;
        }

        let mut crc = 0u16;
        for byte in &self.stream {
            crc = self.crc_get16(crc, *byte);
        }

        // [N-2, N-1] CRC
        self.stream
            .extend_from_slice(&crc.to_le_bytes())
            .map_err(|_e| FitError::Failed(0))?;

        Ok(&self.stream)
    }

    pub fn define(
        &mut self,
        global_msg_num: FitGlobalMessageNumber,
        fields_def: &[FitFieldDefinitionContent],
    ) -> Result<(), u8> {
        self.build_record_header(FitMessageType::DefinitionMessage)?;
        self.build_message_definition_content(global_msg_num, fields_def)?;
        Ok(())
    }

    /// Push any data as it was defined by a previous Definition Message.
    pub fn push(&mut self, data: &[u8]) -> Result<(), u8> {
        self.build_record_header(FitMessageType::DataMessage)?;
        self.stream.extend_from_slice(data).map_err(|_e| 0)?;
        Ok(())
    }

    pub fn size(&mut self) -> usize {
        self.stream.len()
    }
}
