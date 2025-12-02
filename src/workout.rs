use futures::TryStreamExt;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::UtcDateTime;
use uuid::Uuid;

use crate::{
    services::{AdditionalStatusOne, AdditionalStrokeData, GeneralStatus, StrokeData},
    types::*,
};

pub mod columns {
    pub const ELAPSED_TIME: &str = "elapsed_time";
    pub const DISTANCE: &str = "distance";
    pub const WORKOUT_TYPE: &str = "workout_type";
    pub const INTERVAL_TYPE: &str = "interval_type";
    pub const WORKOUT_STATE: &str = "workout_state";
    pub const ROWING_STATE: &str = "rowing_state";
    pub const STROKE_STATE: &str = "stroke_state";
    pub const TOTAL_WORK_DISTANCE: &str = "total_work_distance";
    pub const WORKOUT_DURATION: &str = "workout_duration";
    pub const WORKOUT_DURATION_TYPE: &str = "workout_duration_type";
    pub const DRAG_FACTOR: &str = "drag_factor";

    pub const STROKE_POWER: &str = "stroke_power";
    pub const STROKE_CALORIES: &str = "stroke_calories";
    pub const STROKE_COUNT: &str = "stroke_count";
    pub const PROJECTED_WORK_TIME: &str = "projected_work_time";
    pub const PROJECTED_WORK_DISTANCE: &str = "projected_work_distance";

    pub const DRIVE_LENGTH: &str = "drive_length";
    pub const DRIVE_TIME: &str = "drive_time";
    pub const STROKE_RECOVERY: &str = "stroke_recovery";
    pub const STROKE_DISTANCE: &str = "stroke_distance";
    pub const PEAK_DRIVE_FORCE: &str = "peak_drive_force";
    pub const AVG_DRIVE_FORCE: &str = "avg_drive_force";
    pub const WORK_PER_STROKE: &str = "work_per_stroke";

    pub const SPEED: &str = "speed";
    pub const STROKE_RATE: &str = "stroke_rate";
    pub const HEART_RATE: &str = "heart_rate";
    pub const CURRENT_PACE: &str = "current_pace";
    pub const AVERAGE_PACE: &str = "average_pace";
    pub const REST_DISTANCE: &str = "rest_distance";
    pub const REST_TIME: &str = "rest_time";

    pub const DURATION: &str = "duration";
    pub const POWER_ZONE: &str = "power_zone";

}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutSummary {
    pub workout_id: Uuid,
    pub duration: Time,
    pub total_distance: Distance,
    pub avg_heart_rate_bpm: Option<HeartRate>,
    pub max_heart_rate_bpm: Option<HeartRate>,
    pub avg_power_watts: Option<Power>,
    pub avg_stroke_rate: Option<StrokeRate>,
    pub avg_pace_per_500m: Option<Pace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerZoneStats {
    pub zone_name: String,
    pub avg_power_watts: Option<f64>,
    pub avg_heart_rate_bpm: Option<f64>,
    pub time_in_zone: u64,
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
                time_in_zone,
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
    general_status_df: DataFrame,
    additional_status_df: DataFrame,
    stroke_data_df: DataFrame,
    additional_stroke_data_df: DataFrame,
    env_forces_df: DataFrame,
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
            env_forces_df: Default::default(),
        }
    }

    pub fn workout_id(&self) -> Uuid {
        self.workout_id
    }

    pub fn add_environmental_forces(
        &mut self,
        environment::Forces {
            elapsed_time,
            slope_percent,
            wind_resistance,
        }: environment::Forces,
    ) {
        let new_df = DataFrame::new(vec![
            Column::new(columns::ELAPSED_TIME.into(), vec![elapsed_time.as_u32()]),
            Column::new(environment::columns::SLOPE_PERCENT.into(), vec![slope_percent]),
            Column::new(environment::columns::WIND_RESISTANCE.into(), vec![wind_resistance]),
        ])
        .unwrap();

        if self.env_forces_df.is_empty() {
            self.env_forces_df = new_df;
        } else {
            let _ = self.env_forces_df.vstack_mut(&new_df);
        }
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
        let new_df = DataFrame::new(vec![
            Column::new(columns::ELAPSED_TIME.into(), vec![elapsed_time.as_u32()]),
            Column::new(columns::DISTANCE.into(), vec![distance.as_u32()]),
            Column::new(columns::WORKOUT_TYPE.into(), vec![workout_type as u8]),
            Column::new(columns::INTERVAL_TYPE.into(), vec![interval_type as u8]),
            Column::new(columns::WORKOUT_STATE.into(), vec![workout_state as u8]),
            Column::new(columns::ROWING_STATE.into(), vec![rowing_state as u8]),
            Column::new(columns::STROKE_STATE.into(), vec![stroke_state as u8]),
            Column::new(columns::TOTAL_WORK_DISTANCE.into(), vec![total_work_distance.0.as_u32()]),
            Column::new(columns::WORKOUT_DURATION.into(), vec![workout_duration.0.as_u32()]),
            Column::new(columns::WORKOUT_DURATION_TYPE.into(), vec![workout_duration_type as u8]),
            Column::new(columns::DRAG_FACTOR.into(), vec![drag_factor.0]),
        ])
        .unwrap();

        if self.general_status_df.is_empty() {
            self.general_status_df = new_df;
        } else {
            let _ = self.general_status_df.vstack_mut(&new_df);
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
            machine_type: _,
        }: AdditionalStatusOne,
    ) {
        let new_df = DataFrame::new(vec![
            Column::new(columns::ELAPSED_TIME.into(), vec![elapsed_time.as_u32()]),
            Column::new(columns::SPEED.into(), vec![speed.0]),
            Column::new(columns::STROKE_RATE.into(), vec![stroke_rate.0]),
            Column::new(columns::HEART_RATE.into(), vec![heart_rate.0]),
            Column::new(columns::CURRENT_PACE.into(), vec![current_pace.0]),
            Column::new(columns::AVERAGE_PACE.into(), vec![average_pace.0]),
            Column::new(columns::REST_DISTANCE.into(), vec![rest_distance.0]),
            Column::new(columns::REST_TIME.into(), vec![rest_time.as_u32()]),
        ])
        .unwrap();

        if self.additional_status_df.is_empty() {
            self.additional_status_df = new_df;
        } else {
            let _ = self.additional_status_df.vstack_mut(&new_df);
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
        let new_df = DataFrame::new(vec![
            Column::new(columns::ELAPSED_TIME.into(), vec![elapsed_time.as_u32()]),
            Column::new(columns::DISTANCE.into(), vec![distance.as_u32()]),
            Column::new(columns::DRIVE_LENGTH.into(), vec![drive_length.0]),
            Column::new(columns::DRIVE_TIME.into(), vec![drive_time.0]),
            Column::new(columns::STROKE_RECOVERY.into(), vec![stroke_recovery.0]),
            Column::new(columns::STROKE_DISTANCE.into(), vec![stroke_distance.0]),
            Column::new(columns::PEAK_DRIVE_FORCE.into(), vec![peak_drive_force.0]),
            Column::new(columns::AVG_DRIVE_FORCE.into(), vec![avg_drive_force.0]),
            Column::new(columns::WORK_PER_STROKE.into(), vec![work_per_stroke.0]),
            Column::new(columns::STROKE_COUNT.into(), vec![stroke_count.0]),
        ])
        .unwrap();

        if self.stroke_data_df.is_empty() {
            self.stroke_data_df = new_df;
        } else {
            let _ = self.stroke_data_df.vstack_mut(&new_df);
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
        let new_df = DataFrame::new(vec![
            Column::new(columns::ELAPSED_TIME.into(), vec![elapsed_time.as_u32()]),
            Column::new(columns::STROKE_POWER.into(), vec![stroke_power.0]),
            Column::new(columns::STROKE_CALORIES.into(), vec![stroke_calories.0]),
            Column::new(columns::STROKE_COUNT.into(), vec![stroke_count.0]),
            Column::new(columns::PROJECTED_WORK_TIME.into(), vec![projected_work_time.as_u32()]),
            Column::new(columns::PROJECTED_WORK_DISTANCE.into(), vec![projected_work_distance.as_u32()]),
        ])
        .unwrap();

        if self.additional_stroke_data_df.is_empty() {
            self.additional_stroke_data_df = new_df;
        } else {
            let _ = self.additional_stroke_data_df.vstack_mut(&new_df);
        }
    }

    pub fn dataframe(&self) -> PolarsResult<DataFrame> {
        let lf = self.lazy();
        lf.collect()
    }

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
                [col(columns::ELAPSED_TIME)],
                [col(columns::ELAPSED_TIME)],
                JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
            );
        }

        // Join stroke data if it has data
        if !self.stroke_data_df.is_empty() {
            merged = merged.join(
                stroke_lf,
                [col(columns::ELAPSED_TIME)],
                [col(columns::ELAPSED_TIME)],
                JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
            );
        }

        // Join additional stroke data if it has data
        if !self.additional_stroke_data_df.is_empty() {
            merged = merged.join(
                additional_stroke_lf,
                [col(columns::ELAPSED_TIME)],
                [col(columns::ELAPSED_TIME)],
                JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
            );
        }

        if !self.env_forces_df.is_empty() {
            let env_lf = self.env_forces_df.clone().lazy();
            merged = merged.join(
                env_lf,
                [col(columns::ELAPSED_TIME)],
                [col(columns::ELAPSED_TIME)],
                JoinArgs::new(JoinType::Left).with_coalesce(JoinCoalesce::CoalesceColumns),
            );
        }

        // Sort by elapsed_time and deduplicate
        merged
            .sort([columns::ELAPSED_TIME], Default::default())
            .unique(
                Some(Selector::ByName {
                    names: Arc::new([columns::ELAPSED_TIME.into()]),
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
                    col(columns::HEART_RATE).mean().alias("avg_hr"),
                    col(columns::HEART_RATE).max().alias("max_hr"),
                    col(columns::STROKE_POWER).mean().alias("avg_power"),
                    col(columns::STROKE_RATE).mean().alias("avg_stroke_rate"),
                    col(columns::ELAPSED_TIME).max().alias("total_time"),
                    col(columns::DISTANCE).max().alias("total_distance"),
                    col(columns::CURRENT_PACE).mean().alias("avg_pace"),
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
                duration: total_time.unwrap_or_default(),
                total_distance: total_distance.unwrap_or_default(),
                avg_heart_rate_bpm: avg_hr,
                max_heart_rate_bpm: max_hr,
                avg_power_watts: avg_power,
                avg_stroke_rate,
                avg_pace_per_500m: avg_pace,
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
                .filter(col(columns::STROKE_POWER).gt(lit(250)))
                .collect()?;

            let high_intensity_count = high_intensity.height();

            let zone_summary = with_zones
                .group_by([columns::POWER_ZONE])
                .agg([
                    col(columns::STROKE_POWER).mean().alias("avg_power"),
                    col(columns::HEART_RATE).mean().alias("avg_hr"),
                    col(columns::DURATION).sum().alias("time_in_zone_ms"),
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

pub struct WorkoutAnalytics;

impl WorkoutAnalytics {
    pub fn delta_time(lf: LazyFrame) -> LazyFrame {
        lf.with_column(
            (col(columns::ELAPSED_TIME) - col(columns::ELAPSED_TIME).shift(lit(1)))
                .alias(columns::DURATION),
        )
        .with_column(
            when(col(columns::DURATION).is_null())
                .then(lit(0))
                .otherwise(col(columns::DURATION))
                .alias(columns::DURATION),
        )
    }

    pub fn power_zones(lf: LazyFrame, profile: &Profile) -> LazyFrame {
        lf.with_column(
            when(col(columns::STROKE_POWER).gt_eq(lit(profile.zones.z5.0)))
                .then(lit("Zone 5: VO2 Max"))
                .when(col(columns::STROKE_POWER).gt_eq(lit(profile.zones.z4.0)))
                .then(lit("Zone 4: Threshold"))
                .when(col(columns::STROKE_POWER).gt_eq(lit(profile.zones.z3.0)))
                .then(lit("Zone 3: Tempo"))
                .when(col(columns::STROKE_POWER).gt_eq(lit(profile.zones.z2.0)))
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

mod environment {
    use super::*;

    pub mod columns {
        pub const SLOPE_PERCENT: &str = "slope_percent";
        pub const WIND_RESISTANCE: &str = "wind_resistance";
    }


    /// Environmental forces data structure
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Forces {
        pub elapsed_time: Time,
        pub slope_percent: f64,
        pub wind_resistance: f64,
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;


    async fn record_workout_example(profile: &Profile) -> anyhow::Result<Workout> {
        let mut recorder = WorkoutRecorder::new();
        for i in 0..=3600 {
            let elapsed = Time::from_secs(i as f32);
            let distance = Distance::from_meters((4 * i) as _);
            let hr = 120 + (i % 20) as u8;
            let stroke_rate = 45 + (i % 5) as u8;
            let pace = Pace::from_secs(125.0); // Getting slightly slower

            recorder.add_stroke_data(StrokeData {
                elapsed_time: elapsed,
                distance: distance,
                drive_length: DriveLength(5),
                drive_time: DriveTime(2),
                stroke_recovery: StrokeRecoveryTime(5),
                stroke_distance: StrokeDistance(5),
                peak_drive_force: Force(260),
                avg_drive_force: Force(120),
                work_per_stroke: Work(2),
                stroke_count: StrokeCount(i as u16 / 3),
            });

            recorder.add_environmental_forces(environment::Forces {
                elapsed_time: elapsed,
                slope_percent: 2.0,
                wind_resistance: 0.0
            });

            recorder.add_additional_stroke_data(AdditionalStrokeData {
                elapsed_time: elapsed,
                stroke_power: {
                    if i % 3 == 0 {
                        profile.zones.z5
                    } else if i % 10 == 0 {
                        profile.zones.z4
                    } else {
                        profile.zones.z2
                    }
                },
                stroke_calories: Calories(20),
                stroke_count: StrokeCount(80),
                projected_work_time: Time(U24::new(10)),
                projected_work_distance: Distance(U24::new(30)),
            });

            recorder.add_general_status(GeneralStatus {
                elapsed_time: elapsed,
                distance: distance,
                workout_type: WorkoutType::JustrowSplits,
                interval_type: IntervalType::Time,
                workout_state: WorkoutState::WorkoutRow,
                rowing_state: RowingState::Active,
                stroke_state: StrokeState::DrivingState,
                total_work_distance: distance,
                workout_duration: elapsed,
                workout_duration_type: WorkoutDurationType::Time,
                drag_factor: DragFactor(10),
            });

            recorder.add_additional_status_one(AdditionalStatusOne {
                elapsed_time: elapsed,
                speed: Speed(10),
                stroke_rate: StrokeRate(stroke_rate),
                heart_rate: HeartRate(hr),
                current_pace: pace,
                average_pace: pace,
                rest_distance: RestDistance(0),
                rest_time: Time(U24::new(40)),
                machine_type: ErgMachineType::MultiergSki,
            });
        }

        let storage = WorkoutStorage::new_mem("rowing-workouts").await?;
        storage.save_workout("races", &recorder).await
    }


    #[tokio::test(flavor = "multi_thread")]
    async fn test_summary() -> anyhow::Result<()> {
        let storage = WorkoutStorage::new_disk("real-workouts").await?;
        let workout = storage
            .load_workout_lazy(
                "races",
                Uuid::from_str("019aca7f-33d9-7490-a550-d6d052c6283e")?,
            )
            .await?;
        let summary = workout.generate_summary()?;
        dbg!(summary.duration.as_secs());
        dbg!(summary.total_distance.as_meters());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_simple_summary() -> anyhow::Result<()> {
        let workout = record_workout_example(&Default::default()).await?;
        let summary = workout.generate_summary()?;
        dbg!(summary);
        Ok(())
    }
}
