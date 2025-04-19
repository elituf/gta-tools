use crate::util::consts::game::{EXE_ENHANCED, EXE_LEGACY};
use std::path::Path;
use sysinfo::System;
use windows::{
    Win32::{
        Foundation::E_INVALIDARG,
        NetworkManagement::WindowsFirewall::{
            INetFwPolicy2, INetFwRule, NET_FW_ACTION_BLOCK, NET_FW_IP_PROTOCOL_ANY,
            NET_FW_RULE_DIR_IN, NET_FW_RULE_DIR_OUT, NetFwPolicy2, NetFwRule,
        },
        System::Com::{
            CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx,
            CoUninitialize,
        },
    },
    core::{BSTR, HRESULT},
};

const FILTER_NAME_IN: &str = "[GTA Tools] Block all inbound traffic for GTA V";
const FILTER_NAME_OUT: &str = "[GTA Tools] Block all outbound traffic for GTA V";

#[derive(Debug)]
pub struct GameNetworking {
    pub is_blocked: bool,
    com_initialized: bool,
    policy: INetFwPolicy2,
}

impl Default for GameNetworking {
    fn default() -> Self {
        let result = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
        let mut gn = Self {
            is_blocked: false,
            com_initialized: result != HRESULT(0x80010106u32 as i32),
            policy: unsafe { CoCreateInstance(&NetFwPolicy2, None, CLSCTX_INPROC_SERVER).unwrap() },
        };
        gn.is_blocked = gn.is_blocked();
        gn
    }
}

impl Drop for GameNetworking {
    fn drop(&mut self) {
        unsafe {
            if self.com_initialized {
                CoUninitialize();
            }
        }
    }
}

impl GameNetworking {
    pub fn block_all(&mut self, sysinfo: &mut System) {
        let Some(exe_path) = get_game_exe_path(sysinfo) else {
            return;
        };
        let rules = unsafe { self.policy.Rules().unwrap() };
        let filter_name_in = BSTR::from(FILTER_NAME_IN);
        let filter_name_out = BSTR::from(FILTER_NAME_OUT);
        unsafe {
            let _ = rules.Remove(&filter_name_in);
            let _ = rules.Remove(&filter_name_out);
        }
        let exe_path = BSTR::from(exe_path.to_string_lossy().to_string());
        unsafe {
            let inbound_rule: INetFwRule =
                CoCreateInstance(&NetFwRule, None, CLSCTX_INPROC_SERVER).unwrap();
            inbound_rule.SetName(&filter_name_in).unwrap();
            inbound_rule.SetApplicationName(&exe_path).unwrap();
            inbound_rule.SetDirection(NET_FW_RULE_DIR_IN).unwrap();
            inbound_rule.SetEnabled(true.into()).unwrap();
            inbound_rule.SetAction(NET_FW_ACTION_BLOCK).unwrap();
            inbound_rule.SetProtocol(NET_FW_IP_PROTOCOL_ANY.0).unwrap();
            rules.Add(&inbound_rule).unwrap();
        }
        unsafe {
            let outbound_rule: INetFwRule =
                CoCreateInstance(&NetFwRule, None, CLSCTX_INPROC_SERVER).unwrap();
            outbound_rule.SetName(&filter_name_out).unwrap();
            outbound_rule.SetApplicationName(&exe_path).unwrap();
            outbound_rule.SetDirection(NET_FW_RULE_DIR_OUT).unwrap();
            outbound_rule.SetEnabled(true.into()).unwrap();
            outbound_rule.SetAction(NET_FW_ACTION_BLOCK).unwrap();
            outbound_rule.SetProtocol(NET_FW_IP_PROTOCOL_ANY.0).unwrap();
            rules.Add(&outbound_rule).unwrap();
        }
        self.is_blocked = self.is_blocked();
    }

    pub fn unblock_all(&mut self) {
        let rules = unsafe { self.policy.Rules().unwrap() };
        unsafe {
            let result = rules.Remove(&BSTR::from(FILTER_NAME_IN));
            if let Err(ref why) = result {
                if why.code() != E_INVALIDARG {
                    result.unwrap();
                }
            }
            let result = rules.Remove(&BSTR::from(FILTER_NAME_OUT));
            if let Err(ref why) = result {
                if why.code() != E_INVALIDARG {
                    result.unwrap();
                }
            }
        }
        self.is_blocked = self.is_blocked();
    }

    fn is_blocked(&self) -> bool {
        let rules = unsafe { self.policy.Rules().unwrap() };
        let in_rule_exists = unsafe { rules.Item(&BSTR::from(FILTER_NAME_IN)).is_ok() };
        let out_rule_exists = unsafe { rules.Item(&BSTR::from(FILTER_NAME_OUT)).is_ok() };
        in_rule_exists || out_rule_exists
    }
}

fn get_game_exe_path(sysinfo: &mut System) -> Option<&Path> {
    sysinfo.refresh_all();
    if let Some((_, process)) = sysinfo
        .processes()
        .iter()
        .find(|(_, p)| p.name() == EXE_ENHANCED || p.name() == EXE_LEGACY)
    {
        process.exe()
    } else {
        None
    }
}
