use std::{
    os::windows::process::CommandExt,
    path::{Path, PathBuf},
    process::Command,
};
use windows::{
    Win32::System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, PROCESSENTRY32, Process32First, Process32Next,
            TH32CS_SNAPPROCESS,
        },
        Threading::{
            CREATE_NO_WINDOW, OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
            QueryFullProcessImageNameW,
        },
    },
    core::PWSTR,
};

#[derive(Clone, Debug)]
pub struct Process {
    pid: u32,
    name: String,
    exe: Option<PathBuf>,
}

impl Process {
    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn exe(&self) -> Option<&Path> {
        self.exe.as_deref()
    }

    pub fn kill(&self) -> bool {
        let mut taskkill = Command::new("taskkill.exe");
        taskkill.creation_flags(CREATE_NO_WINDOW.0);
        taskkill.arg("/F").arg("/PID").arg(self.pid.to_string());
        match taskkill.output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
}

#[derive(Debug, Default)]
pub struct SystemInfo {
    processes: Vec<Process>,
}

impl SystemInfo {
    pub fn refresh(&mut self) {
        let mut processes = Vec::new();
        let snapshot_handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap() };
        let mut process_entry = PROCESSENTRY32 {
            dwSize: size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };
        unsafe { Process32First(snapshot_handle, &raw mut process_entry).unwrap() };
        let exe_full_path = get_exe_full_path(&process_entry);
        processes.push(Process {
            pid: process_entry.th32ProcessID,
            name: c_char_arr_to_string(&process_entry.szExeFile),
            exe: exe_full_path,
        });
        while unsafe { Process32Next(snapshot_handle, &raw mut process_entry) }.is_ok() {
            let exe_full_path = get_exe_full_path(&process_entry);
            processes.push(Process {
                pid: process_entry.th32ProcessID,
                name: c_char_arr_to_string(&process_entry.szExeFile),
                exe: exe_full_path,
            });
        }
        self.processes = processes;
    }

    pub fn processes(&self) -> &[Process] {
        &self.processes
    }
}

fn get_exe_full_path(process_entry: &PROCESSENTRY32) -> Option<PathBuf> {
    let process_handle_result = unsafe {
        OpenProcess(
            PROCESS_QUERY_LIMITED_INFORMATION,
            false,
            process_entry.th32ProcessID,
        )
    };
    let mut exename = [0u16; 260];
    let mut dwsize = exename.len() as u32;
    process_handle_result.map_or(None, |process_handle| {
        let image_name_result = unsafe {
            QueryFullProcessImageNameW(
                process_handle,
                PROCESS_NAME_WIN32,
                PWSTR(exename.as_mut_ptr()),
                &raw mut dwsize,
            )
        };
        match image_name_result {
            Ok(()) => Some(PathBuf::from(
                unsafe { PWSTR(exename.as_mut_ptr()).to_string() }.unwrap(),
            )),
            Err(_) => None,
        }
    })
}

fn c_char_arr_to_string(arr: &[i8]) -> String {
    arr.iter()
        .take_while(|&&b| b != 0)
        .map(|&b| b as u8 as char)
        .collect()
}
