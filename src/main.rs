struct Sensor {
    id: String,
    temperature: f32,
    history: Vec<f32>,
}

impl Sensor {
    fn new(id: &str, temperature: f32) -> Self {
        Self {
            id: id.to_string(),
            temperature,
            history: vec![temperature], // arranca con el primer valor
        }
    }

    fn update_temperature(&mut self, new_temp: f32) {
        self.temperature = new_temp;
        self.history.push(new_temp);
    }

    fn print(&self) {
        println!("Sensor {} -> {}°C", self.id, self.temperature);
    }

    fn is_hot(&self) -> bool {
        self.temperature > 28.0
    }

    fn print_history(&self) {
        println!("History for {}: {:?}", self.id, self.history);
    }
}

fn main() {
    let mut sensor = Sensor::new("esp32-01", 25.0);

    sensor.update_temperature(27.0);
    sensor.update_temperature(29.5);
    sensor.update_temperature(30.1);

    sensor.print();
    sensor.print_history();

    if sensor.is_hot() {
        println!("🔥 Sensor is hot!");
    }
}

