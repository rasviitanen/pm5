use anyhow::{Context, Result};

use crate::{
    csafe::CSafeBuffer,
    csafe_defs::{PmLongPullDataCmds, PmLongPushCfgCmds, PmLongPushDataCmds},
    types::{ScreenType, ScreenValueCsafe},
};

// PM5 Display Constants (Correct resolution)
const SCREEN_WIDTH: usize = 240;
const SCREEN_HEIGHT: usize = 64;
const BYTES_PER_ROW: usize = SCREEN_WIDTH / 8; // 30 bytes per row
const BUFFER_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) / 8; // 1920 bytes

pub struct Pm5Bitmap {
    data: Vec<u8>,
}

impl Pm5Bitmap {
    pub fn new() -> Self {
        Pm5Bitmap {
            data: vec![0; BUFFER_SIZE],
        }
    }

    /// Sets a specific pixel
    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return;
        }

        let byte_index = (y * BYTES_PER_ROW) + (x / 8);
        let bit_offset = 7 - (x % 8); // MSB = leftmost pixel

        if value {
            self.data[byte_index] |= 1 << bit_offset;
        } else {
            self.data[byte_index] &= !(1 << bit_offset);
        }
    }
    /// Draw text using a simple 5x7 font
    pub fn draw_text(&mut self, x: usize, y: usize, text: &str) {
        for (i, ch) in text.chars().enumerate() {
            let char_x = x + i * 6; // 5 pixels + 1 space
            self.draw_char(char_x, y, ch);
        }
    }

    /// Draw a single character (simplified 5x7 font)
    fn draw_char(&mut self, x: usize, y: usize, ch: char) {
        // Simple block character for demo
        // You'd want a proper font lookup table here
        for dy in 0..7 {
            for dx in 0..5 {
                self.set_pixel(x + dx, y + dy, true);
            }
        }
    }

    /// Draw a filled rectangle
    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, value: bool) {
        for dy in 0..height {
            for dx in 0..width {
                self.set_pixel(x + dx, y + dy, value);
            }
        }
    }

    /// Clears the buffer
    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    /// Convert bitmap to CSAFE packets
    pub fn to_csafe_packets(&self) -> Vec<Vec<u8>> {
        let mut packets = Vec::new();

        // Use exactly 64 bytes per chunk as per the spec (Data Index 0-63)
        let chunk_size = 64;

        for (chunk_idx, chunk) in self.data.chunks(chunk_size).enumerate() {
            let start_byte_index = chunk_idx * chunk_size;

            // Block length should be the actual data length in this chunk
            let block_length = chunk.len() as u8;

            // Build payload exactly as spec shows:
            // Byte 0: Bitmap index MSB
            // Byte 1: Bitmap index LSB
            // Byte 2: Block length
            // Byte 3-66: Data (index 0 through 63)
            let mut cmd_payload = Vec::with_capacity(3 + chunk.len());
            cmd_payload.extend_from_slice(&(start_byte_index as u16).to_le_bytes());
            cmd_payload.push(block_length);
            cmd_payload.extend_from_slice(chunk);

            let buffer = CSafeBuffer::new()
                .append_data_cmd(PmLongPushDataCmds::SetDisplayBitmap, &cmd_payload);

            packets.push(buffer.finalize());
        }

        packets
    }

    /// Helper function to send bitmap and activate display
    pub fn display_bitmap(self) -> Vec<Vec<u8>> {
        let mut packets = self.to_csafe_packets();
        // packets.push(
        //     CSafeBuffer::new()
        //         .append(
        //             PmLongPushCfgCmds::SetScreenState,
        //             &[
        //                 ScreenType::Csafe as u8,
        //                 ScreenValueCsafe::ScreenRedraw as u8,
        //             ],
        //         )
        //         .finalize(),
        // );

        // packets.push(
        //     CSafeBuffer::new()
        //         .append(
        //             PmLongPushCfgCmds::SetScreenState,
        //             &[ScreenType::Csafe as u8, ScreenValueCsafe::Custom as u8],
        //         )
        //         .finalize(),
        // );

        packets
    }
}
