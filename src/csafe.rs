use std::io::{self, Write};

use crate::{csafe_defs::*, types::WorkoutType};

#[derive(Debug)]
pub struct CsafeBuffer {
    data: Vec<u8>,
}

impl CsafeBuffer {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Start a new frame
    pub fn start_frame(&mut self) {
        self.data.clear();
        self.data.push(FRAME_START_BYTE);
    }

    /// Add a short command (no parameters)
    pub fn add_short_command(&mut self, command: u8) {
        self.data.push(command);
    }

    /// Add a long command with data
    pub fn add_long_command(&mut self, command: u8, data: &[u8]) {
        self.data.push(command);
        self.data.push(data.len() as u8);
        self.data.extend_from_slice(data);
    }

    /// Add a PM proprietary command wrapper
    pub fn add_pm_command(&mut self, wrapper_cmd: u8, pm_cmd: u8, data: &[u8]) {
        // Wrapper command byte
        self.data.push(wrapper_cmd);
        // Total length: pm_cmd (1 byte) + pm_data_length (1 byte) + data
        let total_length = 1 + 1 + data.len();
        self.data.push(total_length as u8);
        // PM-specific command
        self.data.push(pm_cmd);
        // PM data length
        self.data.push(data.len() as u8);
        // PM data
        self.data.extend_from_slice(data);
    }

    /// Calculate checksum (XOR of all bytes except start/end markers)
    fn calculate_checksum(&self) -> u8 {
        let mut checksum = 0u8;
        // Skip the start byte (index 0)
        for &byte in &self.data[1..] {
            checksum ^= byte;
        }
        checksum
    }

    /// Apply byte stuffing to the frame
    fn apply_stuffing(data: &[u8]) -> Vec<u8> {
        let mut stuffed = Vec::new();

        for &byte in data {
            if byte >= 0xF0 && byte <= 0xF3 {
                stuffed.push(FRAME_STUFF_BYTE);
                stuffed.push(byte - FRAME_MAX_STUFF_OFFSET_BYTE);
            } else {
                stuffed.push(byte);
            }
        }

        stuffed
    }

    /// Finalize the frame with checksum and end byte
    pub fn finalize(&mut self) -> Vec<u8> {
        // Calculate checksum
        let checksum = self.calculate_checksum();
        self.data.push(checksum);

        // Extract the payload (everything except start byte)
        let payload = &self.data[1..];

        // Apply byte stuffing to the payload
        let mut result = vec![FRAME_START_BYTE];
        result.extend(Self::apply_stuffing(payload));
        result.push(FRAME_END_BYTE);

        result
    }

    /// Get the current buffer without finalizing
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

/// Builder for creating workout commands
pub struct WorkoutCommandBuilder {
    buffer: CsafeBuffer,
}

impl WorkoutCommandBuilder {
    pub fn new() -> Self {
        Self {
            buffer: CsafeBuffer::new(),
        }
    }

    /// Set up a fixed distance workout
    pub fn fixed_distance_workout(
        &mut self,
        distance_meters: u32,
        workout_type: WorkoutType,
    ) -> &mut Self {
        self.buffer.start_frame();

        // Set workout type
        let workout_type_data = [workout_type as u8];
        self.buffer
            .add_pm_command(SETPMCFG_CMD, PM_SET_WORKOUTTYPE, &workout_type_data);

        // Set workout duration (distance in meters)
        // Format: distance (4 bytes, little-endian), units (1 byte)
        let mut duration_data = Vec::new();
        duration_data.extend_from_slice(&distance_meters.to_le_bytes());
        duration_data.push(0x24); // DISTANCE_METER_0_0
        self.buffer
            .add_pm_command(SETPMCFG_CMD, PM_SET_WORKOUTDURATION, &duration_data);

        self
    }

    /// Set up a fixed time workout
    pub fn fixed_time_workout(&mut self, seconds: u32, workout_type: WorkoutType) -> &mut Self {
        self.buffer.start_frame();

        // Set workout type
        let workout_type_data = [workout_type as u8];
        self.buffer
            .add_pm_command(SETPMCFG_CMD, PM_SET_WORKOUTTYPE, &workout_type_data);

        // Set workout duration (time in centiseconds)
        // Format: time (4 bytes, little-endian), units (1 byte = 0 for time)
        let mut duration_data = Vec::new();
        let centiseconds = seconds * 100;
        duration_data.extend_from_slice(&centiseconds.to_le_bytes());
        duration_data.push(0x00); // Time units
        self.buffer
            .add_pm_command(SETPMCFG_CMD, PM_SET_WORKOUTDURATION, &duration_data);

        self
    }

    /// Start the workout (transition to "in use" state)
    pub fn start(&mut self) -> &mut Self {
        self.buffer.add_short_command(GOINUSE_CMD);
        self
    }

    /// Build and return the final command buffer
    pub fn build(&mut self) -> Vec<u8> {
        self.buffer.finalize()
    }
}

/// Helper function to create a 5km race workout command
pub fn create_5km_race_command() -> Vec<u8> {
    WorkoutCommandBuilder::new()
        .fixed_distance_workout(5000, WorkoutType::FixedDistanceNoSplits)
        .start()
        .build()
}

/// Helper function to create a timed workout command
pub fn create_timed_workout_command(minutes: u32) -> Vec<u8> {
    WorkoutCommandBuilder::new()
        .fixed_time_workout(minutes * 60, WorkoutType::FixedTimeNoSplits)
        .start()
        .build()
}

// Example usage and testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5km_race() {
        let command = create_5km_race_command();
        println!("5km race command: {:02X?}", command);
        assert!(!command.is_empty());
        assert_eq!(command[0], FRAME_START_BYTE);
        assert_eq!(command[command.len() - 1], FRAME_END_BYTE);
    }

    #[test]
    fn test_20min_workout() {
        let command = create_timed_workout_command(20);
        println!("20min workout command: {:02X?}", command);
        assert!(!command.is_empty());
    }
}
