use anyhow::bail;

use crate::csafe_defs::*;
use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseStatus {
    Ok,
    Rejected,
    BadFrame,
    NotReady,
}

impl ResponseStatus {
    pub fn from_status_byte(status: u8) -> Self {
        match status & PREVFRAMESTATUS_MSK {
            PREVOK_FLG => ResponseStatus::Ok,
            PREVREJECT_FLG => ResponseStatus::Rejected,
            PREVBAD_FLG => ResponseStatus::BadFrame,
            PREVNOTRDY_FLG => ResponseStatus::NotReady,
            _ => ResponseStatus::BadFrame,
        }
    }
}

#[derive(Debug)]
pub struct CsafeResponse {
    pub status: ResponseStatus,
    pub data: Vec<u8>,
}

impl CsafeResponse {
    pub fn parse(frame: &[u8]) -> anyhow::Result<Self> {
        if frame.len() < 3 {
            bail!("Frame too short");
        }
        if frame[0] != FRAME_START_BYTE {
            bail!("Invalid start byte");
        }
        if frame[frame.len() - 1] != FRAME_END_BYTE {
            bail!("Invalid end byte");
        }

        // Unstuff the frame
        let unstuffed = Self::remove_stuffing(&frame[1..frame.len() - 1]);

        if unstuffed.len() < 2 {
            bail!("Response too short");
        }

        // First byte after unstuffing is status
        let status = ResponseStatus::from_status_byte(unstuffed[0]);

        // Rest is data (excluding checksum at the end)
        let data = unstuffed[1..unstuffed.len() - 1].to_vec();

        Ok(CsafeResponse { status, data })
    }

    fn remove_stuffing(data: &[u8]) -> Vec<u8> {
        let mut unstuffed = Vec::new();
        let mut i = 0;

        while i < data.len() {
            if data[i] == FRAME_STUFF_BYTE && i + 1 < data.len() {
                unstuffed.push(data[i + 1] + FRAME_MAX_STUFF_OFFSET_BYTE);
                i += 2;
            } else {
                unstuffed.push(data[i]);
                i += 1;
            }
        }

        unstuffed
    }
}

/// Represents a single command in a workout sequence
#[derive(Debug, Clone)]
pub struct WorkoutCommand {
    pub name: String,
    pub data: Vec<u8>,
}

pub struct CSafeBuffer {
    buf: Vec<u8>,
}

impl CSafeBuffer {
    pub fn new() -> Self {
        Self {
            buf: vec![0xF1, 0x76, 0x00],
        }
    }

    pub fn append(mut self, cmd: PmLongPushCfgCmds, bytes: &[u8]) -> Self {
        self.buf.extend_from_slice(&[cmd as u8, bytes.len() as u8]);
        self.buf.extend_from_slice(&bytes);
        self
    }

    pub fn append_data_cmd(mut self, cmd: PmLongPushDataCmds, bytes: &[u8]) -> Self {
        self.buf.extend_from_slice(&[cmd as u8, bytes.len() as u8]);
        self.buf.extend_from_slice(&bytes);
        self
    }

    pub fn just_row(self) -> Self {
        self.append(
            PmLongPushCfgCmds::SetWorkoutType,
            &[WorkoutType::JustrowSplits as u8],
        )
        .append(
            PmLongPushCfgCmds::SetScreenState,
            &[
                ScreenType::Workout as u8,
                ScreenValueWorkoutType::PrepareToRowWorkout as u8,
            ],
        )
    }

    pub fn distance_splits(self, distance: Distance, splits: Distance) -> Self {
        self.append(
            PmLongPushCfgCmds::SetWorkoutType,
            &[WorkoutType::FixedDistanceSplits as u8],
        )
        .append(
            PmLongPushCfgCmds::SetWorkoutDuration,
            [WorkoutDurationType::Distance as u8]
                .into_iter()
                .chain(distance.to_le_bytes().into_iter().rev())
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .append(
            PmLongPushCfgCmds::SetSplitDuration,
            [WorkoutDurationType::Distance as u8]
                .into_iter()
                .chain(splits.to_le_bytes().into_iter().rev())
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .append(PmLongPushCfgCmds::ConfigureWorkout, &[0x01])
        .append(
            PmLongPushCfgCmds::SetScreenState,
            &[
                ScreenType::Workout as u8,
                ScreenValueWorkoutType::PrepareToRowWorkout as u8,
            ],
        )
    }

    pub fn finalize(mut self) -> Vec<u8> {
        let mut checksum = 0u8;
        self.buf[2] = (self.buf.len() - 3) as u8;
        for byte in &self.buf[1..] {
            checksum ^= byte;
        }
        self.buf.push(checksum);
        self.buf.push(0xF2);
        self.buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_just_row() {
        println!("{:02X?}", CSafeBuffer::new().just_row().finalize())
    }

    #[test]
    fn test_distance_splits() {
        println!(
            "{:02X?}",
            CSafeBuffer::new()
                .distance_splits(Distance(U24::new(2000)), Distance(U24::new(400)))
                .finalize()
        )
    }
}
