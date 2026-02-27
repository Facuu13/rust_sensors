struct Sensor {
    id: String,
    temperature: f32,
}

impl Sensor {
    fn new(id: &str, temperature: f32) -> Self {
        Self {
            id: id.to_string(),
            temperature,
        }
    }

    fn update_temperature(&mut self, new_temp: f32) {
        self.temperature = new_temp;
    }

    fn print(&self) {
        println!("Sensor {} -> {}°C", self.id, self.temperature);
    }

    fn is_hot(&self) -> bool {
        self.temperature > 28.0
    }
}

fn main() {
    let mut sensor = Sensor::new("esp32-01", 25.0);

    sensor.print();

    sensor.update_temperature(31.5);

    sensor.print();

    if sensor.is_hot() {
        println!("🔥 Sensor is hot!");
    }
}

