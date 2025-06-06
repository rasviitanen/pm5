use uuid::Uuid;

use crate::types::*;

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
    use super::*;

    #[test]
    fn test_characteristic() {
        let id = Rowing::MultiplexedInformation.id();
        assert_eq!(id, Uuid::from_u128(0xCE06003F_43E5_11E4_916C_0800200C9A66));
    }
}
