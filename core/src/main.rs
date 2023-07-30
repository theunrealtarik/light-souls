#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[path = "lib/mod.rs"]
mod lib;

use std;

use serde::{Deserialize, Serialize};
use tauri::api::dialog;
use tauri::{Manager, State};

use lib::mem::Memory;
use lib::{GAME_MANAGER_OFFSET, GAME_PROCESS_NAME};

#[derive(Debug, Serialize)]
struct Data {
  pid: usize,
  phandle: usize,
}

#[allow(unused)]
enum Action {
  Set,
  Add,
}

impl Action {
  fn as_str(&self) -> &str {
    match self {
      Action::Set => "set",
      Action::Add => "add",
    }
  }
}

#[derive(Debug, Deserialize)]
struct Signal {
  action: String,
  value: Option<i32>,
}

#[tauri::command]
fn init_data(data: State<'_, Data>) -> Data {
  Data {
    pid: data.pid,
    phandle: data.phandle,
  }
}

#[tauri::command(rename_all = "snake_case")]
fn add_souls(amount: u32) {
  println!("adding {} of souls to the player", amount)
}

fn main() {
  match Memory::new(GAME_PROCESS_NAME) {
    Ok(mem) => {
      tauri::Builder::default()
        .manage(Data {
          pid: mem.pid as usize,
          phandle: mem.handle as usize,
        })
        .setup(|app| {
          let handle = app.app_handle();
          handle.listen_global("events::souls", |event| {
            let mem = Memory::new(GAME_PROCESS_NAME).unwrap();
            let payload = event.payload().unwrap();
            let signal: Result<Signal, serde_json::Error> = serde_json::from_str(payload);

            if let Ok(signal) = signal {
              let player_souls_address = mem.calculate_pointer(
                mem.entry.modBaseAddr as usize + GAME_MANAGER_OFFSET,
                vec![0x10, 0x94],
              );
              let current_player_souls = mem.read::<i32>(player_souls_address).unwrap();

              if let Some(mut value) = signal.value {
                if signal.action == Action::Add.as_str() {
                  value = current_player_souls + value;
                }

                if value < i32::MAX {
                  mem
                    .write::<i32>(player_souls_address, value)
                    .expect("failed to manipulate player souls")
                }
              }
            }
          });

          Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_souls, init_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }
    Err(err) => {
      dialog::MessageDialogBuilder::new("Error", err)
        .kind(dialog::MessageDialogKind::Error)
        .show(|_| std::process::exit(1));
    }
  }
}
