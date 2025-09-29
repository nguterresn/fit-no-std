use heapless::Vec;

use crate::{
    profile::{FitFieldDefinition, FitFileIdFieldDefinition},
    types::{FitFileManufacturerType, FitFileType, FitGlobalMessageType},
};

#[derive(Debug, PartialEq)]
pub enum FitError {
    Failed(u8),
}

#[derive(Debug)]
enum FitMessageType {
    DataMessage,
    DefinitionMessage = 64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FitMessageArchitecture {
    LSB,
    MSB,
}

pub enum FitProtocolVersion {
    Version1 = 1,
    Version2 = 2,
}

#[derive(Debug, PartialEq)]
pub struct FitFile<const N: usize> {
    stream: Vec<u8, N>,
    arch: FitMessageArchitecture,
    manufacturer: FitFileManufacturerType,
}

impl<const N: usize> FitFile<N> {
    pub fn new(file_type: FitFileType, ts: u32) -> Result<Self, FitError> {
        let mut fit_file = Self {
            stream: Vec::new(),
            arch: FitMessageArchitecture::LSB,
            manufacturer: FitFileManufacturerType::Development,
        };
        fit_file.build_header().map_err(|e| FitError::Failed(e))?;
        fit_file
            .build_file_id(file_type, ts)
            .map_err(|e| FitError::Failed(e))?;
        Ok(fit_file)
    }

    fn build_header(&mut self) -> Result<(), u8> {
        // Header Size
        self.stream.push(14)?;

        // Protocol Version
        self.stream.push(4 << FitProtocolVersion::Version2 as u8)?;

        let profile_version: u16 = 21 as u16 * 1000 + 171 as u16;
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
            FitGlobalMessageType::FileId,
            &[
                FitFileIdFieldDefinition::Type,
                FitFileIdFieldDefinition::Manufacturer,
                FitFileIdFieldDefinition::TimeCreated,
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

    fn build_message_definition_content<T: FitFieldDefinition>(
        &mut self,
        gmsg_num: FitGlobalMessageType,
        fields_def: &[T],
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
            self.build_field_definition_content(def)?;
        }

        Ok(())
    }

    fn build_field_definition_content<T: FitFieldDefinition>(
        &mut self,
        field: &T,
    ) -> Result<(), u8> {
        self.stream
            .extend_from_slice(&field.get())
            .map_err(|_e| 0)?;
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

    pub fn define<T: FitFieldDefinition>(
        &mut self,
        global_msg_num: FitGlobalMessageType,
        fields_def: &[T],
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
