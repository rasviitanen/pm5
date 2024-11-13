use uuid::Uuid;

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
            Rowing::GeneralStatus => 0x0001,
            Rowing::AdditionalStatusOne => 0x0002,
            Rowing::AdditionalStatusTwo => 0x0003,
            Rowing::GeneralStatusRate => 0x0004,
            Rowing::StrokeData => 0x0005,
            Rowing::AdditionalStrokeData => 0x0006,
            Rowing::SplitIntervalData => 0x0007,
            Rowing::AdditionalSplitIntervalData => 0x0008,
            Rowing::EndOfWorkoutSummaryData => 0x0009,
            Rowing::AdditionalEndOfWorkoutSummaryData => 0x000A,
            Rowing::HeartRateBeltInformation => 0x000B,
            Rowing::AdditionalEndOfWorkoutSummaryDataTwo => 0x000C,
            Rowing::ForceCurveData => 0x000D,
            Rowing::AdditionalStatusThree => 0x000E,
            Rowing::MultiplexedInformation => 0x000F,
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
