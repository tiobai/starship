use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let update_counter = counter.clone();
    thread::spawn(move || {
        update_counter_value(update_counter);
    });
    loop {
        let value = counter.lock().unwrap();
        println!("Counter: {}", *value);
        thread::sleep(Duration::from_millis(30));
    }
}

// Función para actualizar el contador
fn update_counter_value(counter: Arc<Mutex<i32>>) {
    let mut current_value = 0;
    loop {
        // Dormir por 1 segundo
        thread::sleep(Duration::from_secs(1));

        // Actualizar el valor del contador
        let mut value = counter.lock().unwrap();
        current_value += 1;
        *value = current_value;
    }
}
