use std::collections::HashSet;

use macroquad::prelude::{info, warn};

const COMPLETED_LEVELS_KEY: &str = "COMPLETED_LEVELS";

pub fn save_completed_levels(completed: &HashSet<usize>) {
    info!("Saving completed levels {:?}", completed);
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    let values: Vec<String> = completed.iter().map(|x| format!("{x}")).collect();
    let to_save = values.join(" ");

    storage.set(COMPLETED_LEVELS_KEY, &to_save);
}

pub fn load_completed_levels() -> HashSet<usize> {
    info!("Loading completed levels");
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    if let Some(completed_str) = storage.get(COMPLETED_LEVELS_KEY) {
        let parsed: Result<_, _> = completed_str.split(" ").map(|s| s.parse()).collect();

        parsed.unwrap_or_else(|err| {
            warn!("load_completed_levels: {}", err);
            HashSet::new()
        })
    } else {
        info!("Completed levels not found");
        HashSet::new()
    }
}
