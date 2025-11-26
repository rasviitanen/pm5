use futures::TryStreamExt;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use time::UtcDateTime;
use uuid::Uuid;

use crate::{
    services::{AdditionalStatusOne, AdditionalStrokeData, GeneralStatus, StrokeData},
    types::*,
};

// Type-safe column names
pub mod columns {
    pub const TIMESTAMP: &str = "timestamp";
    pub const TIMESTAMP_ADDITIONAL: &str = "timestamp_additional";
    pub const TIMESTAMP_STROKE: &str = "timestamp_stroke";
    pub const ELAPSED_TIME_MS: &str = "elapsed_time_ms";
    pub const DISTANCE_M: &str = "distance_m";
    pub const DISTANCE_M_STROKE: &str = "distance_m_stroke";
    pub const DRAG_FACTOR: &str = "drag_factor";
    pub const HEART_RATE_BPM: &str = "heart_rate_bpm";
    pub const STROKE_RATE: &str = "stroke_rate";
    pub const PACE_MS_PER_500M: &str = "pace_ms_per_500m";
    pub const DRIVE_LENGTH_CM: &str = "drive_length_cm";
    pub const DRIVE_TIME_MS: &str = "drive_time_ms";
    pub const PEAK_DRIVE_FORCE_N: &str = "peak_drive_force_n";
    pub const AVG_DRIVE_FORCE_N: &str = "avg_drive_force_n";
    pub const WORK_PER_STROKE_J: &str = "work_per_stroke_j";
    pub const POWER_WATTS: &str = "power_watts";
    pub const DURATION_MS: &str = "duration_ms";
    pub const POWER_ZONE: &str = "power_zone";
}

#[derive(Debug, Clone)]
pub struct WorkoutSample;

impl WorkoutSample {
    /// Convert general status samples to a Polars DataFrame
    pub fn general_to_dataframe(
        samples: &[(i128, Time, Distance, DragFactor)],
    ) -> PolarsResult<DataFrame> {
        let timestamps: Vec<i64> = samples.iter().map(|s| s.0 as i64).collect();
        let elapsed_times: Vec<u32> = samples.iter().map(|s| s.1 .0.as_u32()).collect();
        let distances: Vec<u32> = samples.iter().map(|s| s.2 .0.as_u32()).collect();
        let drag_factors: Vec<u32> = samples.iter().map(|s| s.3 .0 as u32).collect();

        DataFrame::new(vec![
            Series::new(columns::TIMESTAMP.into(), timestamps).into(),
            Series::new(columns::ELAPSED_TIME_MS.into(), elapsed_times).into(),
            Series::new(columns::DISTANCE_M.into(), distances).into(),
            Series::new(columns::DRAG_FACTOR.into(), drag_factors).into(),
        ])
    }

    /// Convert additional status samples to a Polars DataFrame
    pub fn additional_to_dataframe(
        samples: &[(i128, Time, HeartRate, StrokeRate, Pace)],
    ) -> PolarsResult<DataFrame> {
        let timestamps: Vec<i64> = samples.iter().map(|s| s.0 as i64).collect();
        let elapsed_times: Vec<u32> = samples.iter().map(|s| s.1 .0.as_u32()).collect();
        let heart_rates: Vec<u32> = samples.iter().map(|s| s.2 .0 as u32).collect();
        let stroke_rates: Vec<u32> = samples.iter().map(|s| s.3 .0 as u32).collect();
        let paces: Vec<u32> = samples.iter().map(|s| s.4 .0 as u32).collect();

        DataFrame::new(vec![
            Series::new(columns::TIMESTAMP_ADDITIONAL.into(), timestamps).into(),
            Series::new(columns::ELAPSED_TIME_MS.into(), elapsed_times).into(),
            Series::new(columns::HEART_RATE_BPM.into(), heart_rates).into(),
            Series::new(columns::STROKE_RATE.into(), stroke_rates).into(),
            Series::new(columns::PACE_MS_PER_500M.into(), paces).into(),
        ])
    }

    /// Convert stroke data samples to a Polars DataFrame
    pub fn stroke_to_dataframe(
        samples: &[(
            i128,
            Time,
            Distance,
            DriveLength,
            DriveTime,
            Force,
            Force,
            Work,
        )],
    ) -> PolarsResult<DataFrame> {
        let timestamps: Vec<i64> = samples.iter().map(|s| s.0 as i64).collect();
        let elapsed_times: Vec<u32> = samples.iter().map(|s| s.1 .0.as_u32()).collect();
        let distances: Vec<u32> = samples.iter().map(|s| s.2 .0.as_u32()).collect();
        let drive_lengths: Vec<u32> = samples.iter().map(|s| s.3 .0 as u32).collect();
        let drive_times: Vec<u32> = samples.iter().map(|s| s.4 .0 as u32).collect();
        let peak_forces: Vec<u32> = samples.iter().map(|s| s.5 .0 as u32).collect();
        let avg_forces: Vec<u32> = samples.iter().map(|s| s.6 .0 as u32).collect();
        let work_per_strokes: Vec<u32> = samples.iter().map(|s| s.7 .0 as u32).collect();

        DataFrame::new(vec![
            Series::new(columns::TIMESTAMP_STROKE.into(), timestamps).into(),
            Series::new(columns::ELAPSED_TIME_MS.into(), elapsed_times).into(),
            Series::new(columns::DISTANCE_M_STROKE.into(), distances).into(),
            Series::new(columns::DRIVE_LENGTH_CM.into(), drive_lengths).into(),
            Series::new(columns::DRIVE_TIME_MS.into(), drive_times).into(),
            Series::new(columns::PEAK_DRIVE_FORCE_N.into(), peak_forces).into(),
            Series::new(columns::AVG_DRIVE_FORCE_N.into(), avg_forces).into(),
            Series::new(columns::WORK_PER_STROKE_J.into(), work_per_strokes).into(),
        ])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutSummary {
    pub workout_id: Uuid,
    pub duration_ms: Time,
    pub total_distance_m: Distance,
    pub avg_heart_rate_bpm: Option<HeartRate>,
    pub max_heart_rate_bpm: Option<HeartRate>,
    pub avg_power_watts: Option<Power>,
    pub avg_stroke_rate: Option<StrokeRate>,
    pub avg_pace_ms_per_500m: Option<Pace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerZoneStats {
    pub zone_name: String,
    pub avg_power_watts: Option<f64>,
    pub avg_heart_rate_bpm: Option<f64>,
    pub time_in_zone_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutDetails {
    pub high_intensity_sample_count: usize,
    pub power_zone_distribution: Vec<PowerZoneStats>,
}

impl WorkoutDetails {
    /// Create from a power zone distribution DataFrame
    fn from_zone_df(zone_df: &DataFrame, high_intensity_count: usize) -> PolarsResult<Self> {
        let mut distribution = Vec::new();

        for i in 0..zone_df.height() {
            let row = zone_df.get(i).ok_or_else(|| {
                PolarsError::ComputeError(format!("Failed to get row {}", i).into())
            })?;

            // Extract string using pattern matching on AnyValue
            let zone_name = match &row[0] {
                polars::prelude::AnyValue::String(s) => s.to_string(),
                polars::prelude::AnyValue::StringOwned(s) => s.to_string(),
                _ => "Unknown".to_string(),
            };

            let avg_power = row[1].try_extract::<f64>().ok();
            let avg_hr = row[2].try_extract::<f64>().ok();
            let time_in_zone = row[3].try_extract::<f64>().unwrap_or(0.0) as u64;

            distribution.push(PowerZoneStats {
                zone_name,
                avg_power_watts: avg_power,
                avg_heart_rate_bpm: avg_hr,
                time_in_zone_ms: time_in_zone,
            });
        }

        Ok(WorkoutDetails {
            high_intensity_sample_count: high_intensity_count,
            power_zone_distribution: distribution,
        })
    }
}

#[derive(Clone)]
pub struct WorkoutRecorder {
    workout_id: Uuid,
    start_time: UtcDateTime,
    // Store separate DataFrames for each data source
    general_status_df: DataFrame,
    additional_status_df: DataFrame,
    stroke_data_df: DataFrame,
    additional_stroke_data_df: DataFrame,
}

impl WorkoutRecorder {
    pub fn new() -> Self {
        Self {
            workout_id: Uuid::now_v7(),
            start_time: UtcDateTime::now(),
            general_status_df: Default::default(),
            additional_status_df: Default::default(),
            stroke_data_df: Default::default(),
            additional_stroke_data_df: Default::default(),
        }
    }

    pub fn workout_id(&self) -> Uuid {
        self.workout_id
    }

    pub fn add_general_status(
        &mut self,
        GeneralStatus {
            elapsed_time,
            distance,
            workout_type,
            interval_type,
            workout_state,
            rowing_state,
            stroke_state,
            total_work_distance,
            workout_duration,
            workout_duration_type,
            drag_factor,
        }: GeneralStatus,
    ) {
        let timestamp = UtcDateTime::now().unix_timestamp_nanos();

        if let Ok(new_df) =
            WorkoutSample::general_to_dataframe(&[(timestamp, elapsed_time, distance, drag_factor)])
        {
            if self.general_status_df.is_empty() {
                self.general_status_df = new_df;
            } else {
                let _ = self.general_status_df.vstack_mut(&new_df);
            }
        }
    }

    pub fn add_additional_status_one(
        &mut self,
        AdditionalStatusOne {
            elapsed_time,
            speed,
            stroke_rate,
            heart_rate,
            current_pace,
            average_pace,
            rest_distance,
            rest_time,
            machine_type,
        }: AdditionalStatusOne,
    ) {
        let timestamp = UtcDateTime::now().unix_timestamp_nanos();

        if let Ok(new_df) = WorkoutSample::additional_to_dataframe(&[(
            timestamp,
            elapsed_time,
            heart_rate,
            stroke_rate,
            current_pace,
        )]) {
            if self.additional_status_df.is_empty() {
                self.additional_status_df = new_df;
            } else {
                let _ = self.additional_status_df.vstack_mut(&new_df);
            }
        }
    }

    pub fn add_stroke_data(
        &mut self,
        StrokeData {
            elapsed_time,
            distance,
            drive_length,
            drive_time,
            stroke_recovery,
            stroke_distance,
            peak_drive_force,
            avg_drive_force,
            work_per_stroke,
            stroke_count,
        }: StrokeData,
    ) {
        let timestamp = UtcDateTime::now().unix_timestamp_nanos();

        if let Ok(new_df) = WorkoutSample::stroke_to_dataframe(&[(
            timestamp,
            elapsed_time,
            distance,
            drive_length,
            drive_time,
            peak_drive_force,
            avg_drive_force,
            work_per_stroke,
        )]) {
            if self.stroke_data_df.is_empty() {
                self.stroke_data_df = new_df;
            } else {
                let _ = self.stroke_data_df.vstack_mut(&new_df);
            }
        }
    }

    pub fn add_additional_stroke_data(
        &mut self,
        AdditionalStrokeData {
            elapsed_time,
            stroke_power,
            stroke_calories,
            stroke_count,
            projected_work_time,
            projected_work_distance,
        }: AdditionalStrokeData,
    ) {
        let timestamp = UtcDateTime::now().unix_timestamp_nanos();

        let new_df = DataFrame::new(vec![
            Column::new(columns::TIMESTAMP_STROKE.into(), vec![timestamp as i64]),
            Column::new(
                columns::ELAPSED_TIME_MS.into(),
                vec![elapsed_time.0.as_u32()],
            ),
            Column::new(columns::POWER_WATTS.into(), vec![stroke_power.0 as u32]),
        ])
        .unwrap();

        if self.additional_stroke_data_df.is_empty() {
            self.additional_stroke_data_df = new_df;
        } else {
            let _ = self.additional_stroke_data_df.vstack_mut(&new_df);
        }
    }

    /// Get merged DataFrame joining all sources by elapsed_time_ms
    pub fn dataframe(&self) -> PolarsResult<DataFrame> {
        let lf = self.lazy();
        lf.collect()
    }

    /// Get lazy representation with all data sources merged by elapsed_time_ms
    pub fn lazy(&self) -> LazyFrame {
        let general_lf = self.general_status_df.clone().lazy();
        let additional_lf = self.additional_status_df.clone().lazy();
        let stroke_lf = self.stroke_data_df.clone().lazy();
        let additional_stroke_lf = self.additional_stroke_data_df.clone().lazy();

        // Start with general status (has timestamp, elapsed_time, distance, drag_factor)
        let mut merged = general_lf;

        // Join additional status if it has data
        if !self.additional_status_df.is_empty() {
            merged = merged.join(
                additional_lf,
                [col(columns::ELAPSED_TIME_MS)],
                [col(columns::ELAPSED_TIME_MS)],
                JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
            );
        }

        // Join stroke data if it has data
        if !self.stroke_data_df.is_empty() {
            merged = merged.join(
                stroke_lf,
                [col(columns::ELAPSED_TIME_MS)],
                [col(columns::ELAPSED_TIME_MS)],
                JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
            );
        }

        // Join additional stroke data if it has data
        if !self.additional_stroke_data_df.is_empty() {
            merged = merged.join(
                additional_stroke_lf,
                [col(columns::ELAPSED_TIME_MS)],
                [col(columns::ELAPSED_TIME_MS)],
                JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
            );
        }

        // Sort by elapsed_time and deduplicate
        merged
            .sort([columns::ELAPSED_TIME_MS], Default::default())
            .unique(
                Some(Selector::ByName {
                    names: Arc::new([columns::ELAPSED_TIME_MS.into()]),
                    strict: false,
                }),
                UniqueKeepStrategy::Last,
            )
    }

    /// Get individual DataFrames for inspection
    pub fn general_status_dataframe(&self) -> DataFrame {
        self.general_status_df.clone()
    }

    pub fn additional_status_dataframe(&self) -> DataFrame {
        self.additional_status_df.clone()
    }

    pub fn stroke_data_dataframe(&self) -> DataFrame {
        self.stroke_data_df.clone()
    }

    pub fn additional_stroke_data_dataframe(&self) -> DataFrame {
        self.additional_stroke_data_df.clone()
    }
}

pub struct WorkoutStorage {
    operator: opendal::Operator,
}

impl WorkoutStorage {
    pub async fn new_disk(root: &str) -> anyhow::Result<Self> {
        let builder = opendal::services::Fs::default().root(&format!("./{root}"));
        let operator = opendal::Operator::new(builder)?.finish();
        Ok(Self { operator })
    }

    pub async fn new_mem(root: &str) -> anyhow::Result<Self> {
        let builder = opendal::services::Memory::default().root(&format!("./{root}"));
        let operator = opendal::Operator::new(builder)?.finish();
        Ok(Self { operator })
    }

    pub async fn save_workout(
        &self,
        namespace: &str,
        recorder: &WorkoutRecorder,
    ) -> anyhow::Result<Workout> {
        let df = recorder.dataframe()?;

        let path = format!("workouts/{}/{}.parquet", namespace, recorder.workout_id);

        let mut buffer = Vec::new();
        ParquetWriter::new(&mut buffer)
            .with_compression(ParquetCompression::Snappy)
            .finish(&mut df.clone())?;

        self.operator.write(&path, buffer).await?;

        Ok(Workout {
            id: recorder.workout_id,
            df,
        })
    }

    pub async fn load_workout_lazy(
        &self,
        namespace: &str,
        workout_id: uuid::Uuid,
    ) -> anyhow::Result<Workout> {
        let path = format!("workouts/{}/{}.parquet", namespace, workout_id);
        let data = self.operator.read(&path).await?;
        let bytes: Vec<_> = data.into_iter().flat_map(|bytes| bytes.to_vec()).collect();

        let df = ParquetReader::new(std::io::Cursor::new(bytes)).finish()?;
        Ok(Workout { id: workout_id, df })
    }

    pub async fn scan_workouts(&self, namespace: &str) -> anyhow::Result<Vec<String>> {
        let prefix = format!("workouts/{}/", namespace);
        let mut paths = Vec::new();

        let mut lister = self.operator.lister(&prefix).await?;
        while let Some(entry) = lister.try_next().await? {
            if entry.path().ends_with(".parquet") {
                paths.push(entry.path().to_owned());
            }
        }

        Ok(paths)
    }
}

pub struct Workout {
    id: Uuid,
    df: DataFrame,
}

impl Workout {
    pub fn generate_summary(&self) -> PolarsResult<WorkoutSummary> {
        let lf = self.df.clone().lazy();
        tokio::task::block_in_place(|| {
            let agg_df = lf
                .select([
                    col(columns::HEART_RATE_BPM).mean().alias("avg_hr"),
                    col(columns::HEART_RATE_BPM).max().alias("max_hr"),
                    col(columns::POWER_WATTS).mean().alias("avg_power"),
                    col(columns::STROKE_RATE).mean().alias("avg_stroke_rate"),
                    col(columns::ELAPSED_TIME_MS).max().alias("total_time"),
                    col(columns::DISTANCE_M).max().alias("total_distance"),
                    col(columns::PACE_MS_PER_500M).mean().alias("avg_pace"),
                ])
                .collect()?;

            let row = agg_df
                .get(0)
                .ok_or_else(|| PolarsError::ComputeError("No data to aggregate".into()))?;

            let avg_hr = row[0].try_extract::<f64>().ok().map(|v| HeartRate(v as u8));
            let max_hr = row[1].try_extract::<f64>().ok().map(|v| HeartRate(v as u8));
            let avg_power = row[2].try_extract::<f64>().ok().map(|v| Power(v as u16));
            let avg_stroke_rate = row[3]
                .try_extract::<f64>()
                .ok()
                .map(|v| StrokeRate(v as u8));
            let total_time = row[4]
                .try_extract::<f64>()
                .ok()
                .map(|v| Time(U24::new(v as _)));
            let total_distance = row[5]
                .try_extract::<f64>()
                .ok()
                .map(|v| Distance(U24::new(v as _)));
            let avg_pace = row[6].try_extract::<f64>().ok().map(|v| Pace(v as _));

            Ok(WorkoutSummary {
                workout_id: self.id,
                duration_ms: total_time.unwrap_or_default(),
                total_distance_m: total_distance.unwrap_or_default(),
                avg_heart_rate_bpm: avg_hr,
                max_heart_rate_bpm: max_hr,
                avg_power_watts: avg_power,
                avg_stroke_rate,
                avg_pace_ms_per_500m: avg_pace,
            })
        })
    }

    pub fn generate_details(&self, profile: &Profile) -> PolarsResult<WorkoutDetails> {
        tokio::task::block_in_place(|| {
            let lf = self.df.clone().lazy();
            let with_zones = WorkoutAnalytics::power_zones(lf.clone(), profile);
            let with_zones = WorkoutAnalytics::delta_time(with_zones);

            let high_intensity = with_zones
                .clone()
                .filter(col(columns::POWER_WATTS).gt(lit(250)))
                .collect()?;

            let high_intensity_count = high_intensity.height();

            let zone_summary = with_zones
                .group_by([col(columns::POWER_ZONE)])
                .agg([
                    col(columns::POWER_WATTS).mean().alias("avg_power"),
                    col(columns::HEART_RATE_BPM).mean().alias("avg_hr"),
                    col(columns::DURATION_MS).sum().alias("time_in_zone_ms"),
                ])
                .sort(
                    ["time_in_zone_ms"],
                    SortMultipleOptions::default().with_order_descending(true),
                )
                .collect()?;

            WorkoutDetails::from_zone_df(&zone_summary, high_intensity_count)
        })
    }
}

/// Analytics utilities using LazyFrame
pub struct WorkoutAnalytics;

impl WorkoutAnalytics {
    pub fn delta_time(lf: LazyFrame) -> LazyFrame {
        lf.with_column(
            (col(columns::ELAPSED_TIME_MS) - col(columns::ELAPSED_TIME_MS).shift(lit(1)))
                .alias(columns::DURATION_MS),
        )
        .with_column(
            when(col(columns::DURATION_MS).is_null())
                .then(lit(0))
                .otherwise(col(columns::DURATION_MS))
                .alias(columns::DURATION_MS),
        )
    }

    pub fn power_zones(lf: LazyFrame, profile: &Profile) -> LazyFrame {
        lf.with_column(
            when(col(columns::POWER_WATTS).gt_eq(lit(profile.zones.z5.0)))
                .then(lit("Zone 5: VO2 Max"))
                .when(col(columns::POWER_WATTS).gt_eq(lit(profile.zones.z4.0)))
                .then(lit("Zone 4: Threshold"))
                .when(col(columns::POWER_WATTS).gt_eq(lit(profile.zones.z3.0)))
                .then(lit("Zone 3: Tempo"))
                .when(col(columns::POWER_WATTS).gt_eq(lit(profile.zones.z2.0)))
                .then(lit("Zone 2: Endurance"))
                .otherwise(lit("Zone 1: Recovery"))
                .alias(columns::POWER_ZONE),
        )
    }
}

#[derive(Debug)]
pub struct PowerZones {
    pub z2: Power,
    pub z3: Power,
    pub z4: Power,
    pub z5: Power,
}

#[derive(Debug)]
pub struct Profile {
    pub zones: PowerZones,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            zones: PowerZones {
                z2: Power(110),
                z3: Power(120),
                z4: Power(130),
                z5: Power(140),
            },
        }
    }
}
