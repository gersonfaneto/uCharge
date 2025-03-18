#[allow(dead_code)]
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Vehicle {
    model: String,
    autonomy: f64, // ¹How can I decrease this over time? ²This is supposed to be in kWh
    is_running: bool,
}

impl Vehicle {
    fn new(model: String, autonomy: f64) -> Self {
        Self {
            model,
            autonomy,
            is_running: false,
        }
    }

    fn start(&mut self) {
        self.is_running = true
    }

    fn stop(&mut self) {
        self.is_running = false
    }

    fn run(&mut self, decrement: f64) {
        if self.is_running {
            self.autonomy -= decrement;
            if self.autonomy < 0.0 {
                self.autonomy = 0.0;
                self.stop()
            }
        }
    }
}

#[derive(Debug)]
struct Driver {
    name: String,
    email: String,
    password: String,
    vehichles: HashMap<String, Vehicle>,
}

impl Driver {
    fn new(name: String, email: String, password: String) -> Self {
        Self {
            name,
            email,
            password,
            vehichles: HashMap::new(),
        }
    }

    fn add_vehicle(&mut self, vehicle: Vehicle, alias: String) {
        self.vehichles.entry(alias).or_insert(vehicle);
    }

    fn remove_vehicle(&mut self, alias: String) -> Option<(String, Vehicle)> {
        self.vehichles.remove_entry(&alias)
    }
}

#[derive(Debug)]
struct ChargingStation {
    location: String,
    avaliable: u32,
    occupied: u32,
    charging_rate: f64, // ¹This is supposed to be in kW
}

impl ChargingStation {
    fn new(location: String, avaliable: u32, charging_rate: f64) -> Self {
        Self {
            location,
            avaliable,
            occupied: 0,
            charging_rate,
        }
    }
}

fn main() {
    let vehicle = Arc::new(Mutex::new(Vehicle::new("Tesla Model S".to_string(), 100.0)));

    let vehicle_clone = Arc::clone(&vehicle);

    {
        let mut vehicle = vehicle.lock().unwrap();
        vehicle.start();
    }

    thread::spawn(move || loop {
        {
            let mut vehicle = vehicle_clone.lock().unwrap();
            vehicle.run(1.0);
            println!("{:?}", vehicle);
        }

        thread::sleep(Duration::from_secs(1));

        {
            let vehicle = vehicle_clone.lock().unwrap();
            if vehicle.autonomy == 90.0 {
                break;
            }
        }
    }).join().unwrap(); // This make sures the `main` thread will wait for this thread to end.
    
    {
        let mut vehicle = vehicle.lock().unwrap();
        vehicle.stop();
    }
}
