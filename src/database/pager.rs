use std::{
    fs::File,
    io::{self, Read, Seek, Write},
};

use crc_fast::crc32_iscsi;

const MINIDB_NUMBER: [u8; 8] = *b"MiniDB\0\0";
const PAGE_SIZE: usize = 4096;
const RESERVED_OFFSET: usize = 36;
const SCHEMA_OFFSET: usize = 64;

// Column Types
#[derive(Debug, Clone)]
enum ColumnType {
    Int64,
    Int32,
    Boolean,
    Text,
    Date,
}

/// Column Definitions
/// name, type (uint / string / date)
#[derive(Debug, Clone)]
struct ColumnDef {
    pub name: String,
    pub typ: ColumnType,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnDef>,
    pub row_size: u32,
    pub first_data_page: u64,
    pub row_count: u64,
}

#[derive(Debug, Clone)]
pub struct HeaderPage {
    pub file_signature: [u8; 8],   // size 8
    pub version: u32,              // size 4
    pub page_size: u32,            // size 4
    pub next_page_id: u64,         // size 8
    pub crc_32: u32,               // size 4
    pub total_row_count: u64,      // size 8
    pub ser_schema_bytes: Vec<u8>, // size 24, needs drop
    pub reserved: [u8; 26],        // size 26, reserved zero space
}

impl HeaderPage {
    pub fn new() -> Self {
        HeaderPage {
            file_signature: MINIDB_NUMBER,
            version: 0,
            page_size: PAGE_SIZE as u32,
            next_page_id: 1,
            crc_32: 0,
            total_row_count: 0,
            ser_schema_bytes: Vec::new(),
            reserved: [0; 26],
        }
    }

    pub fn read_from(file: &mut File) -> io::Result<Self> {
        file.seek(io::SeekFrom::Start(0))?;

        let mut buffer = [0u8; PAGE_SIZE];
        file.read_exact(&mut buffer)?;

        let file_signature: [u8; 8] = buffer[0..8].try_into().unwrap();
        if file_signature != MINIDB_NUMBER {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unkonwn file signature",
            ));
        }

        let version = u32::from_le_bytes(buffer[8..12].try_into().unwrap());
        let page_size = u32::from_le_bytes(buffer[12..16].try_into().unwrap());
        let next_page_id = u64::from_le_bytes(buffer[16..24].try_into().unwrap());
        let crc_32 = u32::from_le_bytes(buffer[24..28].try_into().unwrap());
        let total_row_count = u64::from_le_bytes(buffer[28..36].try_into().unwrap());
        let ser_schema_bytes: Vec<u8> = buffer[64..].to_vec();

        Ok(HeaderPage {
            file_signature,
            version,
            page_size,
            next_page_id: next_page_id as u64,
            crc_32,
            total_row_count: total_row_count as u64,
            ser_schema_bytes,
            reserved: [0; 26],
        })
    }

    pub fn write(&self, file: &mut File) -> io::Result<()> {
        file.seek(io::SeekFrom::Start(0))?;

        let mut buffer = vec![0u8; PAGE_SIZE as usize];

        buffer[0..8].copy_from_slice(&self.file_signature); // size 8
        buffer[8..12].copy_from_slice(&self.version.to_le_bytes()); // size 4
        buffer[12..16].copy_from_slice(&self.page_size.to_le_bytes()); // size 4
        buffer[16..24].copy_from_slice(&self.next_page_id.to_le_bytes()); // size 8 
        // leave 24..28 placeholder for crc
        buffer[28..36].copy_from_slice(&self.total_row_count.to_le_bytes()); // size 8

        buffer[RESERVED_OFFSET..RESERVED_OFFSET + 26].copy_from_slice(&self.reserved); // size 24

        if self.ser_schema_bytes.len() > PAGE_SIZE - SCHEMA_OFFSET as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Schema too large for header page",
            ));
        };

        buffer[SCHEMA_OFFSET..SCHEMA_OFFSET + self.ser_schema_bytes.len()]
            .copy_from_slice(&self.ser_schema_bytes);

        let crc = crc32_iscsi(&buffer[0..PAGE_SIZE - 4]); // size 4
        buffer[24..28].copy_from_slice(&crc.to_le_bytes());

        file.write_all(&buffer)?;
        file.flush()?;

        Ok(())
    }
}
