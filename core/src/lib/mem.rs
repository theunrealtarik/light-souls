#[cfg(windows)]
extern crate winapi;
use std;
use std::mem::{self, MaybeUninit};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

use winapi::ctypes::c_void;
use winapi::shared::minwindef::MAX_PATH;
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32::{
  CreateToolhelp32Snapshot, Module32First, MAX_MODULE_NAME32, MODULEENTRY32, TH32CS_SNAPMODULE,
};
use winapi::um::winnt::{HANDLE, PROCESS_ALL_ACCESS};

pub struct Memory {
  pub pid: u32,
  pub handle: HANDLE,
  pub entry: MODULEENTRY32,
}

impl Memory {
  pub fn new(process_name: &str) -> Result<Self, String> {
    if !System::IS_SUPPORTED {
      std::process::exit(1);
    }

    let mut system = System::new_all();
    system.refresh_all();

    let process = system
      .processes()
      .iter()
      .find(|(_, p)| p.name().to_lowercase() == process_name.to_lowercase())
      .map(|(_, p)| p);

    let pid = match process {
      Some(p) => p.pid().as_u32(),
      None => return Err("failed to get process pid".to_owned()),
    };

    let h_process: HANDLE = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false as i32, pid) };
    if h_process.is_null() {
      return Err("failed to get process handle".to_owned());
    }

    let mut module_entry: MODULEENTRY32 = MODULEENTRY32 {
      dwSize: mem::size_of::<MODULEENTRY32>() as u32,
      th32ModuleID: 0,
      th32ProcessID: 0,
      GlblcntUsage: 0,
      ProccntUsage: 0,
      modBaseAddr: std::ptr::null_mut(),
      modBaseSize: 0,
      hModule: std::ptr::null_mut(),
      szModule: [0; MAX_MODULE_NAME32 + 1],
      szExePath: [0; MAX_PATH],
    };

    let snapshot: HANDLE = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid) };

    unsafe { Module32First(snapshot, &mut module_entry) };

    Ok(Self {
      pid,
      handle: h_process,
      entry: module_entry,
    })
  }

  pub fn calculate_pointer(&self, initial_address: usize, offsets: Vec<usize>) -> usize {
    if offsets.is_empty() {
      initial_address
    } else {
      let mut pointer: usize = initial_address;
      for offset in offsets.iter() {
        let addr = self.read::<usize>(pointer).unwrap();
        pointer = addr + offset;
      }
      return pointer;
    }
  }

  pub fn read<T>(&self, base_address: usize) -> Result<T, String> {
    let mut buffer: MaybeUninit<T> = MaybeUninit::uninit();

    let success: bool = unsafe {
      ReadProcessMemory(
        self.handle,
        base_address as *const c_void,
        buffer.as_mut_ptr() as *mut c_void,
        mem::size_of::<T>(),
        std::ptr::null_mut(),
      )
    } != 0;

    if !success {
      return Err(format!("failed to read memory at {}", base_address));
    }

    Ok(unsafe { buffer.assume_init() })
  }

  pub fn write<T>(&self, base_address: usize, value: T) -> Result<(), String> {
    let success: bool = unsafe {
      WriteProcessMemory(
        self.handle,
        base_address as *mut c_void,
        &value as *const T as *const c_void,
        mem::size_of::<T>(),
        std::ptr::null_mut(),
      )
    } != 0;

    if !success {
      return Err(format!("failed to write to memory at {}", base_address));
    }

    Ok(())
  }

  fn close_handle(process_handle: HANDLE) {
    let closed = unsafe { CloseHandle(process_handle) } != 0;
    if !closed {
      panic!("failed to close process handle")
    }
  }
}
