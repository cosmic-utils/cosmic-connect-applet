use std::collections::HashSet;

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, CosmicConfigEntry, PartialEq, Eq)]
#[version = 1]
pub struct FeaturesConfig {
    /// Whether to show the info splash view on window open
    pub show_infosplash: bool,
    pub sms_notifications: bool,
    pub clipboard_sync: ClipboardSyncConfig,
    pub share_notifications: ShareNotificationsConfig,
    pub battery: BatteryConfig,
    pub telephone: TelephoneConfig,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            show_infosplash: true,
            sms_notifications: true,
            clipboard_sync: ClipboardSyncConfig::default(),
            share_notifications: ShareNotificationsConfig::default(),
            battery: BatteryConfig::default(),
            telephone: TelephoneConfig::default(),
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BatteryConfig {
    pub battery_low_notification: bool,
    pub charged_to_notification: bool,
    pub battery_full_notification: bool,
}

impl Default for BatteryConfig {
    fn default() -> Self {
        Self {
            battery_low_notification: true,
            charged_to_notification: true,
            battery_full_notification: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelephoneConfig {
    pub pause_on_call: bool,
}

impl Default for TelephoneConfig {
    fn default() -> Self {
        Self {
            pause_on_call: true,
        }
    }
}