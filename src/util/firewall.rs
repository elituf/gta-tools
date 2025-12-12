use std::{error::Error, path::PathBuf};
use windows::{
    Win32::{
        NetworkManagement::WindowsFirewall::{
            INetFwPolicy2, INetFwRule, NET_FW_ACTION_BLOCK, NET_FW_IP_PROTOCOL_ANY,
            NET_FW_IP_PROTOCOL_TCP, NET_FW_IP_PROTOCOL_UDP, NET_FW_RULE_DIR_IN,
            NET_FW_RULE_DIR_OUT, NetFwPolicy2, NetFwRule,
        },
        System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance},
    },
    core::BSTR,
};

#[derive(Debug)]
pub struct Firewall {
    policy: INetFwPolicy2,
}

impl Default for Firewall {
    fn default() -> Self {
        Self {
            policy: unsafe { CoCreateInstance(&NetFwPolicy2, None, CLSCTX_INPROC_SERVER) }.unwrap(),
        }
    }
}

impl Firewall {
    pub fn add(
        &self,
        name: &str,
        mode: RuleMode,
        direction: RuleDirection,
        protocol: RuleProtocol,
    ) -> Result<(), Box<dyn Error>> {
        let rules = unsafe { self.policy.Rules() }?;
        unsafe { rules.Remove(&BSTR::from(name)) }?;
        let rule: INetFwRule = unsafe { CoCreateInstance(&NetFwRule, None, CLSCTX_INPROC_SERVER) }?;
        unsafe { rule.SetName(&BSTR::from(name)) }?;
        match mode {
            RuleMode::Executable(exe) => {
                unsafe { rule.SetApplicationName(&BSTR::from(exe.to_string_lossy().to_string())) }?
            }
            RuleMode::Address(ip) => unsafe { rule.SetRemoteAddresses(&BSTR::from(ip)) }?,
        }
        match direction {
            RuleDirection::In => unsafe { rule.SetDirection(NET_FW_RULE_DIR_IN) }?,
            RuleDirection::Out => unsafe { rule.SetDirection(NET_FW_RULE_DIR_OUT) }?,
        }
        unsafe { rule.SetEnabled(true.into()) }?;
        unsafe { rule.SetAction(NET_FW_ACTION_BLOCK) }?;
        match protocol {
            RuleProtocol::Any => unsafe { rule.SetProtocol(NET_FW_IP_PROTOCOL_ANY.0) }?,
            RuleProtocol::Tcp => unsafe { rule.SetProtocol(NET_FW_IP_PROTOCOL_TCP.0) }?,
            RuleProtocol::Udp => unsafe { rule.SetProtocol(NET_FW_IP_PROTOCOL_UDP.0) }?,
        }
        unsafe { rules.Add(&rule) }?;
        Ok(())
    }

    pub fn remove(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let rules = unsafe { self.policy.Rules() }?;
        unsafe { rules.Remove(&BSTR::from(name)) }?;
        Ok(())
    }

    pub fn is_blocked(&self, name: &str) -> Result<bool, Box<dyn Error>> {
        let rules = unsafe { self.policy.Rules() }?;
        let rule_exists = unsafe { rules.Item(&BSTR::from(name)) }.is_ok();
        Ok(rule_exists)
    }
}

pub enum RuleMode {
    Executable(PathBuf),
    Address(String),
}

pub enum RuleDirection {
    In,
    Out,
}

pub enum RuleProtocol {
    Any,
    #[allow(unused)]
    Tcp,
    Udp,
}
