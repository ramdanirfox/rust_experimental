use std::sync::{Arc, Mutex, OnceLock};

#[derive(Debug, Default)]
pub struct SharedData {
    pub count: serde_json::Value
}

static SHARED_DATA: OnceLock<Arc<Mutex<SharedData>>> = OnceLock::new();

pub fn get_data() -> Arc<Mutex<SharedData>> {
    SHARED_DATA.get_or_init(|| Arc::new(Mutex::new(SharedData::default()))).clone()
}

pub fn set_data(param: serde_json::Value) {
    let data_arc = get_data();
    let mut data = data_arc.lock().unwrap();
    data.count = param;
}