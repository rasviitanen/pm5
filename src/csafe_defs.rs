// CSAFE Protocol Definitions for Concept2 Ergometers

// Frame contents
pub const EXT_FRAME_START_BYTE: u8 = 0xF0;
pub const FRAME_START_BYTE: u8 = 0xF1;
pub const FRAME_END_BYTE: u8 = 0xF2;
pub const FRAME_STUFF_BYTE: u8 = 0xF3;

pub const FRAME_MAX_STUFF_OFFSET_BYTE: u8 = 0x03;

pub const FRAME_FLG_LEN: usize = 2;
pub const EXT_FRAME_ADDR_LEN: usize = 2;
pub const FRAME_CHKSUM_LEN: usize = 1;

pub const SHORT_CMD_TYPE_MSK: u8 = 0x80;
pub const LONG_CMD_HDR_LENGTH: usize = 2;
pub const LONG_CMD_BYTE_CNT_OFFSET: usize = 1;
pub const RSP_HDR_LENGTH: usize = 2;

pub const FRAME_STD_TYPE: u8 = 0;
pub const FRAME_EXT_TYPE: u8 = 1;

pub const DESTINATION_ADDR_HOST: u8 = 0x00;
pub const DESTINATION_ADDR_ERG_MASTER: u8 = 0x01;
pub const DESTINATION_ADDR_BROADCAST: u8 = 0xFF;
pub const DESTINATION_ADDR_ERG_DEFAULT: u8 = 0xFD;

pub const FRAME_MAXSIZE: usize = 96;
pub const INTERFRAMEGAP_MIN: u32 = 50; // msec
pub const CMDUPLIST_MAXSIZE: usize = 10;
pub const MEMORY_BLOCKSIZE: usize = 64;
pub const FORCEPLOT_BLOCKSIZE: usize = 32;
pub const HEARTBEAT_BLOCKSIZE: usize = 32;

// Manufacturer Info
pub const MANUFACTURE_ID: u8 = 22; // assigned by Fitlinxx for Concept2
pub const CLASS_ID: u8 = 2; // standard CSAFE equipment
pub const MODEL_NUM: u8 = 5; // PM4

pub const UNITS_TYPE: u8 = 0; // Metric
pub const SERIALNUM_DIGITS: usize = 9;

pub const HMS_FORMAT_CNT: usize = 3;
pub const YMD_FORMAT_CNT: usize = 3;
pub const ERRORCODE_FORMAT_CNT: usize = 3;

// Command space partitioning for standard commands
pub const CTRL_CMD_LONG_MIN: u8 = 0x01;
pub const CFG_CMD_LONG_MIN: u8 = 0x10;
pub const DATA_CMD_LONG_MIN: u8 = 0x20;
pub const AUDIO_CMD_LONG_MIN: u8 = 0x40;
pub const TEXTCFG_CMD_LONG_MIN: u8 = 0x60;
pub const TEXTSTATUS_CMD_LONG_MIN: u8 = 0x65;
pub const CAP_CMD_LONG_MIN: u8 = 0x70;
pub const PMPROPRIETARY_CMD_LONG_MIN: u8 = 0x76;

pub const CTRL_CMD_SHORT_MIN: u8 = 0x80;
pub const STATUS_CMD_SHORT_MIN: u8 = 0x91;
pub const DATA_CMD_SHORT_MIN: u8 = 0xA0;
pub const AUDIO_CMD_SHORT_MIN: u8 = 0xC0;
pub const TEXTCFG_CMD_SHORT_MIN: u8 = 0xE0;
pub const TEXTSTATUS_CMD_SHORT_MIN: u8 = 0xE5;

// Standard Short Control Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortCtrlCmds {
    GetStatus = 0x80,
    Reset = 0x81,
    GoIdle = 0x82,
    GoHaveId = 0x83,
    GoInUse = 0x85,
    GoFinished = 0x86,
    GoReady = 0x87,
    BadId = 0x88,
}

// Standard Short Status Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortStatusCmds {
    GetVersion = 0x91,
    GetId = 0x92,
    GetUnits = 0x93,
    GetSerial = 0x94,
    GetList = 0x98,
    GetUtilization = 0x99,
    GetMotorCurrent = 0x9A,
    GetOdometer = 0x9B,
    GetErrorCode = 0x9C,
    GetServiceCode = 0x9D,
    GetUserCfg1 = 0x9E,
    GetUserCfg2 = 0x9F,
}

// Standard Short Data Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortDataCmds {
    GetTWork = 0xA0,
    GetHorizontal = 0xA1,
    GetVertical = 0xA2,
    GetCalories = 0xA3,
    GetProgram = 0xA4,
    GetSpeed = 0xA5,
    GetPace = 0xA6,
    GetCadence = 0xA7,
    GetGrade = 0xA8,
    GetGear = 0xA9,
    GetUpList = 0xAA,
    GetUserInfo = 0xAB,
    GetTorque = 0xAC,
    GetHrCur = 0xB0,
    GetHrTZone = 0xB2,
    GetMets = 0xB3,
    GetPower = 0xB4,
    GetHrAvg = 0xB5,
    GetHrMax = 0xB6,
    GetUserData1 = 0xBE,
    GetUserData2 = 0xBF,
}

// Standard Short Audio Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortAudioCmds {
    GetAudioChannel = 0xC0,
    GetAudioVolume = 0xC1,
    GetAudioMute = 0xC2,
}

// Standard Short Text Configuration Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortTextCfgCmds {
    EndText = 0xE0,
    DisplayPopup = 0xE1,
}

// Standard Short Text Status Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortTextStatusCmds {
    GetPopupStatus = 0xE5,
}

// Standard Long Control Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongCtrlCmds {
    AutoUpload = 0x01,
    UpList = 0x02,
    UpStatusSec = 0x04,
    UpListSec = 0x05,
}

// Standard Long Configuration Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongCfgCmds {
    IdDigits = 0x10,
    SetTime = 0x11,
    SetDate = 0x12,
    SetTimeout = 0x13,
    SetUserCfg1 = 0x1A,
    SetUserCfg2 = 0x1B,
}

// Standard Long Data Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongDataCmds {
    SetTWork = 0x20,
    SetHorizontal = 0x21,
    SetVertical = 0x22,
    SetCalories = 0x23,
    SetProgram = 0x24,
    SetSpeed = 0x25,
    SetGrade = 0x28,
    SetGear = 0x29,
    SetUserInfo = 0x2B,
    SetTorque = 0x2C,
    SetLevel = 0x2D,
    SetTargetHr = 0x30,
    SetGoal = 0x32,
    SetMets = 0x33,
    SetPower = 0x34,
    SetHrZone = 0x35,
    SetHrMax = 0x36,
}

// Standard Long Audio Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongAudioCmds {
    SetChannelRange = 0x40,
    SetVolumeRange = 0x41,
    SetAudioMute = 0x42,
    SetAudioChannel = 0x43,
    SetAudioVolume = 0x44,
}

// Standard Long Text Configuration Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongTextCfgCmds {
    StartText = 0x60,
    AppendText = 0x61,
}

// Standard Long Text Status Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongTextStatusCmds {
    GetTextStatus = 0x65,
}

// Standard Long Capabilities Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongCapCmds {
    GetCaps = 0x70,
    GetUserCaps1 = 0x7E,
    GetUserCaps2 = 0x7F,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProprietaryGetCmds {
    GetPmCfg = 0x7E,
    GetPmData = 0x7F,
}

// Standard Long PM Proprietary Commands
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongPmProprietaryCmds {
    SetPmCfg = 0x76,
    SetPmData = 0x77,
    GetPmCfg = 0x7E,
    GetPmData = 0x7F,
}

// Command space partitioning for PM proprietary commands
pub const GETPMCFG_CMD_SHORT_MIN: u8 = 0x80;
pub const GETPMCFG_CMD_LONG_MIN: u8 = 0x50;
pub const SETPMCFG_CMD_SHORT_MIN: u8 = 0xE0;
pub const SETPMCFG_CMD_LONG_MIN: u8 = 0x00;
pub const GETPMDATA_CMD_SHORT_MIN: u8 = 0xA0;
pub const GETPMDATA_CMD_LONG_MIN: u8 = 0x68;
pub const SETPMDATA_CMD_SHORT_MIN: u8 = 0xD0;
pub const SETPMDATA_CMD_LONG_MIN: u8 = 0x30;

// Custom Short PULL Configuration Commands for PM
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PmShortPullCfgCmds {
    GetFwVersion = 0x80,
    GetHwVersion = 0x81,
    GetHwAddress = 0x82,
    GetTickTimebase = 0x83,
    GetHrm = 0x84,
    GetScreenStateStatus = 0x86,
    GetRaceLaneRequest = 0x87,
    GetErgLogicalAddrRequest = 0x88,
    GetWorkoutType = 0x89,
    GetDisplayType = 0x8A,
    GetDisplayUnits = 0x8B,
    GetLanguageType = 0x8C,
    GetWorkoutState = 0x8D,
    GetIntervalType = 0x8E,
    GetOperationalState = 0x8F,
    GetLogCardState = 0x90,
    GetLogCardStatus = 0x91,
    GetPowerUpState = 0x92,
    GetRowingState = 0x93,
    GetScreenContentVersion = 0x94,
    GetCommunicationState = 0x95,
    GetRaceParticipantCount = 0x96,
    GetBatteryLevelPercent = 0x97,
    GetRaceModeStatus = 0x98,
    GetInternalLogParams = 0x99,
    GetProductConfiguration = 0x9A,
    GetErgSlaveDiscoverRequestStatus = 0x9B,
    GetWifiConfig = 0x9C,
    GetCpuTickRate = 0x9D,
    GetLogCardCensus = 0x9E,
    GetWorkoutIntervalCount = 0x9F,
}

// Custom Short PULL Data Commands for PM
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PmShortPullDataCmds {
    GetWorkTime = 0xA0,
    GetProjectedWorkTime = 0xA1,
    GetTotalRestTime = 0xA2,
    GetWorkDistance = 0xA3,
    GetTotalWorkDistance = 0xA4,
    GetProjectedWorkDistance = 0xA5,
    GetRestDistance = 0xA6,
    GetTotalRestDistance = 0xA7,
    GetStroke500mPace = 0xA8,
    GetStrokePower = 0xA9,
    GetStrokeCaloricBurnRate = 0xAA,
    GetSplitAvg500mPace = 0xAB,
    GetSplitAvgPower = 0xAC,
    GetSplitAvgCaloricBurnRate = 0xAD,
    GetSplitAvgCalories = 0xAE,
    GetTotalAvg500mPace = 0xAF,
    GetTotalAvgPower = 0xB0,
    GetTotalAvgCaloricBurnRate = 0xB1,
    GetTotalAvgCalories = 0xB2,
    GetStrokeRate = 0xB3,
    GetSplitAvgStrokeRate = 0xB4,
    GetTotalAvgStrokeRate = 0xB5,
    GetAvgHeartRate = 0xB6,
    GetEndingAvgHeartRate = 0xB7,
    GetRestAvgHeartRate = 0xB8,
    GetSplitTime = 0xB9,
    GetLastSplitTime = 0xBA,
    GetSplitDistance = 0xBB,
    GetLastSplitDistance = 0xBC,
    GetLastRestDistance = 0xBD,
    GetTargetPaceTime = 0xBE,
    GetStrokeState = 0xBF,
    GetStrokeRateState = 0xC0,
    GetDragFactor = 0xC1,
    GetEncoderPeriod = 0xC2,
    GetHeartRateState = 0xC3,
    GetSyncData = 0xC4,
    GetSyncDataAll = 0xC5,
    GetRaceData = 0xC6,
    GetTickTime = 0xC7,
    GetErrorType = 0xC8,
    GetErrorValue = 0xC9,
    GetStatusType = 0xCA,
    GetStatusValue = 0xCB,
    GetEpmStatus = 0xCC,
    GetDisplayUpdateTime = 0xCD,
    GetSyncFractionalTime = 0xCE,
    GetRestTime = 0xCF,
}

// Custom Short PUSH Data Commands for PM
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PmShortPushDataCmds {
    SetSyncDistance = 0xD0,
    SetSyncStrokePace = 0xD1,
    SetSyncAvgHeartRate = 0xD2,
    SetSyncTime = 0xD3,
    SetSyncSplitData = 0xD4,
    SetSyncEncoderPeriod = 0xD5,
    SetSyncVersionInfo = 0xD6,
    SetSyncRaceTickTime = 0xD7,
    SetSyncDataAll = 0xD8,
}

// Custom Short PUSH Configuration Commands for PM
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PmShortPushCfgCmds {
    SetResetAll = 0xE0,
    SetResetErgNumber = 0xE1,
}

// Custom Long PUSH Configuration Commands for PM
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PmLongPushCfgCmds {
    SetBaudRate = 0x00,
    SetWorkoutType = 0x01,
    SetStartType = 0x02,
    SetWorkoutDuration = 0x03,
    SetRestDuration = 0x04,
    SetSplitDuration = 0x05,
    SetTargetPaceTime = 0x06,
    SetIntervalIdentifier = 0x07,
    SetOperationalState = 0x08,
    SetRaceType = 0x09,
    SetWarmupDuration = 0x0A,
    SetRaceLaneSetup = 0x0B,
    SetRaceLaneVerify = 0x0C,
    SetRaceStartParams = 0x0D,
    SetErgSlaveDiscoveryRequest = 0x0E,
    SetBoatNumber = 0x0F,
    SetErgNumber = 0x10,
    SetCommunicationState = 0x11,
    SetCmdUpList = 0x12,
    SetScreenState = 0x13,
    ConfigureWorkout = 0x14,
    SetTargetAvgWatts = 0x15,
    SetTargetCalsPerHr = 0x16,
    SetIntervalType = 0x17,
    SetWorkoutIntervalCount = 0x18,
    SetDisplayUpdateRate = 0x19,
    SetAuthenPassword = 0x1A,
    SetTickTime = 0x1B,
    SetTickTimeOffset = 0x1C,
    SetRaceDataSampleTicks = 0x1D,
    SetRaceOperationType = 0x1E,
    SetRaceStatusDisplayTicks = 0x1F,
    SetRaceStatusWarningTicks = 0x20,
    SetRaceIdleModeParams = 0x21,
    SetDateTime = 0x22,
    SetLanguageType = 0x23,
    SetWifiConfig = 0x24,
    SetCpuTickRate = 0x25,
    SetLogCardUser = 0x26,
    SetScreenErrorMode = 0x27,
    SetCableTest = 0x28,
    SetUserId = 0x29,
    SetUserProfile = 0x2A,
    SetHrm = 0x2B,
    SetSensorChannel = 0x2F,
}

// Custom Long PUSH Data Commands for PM
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PmLongPushDataCmds {
    SetTeamDistance = 0x30,
    SetTeamFinishTime = 0x31,
    SetRaceParticipant = 0x32,
    SetRaceStatus = 0x33,
    SetLogCardMemory = 0x34,
    SetDisplayString = 0x35,
    SetDisplayBitmap = 0x36,
    SetLocalRaceParticipant = 0x37,
    SetAntRfMode = 0x4E,
    SetMemory = 0x4F,
}

// Custom Long PULL Configuration Commands for PM
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PmLongPullCfgCmds {
    GetErgNumber = 0x50,
    GetErgNumberRequest = 0x51,
    GetUserIdString = 0x52,
    GetLocalRaceParticipant = 0x53,
    GetUserId = 0x54,
    GetUserProfile = 0x55,
}

// Custom Long PULL Data Commands for PM
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PmLongPullDataCmds {
    GetMemory = 0x68,
    GetLogCardMemory = 0x69,
    GetInternalLogMemory = 0x6A,
    GetForcePlotData = 0x6B,
    GetHeartBeatData = 0x6C,
    GetUiEvents = 0x6D,
    GetStrokeStats = 0x6E,
    GetDiagLogRecordNum = 0x70,
    GetDiagLogRecord = 0x71,
}

// Status byte flag and mask definitions
pub const PREVOK_FLG: u8 = 0x00;
pub const PREVREJECT_FLG: u8 = 0x10;
pub const PREVBAD_FLG: u8 = 0x20;
pub const PREVNOTRDY_FLG: u8 = 0x30;
pub const PREVFRAMESTATUS_MSK: u8 = 0x30;

pub const SLAVESTATE_ERR_FLG: u8 = 0x00;
pub const SLAVESTATE_RDY_FLG: u8 = 0x01;
pub const SLAVESTATE_IDLE_FLG: u8 = 0x02;
pub const SLAVESTATE_HAVEID_FLG: u8 = 0x03;
pub const SLAVESTATE_INUSE_FLG: u8 = 0x05;
pub const SLAVESTATE_PAUSE_FLG: u8 = 0x06;
pub const SLAVESTATE_FINISH_FLG: u8 = 0x07;
pub const SLAVESTATE_MANUAL_FLG: u8 = 0x08;
pub const SLAVESTATE_OFFLINE_FLG: u8 = 0x09;

pub const FRAMECNT_FLG: u8 = 0x80;
pub const SLAVESTATE_MSK: u8 = 0x0F;

// AUTOUPLOAD_CMD flag definitions
pub const AUTOSTATUS_FLG: u8 = 0x01;
pub const UPSTATUS_FLG: u8 = 0x02;
pub const UPLIST_FLG: u8 = 0x04;
pub const ACK_FLG: u8 = 0x10;
pub const EXTERNCONTROL_FLG: u8 = 0x40;

// CSAFE Slave Capabilities Codes
pub const CAPCODE_PROTOCOL: u8 = 0x00;
pub const CAPCODE_POWER: u8 = 0x01;
pub const CAPCODE_TEXT: u8 = 0x02;

// CSAFE units format definitions
pub const DISTANCE_MILE_0_0: u8 = 0x01;
pub const DISTANCE_MILE_0_1: u8 = 0x02;
pub const DISTANCE_MILE_0_2: u8 = 0x03;
pub const DISTANCE_MILE_0_3: u8 = 0x04;
pub const DISTANCE_FEET_0_0: u8 = 0x05;
pub const DISTANCE_INCH_0_0: u8 = 0x06;
pub const WEIGHT_LBS_0_0: u8 = 0x07;
pub const WEIGHT_LBS_0_1: u8 = 0x08;
pub const DISTANCE_FEET_1_0: u8 = 0x0A;
pub const SPEED_MILEPERHOUR_0_0: u8 = 0x10;
pub const SPEED_MILEPERHOUR_0_1: u8 = 0x11;
pub const SPEED_MILEPERHOUR_0_2: u8 = 0x12;
pub const SPEED_FEETPERMINUTE_0_0: u8 = 0x13;
pub const DISTANCE_KM_0_0: u8 = 0x21;
pub const DISTANCE_KM_0_1: u8 = 0x22;
pub const DISTANCE_KM_0_2: u8 = 0x23;
pub const DISTANCE_METER_0_0: u8 = 0x24;
pub const DISTANCE_METER_0_1: u8 = 0x25;
pub const DISTANCE_CM_0_0: u8 = 0x26;
pub const WEIGHT_KG_0_0: u8 = 0x27;
pub const WEIGHT_KG_0_1: u8 = 0x28;
pub const SPEED_KMPERHOUR_0_0: u8 = 0x30;
pub const SPEED_KMPERHOUR_0_1: u8 = 0x31;
pub const SPEED_KMPERHOUR_0_2: u8 = 0x32;
pub const SPEED_METERPERMINUTE_0_0: u8 = 0x33;
pub const PACE_MINUTEPERMILE_0_0: u8 = 0x37;
pub const PACE_MINUTEPERKM_0_0: u8 = 0x38;
pub const PACE_SECONDSPERKM_0_0: u8 = 0x39;
pub const PACE_SECONDSPERMILE_0_0: u8 = 0x3A;
pub const DISTANCE_FLOORS_0_0: u8 = 0x41;
pub const DISTANCE_FLOORS_0_1: u8 = 0x42;
pub const DISTANCE_STEPS_0_0: u8 = 0x43;
pub const DISTANCE_REVS_0_0: u8 = 0x44;
pub const DISTANCE_STRIDES_0_0: u8 = 0x45;
pub const DISTANCE_STROKES_0_0: u8 = 0x46;
pub const MISC_BEATS_0_0: u8 = 0x47;
pub const ENERGY_CALORIES_0_0: u8 = 0x48;
pub const GRADE_PERCENT_0_0: u8 = 0x4A;
pub const GRADE_PERCENT_0_2: u8 = 0x4B;
pub const GRADE_PERCENT_0_1: u8 = 0x4C;
pub const CADENCE_FLOORSPERMINUTE_0_1: u8 = 0x4F;
pub const CADENCE_FLOORSPERMINUTE_0_0: u8 = 0x50;
pub const CADENCE_STEPSPERMINUTE_0_0: u8 = 0x51;
pub const CADENCE_REVSPERMINUTE_0_0: u8 = 0x52;
pub const CADENCE_STRIDESPERMINUTE_0_0: u8 = 0x53;
pub const CADENCE_STROKESPERMINUTE_0_0: u8 = 0x54;
pub const MISC_BEATSPERMINUTE_0_0: u8 = 0x55;
pub const BURN_CALORIESPERMINUTE_0_0: u8 = 0x56;
pub const BURN_CALORIESPERHOUR_0_0: u8 = 0x57;
pub const POWER_WATTS_0_0: u8 = 0x58;
pub const ENERGY_INCHLB_0_0: u8 = 0x5A;
pub const ENERGY_FOOTLB_0_0: u8 = 0x5B;
pub const ENERGY_NM_0_0: u8 = 0x5C;

// Conversion constants
pub const KG_TO_LBS: f64 = 2.2046;
pub const LBS_TO_KG: f64 = 1.0 / KG_TO_LBS;

// ID Digits
pub const IDDIGITS_MIN: u8 = 2;
pub const IDDIGITS_MAX: u8 = 5;
pub const DEFAULT_IDDIGITS: u8 = 5;
pub const DEFAULT_ID: u32 = 0;
pub const MANUAL_ID: u32 = 999999999;

// Slave State Timeout Parameters (seconds)
pub const DEFAULT_SLAVESTATE_TIMEOUT: u32 = 20;
pub const PAUSED_SLAVESTATE_TIMEOUT: u32 = 220;
pub const INUSE_SLAVESTATE_TIMEOUT: u32 = 6;
pub const IDLE_SLAVESTATE_TIMEOUT: u32 = 30;

// Base Year
pub const BASE_YEAR: u32 = 1900;

// Default time intervals (seconds)
pub const DEFAULT_STATUSUPDATE_INTERVAL: u32 = 256;
pub const DEFAULT_CMDUPLIST_INTERVAL: u32 = 256;
