use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
pub fn update_counter_value(
    position_y: &Arc<Mutex<u16>>,
    position_x: &Arc<Mutex<u16>>,
    limit_y: u16,
    limit_x: u16,
) {
    loop {
        thread::sleep(Duration::from_millis(100));
        let mut value_y = position_y.lock().unwrap();
        let mut value_x = position_x.lock().unwrap();
        if *value_y == limit_y - 1 {
            *value_y = 0;
            *value_x = *value_x + 10;
            if *value_x >= limit_x - 5 {
                *value_x = 0;
            }
        } else {
            *value_y = *value_y + 1;
        }
    }
}
