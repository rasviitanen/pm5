use futures::TryStreamExt;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use time::UtcDateTime;
use uuid::Uuid;

use crate::{
    services::{AdditionalStatusOne, AdditionalStrokeData, GeneralStatus, StrokeData},
    types::*,
};

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
            Series::new("timestamp".into(), timestamps).into(),
            Series::new("elapsed_time_ms".into(), elapsed_times).into(),
            Series::new("distance_m".into(), distances).into(),
            Series::new("drag_factor".into(), drag_factors).into(),
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
            Series::new("timestamp_additional".into(), timestamps).into(),
            Series::new("elapsed_time_ms".into(), elapsed_times).into(),
            Series::new("heart_rate_bpm".into(), heart_rates).into(),
            Series::new("stroke_rate".into(), stroke_rates).into(),
            Series::new("pace_ms_per_500m".into(), paces).into(),
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
            Series::new("timestamp_stroke".into(), timestamps).into(),
            Series::new("elapsed_time_ms".into(), elapsed_times).into(),
            Series::new("distance_m_stroke".into(), distances).into(),
            Series::new("drive_length_cm".into(), drive_lengths).into(),
            Series::new("drive_time_ms".into(), drive_times).into(),
            Series::new("peak_drive_force_n".into(), peak_forces).into(),
            Series::new("avg_drive_force_n".into(), avg_forces).into(),
            Series::new("work_per_stroke_j".into(), work_per_strokes).into(),
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
            Column::new("timestamp_stroke".into(), vec![timestamp as i64]),
            Column::new("elapsed_time_ms".into(), vec![elapsed_time.0.as_u32()]),
            Column::new("power_watts".into(), vec![stroke_power.0 as u32]),
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
                [col("elapsed_time_ms")],
                [col("elapsed_time_ms")],
                JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
            );
        }

        // Join stroke data if it has data
        if !self.stroke_data_df.is_empty() {
            merged = merged.join(
                stroke_lf,
                [col("elapsed_time_ms")],
                [col("elapsed_time_ms")],
                JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
            );
        }

        // Join stroke data if it has data
        if !self.additional_stroke_data_df.is_empty() {
            merged = merged.join(
                additional_stroke_lf,
                [col("elapsed_time_ms")],
                [col("elapsed_time_ms")],
                JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
            );
        }

        // Sort by elapsed_time and deduplicate
        merged.sort(["elapsed_time_ms"], Default::default()).unique(
            Some(Selector::ByName {
                names: Arc::new(["elapsed_time_ms".into()]),
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
                    col("heart_rate_bpm").mean().alias("avg_hr"),
                    col("heart_rate_bpm").max().alias("max_hr"),
                    col("power_watts").mean().alias("avg_power"),
                    col("stroke_rate").mean().alias("avg_stroke_rate"),
                    col("elapsed_time_ms").max().alias("total_time"),
                    col("distance_m").max().alias("total_distance"),
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

            Ok(WorkoutSummary {
                workout_id: self.id,
                duration_ms: total_time.unwrap_or_default(),
                total_distance_m: total_distance.unwrap_or_default(),
                avg_heart_rate_bpm: avg_hr,
                max_heart_rate_bpm: max_hr,
                avg_power_watts: avg_power,
                avg_stroke_rate,
                avg_pace_ms_per_500m: None,
            })
        })
    }

    pub fn generate_details(&self) -> anyhow::Result<()> {
        tokio::task::block_in_place(|| {
            let lf = self.df.clone().lazy();
            let with_zones = WorkoutAnalytics::power_zones(lf.clone());
            let with_zones = WorkoutAnalytics::delta_time(with_zones);
            let high_intensity = with_zones
                .clone()
                .filter(col("power_watts").gt(lit(250)))
                .collect()?;

            println!("High intensity samples: {}", high_intensity.height());
            let zone_summary = with_zones
                .group_by([col("power_zone")])
                .agg([
                    col("power_watts").mean().alias("avg_power"),
                    col("heart_rate_bpm").mean().alias("avg_hr"),
                    col("duration_ms").sum().alias("time_in_zone_ms"),
                ])
                .sort(
                    ["time_in_zone_ms"],
                    SortMultipleOptions::default().with_order_descending(true),
                )
                .collect()?;

            println!("\nPower Zone Distribution:");
            println!("{}", zone_summary);
            Ok::<_, anyhow::Error>(())
        })
    }
}

/// Analytics utilities using LazyFrame
pub struct WorkoutAnalytics;

impl WorkoutAnalytics {
    pub fn delta_time(lf: LazyFrame) -> LazyFrame {
        lf.with_column(
            (col("elapsed_time_ms") - col("elapsed_time_ms").shift(lit(1))).alias("duration_ms"),
        )
        .with_column(
            when(col("duration_ms").is_null())
                .then(lit(0))
                .otherwise(col("duration_ms"))
                .alias("duration_ms"),
        )
    }

    pub fn power_zones(lf: LazyFrame) -> LazyFrame {
        lf.with_column(
            when(col("power_watts").lt(lit(150)))
                .then(lit("Zone 1: Recovery"))
                .when(col("power_watts").lt(lit(200)))
                .then(lit("Zone 2: Endurance"))
                .when(col("power_watts").lt(lit(250)))
                .then(lit("Zone 3: Tempo"))
                .when(col("power_watts").lt(lit(300)))
                .then(lit("Zone 4: Threshold"))
                .otherwise(lit("Zone 5: VO2 Max"))
                .alias("power_zone"),
        )
    }
}
