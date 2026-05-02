use std::io::Cursor;

use uuid::Uuid;

use crate::{csafe::CsafeResponse, parse::Parse, types::*};

pub enum Pm5 {
    Information(Information),
    Control(Control),
    Rowing(Rowing),
    HeartRate(HeartRate),
}

impl Pm5 {
    pub fn rowing() -> &'static [Rowing] {
        &[
            Rowing::GeneralStatus,
            Rowing::AdditionalStatusOne,
            Rowing::StrokeData,
            Rowing::AdditionalStrokeData,
        ]
    }
}

#[derive(Debug)]
pub enum Pm5Data {
    Rowing(RowingData),
    Control(ControlData),
}

impl ServiceData for Pm5 {
    type Data = Pm5Data;

    fn parse(uuid: Uuid, data: Vec<u8>) -> Result<Self::Data, ServiceDataError> {
        if Rowing::characteristic_is_part_of_service(uuid) {
            Rowing::parse(uuid, data).map(Pm5Data::Rowing)
        } else if Control::characteristic_is_part_of_service(uuid) {
            Control::parse(uuid, data).map(Pm5Data::Control)
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

#[derive(Debug, Clone, Default)]
pub struct GeneralStatusRate {
    pub interval: SampleRate,
}

#[derive(Debug, Clone, Default)]
pub struct AdditionalStatusTwo {
    pub elapsed_time: Time,
    pub interval_count: IntervalCount,
    pub average_power: Power,
    pub total_calories: Calories,
    pub split_interval_avg_pace: Pace,
    pub split_interval_avg_power: Power,
    pub split_interval_avg_calories: Calories,
    pub last_split_time: Time,
    pub last_split_distance: Distance,
}

#[derive(Debug, Clone, Default)]
pub struct GeneralStatus {
    pub elapsed_time: Time,
    pub distance: Distance,
    pub workout_type: WorkoutType,
    pub interval_type: IntervalType,
    pub workout_state: WorkoutState,
    pub rowing_state: RowingState,
    pub stroke_state: StrokeState,
    pub total_work_distance: Distance,
    pub workout_duration: Time,
    pub workout_duration_type: WorkoutDurationType,
    pub drag_factor: DragFactor,
}

#[derive(Debug, Clone, Default)]
pub struct AdditionalStatusOne {
    pub elapsed_time: Time,
    pub speed: Speed,
    pub stroke_rate: StrokeRate,
    pub heart_rate: HeartRate,
    pub current_pace: Pace,
    pub average_pace: Pace,
    pub rest_distance: RestDistance,
    pub rest_time: Time, // Strange that this is 3 bytes, but other rest times are 2?
    pub machine_type: ErgMachineType,
}

#[derive(Debug, Clone, Default)]
pub struct StrokeData {
    pub elapsed_time: Time,
    pub distance: Distance,
    pub drive_length: DriveLength,
    pub drive_time: DriveTime,
    pub stroke_recovery: StrokeRecoveryTime,
    pub stroke_distance: StrokeDistance,
    pub peak_drive_force: Force,
    pub avg_drive_force: Force,
    pub work_per_stroke: Work,
    pub stroke_count: StrokeCount,
}

#[derive(Debug, Clone, Default)]
pub struct AdditionalStrokeData {
    pub elapsed_time: Time,
    pub stroke_power: Power,
    pub stroke_calories: Calories,
    pub stroke_count: StrokeCount,
    pub projected_work_time: Time,
    pub projected_work_distance: Distance,
}

#[derive(Debug, Clone, Default)]
pub struct SplitIntervalData {
    pub elapsed_time: Time,
    pub distance: Distance,
    pub split_interval_time: Time,
    pub split_interval_distance: Distance,
    pub interval_rest_time: RestTime,
    pub interval_rest_distance: RestDistance,
    pub split_interval_type: IntervalType,
    pub split_interval_number: IntervalCount,
}

#[derive(Debug, Clone, Default)]
pub struct AdditionalSplitIntervalData {
    pub elapsed_time: Time,
    pub split_interval_avg_stroke_rate: StrokeRate,
    pub split_interval_work_heartrate: HeartRate,
    pub split_interval_rest_heartrate: HeartRate,
    pub split_interval_avg_pace: Pace,
    pub split_interval_total_calories: Calories,
    pub split_interval_avg_calories: Calories,
    pub split_interval_speed: Speed,
    pub split_interval_power: Power,
    pub split_avg_drag_factor: DragFactor,
    pub split_interval_number: IntervalCount,
    pub erg_machine_type: ErgMachineType,
}

#[derive(Debug, Clone, Default)]
pub struct EndOfWorkoutSummaryData {
    pub log_entry_date: LogEntryDate,
    pub log_entry_time: LogEntryTime,
    pub elapsed_time: Time,
    pub distance: Distance,
    pub avg_stroke_rate: StrokeRate,
    pub ending_heartrate: HeartRate,
    pub avg_heartrate: HeartRate,
    pub min_heartrate: HeartRate,
    pub max_heartrate: HeartRate,
    pub drag_factor_avg: DragFactor,
    pub recover_heartrate: HeartRate,
    pub workout_type: WorkoutType,
    pub avg_pace: Pace,
}

#[derive(Debug, Clone, Default)]
pub struct AdditionalEndOfWorkoutSummaryData {
    pub log_entry_date: LogEntryDate,
    pub log_entry_time: LogEntryTime,
    pub split_interval_type: IntervalType,
    pub split_interval_size: Size,
    pub split_interval_count: IntervalCount,
    pub total_calories: Calories,
    pub watts: Work,
    pub total_rest_distance: Distance,
    pub interval_rest_time: RestTime,
    pub avg_calories: Calories,
}

#[derive(Debug, Clone, Default)]
pub struct HeartRateBeltInformation {
    pub manufacturer_id: u8,
    pub device_type: u8,
    pub belt_id: u32,
}

#[derive(Debug, Clone, Default)]
pub struct AdditionalEndOfWorkoutSummaryDataTwo {
    pub log_entry_date: LogEntryDate,
    pub log_entry_time: LogEntryTime,
    pub avg_pace: Pace,
    pub game_id: GameId,
    pub game_score: GameScore,
    pub erg_machine_type: ErgMachineType,
}

#[derive(Debug, Clone, Default)]
pub struct ForceCurveData {
    pub data: crate::types::ForceCurveData,
}

#[derive(Debug, Clone, Default)]
pub struct AdditionalStatusThree {}
#[derive(Debug, Clone, Default)]
pub struct MultiplexedInformation {}

#[derive(Debug, Clone)]
pub enum RowingData {
    GeneralStatus(GeneralStatus),
    AdditionalStatusOne(AdditionalStatusOne),
    AdditionalStatusTwo(AdditionalStatusTwo),
    GeneralStatusRate(GeneralStatusRate),
    StrokeData(StrokeData),
    AdditionalStrokeData(AdditionalStrokeData),
    SplitIntervalData(SplitIntervalData),
    AdditionalSplitIntervalData(AdditionalSplitIntervalData),
    EndOfWorkoutSummaryData(EndOfWorkoutSummaryData),
    AdditionalEndOfWorkoutSummaryData(AdditionalEndOfWorkoutSummaryData),
    HeartRateBeltInformation(HeartRateBeltInformation),
    AdditionalEndOfWorkoutSummaryDataTwo(AdditionalEndOfWorkoutSummaryDataTwo),
    ForceCurveData(ForceCurveData),
    AdditionalStatusThree(AdditionalStatusThree),
    MultiplexedInformation(MultiplexedInformation),
}

pub trait Service {
    const UUID: Uuid;

    fn id(&self) -> Uuid;

    fn characteristic_is_part_of_service(characteristic: Uuid) -> bool {
        Self::UUID.as_u128() == (characteristic.as_u128() & !(0x000F << 96))
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
    #[error("unknown service")]
    InvalidCsafe,
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
            return Ok(RowingData::GeneralStatus(GeneralStatus {
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
            }));
        }

        if Rowing::AdditionalStatusOne.id() == uuid {
            return Ok(RowingData::AdditionalStatusOne(AdditionalStatusOne {
                elapsed_time: Parse::parse(&mut data)?,
                speed: Parse::parse(&mut data)?,
                stroke_rate: Parse::parse(&mut data)?,
                heart_rate: Parse::parse(&mut data)?,
                current_pace: Parse::parse(&mut data)?,
                average_pace: Parse::parse(&mut data)?,
                rest_distance: Parse::parse(&mut data)?,
                rest_time: Parse::parse(&mut data)?,
                machine_type: Parse::parse(&mut data)?,
            }));
        }

        if Rowing::StrokeData.id() == uuid {
            return Ok(RowingData::StrokeData(StrokeData {
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
            }));
        }

        if Rowing::AdditionalStrokeData.id() == uuid {
            return Ok(RowingData::AdditionalStrokeData(AdditionalStrokeData {
                elapsed_time: Parse::parse(&mut data)?,
                stroke_power: Parse::parse(&mut data)?,
                stroke_calories: Parse::parse(&mut data)?,
                stroke_count: Parse::parse(&mut data)?,
                projected_work_time: Parse::parse(&mut data)?,
                projected_work_distance: Parse::parse(&mut data)?,
            }));
        }

        if Rowing::SplitIntervalData.id() == uuid {
            return Ok(RowingData::SplitIntervalData(SplitIntervalData {
                elapsed_time: Parse::parse(&mut data)?,
                distance: Parse::parse(&mut data)?,
                split_interval_time: Parse::parse(&mut data)?,
                split_interval_distance: Parse::parse(&mut data)?,
                interval_rest_time: Parse::parse(&mut data)?,
                interval_rest_distance: Parse::parse(&mut data)?,
                split_interval_type: Parse::parse(&mut data)?,
                split_interval_number: Parse::parse(&mut data)?,
            }));
        }

        if Rowing::AdditionalSplitIntervalData.id() == uuid {
            return Ok(RowingData::AdditionalSplitIntervalData(
                AdditionalSplitIntervalData {
                    elapsed_time: Parse::parse(&mut data)?,
                    split_interval_avg_stroke_rate: Parse::parse(&mut data)?,
                    split_interval_work_heartrate: Parse::parse(&mut data)?,
                    split_interval_rest_heartrate: Parse::parse(&mut data)?,
                    split_interval_avg_pace: Parse::parse(&mut data)?,
                    split_interval_total_calories: Parse::parse(&mut data)?,
                    split_interval_avg_calories: Parse::parse(&mut data)?,
                    split_interval_speed: Parse::parse(&mut data)?,
                    split_interval_power: Parse::parse(&mut data)?,
                    split_avg_drag_factor: Parse::parse(&mut data)?,
                    split_interval_number: Parse::parse(&mut data)?,
                    erg_machine_type: Parse::parse(&mut data)?,
                },
            ));
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

#[derive(Debug)]
pub struct ControlData {
    response: CsafeResponse,
}

impl ControlData {
    pub fn is_ok(&self) -> bool {
        matches!(self.response.status, crate::csafe::ResponseStatus::Ok)
    }

    pub fn data(&self) -> &[u8] {
        &self.response.data
    }
}

impl ServiceData for Control {
    type Data = ControlData;

    fn parse(_uuid: Uuid, data: Vec<u8>) -> Result<Self::Data, ServiceDataError> {
        Ok(ControlData {
            response: CsafeResponse::parse(&data).map_err(|_| ServiceDataError::InvalidCsafe)?,
        })
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
    fn test_recv_characteristic() {
        let id = dbg!(Control::Receive.id());
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
