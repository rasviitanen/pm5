use std::time::Duration;

use pm5::{
    csafe::{CSafeBuffer, WorkoutCommand},
    display::Pm5Bitmap,
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
    let mut cmd = app.control(&connected).await?;

    // let mut screen = pm5::display::Pm5Bitmap::new();
    // screen.draw_text(10, 10, "LEADERBOARD");
    // screen.draw_text(10, 30, "1. Rasmus");
    // screen.draw_text(10, 45, "2. Simon");

    // let mut screen = Pm5Bitmap::new();
    // for y in 0..64 {
    //     for x in 0..240 {
    //         screen.set_pixel(x, y, (x + y) % 2 == 0);
    //     }
    // }

    // for packet in screen.display_bitmap() {
    //     cmd.send(packet);
    // }
    // let mut packets = screen.display_bitmap().into_iter();

    cmd.send(
        CSafeBuffer::new()
            .distance_splits(5000.into(), 400.into())
            .finalize(),
    )?;

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
                services::Pm5Data::Control(r) => {
                    if r.is_ok() {
                        println!("Got response: {:02X?}", r.data());

                        // if let Some(c) = start_workout.next() {
                        //     cmd.send(c)?;
                        // }
                    } else {
                        println!("Uh oh, got bad control response: {:02X?}", r.data());
                    }

                    // if let Some(packet) = packets.next() {
                    //     cmd.send(packet)?;
                    // }
                }
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
