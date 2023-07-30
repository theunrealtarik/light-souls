#![allow(dead_code)]
pub mod mem;
pub const PREFERRED_IMAGEBASE: usize = 0x140000000;
pub const GAME_MANAGER_OFFSET: usize = 0x1D00F50;
pub const GAME_PROCESS_NAME: &str = "DarkSoulsRemastered.exe";
