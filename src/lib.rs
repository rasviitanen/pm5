pub mod services;
pub mod types;

use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use services::Service;
use std::error::Error;
use std::time::Duration;
use tokio::time;

const PERIPHERAL_NAME_MATCH_PREFIX_FILTER: &str = "PM5";

pub struct App;

impl App {
    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let manager = Manager::new().await?;
        let adapter_list = manager.adapters().await?;
        if adapter_list.is_empty() {
            eprintln!("No Bluetooth adapters found");
        }

        for adapter in adapter_list.iter() {
            println!("Starting scan...");
            adapter
                .start_scan(ScanFilter::default())
                .await
                .expect("Can't scan BLE adapter for connected devices...");
            time::sleep(Duration::from_secs(2)).await;
            let peripherals = adapter.peripherals().await?;

            if peripherals.is_empty() {
                eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
            } else {
                // All peripheral devices in range.
                for peripheral in peripherals.iter() {
                    let properties = peripheral.properties().await?;
                    let is_connected = peripheral.is_connected().await?;
                    let local_name = properties
                        .unwrap()
                        .local_name
                        .unwrap_or(String::from("(peripheral name unknown)"));
                    println!(
                        "Peripheral {:?} is connected: {:?}",
                        &local_name, is_connected
                    );
                    // Check if it's the peripheral we want.
                    if local_name.starts_with(PERIPHERAL_NAME_MATCH_PREFIX_FILTER) {
                        println!("Found matching peripheral {:?}...", &local_name);
                        if !is_connected {
                            // Connect if we aren't already connected.
                            if let Err(err) = peripheral.connect().await {
                                eprintln!("Error connecting to peripheral, skipping: {}", err);
                                continue;
                            }
                        }
                        let is_connected = peripheral.is_connected().await?;
                        println!(
                            "Now connected ({:?}) to peripheral {:?}.",
                            is_connected, &local_name
                        );
                        if is_connected {
                            println!("Discover peripheral {:?} services...", local_name);
                            peripheral.discover_services().await?;
                            for characteristic in peripheral.characteristics() {
                                println!("Checking characteristic {:?}", characteristic);
                                // Subscribe to notifications from the characteristic with the selected
                                // UUID.
                                if (characteristic.uuid == services::Rowing::GeneralStatus.id())
                                    && characteristic.properties.contains(CharPropFlags::NOTIFY)
                                {
                                    println!(
                                        "Subscribing to characteristic {:?}",
                                        characteristic.uuid
                                    );
                                    peripheral.subscribe(&characteristic).await?;
                                    // Print the first 4 notifications received.
                                    let mut notification_stream =
                                        peripheral.notifications().await?.take(4);
                                    // Process while the BLE connection is not broken or stopped.
                                    while let Some(data) = notification_stream.next().await {
                                        println!(
                                            "Received data from {:?} [{:?}]: {:?}",
                                            local_name, data.uuid, data.value
                                        );
                                    }
                                }
                            }
                            println!("Disconnecting from peripheral {:?}...", local_name);
                            peripheral.disconnect().await?;
                        }
                    } else {
                        println!("Skipping unknown peripheral {:#?}", peripheral);
                    }
                }
            }
        }
        Ok(())
    }
}
