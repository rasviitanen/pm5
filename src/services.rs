use std::io::Cursor;

use uuid::Uuid;

use crate::{parse::Parse, types::*};

pub enum Pm5 {
    Information(Information),
    Control(Control),
    Rowing(Rowing),
    HeartRate(HeartRate),
}

impl Pm5 {
    pub fn rowing() -> &'static [Rowing] {
        &[Rowing::GeneralStatus, Rowing::StrokeData]
    }
}

#[derive(Debug)]
pub enum Pm5Data {
    Rowing(RowingData),
}

impl ServiceData for Pm5 {
    type Data = Pm5Data;

    fn parse(uuid: Uuid, data: Vec<u8>) -> Result<Self::Data, ServiceDataError> {
        if Rowing::characteristic_is_part_of_service(uuid) {
            Rowing::parse(uuid, data).map(Pm5Data::Rowing)
        } else {
            Err(ServiceDataError::UnkownService)
        }
    }
}

pub enum Information {
    ModelNumber,
    SerialNumber,
    HardwareRevision,
    FirmwareRevision,
    ManufacturerName,
    MachineType,
}

pub enum Control {
    Receive,
    Transmit,
}

#[derive(Debug)]
pub enum Rowing {
    GeneralStatus,
    AdditionalStatusOne,
    AdditionalStatusTwo,
    GeneralStatusRate,
    StrokeData,
    AdditionalStrokeData,
    SplitIntervalData,
    AdditionalSplitIntervalData,
    EndOfWorkoutSummaryData,
    AdditionalEndOfWorkoutSummaryData,
    HeartRateBeltInformation,
    AdditionalEndOfWorkoutSummaryDataTwo,
    ForceCurveData,
    AdditionalStatusThree,
    MultiplexedInformation,
}

#[derive(Debug)]
pub enum RowingData {
    GeneralStatus {
        elapsed_time: Time,
        distance: Distance,
        workout_type: WorkoutType,
        interval_type: IntervalType,
        workout_state: WorkoutState,
        rowing_state: RowingState,
        stroke_state: StrokeState,
        total_work_distance: Distance,
        workout_duration: Time,
        workout_duration_type: WorkoutDurationType,
        drag_factor: DragFactor,
    },
    AdditionalStatusOne {
        elapsed_time: Time,
        speed: Speed,
        stroke_rate: StrokeRate,
        heart_rate: HeartRate,
        current_pace: Pace,
        average_pace: Pace,
        rest_distance: RestDistance,
        rest_time: Time, // Strange that this is 3 bytes, but other rest times are 2?
        machine_type: ErgMachineType,
    },
    AdditionalStatusTwo {
        elapsed_time: Time,
        interval_count: IntervalCount,
        average_power: Power,
        total_calories: Calories,
        split_interval_avg_pace: Pace,
        split_interval_avg_power: Power,
        split_interval_avg_calories: Calories,
        last_split_time: Time,
        last_split_distance: Distance,
    },
    GeneralStatusRate {
        interval: SampleRate,
    },
    StrokeData {
        elapsed_time: Time,
        distance: Distance,
        drive_length: DriveLength,
        drive_time: DriveTime,
        stroke_recovery: StrokeRecoveryTime,
        stroke_distance: StrokeDistance,
        peak_drive_force: Force,
        avg_drive_force: Force,
        work_per_stroke: Work,
        stroke_count: StrokeCount,
    },
    AdditionalStrokeData {
        elapsed_time: Time,
        stroke_power: Power,
        stroke_calories: Calories,
        stroke_count: StrokeCount,
        projected_work_time: Time,
        projected_work_distance: Distance,
    },
    SplitIntervalData {
        elapsed_time: Time,
        distance: Distance,
        split_interval_time: Time,
        split_interval_distance: Distance,
        interval_rest_time: RestTime,
        interval_rest_distance: RestDistance,
        split_interval_type: IntervalType,
        split_interval_number: IntervalCount,
    },
    AdditionalSplitIntervalData {
        elapsed_time: Time,
        split_interval_avg_stroke_rate: StrokeRate,
        split_interval_work_heartrate: HeartRate,
        split_interval_rest_heartrate: HeartRate,
        split_interval_avg_pace: Pace,
        split_interval_total_calories: Calories,
        split_interval_avg_calories: Calories,
        split_interval_speed: Speed,
        split_interval_power: Power,
        split_avg_drag_factor: DragFactor,
        split_interval_number: IntervalCount,
        erg_machine_type: ErgMachineType,
    },
    EndOfWorkoutSummaryData {
        log_entry_date: LogEntryDate,
        log_entry_time: LogEntryTime,
        elapsed_time: Time,
        distance: Distance,
        avg_stroke_rate: StrokeRate,
        ending_heartrate: HeartRate,
        avg_heartrate: HeartRate,
        min_heartrate: HeartRate,
        max_heartrate: HeartRate,
        drag_factor_avg: DragFactor,
        recover_heartrate: HeartRate,
        workout_type: WorkoutType,
        avg_pace: Pace,
    },
    AdditionalEndOfWorkoutSummaryData {
        log_entry_date: LogEntryDate,
        log_entry_time: LogEntryTime,
        split_interval_type: IntervalType,
        split_interval_size: Size,
        split_interval_count: IntervalCount,
        total_calories: Calories,
        watts: Work,
        total_rest_distance: Distance,
        interval_rest_time: RestTime,
        avg_calories: Calories,
    },
    HeartRateBeltInformation {
        manufacturer_id: u8,
        device_type: u8,
        belt_id: u32,
    },
    AdditionalEndOfWorkoutSummaryDataTwo {
        log_entry_date: LogEntryDate,
        log_entry_time: LogEntryTime,
        avg_pace: Pace,
        game_id: GameId,
        game_score: GameScore,
        erg_machine_type: ErgMachineType,
    },
    ForceCurveData {
        data: ForceCurveData,
    },
    AdditionalStatusThree {},
    MultiplexedInformation {},
}

pub trait Service {
    const UUID: Uuid;

    fn id(&self) -> Uuid;

    fn characteristic_is_part_of_service(characteristic: Uuid) -> bool {
        Self::UUID.as_u128() == (characteristic.as_u128() & !(0x000F << 96))
    }
}

#[cfg(test)]
mod tests22 {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_name22() {
        let uuid = Uuid::from_str("ce060031-43e5-11e4-916c-0800200c9a66").unwrap();
        println!("{}", Uuid::from_u128(uuid.as_u128() & !(0x000F << 96)));
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceDataError {
    #[error("invalid bytes")]
    Data(#[from] crate::parse::ParseError),
    #[error("invalid id")]
    Id,
    #[error("unknown service")]
    UnkownService,
}

pub trait ServiceData {
    type Data;

    fn parse(uuid: Uuid, data: Vec<u8>) -> Result<Self::Data, ServiceDataError>;
}

impl ServiceData for Rowing {
    type Data = RowingData;

    fn parse(uuid: Uuid, data: Vec<u8>) -> Result<Self::Data, ServiceDataError> {
        let mut data = Cursor::new(data);
        if Rowing::GeneralStatus.id() == uuid {
            return Ok(RowingData::GeneralStatus {
                elapsed_time: Parse::parse(&mut data)?,
                distance: Parse::parse(&mut data)?,
                workout_type: Parse::parse(&mut data)?,
                interval_type: Parse::parse(&mut data)?,
                workout_state: Parse::parse(&mut data)?,
                rowing_state: Parse::parse(&mut data)?,
                stroke_state: Parse::parse(&mut data)?,
                total_work_distance: Parse::parse(&mut data)?,
                workout_duration: Parse::parse(&mut data)?,
                workout_duration_type: Parse::parse(&mut data)?,
                drag_factor: Parse::parse(&mut data)?,
            });
        }

        if Rowing::AdditionalStatusOne.id() == uuid {
            return Ok(RowingData::AdditionalStatusOne {
                elapsed_time: Parse::parse(&mut data)?,
                speed: Parse::parse(&mut data)?,
                stroke_rate: Parse::parse(&mut data)?,
                heart_rate: Parse::parse(&mut data)?,
                current_pace: Parse::parse(&mut data)?,
                average_pace: Parse::parse(&mut data)?,
                rest_distance: Parse::parse(&mut data)?,
                rest_time: Parse::parse(&mut data)?,
                machine_type: Parse::parse(&mut data)?,
            });
        }

        if Rowing::StrokeData.id() == uuid {
            return Ok(RowingData::StrokeData {
                elapsed_time: Parse::parse(&mut data)?,
                distance: Parse::parse(&mut data)?,
                drive_length: Parse::parse(&mut data)?,
                drive_time: Parse::parse(&mut data)?,
                stroke_recovery: Parse::parse(&mut data)?,
                stroke_distance: Parse::parse(&mut data)?,
                peak_drive_force: Parse::parse(&mut data)?,
                avg_drive_force: Parse::parse(&mut data)?,
                work_per_stroke: Parse::parse(&mut data)?,
                stroke_count: Parse::parse(&mut data)?,
            });
        }

        if Rowing::AdditionalStrokeData.id() == uuid {
            return Ok(RowingData::AdditionalStrokeData {
                elapsed_time: Parse::parse(&mut data)?,
                stroke_power: Parse::parse(&mut data)?,
                stroke_calories: Parse::parse(&mut data)?,
                stroke_count: Parse::parse(&mut data)?,
                projected_work_time: Parse::parse(&mut data)?,
                projected_work_distance: Parse::parse(&mut data)?,
            });
        }

        Err(ServiceDataError::Id)
    }
}

impl Service for Information {
    const UUID: Uuid = Uuid::from_u128(0xCE060010_43E5_11E4_916C_0800200C9A66);

    fn id(&self) -> Uuid {
        let b = match self {
            Information::ModelNumber => 0x0001,
            Information::SerialNumber => 0x0002,
            Information::HardwareRevision => 0x0003,
            Information::FirmwareRevision => 0x0004,
            Information::ManufacturerName => 0x0005,
            Information::MachineType => 0x0006,
        };
        Uuid::from_u128(Self::UUID.as_u128() | b << 96)
    }
}

impl Service for Control {
    const UUID: Uuid = Uuid::from_u128(0xCE060020_43E5_11E4_916C_0800200C9A66);

    fn id(&self) -> Uuid {
        let b = match self {
            Control::Receive => 0x0001,
            Control::Transmit => 0x0002,
        };
        Uuid::from_u128(Self::UUID.as_u128() | b << 96)
    }
}

impl Service for Rowing {
    const UUID: Uuid = Uuid::from_u128(0xCE060030_43E5_11E4_916C_0800200C9A66);

    #[inline]
    fn id(&self) -> Uuid {
        let b = match self {
            Rowing::GeneralStatus { .. } => 0x0001,
            Rowing::AdditionalStatusOne { .. } => 0x0002,
            Rowing::AdditionalStatusTwo { .. } => 0x0003,
            Rowing::GeneralStatusRate { .. } => 0x0004,
            Rowing::StrokeData { .. } => 0x0005,
            Rowing::AdditionalStrokeData { .. } => 0x0006,
            Rowing::SplitIntervalData { .. } => 0x0007,
            Rowing::AdditionalSplitIntervalData { .. } => 0x0008,
            Rowing::EndOfWorkoutSummaryData { .. } => 0x0009,
            Rowing::AdditionalEndOfWorkoutSummaryData { .. } => 0x000A,
            Rowing::HeartRateBeltInformation { .. } => 0x000B,
            Rowing::AdditionalEndOfWorkoutSummaryDataTwo { .. } => 0x000C,
            Rowing::ForceCurveData { .. } => 0x000D,
            Rowing::AdditionalStatusThree { .. } => 0x000E,
            Rowing::MultiplexedInformation { .. } => 0x000F,
        };
        Uuid::from_u128(Self::UUID.as_u128() | b << 96)
    }
}

pub enum Heartrate {
    Receive,
}

impl Service for Heartrate {
    const UUID: Uuid = Uuid::from_u128(0xCE060040_43E5_11E4_916C_0800200C9A66);

    fn id(&self) -> Uuid {
        let b = match self {
            Heartrate::Receive => 0x0001,
        };
        Uuid::from_u128(Self::UUID.as_u128() | b << 96)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::parse::Parse;

    use super::*;

    #[test]
    fn test_characteristic() {
        let id = Rowing::MultiplexedInformation.id();
        assert_eq!(id, Uuid::from_u128(0xCE06003F_43E5_11E4_916C_0800200C9A66));
    }

    #[test]
    fn test_data() {
        let samples = vec![
            [
                186u8, 5, 0, 237, 1, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [30, 6, 0, 19, 2, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 79],
            [
                131, 6, 0, 58, 2, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [
                231, 6, 0, 95, 2, 0, 1, 1, 1, 1, 3, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [
                77, 7, 0, 134, 2, 0, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [
                179, 7, 0, 174, 2, 0, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [
                24, 8, 0, 213, 2, 0, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [
                125, 8, 0, 252, 2, 0, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [
                226, 8, 0, 35, 3, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 78,
            ],
            [70, 9, 0, 73, 3, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 79],
            [
                169, 9, 0, 108, 3, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [
                13, 10, 0, 141, 3, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [
                115, 10, 0, 172, 3, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [
                215, 10, 0, 199, 3, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [
                60, 11, 0, 225, 3, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [
                102, 11, 0, 236, 3, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 128, 79,
            ],
            [
                4, 12, 0, 252, 3, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 80,
            ],
            [
                106, 12, 0, 18, 4, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 80,
            ],
            [
                205, 12, 0, 38, 4, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 80,
            ],
            [
                50, 13, 0, 57, 4, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 80,
            ],
            [
                153, 13, 0, 76, 4, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 80,
            ],
            [
                254, 13, 0, 93, 4, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 80,
            ],
            [
                95, 14, 0, 104, 4, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 87,
            ],
            [
                194, 14, 0, 119, 4, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 87,
            ],
            [
                42, 15, 0, 135, 4, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 87,
            ],
            [
                142, 15, 0, 150, 4, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 87,
            ],
            [
                241, 15, 0, 164, 4, 0, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 128, 87,
            ],
        ];

        for sample in samples {
            let mut data: Cursor<Vec<u8>> = Cursor::new(sample.to_vec());
            // let status = RowingData::GeneralStatus {
            //     elapsed_time: ,
            //     distance: Default::default(),
            //     workout_type: Default::default(),
            //     interval_type: Default::default(),
            //     workout_state: Default::default(),
            //     rowing_state: Default::default(),
            //     stroke_state: Default::default(),
            //     total_work_distance: Default::default(),
            //     workout_duration: Default::default(),
            //     workout_duration_type: Default::default(),
            //     drag_factor: Default::default(),
            // };
            // println!("data: {:#?}", Rowing::GeneralStatus.parse(&mut data));
        }
    }
}
