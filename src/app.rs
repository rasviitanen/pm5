use crate::services::{Control, Service};
use anyhow::bail;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};
use futures::stream::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

use crate::services::{Pm5, Pm5Data, Rowing, ServiceData, ServiceDataError};

const PERIPHERAL_NAME_MATCH_PREFIX_FILTER: &str = "PM5";

type Receiver = mpsc::UnboundedReceiver<Result<Pm5Data, ServiceDataError>>;
type CmdSender = mpsc::UnboundedSender<Vec<u8>>;

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn scan(&mut self) -> anyhow::Result<Vec<Peripheral>> {
        let manager = Manager::new().await?;
        let adapter_list = manager.adapters().await?;
        if adapter_list.is_empty() {
            bail!("No Bluetooth adapters found");
        }

        let mut peripherals = Vec::new();
        for adapter in adapter_list.iter() {
            println!("Starting scan...");
            adapter
                .start_scan(ScanFilter { services: vec![] })
                .await
                .expect("Can't scan BLE adapter for connected devices...");
            time::sleep(Duration::from_secs(2)).await;
            peripherals.extend(adapter.peripherals().await?);
            adapter.stop_scan().await?;
        }

        Ok(peripherals)
    }

    pub async fn listen(&mut self, peripheral: Peripheral) -> anyhow::Result<Receiver> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut notification_stream = peripheral.notifications().await?;

        tokio::spawn(async move {
            while let Some(data) = notification_stream.next().await {
                let _ = tx.send(Pm5::parse(data.uuid, data.value));
            }
            println!("Disconnecting from peripheral");
            peripheral.disconnect().await?;
            Ok::<_, anyhow::Error>(())
        });
        Ok(rx)
    }

    pub async fn control(&mut self, peripheral: &Peripheral) -> anyhow::Result<CmdSender> {
        let (tx, mut rx) = mpsc::unbounded_channel::<Vec<u8>>();

        let Some(send_characteristic) = peripheral
            .characteristics()
            .iter()
            .find(|c| c.service_uuid == Control::UUID && c.uuid == Control::Receive.id())
            .cloned()
        else {
            bail!("control service not supported");
        };

        let Some(recv_characteristic) = peripheral
            .characteristics()
            .iter()
            .find(|c| c.service_uuid == Control::UUID && c.uuid == Control::Transmit.id())
            .cloned()
        else {
            bail!("control service not supported");
        };

        peripheral.subscribe(&recv_characteristic).await?;

        let peripheral = peripheral.clone();
        tokio::spawn(async move {
            while let Some(command) = rx.recv().await {
                println!("Sending command: {:02X?}", command);
                peripheral
                    .write(
                        &send_characteristic,
                        &command,
                        btleplug::api::WriteType::WithoutResponse,
                    )
                    .await?;
            }
            peripheral.unsubscribe(&recv_characteristic).await?;
            Ok::<_, anyhow::Error>(())
        });
        Ok(tx)
    }

    pub async fn connect<'a>(
        &mut self,
        peripherals: &'a [Peripheral],
    ) -> anyhow::Result<&'a Peripheral> {
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
                    for service in peripheral.services() {
                        if Rowing::UUID == service.uuid {
                            print!("Found rowing service");
                            for characteristic in service.characteristics {
                                println!("Checking characteristic {:?}", characteristic);
                                let supported = Pm5::rowing()
                                    .into_iter()
                                    .find(|c| c.id() == characteristic.uuid);
                                if let Some(supported) = supported
                                // .contains(&characteristic.uuid)
                                // && characteristic
                                //     .properties
                                //     .contains(CharPropFlags::NOTIFY)
                                {
                                    println!(
                                        "Subscribing to characteristic {:?} ({})",
                                        supported, characteristic.uuid
                                    );
                                    peripheral.subscribe(&characteristic).await?;
                                } else {
                                    println!("Skipping {:?}", characteristic.uuid);
                                }
                            }
                        }
                    }
                    return Ok(peripheral);
                }
            } else {
                println!("Skipping unknown peripheral {:#?}", peripheral);
            }
        }

        bail!("no peripheral found")
    }

    pub async fn connect_to<'a>(
        &mut self,
        peripheral: &'a Peripheral,
    ) -> anyhow::Result<&'a Peripheral> {
        // All peripheral devices in range.
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
                for service in peripheral.services() {
                    if Rowing::UUID == service.uuid {
                        print!("Found rowing service");
                        for characteristic in service.characteristics {
                            println!("Checking characteristic {:?}", characteristic);
                            let supported = Pm5::rowing()
                                .into_iter()
                                .find(|c| c.id() == characteristic.uuid);
                            if let Some(supported) = supported
                            // .contains(&characteristic.uuid)
                            // && characteristic
                            //     .properties
                            //     .contains(CharPropFlags::NOTIFY)
                            {
                                println!(
                                    "Subscribing to characteristic {:?} ({})",
                                    supported, characteristic.uuid
                                );
                                peripheral.subscribe(&characteristic).await?;
                            } else {
                                println!("Skipping {:?}", characteristic.uuid);
                            }
                        }
                    }
                }
                return Ok(peripheral);
            } else {
                bail!("Failed to connect");
            }
        } else {
            bail!("Skipping unknown peripheral {:#?}", peripheral);
        }
    }
}
