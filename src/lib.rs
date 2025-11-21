pub mod parse;
pub mod services;
pub mod types;
pub mod workout;

// use anyhow::bail;
// use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
// use btleplug::platform::{Manager, Peripheral};
// use futures::stream::StreamExt;
// use services::Service;
// use std::time::Duration;
// use tokio::time;

// use crate::services::{Pm5, Rowing, ServiceData};

// const PERIPHERAL_NAME_MATCH_PREFIX_FILTER: &str = "PM5";

// pub struct App {
//     storage: opendal::Operator,
// }

// impl App {
//     pub fn new() -> anyhow::Result<Self> {
//         let builder = opendal::services::Fs::default().root("./out");
//         let storage: opendal::Operator = opendal::Operator::new(builder)?.finish();
//         let app = App { storage };
//         Ok(app)
//     }

//     pub async fn scan(&mut self) -> anyhow::Result<Vec<Peripheral>> {
//         let manager = Manager::new().await?;
//         let adapter_list = manager.adapters().await?;
//         if adapter_list.is_empty() {
//             eprintln!("No Bluetooth adapters found");
//         }

//         let mut peripherals = Vec::new();
//         for adapter in adapter_list.iter() {
//             println!("Starting scan...");
//             adapter
//                 .start_scan(ScanFilter {
//                     services: vec![
//                         //crate::services::Information::UUID,
//                         //crate::services::Rowing::UUID,
//                     ],
//                 })
//                 .await
//                 .expect("Can't scan BLE adapter for connected devices...");
//             time::sleep(Duration::from_secs(2)).await;
//             peripherals.extend(adapter.peripherals().await?);
//         }

//         Ok(peripherals)
//     }

//     pub async fn listen(&mut self, peripheral: &Peripheral) -> anyhow::Result<()> {
//         let mut notification_stream = peripheral.notifications().await?;
//         //let mut writer = self.storage.writer("data").await?;
//         while let Some(data) = notification_stream.next().await {
//             println!("Received data [{:?}]", data.uuid);
//             let parsed = Pm5::parse(data.uuid, data.value);
//             println!("frame: {:?}", parsed);
//             // writer.write(data.uuid.as_bytes().to_vec()).await?;
//             // writer.write(data.value).await?;
//         }
//         // writer.close().await?;
//         println!("Disconnecting from peripheral");
//         peripheral.disconnect().await?;

//         Ok(())
//     }

//     pub async fn connect<'a>(
//         &mut self,
//         peripherals: &'a [Peripheral],
//     ) -> anyhow::Result<&'a Peripheral> {
//         // All peripheral devices in range.
//         for peripheral in peripherals.iter() {
//             let properties = peripheral.properties().await?;
//             let is_connected = peripheral.is_connected().await?;
//             let local_name = properties
//                 .unwrap()
//                 .local_name
//                 .unwrap_or(String::from("(peripheral name unknown)"));
//             println!(
//                 "Peripheral {:?} is connected: {:?}",
//                 &local_name, is_connected
//             );
//             // Check if it's the peripheral we want.
//             if local_name.starts_with(PERIPHERAL_NAME_MATCH_PREFIX_FILTER) {
//                 println!("Found matching peripheral {:?}...", &local_name);
//                 if !is_connected {
//                     // Connect if we aren't already connected.
//                     if let Err(err) = peripheral.connect().await {
//                         eprintln!("Error connecting to peripheral, skipping: {}", err);
//                         continue;
//                     }
//                 }
//                 let is_connected = peripheral.is_connected().await?;
//                 println!(
//                     "Now connected ({:?}) to peripheral {:?}.",
//                     is_connected, &local_name
//                 );
//                 if is_connected {
//                     println!("Discover peripheral {:?} services...", local_name);
//                     peripheral.discover_services().await?;
//                     for service in peripheral.services() {
//                         if Rowing::UUID == service.uuid {
//                             print!("Found rowing service");
//                             for characteristic in service.characteristics {
//                                 println!("Checking characteristic {:?}", characteristic);
//                                 let supported = Pm5::rowing()
//                                     .into_iter()
//                                     .find(|c| c.id() == characteristic.uuid);
//                                 if let Some(supported) = supported
//                                 // .contains(&characteristic.uuid)
//                                 // && characteristic
//                                 //     .properties
//                                 //     .contains(CharPropFlags::NOTIFY)
//                                 {
//                                     println!(
//                                         "Subscribing to characteristic {:?} ({})",
//                                         supported, characteristic.uuid
//                                     );
//                                     peripheral.subscribe(&characteristic).await?;
//                                 } else {
//                                     println!("Skipping {:?}", characteristic.uuid);
//                                 }
//                             }
//                         }
//                     }
//                     return Ok(peripheral);
//                 }
//             } else {
//                 println!("Skipping unknown peripheral {:#?}", peripheral);
//             }
//         }

//         bail!("no peripheral found")
//     }

//     pub async fn run(mut self) -> anyhow::Result<()> {
//         let peripherals = self.scan().await?;
//         let connected = self.connect(&peripherals).await?;
//         self.listen(&connected).await?;
//         Ok(())
//     }
// }
