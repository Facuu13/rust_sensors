struct Sensor {
    id: String,
    temperature: f32,
}

fn main() {
    let mut sensor = Sensor {
        id: String::from("esp32-01"),
        temperature: 25.0,
    };

    print_sensor(&sensor);

    update_temperature(&mut sensor, 30.5);

    print_sensor(&sensor);
}

fn print_sensor(sensor: &Sensor) {
    println!("Sensor {} -> {}°C", sensor.id, sensor.temperature);
}

fn update_temperature(sensor: &mut Sensor, new_temp: f32) {
    sensor.temperature = new_temp;
}