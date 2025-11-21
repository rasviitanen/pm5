use futures::TryStreamExt;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use time::UtcDateTime;
use uuid::Uuid;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutSample {
    pub timestamp: i128,
    pub elapsed_time_ms: u32,
    pub distance_m: u32,
    pub heart_rate_bpm: Option<u8>,
    pub power_watts: Option<u16>,
    pub stroke_rate: Option<u8>,
    pub pace_ms_per_500m: Option<u32>,
    pub calories: Option<u16>,

    pub drive_length_cm: Option<u16>,
    pub drive_time_ms: Option<u16>,
    pub peak_drive_force_n: Option<u16>,
    pub avg_drive_force_n: Option<u16>,
    pub work_per_stroke_j: Option<u16>,
}

impl WorkoutSample {
    /// Convert samples to a Polars DataFrame
    pub fn to_dataframe(samples: &[WorkoutSample]) -> PolarsResult<DataFrame> {
        // Extract each field into separate vectors
        let timestamps: Vec<i64> = samples.iter().map(|s| s.timestamp as i64).collect();
        let elapsed_times: Vec<u32> = samples.iter().map(|s| s.elapsed_time_ms).collect();
        let distances: Vec<u32> = samples.iter().map(|s| s.distance_m).collect();
        let heart_rates: Vec<Option<u32>> = samples
            .iter()
            .map(|s| s.heart_rate_bpm.map(|v| v as u32))
            .collect();
        let powers: Vec<Option<u32>> = samples
            .iter()
            .map(|s| s.power_watts.map(|v| v as u32))
            .collect();
        let stroke_rates: Vec<Option<u32>> = samples
            .iter()
            .map(|s| s.stroke_rate.map(|v| v as u32))
            .collect();
        let paces: Vec<Option<u32>> = samples
            .iter()
            .map(|s| s.pace_ms_per_500m.map(|v| v as u32))
            .collect();
        let calories: Vec<Option<u32>> = samples
            .iter()
            .map(|s| s.calories.map(|v| v as u32))
            .collect();
        let drive_lengths: Vec<Option<u32>> = samples
            .iter()
            .map(|s| s.drive_length_cm.map(|v| v as u32))
            .collect();
        let drive_times: Vec<Option<u32>> = samples
            .iter()
            .map(|s| s.drive_time_ms.map(|v| v as u32))
            .collect();
        let peak_forces: Vec<Option<u32>> = samples
            .iter()
            .map(|s| s.peak_drive_force_n.map(|v| v as u32))
            .collect();
        let avg_forces: Vec<Option<u32>> = samples
            .iter()
            .map(|s| s.avg_drive_force_n.map(|v| v as u32))
            .collect();
        let work_per_strokes: Vec<Option<u32>> = samples
            .iter()
            .map(|s| s.work_per_stroke_j.map(|v| v as u32))
            .collect();

        // Create series for each column
        let df = DataFrame::new(vec![
            Series::new("timestamp".into(), timestamps).into(),
            Series::new("elapsed_time_ms".into(), elapsed_times).into(),
            Series::new("distance_m".into(), distances).into(),
            Series::new("heart_rate_bpm".into(), heart_rates).into(),
            Series::new("power_watts".into(), powers).into(),
            Series::new("stroke_rate".into(), stroke_rates).into(),
            Series::new("pace_ms_per_500m".into(), paces).into(),
            Series::new("calories".into(), calories).into(),
            Series::new("drive_length_cm".into(), drive_lengths).into(),
            Series::new("drive_time_ms".into(), drive_times).into(),
            Series::new("peak_drive_force_n".into(), peak_forces).into(),
            Series::new("avg_drive_force_n".into(), avg_forces).into(),
            Series::new("work_per_stroke_j".into(), work_per_strokes).into(),
        ])?;

        Ok(df)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutSummary {
    pub workout_id: Uuid,
    pub user_id: String,
    pub start_time: i128,
    pub end_time: i128,
    pub duration_ms: u32,
    pub total_distance_m: u32,
    pub total_calories: u16,

    pub avg_heart_rate_bpm: Option<u8>,
    pub max_heart_rate_bpm: Option<u8>,
    pub avg_power_watts: Option<u16>,
    pub avg_stroke_rate: Option<u8>,
    pub avg_pace_ms_per_500m: Option<u32>,

    pub race_id: Option<String>,
    pub race_position: Option<u16>,
}

pub struct WorkoutRecorder {
    workout_id: Uuid,
    user_id: String,
    start_time: UtcDateTime,
    last_stroke_sample: WorkoutSample,
    samples: Vec<WorkoutSample>,
}

impl WorkoutRecorder {
    pub fn new(user_id: String) -> Self {
        Self {
            workout_id: Uuid::now_v7(),
            user_id,
            last_stroke_sample: Default::default(),
            start_time: UtcDateTime::now(),
            samples: Vec::new(),
        }
    }

    pub fn workout_id(&self) -> Uuid {
        self.workout_id
    }

    pub fn add_general_sample(
        &mut self,
        elapsed_time_ms: u32,
        distance_m: u32,
        heart_rate_bpm: Option<u8>,
        stroke_rate: Option<u8>,
        pace_ms_per_500m: Option<u32>,
    ) {
        self.samples.push(WorkoutSample {
            timestamp: UtcDateTime::now().unix_timestamp_nanos(),
            elapsed_time_ms,
            distance_m,
            heart_rate_bpm,
            stroke_rate,
            pace_ms_per_500m,
            ..self.last_stroke_sample.clone()
        });
    }

    pub fn set_stroke_data(
        &mut self,
        elapsed_time_ms: u32,
        distance_m: u32,
        drive_length_cm: u16,
        drive_time_ms: u16,
        peak_drive_force_n: u16,
        avg_drive_force_n: u16,
        work_per_stroke_j: u16,
        power_watts: Option<u16>,
        calories: Option<u16>,
    ) {
        self.last_stroke_sample = WorkoutSample {
            timestamp: UtcDateTime::now().unix_timestamp_nanos(),
            elapsed_time_ms,
            distance_m,
            heart_rate_bpm: None,
            power_watts,
            stroke_rate: None,
            pace_ms_per_500m: None,
            calories,
            drive_length_cm: Some(drive_length_cm),
            drive_time_ms: Some(drive_time_ms),
            peak_drive_force_n: Some(peak_drive_force_n),
            avg_drive_force_n: Some(avg_drive_force_n),
            work_per_stroke_j: Some(work_per_stroke_j),
        };
    }

    pub fn to_dataframe(&self) -> PolarsResult<DataFrame> {
        WorkoutSample::to_dataframe(&self.samples)
    }

    pub fn to_lazyframe(&self) -> PolarsResult<LazyFrame> {
        Ok(self.to_dataframe()?.lazy())
    }

    pub fn generate_summary(&self, race_id: Option<String>) -> PolarsResult<WorkoutSummary> {
        let lf = self.to_lazyframe()?;

        let agg_df = lf
            .select([
                col("heart_rate_bpm").mean().alias("avg_hr"),
                col("heart_rate_bpm").max().alias("max_hr"),
                col("power_watts").mean().alias("avg_power"),
                col("stroke_rate").mean().alias("avg_stroke_rate"),
            ])
            .collect()?;

        let row = agg_df
            .get(0)
            .ok_or_else(|| PolarsError::ComputeError("No data to aggregate".into()))?;

        let avg_hr = row[0].try_extract::<f64>().ok().map(|v| v as u8);
        let max_hr = row[1].try_extract::<f64>().ok().map(|v| v as u8);
        let avg_power = row[2].try_extract::<f64>().ok().map(|v| v as u16);
        let avg_stroke_rate = row[3].try_extract::<f64>().ok().map(|v| v as u8);

        let last_sample = self
            .samples
            .last()
            .ok_or_else(|| PolarsError::ComputeError("No samples recorded".into()))?;

        Ok(WorkoutSummary {
            workout_id: self.workout_id,
            user_id: self.user_id.clone(),
            start_time: self.start_time.unix_timestamp_nanos(),
            end_time: UtcDateTime::now().unix_timestamp_nanos(),
            duration_ms: last_sample.elapsed_time_ms,
            total_distance_m: last_sample.distance_m,
            total_calories: last_sample.calories.unwrap_or(0),
            avg_heart_rate_bpm: avg_hr,
            max_heart_rate_bpm: max_hr,
            avg_power_watts: avg_power,
            avg_stroke_rate,
            avg_pace_ms_per_500m: None,
            race_id,
            race_position: None,
        })
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

    pub async fn save_workout(
        &self,
        recorder: &WorkoutRecorder,
        summary: &WorkoutSummary,
    ) -> anyhow::Result<()> {
        let df = recorder.to_dataframe()?;

        let path = format!(
            "workouts/{}/{}.parquet",
            summary.user_id, summary.workout_id
        );

        let mut buffer = Vec::new();
        ParquetWriter::new(&mut buffer)
            .with_compression(ParquetCompression::Snappy)
            .finish(&mut df.clone())?;

        self.operator.write(&path, buffer).await?;

        let summary_path = format!("summaries/{}/{}.json", summary.user_id, summary.workout_id);
        let summary_json = serde_json::to_vec(summary)?;
        self.operator.write(&summary_path, summary_json).await?;

        Ok(())
    }

    pub async fn load_workout_lazy(
        &self,
        user_id: &str,
        workout_id: uuid::Uuid,
    ) -> anyhow::Result<LazyFrame> {
        let path = format!("workouts/{}/{}.parquet", user_id, workout_id);
        let data = self.operator.read(&path).await?;
        let bytes: Vec<_> = data.into_iter().flat_map(|bytes| bytes.to_vec()).collect();

        let df = ParquetReader::new(std::io::Cursor::new(bytes)).finish()?;
        Ok(df.lazy())
    }

    pub async fn scan_user_workouts(&self, user_id: &str) -> anyhow::Result<Vec<String>> {
        let prefix = format!("workouts/{}/", user_id);
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

    pub fn rolling_power_avg(lf: LazyFrame, window_size: usize) -> LazyFrame {
        lf.with_column(
            col("power_watts")
                .cast(DataType::Float64)
                .rolling_mean(RollingOptionsFixedWindow {
                    window_size,
                    min_periods: 1,
                    ..Default::default()
                })
                .alias("rolling_avg_power"),
        )
    }

    pub fn rolling_heartrate_avg(lf: LazyFrame, window_size: usize) -> LazyFrame {
        lf.with_column(
            col("heart_rate_bpm")
                .cast(DataType::Float64)
                .rolling_mean(RollingOptionsFixedWindow {
                    window_size,
                    min_periods: 1,
                    ..Default::default()
                })
                .alias("rolling_avg_heartrate"),
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
