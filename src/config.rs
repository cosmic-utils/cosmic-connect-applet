use std::collections::HashSet;

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, CosmicConfigEntry, PartialEq, Eq)]
#[version = 1]
pub struct FeaturesConfig {
    pub sms_notifications: bool,
    pub clipboard_sync: ClipboardSyncConfig,
    pub share_notifications: ShareNotificationsConfig,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            sms_notifications: true,
            clipboard_sync: ClipboardSyncConfig::default(),
            share_notifications: ShareNotificationsConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClipboardSyncConfig {
    pub from_mobile: bool,
    pub to_mobile: bool,
}

impl Default for ClipboardSyncConfig {
    fn default() -> Self {
        Self {
            from_mobile: true,
            to_mobile: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShareNotificationsConfig {
    pub enabled_apps: HashSet<String>,
}

impl Default for ShareNotificationsConfig {
    fn default() -> Self {
        Self {
            // Populate with all identifiable apps by default, probably
            enabled_apps: HashSet::new(),
        }
    }
}
