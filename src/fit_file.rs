#![allow(dead_code)]

use heapless::Vec;

#[derive(Debug, PartialEq)]
pub enum FitError {
    Failed(u8),
}

#[derive(Debug)]
pub enum FitMessageType {
    DataMessage,
    DefinitionMessage,
}

#[derive(Debug)]
pub enum FitGlobalMessageNumber {
    FileId,
    Capabilities,
    Lap = 19,
    Record = 20,
}

#[derive(Debug)]
pub enum FitLapFieldDefinitionNumber {
    StartPositionLat = 3,
    StartPositionLong,
    EndPositionLat,
    EndPositionLong,
}

#[derive(Debug)]
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

// The number is very well connected to the size and the base_type.
// For example, a start_lat will have a size of sint32 and thus 4 bytes of size.
pub struct FitFieldDefinitionContent {
    pub number: u8, // Varies per 'FitGlobalMessageNumber'
    pub size: u8,
    pub base_type: FitBaseType,
}

const FIT_FILE_MAX_SIZE: usize = 32768;

#[derive(Debug, PartialEq)]
pub struct FitFile<const N: usize> {
    stream: Vec<u8, N>,
}

impl<const N: usize> FitFile<N> {
    pub fn new() -> Result<Self, FitError> {
        let mut fit_file = Self { stream: Vec::new() };
        fit_file.build_header().map_err(|e| FitError::Failed(e))?;

        // Definition needs to be created as well.
        // Which fields are going to be send, etc..

        Ok(fit_file)
    }

    fn build_header(&mut self) -> Result<(), u8> {
        // Header Size
        self.stream.push(14)?;

        // Protocol Version
        self.stream.push(0x20)?; // Protocol Version 2.0

        // Profile Version (21.32)
        self.stream
            .extend_from_slice(&2132u16.to_le_bytes())
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

    fn build_record_header(&mut self, msg_type: FitMessageType) -> Result<(), u8> {
        self.stream.push(0b0100_0000 & msg_type as u8)?;
        Ok(())
    }

    fn build_message_definition_content(
        &mut self,
        arch: FitMessageArchitecture,
        gmsg_num: FitGlobalMessageNumber,
        fields_def: &[FitFieldDefinitionContent],
    ) -> Result<(), u8> {
        // [0] Reserved
        self.stream.push(0)?;
        // [1] Architecture LSB (0) or MSB (1)
        // Note: LSB only.
        self.stream.push(arch as u8)?;

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
                .insert(n + 5, ((size >> n) & 0xff) as u8)
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
        msg_type: FitMessageType,
        arch: FitMessageArchitecture,
        global_msg_num: FitGlobalMessageNumber,
        fields_def: &[FitFieldDefinitionContent],
    ) -> Result<(), u8> {
        self.build_record_header(msg_type)?;
        self.build_message_definition_content(arch, global_msg_num, fields_def)?;
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
