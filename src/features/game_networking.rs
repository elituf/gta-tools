use crate::{
    gui::settings::BlockMethod,
    util::{
        firewall::{Firewall, RuleDirection, RuleMode, RuleProtocol},
        system_info::SystemInfo,
    },
};
use anyhow::Result;
use strum::{Display, EnumIter};

const FILTER_NAME_EXE: &str = "[GTA Tools] Block outbound traffic for all of GTA V";
const FILTER_NAME_SAVE_SERVER: &str = "[GTA Tools] Block outbound traffic to Rockstar save server";

#[derive(Clone, Copy, Debug, Default, Display, EnumIter, PartialEq)]
pub enum BlockedStatus {
    #[default]
    Unblocked,
    Server,
    Executable,
}

#[derive(Debug)]
pub struct GameNetworking {
    pub blocked: BlockedStatus,
}

impl Default for GameNetworking {
    fn default() -> Self {
        let firewall = Firewall::default();
        Self {
            blocked: if firewall.is_blocked(FILTER_NAME_SAVE_SERVER).unwrap() {
                BlockedStatus::Server
            } else if firewall.is_blocked(FILTER_NAME_EXE).unwrap() {
                BlockedStatus::Executable
            } else {
                BlockedStatus::Unblocked
            },
        }
    }
}

impl GameNetworking {
    pub fn block_exe(&mut self, system_info: &mut SystemInfo, firewall: &Firewall) -> Result<()> {
        let Some(exe_path) = system_info.get_game_exe_path() else {
            log::warn!("Unable to find game executable path.");
            return Ok(());
        };
        firewall
            .add(
                FILTER_NAME_EXE,
                RuleMode::Executable(exe_path.to_path_buf()),
                RuleDirection::Out,
                RuleProtocol::Any,
            )
            .inspect(|_| self.blocked = BlockedStatus::Executable)
    }

    pub fn unblock_exe(&mut self, firewall: &Firewall) -> Result<()> {
        firewall
            .remove(FILTER_NAME_EXE)
            .inspect(|_| self.blocked = BlockedStatus::Unblocked)
    }

    pub fn block_save_server(&mut self, save_server_ip: &str, firewall: &Firewall) -> Result<()> {
        firewall
            .add(
                FILTER_NAME_SAVE_SERVER,
                RuleMode::Address(save_server_ip.to_owned()),
                RuleDirection::Out,
                RuleProtocol::Any,
            )
            .inspect(|_| self.blocked = BlockedStatus::Server)
    }

    pub fn unblock_save_server(&mut self, firewall: &Firewall) -> Result<()> {
        firewall
            .remove(FILTER_NAME_SAVE_SERVER)
            .inspect(|_| self.blocked = BlockedStatus::Unblocked)
    }

    pub fn ensure_block_exclusivity(
        &mut self,
        block_method: BlockMethod,
        firewall: &Firewall,
    ) -> Result<()> {
        match block_method {
            BlockMethod::EntireGame => {
                if self.blocked == BlockedStatus::Server {
                    self.unblock_save_server(firewall)?;
                }
            }
            BlockMethod::SaveServer => {
                if self.blocked == BlockedStatus::Executable {
                    self.unblock_exe(firewall)?;
                }
            }
        }
        Ok(())
    }
}
