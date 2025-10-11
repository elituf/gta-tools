use std::{
    ffi::{OsStr, OsString},
    os::windows::{ffi::OsStringExt, process::CommandExt},
    path::{Path, PathBuf},
    process::Command,
};
use windows::{
    Win32::System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW,
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
    name: OsString,
    exe: Option<PathBuf>,
}

impl Process {
    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn name(&self) -> &OsStr {
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
        let snapshot_handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }.unwrap();
        let mut process_entry = PROCESSENTRY32W {
            dwSize: size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        unsafe { Process32FirstW(snapshot_handle, &raw mut process_entry) }.unwrap();
        let exe_full_path = get_exe_full_path(&process_entry);
        processes.push(Process {
            pid: process_entry.th32ProcessID,
            name: wide_array_to_os_string(&process_entry.szExeFile),
            exe: exe_full_path,
        });
        while unsafe { Process32NextW(snapshot_handle, &raw mut process_entry) }.is_ok() {
            let exe_full_path = get_exe_full_path(&process_entry);
            processes.push(Process {
                pid: process_entry.th32ProcessID,
                name: wide_array_to_os_string(&process_entry.szExeFile),
                exe: exe_full_path,
            });
        }
        self.processes = processes;
    }

    pub fn processes(&self) -> &[Process] {
        &self.processes
    }
}

fn get_exe_full_path(process_entry: &PROCESSENTRY32W) -> Option<PathBuf> {
    let process_handle_result = unsafe {
        OpenProcess(
            PROCESS_QUERY_LIMITED_INFORMATION,
            false,
            process_entry.th32ProcessID,
        )
    };
    process_handle_result.map_or(None, |process_handle| {
        let mut exe_name = [0u16; 260];
        let mut dw_size = exe_name.len() as u32;
        let image_name_result = unsafe {
            QueryFullProcessImageNameW(
                process_handle,
                PROCESS_NAME_WIN32,
                PWSTR(exe_name.as_mut_ptr()),
                &raw mut dw_size,
            )
        };
        match image_name_result {
            Ok(()) => Some(PathBuf::from(wide_array_to_os_string(&exe_name))),
            Err(_) => None,
        }
    })
}

fn wide_array_to_os_string(wide: &[u16]) -> OsString {
    let null_pos = wide.iter().position(|&x| x == 0).unwrap_or(wide.len());
    OsString::from_wide(&wide[..null_pos])
}
