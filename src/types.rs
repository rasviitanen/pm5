use num_enum::TryFromPrimitive;

mod privat {
    use std::io::Read;

    use super::*;
    use byteorder::{LittleEndian, ReadBytesExt};

    impl crate::parse::Parse for U24 {
        fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::parse::ParseError> {
            Ok(Self(cursor.read_u24::<LittleEndian>()?))
        }
    }

    impl crate::parse::Parse for u32 {
        fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::parse::ParseError> {
            Ok(cursor.read_u32::<LittleEndian>()?)
        }
    }

    impl crate::parse::Parse for u16 {
        fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::parse::ParseError> {
            Ok(cursor.read_u16::<LittleEndian>()?)
        }
    }

    impl crate::parse::Parse for u8 {
        fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::parse::ParseError> {
            Ok(cursor.read_u8()?)
        }
    }

    impl crate::parse::Parse for ForceCurveData {
        fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::parse::ParseError> {
            let mut bytes = Vec::new();
            cursor.read_to_end(&mut bytes)?;
            let mut out = Vec::with_capacity(bytes.len() / 2);
            let mut iter_bytes = bytes.into_iter();
            while let Some(a) = iter_bytes.next() {
                let b = iter_bytes
                    .next()
                    .ok_or(crate::parse::ParseError::UnexpectedNumberOfBytes)?;
                let force = Force(u16::from_le_bytes([a, b]));
                out.push(force);
            }
            Ok(ForceCurveData(out))
        }
    }

    macro_rules! impl_parse_struct_type {
        ( $( $x:path ),* $(,)? ) => {
            $(impl crate::parse::Parse for $x {
                fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::parse::ParseError> {
                Ok(Self(crate::parse::Parse::parse(cursor)?))
                }
            })*
        };
    }

    impl_parse_struct_type![
        Time,
        LogEntryTime,
        LogEntryDate,
        StrokeRecoveryTime,
        Distance,
        RestTime,
        RestDistance,
        Pace,
        Speed,
        StrokeRate,
        HeartRate,
        DragFactor,
        IntervalCount,
        Power,
        DriveLength,
        DriveTime,
        Calories,
        StrokeDistance,
        Force,
        Work,
        Size,
        StrokeCount,
        GameId,
        GameScore,
    ];

    macro_rules! impl_parse_enum_type {
        ( $( $x:path ),* $(,)? ) => {
            $(impl crate::parse::Parse for $x {
                fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::parse::ParseError> {
                    let v: u8 = crate::parse::Parse::parse(cursor)?;
                    Ok(Self::try_from(v).map_err(|_err| crate::parse::ParseError::Variant)?)
                }
            })*
        };
    }

    impl_parse_enum_type![
        OperationalState,
        ErgModelType,
        ErgMachineType,
        WorkoutType,
        IntervalType,
        WorkoutState,
        RowingState,
        StrokeState,
        WorkoutDurationType,
        DisplayUnitType,
        DisplayFormatType,
        WorkoutNumber,
        WorkoutProgrammingMode,
        StrokeRateState,
        StartType,
        RaceOperationType,
        RaceState,
        RaceType,
        RaceStartState,
        ScreenType,
        ScreenValueWorkoutType,
        ScreenValueRaceType,
        ScreenValueCsafe,
        ScreenStatus,
        StatusType,
        DisplayUpdateRate,
    ];
}

#[derive(Default, Debug)]
pub struct U24(u32);

impl std::ops::Deref for U24 {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for U24 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
// u24, little-endian
#[derive(Default, Debug)]
pub struct Time(pub U24);
#[derive(Default, Debug)]
pub struct LogEntryTime(pub u16);
#[derive(Default, Debug)]
pub struct LogEntryDate(pub u16);
#[derive(Default, Debug)]
pub struct StrokeRecoveryTime(pub u16);
// u24, little-endian
#[derive(Default, Debug)]
pub struct Distance(pub U24);
#[derive(Default, Debug)]
pub struct RestTime(pub u16);
#[derive(Default, Debug)]
pub struct RestDistance(pub u16);
#[derive(Default, Debug)]
pub struct Pace(pub u16);
#[derive(Default, Debug)]
pub struct Speed(pub u16);
#[derive(Default, Debug)]
pub struct StrokeRate(pub u8);
#[derive(Default, Debug)]
pub struct HeartRate(pub u8);
#[derive(Default, Debug)]
pub struct DragFactor(pub u8);
#[derive(Default, Debug)]
pub struct IntervalCount(pub u8);
#[derive(Default, Debug)]
pub struct Power(pub u16);
#[derive(Default, Debug)]
pub struct DriveLength(pub u8);
#[derive(Default, Debug)]
pub struct DriveTime(pub u8);
#[derive(Default, Debug)]
pub struct Calories(pub u16);
#[derive(Default, Debug)]
pub struct StrokeDistance(pub u16);
#[derive(Default, Debug)]
pub struct Force(pub u16);
#[derive(Default, Debug)]
pub struct Work(pub u16);
#[derive(Default, Debug)]
pub struct Size(pub u8);
#[derive(Default, Debug)]
pub struct StrokeCount(pub u16);
#[derive(Default, Debug)]
pub struct GameId(pub u8);
#[derive(Default, Debug)]
pub struct GameScore(pub u16);

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum SampleRate {
    Slow,    // 0x0001
    Default, // 0x0001
    Fast,    // 0x0002
    Fastest, // 0x0003
}
#[derive(Debug)]
pub struct ForceCurveData(pub Vec<Force>);

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum OperationalState {
    /// Reset state (0).
    Reset,
    /// Ready state (1).
    Ready,
    /// Workout state (2).
    Workout,
    /// Warm-up state (3).
    Warmup,
    /// Race state (4).
    Race,
    /// Power-off state (5).
    Poweroff,
    /// Pause state (6).
    Pause,
    /// Invoke boot loader state (7).
    Invokebootloader,
    /// Power-off ship state (8).
    PoweroffShip,
    /// Idle charge state (9).
    IdleCharge,
    /// Idle state (10).
    Idle,
    /// Manufacturing test state (11).
    Mfgtest,
    /// Firmware update state (12).
    Fwupdate,
    /// Drag factor state (13).
    Dragfactor,
    /// Drag factor calibration state (100).
    Dfcalibration = 100,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ErgModelType {
    /// Model D/E type (0).
    TypeD,
    /// Model C/B type (1).
    TypeC,
    /// Model A type (2).
    TypeA,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ErgMachineType {
    /// Model D, static type (0).
    StaticD,
    /// Model C, static type (1).
    StaticC,
    /// Model A, static type (2).
    StaticA,
    /// Model B, static type (3).
    StaticB,
    /// Model E, static type (5).
    StaticE = 5,
    /// Rower simulator type (7).
    StaticSimulator = 7,
    /// Dynamic, static type (8).
    StaticDynamic = 8,
    /// Model A, slides type (16).
    SlidesA = 16,
    /// Model B, slides type (17).
    SlidesB,
    /// Model C, slides type (18).
    SlidesC,
    /// Model D, slides type (19).
    SlidesD,
    /// Model E, slides type (20).
    SlidesE,
    /// Dynamic, linked type (32).
    LinkedDynamic = 32,
    /// Dynomometer, static type (32).
    StaticDyno = 64,
    /// Ski Erg, static type (128).
    StaticSki = 128,
    /// Ski simulator type (143).
    StaticSkiSimulator = 143,
    /// Bike, no arms type (192).
    Bike = 192,
    /// Bike, arms type (193).
    BikeArms,
    /// Bike, no arms type (194).
    BikeNoarms,
    /// Bike simulator type (207).
    BikeSimulator = 207,
    /// Multi-erg row type (224).
    MultiergRow = 224,
    /// Multi-erg ski type (225).
    MultiergSki,
    /// Multi-erg bike type (226).
    MultiergBike,
    /// Number of machine types (227).
    Num,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum WorkoutType {
    /// JustRow, no splits (0).
    JustrowNosplits,
    /// JustRow, splits (1).
    JustrowSplits,
    /// Fixed distance, no splits (2).
    FixeddistNosplits,
    /// Fixed distance, splits (3).
    FixeddistSplits,
    /// Fixed time, no splits (4).
    FixedtimeNosplits,
    /// Fixed time, splits (5).
    FixedtimeSplits,
    /// Fixed time interval (6).
    FixedtimeInterval,
    /// Fixed distance interval (7).
    FixeddistInterval,
    /// Variable interval (8).
    VariableInterval,
    /// Variable interval, undefined rest (9).
    VariableUndefinedrestInterval,
    /// Fixed calorie, splits (10).
    FixedcalorieSplits,
    /// Fixed watt-minute, splits (11).
    FixedwattminuteSplits,
    /// Fixed calorie interval (12).
    FixedcalsInterval,
    /// Number of workout types (13).
    Num,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum IntervalType {
    /// Time interval type (0).
    Time,
    /// Distance interval type (1).
    Dist,
    /// Rest interval type (2).
    Rest,
    /// Time undefined rest interval type (3).
    TimerestUndefined,
    /// Distance undefined rest interval type (4).
    DistancerestUndefined,
    /// Undefined rest interval type (5).
    RestUndefined,
    /// Calorie interval type (6).
    Calorie,
    /// Calorie undefined rest interval type (7).
    CalorierestUndefined,
    /// Watt-minute interval type (8).
    Wattminute,
    /// Watt-minute undefined rest interval type (9).
    WattminuterestUndefined,
    /// No interval type (255 ).
    None = 255,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum WorkoutState {
    /// Wait to begin state (0).
    WaitToBegin,
    /// Workout row state (1).
    WorkoutRow,
    /// Countdown pause state (2).
    CountdownPause,
    /// Interval rest state (3).
    IntervalRest,
    /// Interval work time state (4).
    IntervalWorkTime,
    /// Interval work distance state (5).
    IntervalWorkDistance,
    /// Interval rest end to work time state (6).
    IntervalRestEndToWorkTime,
    /// Interval rest end to work distance state (7).
    IntervalRestEndToWorkDistance,
    /// Interval work time to rest state (8).
    IntervalWorkTimeToRest,
    /// Interval work distance to rest state (9).
    IntervalWorkDistanceToRest,
    /// Workout end state (10).
    WorkoutEnd,
    /// Workout terminate state (11).
    Terminate,
    /// Workout logged state (12).
    WorkoutLogged,
    /// Workout rearm state (13).
    Rearm,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RowingState {
    /// Inactive (0).
    Inactive,
    /// Active (1).
    Active,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum StrokeState {
    /// FW to reach min speed state (0).
    WaitingForWheelToReachMinSpeedState,
    /// FW to accelerate state (1).
    WaitingForWheelToAccelerateState,
    /// Driving state (2).
    DrivingState,
    /// Dwelling after drive state (3).
    DwellingAfterDriveState,
    /// Recovery state (4).
    RecoveryState,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum WorkoutDurationType {
    Time = 0,
    Calories = 0x40,
    Distance = 0x80,
    WattMin = 0xC0,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DisplayUnitType {
    /// Time/meters display units (0).
    TimeMeters,
    /// Pace display units (1).
    Pace,
    /// Watts display units (2).
    Watts,
    /// Caloric burn rate display units (3).
    CaloricBurnRate,
    /// Calorie display units (4).
    Calories,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DisplayFormatType {
    /// Standard display type (0).
    Standard,
    /// Force curve display type (1).
    ForceVelocity,
    /// Pace boats display type (2).
    PaceBoat,
    /// Store rate/heart rate display type (3).
    PerStroke,
    /// Large format display type (4).
    Simple,
    /// Target display type (5).
    Target,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum WorkoutNumber {
    /// Programmed (0).
    Programmed,
    /// Standard list 1 (1).
    Default1,
    /// Standard list 2 (2).
    Default2,
    /// Standard list 3 (3).
    Default3,
    /// Standard list 4 (4).
    Default4,
    /// Standard list 5 (5).
    Default5,
    /// Custom list 1 (6).
    Custom1,
    /// Custom list 2 (7).
    Custom2,
    /// Custom list 3 (8).
    Custom3,
    /// Custom list 4 (9).
    Custom4,
    /// Custom list 5 (10).
    Custom5,
    /// Favorite list 1 (11).
    Msd1,
    /// Favorite list 2 (12).
    Msd2,
    /// Favorite list 3 (13).
    Msd3,
    /// Favorite list 4 (14).
    Msd4,
    /// Favorite list 5 (15).
    Msd5,
    /// Number of workouts (16).
    Num,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum WorkoutProgrammingMode {
    /// Disable (0).
    Disable,
    /// Enable (1).
    Enable,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum StrokeRateState {
    /// Idle state (0).
    Idle,
    /// Steady state (1).
    Steady,
    /// Increasing state (2).
    Increasing,
    /// Decreasing state (3).
    Decreasing,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum StartType {
    /// Random type (0).
    Random,
    /// Countdown type (1).
    Countdown,
    /// Random modified type (2).
    RandomModified,
    /// Immediate type (3).
    Immediate,
    /// Wait for flywheel type (4).
    WaitForFlyWheel,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RaceOperationType {
    /// Disable type (0).
    Disable,
    /// Participation request type (1).
    ParticipationRequest,
    /// Sleep type (2).
    Sleep,
    /// Erg initialization type (3).
    ErgInit,
    /// Physical address/lane initialization type (4).
    PhyAddrInit,
    /// Race warmup type (5).
    RaceWarmup,
    /// Race initialization type (6).
    RaceInit,
    /// Time synchronization type (7).
    TimeSync,
    /// Race wait to start type (8).
    RaceWaitToStart,
    /// Race start type (9).
    Start,
    /// Race false start type (10).
    FalseStart,
    /// Race terminate type (11).
    Terminate,
    /// Race idle type (12).
    Idle,
    /// Tach simulator enable type (13).
    TachSimEnable,
    /// Tach simulator disable type (14).
    TachSimDisable,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RaceState {
    /// Race idle state (0).
    Idle,
    /// Race countdown state (1).
    Countdown,
    /// Race rowing state (2).
    Rowing,
    /// Race interval rest state (3).
    IntervalRest,
    /// Race end interval state (4).
    EndInterval,
    /// Race end workout state (5).
    EndWorkoutRace,
    /// Race terminate workout state (6).
    TerminateWorkoutRace,
    /// Race false start state (7).
    Falsestart,
    /// Race inactive state (8).
    Inactive,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RaceType {
    /// Fixed distance, individual type (0).
    FixeddistSingleerg,
    /// Fixed time, individual type (1).
    FixedtimeSingleerg,
    /// Fixed distance, team type (2).
    FixeddistTeamerg,
    /// Fixed time, team type (3).
    FixedtimeTeamerg,
    /// Workout race start type (4).
    Workoutracestart,
    /// Fixed calorie, individual type (5).
    FixedcalSingleerg,
    /// Fixed calorie, team type (6).
    FixedcalTeamerg,
    /// Fixed distance, relay individual type (7).
    FixeddistRelaySingleerg,
    /// Fixed time, relay individual type (8).
    FixedtimeRelaySingleerg,
    /// Fixed calorie, relay individual type (9).
    FixedcalRelaySingleerg,
    /// Fixed distance, relay team type (10).
    FixeddistRelayTeamerg,
    /// Fixed time, relay team type (11).
    FixedtimeRelayTeamerg,
    /// Fixed calorie, relay team type (12).
    FixedcalRelayTeamerg,
    /// Fixed distance, multiactivity individual type, sequential use (13).
    FixeddistMultiactivitySequentialSingleerg,
    /// Fixed time, multiactivity individual type, sequential use (14).
    FixedtimeMultiactivitySequentialSingleerg,
    /// Fixed calorie, multiactivity individual type, sequential use (15).
    FixedcalMultiactivitySequentialSingleerg,
    /// Fixed distance, multiactivity team type, sequential use (16).
    FixeddistMultiactivitySequentialTeamerg,
    /// Fixed time, multiactivity team type, sequential use (17).
    FixedtimeMultiactivitySequentialTeamerg,
    /// Fixed calorie, multiactivity team type, sequential use (18).
    FixedcalMultiactivitySequentialTeamerg,
    /// Fixed distance, Ergathlon type (19).
    FixeddistErgathlon,
    /// Fixed time, Ergathlon type (20).
    FixedtimeErgathlon,
    /// Fixed calorie, Ergathlon type (21).
    FixedcalErgathlon,
    /// Fixed distance, multiactivity individual type, simultaneous use (22).
    FixeddistMultiactivitySimultaneousSingleerg,
    /// Fixed time, multiactivity individual type, simultaneous use (23).
    FixedtimeMultiactivitySimultaneousSingleerg,
    /// Fixed calorie, multiactivity individual type, simultaneous use (24).
    FixedcalMultiactivitySimultaneousSingleerg,
    /// Fixed distance, multiactivity team type, simultaneous use (25).
    FixeddistMultiactivitySimultaneousTeamerg,
    /// Fixed time, multiactivity team type, simultaneous use (26).
    FixedtimeMultiactivitySimultaneousTeamerg,
    /// Fixed calorie, multiactivity team type, simultaneous use (27).
    FixedcalMultiactivitySimultaneousTeamerg,
    /// Fixed distance, Biathlon type (28).
    FixeddistBiathlon,
    /// Fixed calorie, Biathlon type (29).
    FixedcalBiathlon,
    /// Fixed distance, no change prompt, relay individual type (30).
    FixeddistRelayNochangeSingleerg,
    /// Fixed time, no change prompt, relay individual type (31).
    FixedtimeRelayNochangeSingleerg,
    /// Fixed calorie, no change prompt, relay individual type (32).
    FixedcalRelayNochangeSingleerg,
    /// Fixed time, calorie score, individual type (33).
    FixedtimeCalscoreSingleerg,
    /// Fixed time, calorie score, team type (34).
    FixedtimeCalscoreTeamerg,
    /// Fixed time, calorie score, individual type (35).
    FixeddistTimecapSingleerg,
    /// Fixed time, calorie score, team type (36).
    FixedcalTimecapSingleerg,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RaceStartState {
    /// Init state (0).
    Init,
    /// Prepare state (1).
    Prepare,
    /// Wait ready state (2).
    WaitReady,
    /// Wait attention state (3).
    WaitAttention,
    /// Wait row state (4).
    WaitRow,
    /// Countdown state (5).
    Countdown,
    /// Row state (6).
    Row,
    /// False start state (7).
    FalseStart,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ScreenType {
    // FIXME:(rasviitanen) weird None value here should probably eq 0. Should recheck the spec.
    None,
    /// Workout type (0).
    Workout,
    /// Race type (1).
    Race,
    /// CSAFE type (2).
    Csafe,
    /// Diagnostic type (3).
    Diag,
    /// Manufacturing type (4).
    Mfg,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ScreenValueWorkoutType {
    /// None value (0).
    None,
    /// Prepare to workout type (1).
    PrepareToRowWorkout,
    /// Terminate workout type (2).
    TerminateWorkout,
    /// Rearm workout type (3).
    RearmWorkout,
    /// Refresh local copies of logcard structures(4).
    RefreshLogCard,
    /// Prepare to race start (5).
    PrepareToRaceStart,
    /// Goto to main screen (6).
    GoToMainScreen,
    /// Log device busy warning (7).
    LogCardBusyWarning,
    /// Log device select user (8).
    LogCardSelectUser,
    /// Reset race parameters (9).
    ResetRaceParams,
    /// Cable test slave indication(10).
    CableTestSlave,
    /// Fish game (11).
    FishGame,
    /// Display participant info (12).
    DisplayParticipantInfo,
    /// Display participant info w/ confirmation (13).
    DisplayParticipantInfoConfirm,
    /// Display type set to target (20).
    ChangedisPlayTypeTarget = 20,
    /// Display type set to standard (21).
    ChangedisPlayTypeStandard,
    /// Display type set to forcevelocity (22).
    ChangedisPlayTypeForceVelocity,
    /// Display type set to Paceboat (23).
    ChangedisPlayTypePaceBoat,
    /// Display type set to perstroke (24).
    ChangedisPlayTypePerStroke,
    /// Display type set to simple (25).
    ChangedisPlayTypeSimple,
    /// Units type set to timemeters (30).
    ChangeUnitsTypeTimeMeters = 30,
    /// Units type set to pace (31).
    ChangeUnitsTypePace,
    /// Units type set to watts (32).
    ChangeUnitsTypeWatts,
    /// Units type set to caloric burn rate(33).
    ChangeUnitsTypeCaloricBurnRate,
    /// Basic target game (34).
    TargetGameBasic,
    /// Advanced target game (35).
    TargetGameAdvanced,
    /// Dart game (36).
    DartGame,
    /// USB wait ready (37).
    GoToUsbWaitReady,
    /// Tach cable test disable (38).
    TachCableTestDisable,
    /// Tach simulator disable (39).
    TachSimDisable,
    /// Tach simulator enable, rate = 1:12 (40).
    TachSimEnableRate1,
    /// Tach simulator enable, rate = 1:35 (41).
    TachSimEnableRate2,
    /// Tach simulator enable, rate = 1:42 (42).
    TachSimEnableRate3,
    /// Tach simulator enable, rate = 3:04 (43).
    TachSimEnableRate4,
    /// Tach simulator enable, rate = 3:14 (44).
    TachSimEnableRate5,
    /// Tach cable test enable (45).
    TachCableTestEnable,
    /// Units type set to calories(46).
    ChangeUnitsYypeCalories,
    /// Virtual key select A (47).
    VirtualkeyA,
    /// Virtual key select B (48).
    VirtualkeyB,
    /// Virtual key select C (49).
    VirtualkeyC,
    /// Virtual key select D (50).
    VirtualkeyD,
    /// Virtual key select E (51).
    VirtualkeyE,
    /// Virtual key select Units (52).
    VirtualkeyUnits,
    /// Virtual key select Display (53).
    VirtualkeyDisplay,
    /// Virtual key select Menu (54).
    VirtualkeyMenu,
    /// Tach simulator enable, rate = random (55).
    TachSimEnableRateRandom,
    /// Screen redraw (255).
    ScreenRedraw = 255,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ScreenValueRaceType {
    /// None value (0).
    None,
    /// Set physical address (1).
    SetPhysicalAddr,
    /// Confirm physical address (2).
    ConfirmPhysicalAddr,
    /// Warmup for race (3).
    WarmupForRace,
    /// Prepare to race (4).
    PrepareToRace,
    /// False start race (5).
    FalseStartRace,
    /// Terminate race (6).
    TerminateRace,
    /// Automatically set physical address (7).
    AutosetPhysAddr,
    /// Indication that participant list is being set (8).
    SetParticipantList,
    /// Indication that race time sync is occuring (9).
    SyncRaceTime,
    /// Preparation for sleeping erg (10).
    PrepareToSleep,
    /// Reset race parameters (11).
    ResetRaceParams,
    /// Set default communication parameters (12).
    SetDefaultCommParams,
    /// Enter race idle (13).
    RaceIdle,
    /// Display current erg physical address (14).
    ErgAddressStatus,
    /// Enter race idle row (15).
    RaceIdleRow,
    /// Display race bitmap (16).
    DisplayRaceBitmap,
    /// Display race text string (17).
    DisplayRaceTextString,
    /// Set logical address (18).
    SetLogicalAddr,
    /// Confirm logical address (19).
    ConfirmLogicalAddr,
    /// Discover secondary Ergs (20).
    ErgSlaveDiscovery,
    /// Goto to main screen (21).
    GotoMainScreen,
    /// Reset Erg (22).
    ResetErg,
    /// Set units type to default (23).
    SetUnitsTypeDefault,
    /// Tach simulator disable (39).
    TachSimDisable = 39,
    /// Tach simulator enable, rate = 1:12 (40).
    TachSimEnableRate1,
    /// Tach simulator enable, rate = 1:35 (41).
    TachSimEnableRate2,
    /// Tach simulator enable, rate = 1:42 (42).
    TachSimEnableRate3,
    /// Tach simulator enable, rate = 3:04 (43).
    TachSimEnableRate4,
    /// Tach simulator enable, rate = 3:14 (44).
    TachSimEnableRate5,
    /// Tach cable test enable (45).
    TachCableTestEnable,
    /// Ergathlon mode disable (46).
    ErgaThlonModeDisable,
    /// RS-485 firmware update in progress (47).
    Rs485FirmwareUpdateProgress,
    /// Terminate race and preserve results (48).
    TerminateRaceAndPreserveResults,
    /// Tach simulator enable, rate = random (49).
    TachSimEnableRateRandom,
    /// Screen redraw (255).
    ScreenRedraw = 255,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ScreenValueCsafe {
    /// None value (0).
    None,
    /// Enter user ID (1).
    UserId,
    /// Prepare to workout (2).
    PrepareToRowWorkout,
    /// Goto to main screen (3).
    GotoMainScreen,
    /// Goto custom screen (4).
    Custom,
    /// Open racing channel (250).
    RaceChanOpen = 250,
    /// Close racing channel (251).
    RaceChanClose = 251,
    /// Screen redraw (255).
    ScreenRedraw = 255,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum ScreenStatus {
    Inactive,
    Pending,
    Inprogress,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum StatusType {
    /// None (0).
    None,
    /// Battery level 1 warning, status value = (current battery level/max battery value) * 100 (1).
    BatteryLevel1Warning,
    /// Battery level 2 warning, status value = (current battery level/max battery value) * 100 (2).
    BatteryLevel2Warning,
    /// Log device state, status value = log device status (3).
    LogDeviceState,
    /// Log device state, status value = log device status (3).
    // LogcardState = STATUSTYPELOGDEVICESTATE,
    /// Power source, status value = power source status (4).
    PowerSourceState,
    /// Log device workout logged, status value = workout logged status (5).
    LogcardWorkoutloggedStatus,
    /// Flywheel, status value = not turning, turning (6).
    FlywheelState,
    /// Bad utility, status value = correct utilty, wrong utility (7).
    BadUtilityState,
    /// Firmware update, status value = no update pending, update pending, update complete (8).
    FwUpdateStatus,
    /// Unsupported USB host device, status value = unused (9).
    UnsupportedUsbHostDevice,
    /// USB host drive, status value = uninitialized, initialized (10).
    UsbDriveState,
    /// Load control, status value = all loads allowed, usb host not allowed, backlight not allowed, neither allowed (11).
    LoadControlStatus,
    /// USB log book, status value = directory missing/corrupt, file missing/corrupt, validated (12).
    UsbLogbookStatus,
    /// Log storage capacity warning, status value = current used capacity (13).
    LogStorageCapactyWarningStatus,
    /// Full calibration warning, status value = unused (14).
    FactoryCalibrationWarning,
    /// Verify calibration warning, status value = unused (15).
    VerifyCalibrationWarning,
    /// Service calibration warning, status value = unused (16).
    ServiceCalibrationWarning,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DisplayUpdateRate {
    /// 5Hz (0).
    Hz5,
    /// 4Hz (1).
    Hz4,
    /// 2Hz (2).
    Hz2,
}
