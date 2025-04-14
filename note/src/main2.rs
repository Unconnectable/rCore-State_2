use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Duration;
fn main() {
    let mut data = Arc::new(Mutex::new(vec![1, 2, 3]));
    for i in 0..3 {
        let data_clone = data.clone();
        thread::spawn(move || {
            let mut mx_data = data_clone.lock().unwrap();
            
            mx_data[i] += i;
        });
    }
    println!("{:?}", data);
    thread::sleep(Duration::from_millis(50));
}
