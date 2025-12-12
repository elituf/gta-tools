use crate::{
    gui::settings::BlockMethod,
    util::{
        firewall::{Firewall, RuleDirection, RuleMode, RuleProtocol},
        system_info::SystemInfo,
    },
};
use std::error::Error;

const FILTER_NAME_EXE: &str = "[GTA Tools] Block outbound traffic for all of GTA V";
const FILTER_NAME_SAVE_SERVER: &str = "[GTA Tools] Block outbound traffic to Rockstar save server";

#[derive(Debug)]
pub struct GameNetworking {
    pub blocked: bool,
}

impl Default for GameNetworking {
    fn default() -> Self {
        let firewall = Firewall::default();
        Self {
            blocked: if firewall.is_blocked(FILTER_NAME_SAVE_SERVER).unwrap() {
                true
            } else {
                firewall.is_blocked(FILTER_NAME_EXE).unwrap()
            },
        }
    }
}

impl GameNetworking {
    pub fn block_exe(
        &mut self,
        system_info: &mut SystemInfo,
        firewall: &Firewall,
    ) -> Result<(), Box<dyn Error>> {
        let Some(exe_path) = system_info.get_game_exe_path() else {
            return Ok(());
        };
        firewall.add(
            FILTER_NAME_EXE,
            RuleMode::Executable(exe_path.to_path_buf()),
            RuleDirection::Out,
            RuleProtocol::Any,
        )?;
        self.blocked = firewall.is_blocked(FILTER_NAME_EXE)?;
        Ok(())
    }

    pub fn unblock_exe(&mut self, firewall: &Firewall) -> Result<(), Box<dyn Error>> {
        firewall.remove(FILTER_NAME_EXE)?;
        self.blocked = firewall.is_blocked(FILTER_NAME_EXE)?;
        Ok(())
    }

    pub fn block_save_server(
        &mut self,
        save_server_ip: &str,
        firewall: &Firewall,
    ) -> Result<(), Box<dyn Error>> {
        firewall.add(
            FILTER_NAME_SAVE_SERVER,
            RuleMode::Address(save_server_ip.to_owned()),
            RuleDirection::Out,
            RuleProtocol::Any,
        )?;
        self.blocked = firewall.is_blocked(FILTER_NAME_SAVE_SERVER)?;
        Ok(())
    }

    pub fn unblock_save_server(&mut self, firewall: &Firewall) -> Result<(), Box<dyn Error>> {
        firewall.remove(FILTER_NAME_SAVE_SERVER)?;
        self.blocked = firewall.is_blocked(FILTER_NAME_SAVE_SERVER)?;
        Ok(())
    }

    pub fn ensure_block_exclusivity(
        &mut self,
        block_method: BlockMethod,
        firewall: &Firewall,
    ) -> Result<(), Box<dyn Error>> {
        match block_method {
            BlockMethod::EntireGame => {
                if firewall.is_blocked(FILTER_NAME_SAVE_SERVER)? {
                    self.unblock_save_server(firewall)?;
                    self.blocked = firewall.is_blocked(FILTER_NAME_EXE)?;
                }
            }
            BlockMethod::SaveServer => {
                if firewall.is_blocked(FILTER_NAME_EXE)? {
                    self.unblock_exe(firewall)?;
                    self.blocked = firewall.is_blocked(FILTER_NAME_SAVE_SERVER)?;
                }
            }
        }
        Ok(())
    }
}
