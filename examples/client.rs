use pm5::{
    workout::{WorkoutRecorder, WorkoutStorage},
    *,
};
use polars::prelude::*;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut recorder = WorkoutRecorder::new();
    let storage = WorkoutStorage::new_disk("real-workouts").await?;
    let mut app = pm5::app::App::new();
    let peripherals = app.scan().await?;
    let connected = app.connect(&peripherals).await?;
    let mut ch = app.listen(connected.clone()).await?;
    while let Some(msg) = ch.recv().await {
        match msg {
            Ok(data) => match data {
                services::Pm5Data::Rowing(rowing_data) => match rowing_data {
                    services::RowingData::GeneralStatus(general_status) => {
                        recorder.add_general_status(general_status);
                    }
                    services::RowingData::AdditionalStatusOne(additional_status_one) => {
                        recorder.add_additional_status_one(additional_status_one);
                    }
                    services::RowingData::StrokeData(stroke_data) => {
                        recorder.add_stroke_data(stroke_data);
                    }
                    services::RowingData::AdditionalStrokeData(additional_stroke_data) => {
                        recorder.add_additional_stroke_data(additional_stroke_data);
                    }
                    _ => {}
                },
            },
            Err(err) => tracing::error!(%err, "failed to parse message"),
        }
    }
    let workout = storage.save_workout("races", &recorder).await?;
    let summary = workout.generate_summary()?;
    println!("Workout summary: {:#?}", summary);
    workout.generate_details()?;
    Ok(())
}
