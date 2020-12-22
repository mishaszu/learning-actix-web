use std::sync::Mutex;

pub struct MainAppState {
    pub name: String,
}

pub struct CounterAppState {
    pub counter: Mutex<i32>,
}

pub struct SharedAppState {
    pub phase: Mutex<bool>,
}

impl ToString for SharedAppState {
    fn to_string(&self) -> String {
        let value = self.phase.lock().unwrap();
        match *value {
            true => "Before Check".to_string(),
            false => "After Check".to_string(),
        }
    }
}
