use pm5::workout::*;
use polars::prelude::*;
use uuid::Uuid;

async fn record_workout_example() -> anyhow::Result<Uuid> {
    let mut recorder = WorkoutRecorder::new("user_abc123".to_string());

    for i in 0..3000 {
        let elapsed_ms = i * 1000;
        let distance = i * 10; // ~10m per second
        let hr = Some(120 + (i % 20) as u8);
        let stroke_rate = Some(20 + (i % 5) as u8);
        let pace = Some(120000 + i * 100); // Getting slightly slower

        if i % 3 == 0 {
            recorder.set_stroke_data(
                elapsed_ms,
                distance,
                140,
                800,
                450,
                380,
                85,
                Some(240 + (i % 60) as u16),
                Some(10 + i as u16),
            );
        }

        recorder.add_general_sample(elapsed_ms, distance, hr, stroke_rate, pace);
    }

    // Generate summary using LazyFrame
    let summary = recorder.generate_summary(None)?;
    let id = summary.workout_id;
    println!(
        "Workout completed: {} meters in {}ms",
        summary.total_distance_m, summary.duration_ms
    );
    println!("Avg Power: {:?}W", summary.avg_power_watts);

    let storage = WorkoutStorage::new_disk("rowing-workouts").await?;

    storage.save_workout(&recorder, &summary).await?;
    println!("Workout saved to cloud!");

    Ok(id)
}

async fn analyze_single_workout(id: uuid::Uuid) -> anyhow::Result<()> {
    let storage = WorkoutStorage::new_disk("rowing-workouts").await?;

    let lf = storage.load_workout_lazy("user_abc123", id).await?;

    tokio::task::block_in_place(|| {
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

async fn analyze_user_power_trends(id: Uuid) -> anyhow::Result<()> {
    let storage = WorkoutStorage::new_disk("rowing-workouts").await?;

    let lf = storage.load_workout_lazy("user_abc123", id).await?;

    tokio::task::block_in_place(|| {
        let with_rolling = lf
            .clone()
            .with_column(
                col("elapsed_time_ms")
                    .cast(DataType::Datetime(TimeUnit::Milliseconds, None))
                    .alias("time"),
            )
            .with_column(
                col("power_watts")
                    .cast(DataType::Float64)
                    .rolling_mean_by(
                        col("time"),
                        RollingOptionsDynamicWindow {
                            window_size: Duration::parse("60s"),
                            min_periods: 1,
                            closed_window: ClosedWindow::None,
                            fn_params: Default::default(),
                        },
                    )
                    .alias("power_1min_avg"),
            );

        let peak_1min = with_rolling
            .clone()
            .select([col("power_1min_avg").max()])
            .collect()?;

        println!("\nPeak 1-minute power: {:?}", peak_1min);

        let splits = lf
            .clone()
            .with_column(
                (col("distance_m") / lit(500))
                    .cast(DataType::Int32)
                    .alias("split_500m"),
            )
            .group_by([col("split_500m")])
            .agg([
                col("elapsed_time_ms").first().alias("split_start_ms"),
                col("elapsed_time_ms").last().alias("split_end_ms"),
                col("power_watts").mean().alias("split_avg_power"),
                col("heart_rate_bpm").mean().alias("split_avg_hr"),
            ])
            .with_column((col("split_end_ms") - col("split_start_ms")).alias("split_duration_ms"))
            .sort(["split_500m"], Default::default())
            .collect()?;

        println!("\n500m Splits:");
        println!("{}", splits);

        let fastest_split = splits
            .lazy()
            .filter(col("split_duration_ms").is_not_null())
            .sort(["split_duration_ms"], Default::default())
            .limit(1)
            .collect()?;

        println!("\nFastest 500m Split:");
        println!("{}", fastest_split);
        Ok::<_, anyhow::Error>(())
    })
}

async fn calculate_race_results() -> anyhow::Result<()> {
    let race_summaries = vec![
        WorkoutSummary {
            workout_id: Uuid::now_v7(),
            user_id: "user1".into(),
            start_time: 0,
            end_time: 300000,
            duration_ms: 300000, // 5 minutes
            total_distance_m: 1500,
            total_calories: 150,
            avg_power_watts: Some(240),
            avg_heart_rate_bpm: Some(165),
            max_heart_rate_bpm: Some(180),
            avg_stroke_rate: Some(22),
            avg_pace_ms_per_500m: Some(100000),
            race_id: Some("race123".into()),
            race_position: None,
        },
        WorkoutSummary {
            workout_id: Uuid::now_v7(),
            user_id: "user2".into(),
            start_time: 0,
            end_time: 310000,
            duration_ms: 310000,
            total_distance_m: 1500,
            total_calories: 145,
            avg_power_watts: Some(230),
            avg_heart_rate_bpm: Some(170),
            max_heart_rate_bpm: Some(185),
            avg_stroke_rate: Some(24),
            avg_pace_ms_per_500m: Some(103000),
            race_id: Some("race123".into()),
            race_position: None,
        },
        WorkoutSummary {
            workout_id: Uuid::now_v7(),
            user_id: "user3".into(),
            start_time: 0,
            end_time: 295000,
            duration_ms: 295000,
            total_distance_m: 1500,
            total_calories: 155,
            avg_power_watts: Some(250),
            avg_heart_rate_bpm: Some(172),
            max_heart_rate_bpm: Some(188),
            avg_stroke_rate: Some(20),
            avg_pace_ms_per_500m: Some(98000),
            race_id: Some("race123".into()),
            race_position: None,
        },
    ];

    tokio::task::block_in_place(|| {
        let user_ids: Vec<String> = race_summaries.iter().map(|s| s.user_id.clone()).collect();
        let durations: Vec<u32> = race_summaries.iter().map(|s| s.duration_ms).collect();

        let df = DataFrame::new(vec![
            Series::new("user_id".into(), user_ids).into(),
            Series::new("duration_ms".into(), durations).into(),
            Series::from_iter(
                race_summaries
                    .iter()
                    .map(|s| s.avg_power_watts.map(|v| v as u32)),
            )
            .with_name("avg_power_watts".into())
            .into(),
        ])?;

        let leaderboard = df
            .clone()
            .lazy()
            .sort(["duration_ms"], Default::default())
            .with_row_index("position", Some(1))
            .with_column(
                // Calculate time behind leader
                (col("duration_ms") - col("duration_ms").first()).alias("time_behind_ms"),
            )
            .with_column(
                // Format position as "1st", "2nd", "3rd", etc.
                when(col("position").eq(lit(1)))
                    .then(lit("🥇 1st"))
                    .when(col("position").eq(lit(2)))
                    .then(lit("🥈 2nd"))
                    .when(col("position").eq(lit(3)))
                    .then(lit("🥉 3rd"))
                    .otherwise(concat_str([col("position"), lit("th")], "", false))
                    .alias("display_position"),
            )
            .collect()?;

        println!("Race Leaderboard (1500m):");
        println!("{}", leaderboard);

        let power_rankings = df
            .lazy()
            .filter(col("avg_power_watts").is_not_null())
            .sort(
                ["avg_power_watts"],
                SortMultipleOptions::default().with_order_descending(true),
            )
            .with_row_index("power_rank", Some(1))
            .collect()?;

        println!("\nPower Rankings:");
        println!("{}", power_rankings);

        Ok::<_, anyhow::Error>(())
    })
}

async fn analyze_stroke_quality(id: Uuid) -> anyhow::Result<()> {
    let storage = WorkoutStorage::new_disk("rowing-workouts").await?;

    let lf = storage.load_workout_lazy("user_abc123", id).await?;

    tokio::task::block_in_place(|| {
        let analysis = lf
            .filter(col("peak_drive_force_n").is_not_null())
            .with_column(
                (col("peak_drive_force_n").cast(DataType::Float64)
                    / col("avg_drive_force_n").cast(DataType::Float64))
                .alias("force_ratio"),
            )
            .with_column(
                when(col("force_ratio").gt(lit(1.3)))
                    .then(lit("Uneven force application"))
                    .when(col("force_ratio").lt(lit(1.1)))
                    .then(lit("Very consistent"))
                    .otherwise(lit("Good"))
                    .alias("technique_rating"),
            );

        let stats = analysis
            .clone()
            .group_by([col("technique_rating")])
            .agg([
                col("force_ratio").count().alias("stroke_count"),
                col("force_ratio").mean().alias("avg_force_ratio"),
            ])
            .collect()?;

        println!("Stroke Quality Analysis:");
        println!("{}", stats);

        let problematic = analysis
            .filter(col("force_ratio").gt(lit(1.4)))
            .select([
                col("elapsed_time_ms"),
                col("peak_drive_force_n"),
                col("avg_drive_force_n"),
                col("force_ratio"),
            ])
            .sort(
                ["force_ratio"],
                SortMultipleOptions::default().with_order_descending(true),
            )
            .limit(10)
            .collect()?;

        println!("\nTop 10 Most Uneven Strokes:");
        println!("{}", problematic);
        Ok::<_, anyhow::Error>(())
    })
}

async fn time_series_analysis(id: Uuid) -> anyhow::Result<()> {
    let storage = WorkoutStorage::new_disk("rowing-workouts").await?;

    let lf = storage.load_workout_lazy("user_abc123", id).await?;

    tokio::task::block_in_place(|| {
        let resampled = lf
            .clone()
            .with_column(
                (col("elapsed_time_ms") / lit(5000))
                    .cast(DataType::Int32)
                    .alias("interval_5s"),
            )
            .group_by([col("interval_5s")])
            .agg([
                col("power_watts").mean().alias("avg_power"),
                col("heart_rate_bpm").mean().alias("avg_hr"),
                col("stroke_rate").mean().alias("avg_stroke_rate"),
                col("distance_m").max().alias("distance"),
            ])
            .sort(["interval_5s"], Default::default())
            .collect()?;

        tokio::task::block_in_place(|| {
            println!("5-Second Interval Summary:");
            println!("{}", resampled);

            // Calculate fatigue index (power drop from first 20% to last 20%)
            let total_intervals = resampled.height();
            let first_20pct = (total_intervals as f64 * 0.2) as usize;
            let last_20pct_start = (total_intervals as f64 * 0.8) as usize;

            let fatigue_analysis = resampled
                .lazy()
                .with_column(
                    when(col("interval_5s").lt(lit(first_20pct as i32)))
                        .then(lit("first_20pct"))
                        .when(col("interval_5s").gt(lit(last_20pct_start as i32)))
                        .then(lit("last_20pct"))
                        .otherwise(lit("middle"))
                        .alias("segment"),
                )
                .filter(col("segment").neq(lit("middle")))
                .group_by([col("segment")])
                .agg([
                    col("avg_power").mean().alias("segment_avg_power"),
                    col("avg_hr").mean().alias("segment_avg_hr"),
                ])
                .collect()?;

            println!("\nFatigue Analysis (First 20% vs Last 20%):");
            println!("{}", fatigue_analysis);

            // Calculate coefficient of variation for power (consistency metric)
            let power_consistency = lf
                .filter(col("power_watts").is_not_null())
                .select([
                    col("power_watts").mean().alias("mean_power"),
                    col("power_watts").std(1).alias("std_power"),
                ])
                .with_column(
                    (col("std_power") / col("mean_power") * lit(100.0)).alias("cv_percent"),
                )
                .collect()?;

            println!("\nPower Consistency (lower CV% = more consistent):");
            println!("{}", power_consistency);
            Ok::<_, anyhow::Error>(())
        })
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let id = record_workout_example().await?;
    analyze_single_workout(id).await?;
    analyze_user_power_trends(id).await?;
    analyze_stroke_quality(id).await?;
    time_series_analysis(id).await?;
    calculate_race_results().await?;

    Ok(())
}
