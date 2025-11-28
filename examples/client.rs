use std::time::Duration;

use pm5::{
    csafe::{CSafeBuffer, WorkoutCommand},
    display::Pm5Bitmap,
    workout::{Profile, WorkoutRecorder, WorkoutStorage},
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
    let mut subscriptions = app.listen(connected.clone()).await?;
    let mut cmd = app.control(&connected).await?;
    cmd.send(
        CSafeBuffer::new()
            .distance_splits(5000.into(), 400.into())
            .finalize(),
    )?;

    loop {
        tokio::select! {
            biased;
            _ = tokio::signal::ctrl_c() => {
                println!("crtl-c received, stopping");
                break;
            }
            msg =  subscriptions.recv() => {
                match msg {
                    Some(Ok(data)) => match data {
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
                        services::Pm5Data::Control(r) => {
                            if r.is_ok() {
                                println!("Got OK control response: {:02X?}", r.data());
                            } else {
                                eprintln!("Uh oh, got bad control response: {:02X?}", r.data());
                            }
                        }
                    },
                    Some(Err(err)) => eprintln!("failed to parse message: {err}"),
                    None => {
                        eprintln!("channel closed");
                        break;
                    }
                }
            }
        }
    }
    let workout = storage.save_workout("races", &recorder).await?;
    let summary = workout.generate_summary()?;
    println!("Workout summary: {:#?}", summary);
    let profile = Profile::default();
    dbg!(workout.generate_summary()?);
    dbg!(workout.generate_details(&profile)?);
    Ok(())
}
