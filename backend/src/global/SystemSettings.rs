use parking_lot::RwLock;
use std::ops::Deref;
use std::sync::{Arc, OnceLock};

pub struct SystemSettings {
    pub non_active_room_time: u64,
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            non_active_room_time: 600 * 1000,
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
