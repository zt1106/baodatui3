use parking_lot::RwLock;
use std::ops::Deref;
use std::sync::{Arc, OnceLock};

pub struct SystemSettings {
    /// if room remain non-active for some time, remove it
    pub non_active_room_time: u64,
    /// even no rooms are changed, still notify client current all rooms info
    pub passive_notify_all_rooms_info_interval: u64,
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            non_active_room_time: 600 * 1000,
            passive_notify_all_rooms_info_interval: 10000,
        }
    }
}

pub fn system_settings_arc() -> &'static Arc<RwLock<SystemSettings>> {
    static SYSTEM_SETTINGS: OnceLock<Arc<RwLock<SystemSettings>>> = OnceLock::new();
    &SYSTEM_SETTINGS.get_or_init(Default::default)
}

pub fn system_settings() -> impl Deref<Target = SystemSettings> + 'static {
    system_settings_arc().read()
}
