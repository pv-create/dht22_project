use rppal::gpio::{Gpio, InputPin, OutputPin};
use std::{thread, time::Duration};

const GPIO_PIN: u8 = 4;

fn read_dht22() -> Result<(f32, f32), Box<dyn std::error::Error>> {
    let gpio = Gpio::new()?;
    let mut output_pin = gpio.get(GPIO_PIN)?.into_output();

    // Инициализация сенсора
    output_pin.set_low();
    thread::sleep(Duration::from_millis(20));
    output_pin.set_high();
    thread::sleep(Duration::from_micros(40));
    
    // Освобождаем пин и создаем новый InputPin
    drop(output_pin);
    let input_pin = gpio.get(GPIO_PIN)?.into_input();

    // Чтение данных
    let mut data = [0u8; 5];
    let mut bit = 7;
    let mut byte = 0;

    for _ in 0..40 {
        let start = std::time::Instant::now();
        while input_pin.is_low() {
            if start.elapsed() > Duration::from_millis(100) {
                return Err("Timeout waiting for low".into());
            }
        }
        let start = std::time::Instant::now();
        while input_pin.is_high() {
            if start.elapsed() > Duration::from_millis(100) {
                return Err("Timeout waiting for high".into());
            }
        }
        if start.elapsed() > Duration::from_micros(40) {
            data[byte] |= 1 << bit;
        }
        if bit == 0 {
            bit = 7;
            byte += 1;
        } else {
            bit -= 1;
        }
    }

    // Проверка контрольной суммы
    if data[4] != ((data[0] as u16 + data[1] as u16 + data[2] as u16 + data[3] as u16) & 0xFF) as u8 {
        return Err("Checksum mismatch".into());
    }

    // Расчет влажности и температуры
    let humidity = (data[0] as f32 * 256.0 + data[1] as f32) / 10.0;
    let temperature = ((data[2] & 0x7F) as f32 * 256.0 + data[3] as f32) / 10.0;
    let temperature = if data[2] & 0x80 != 0 { -temperature } else { temperature };

    Ok((humidity, temperature))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        match read_dht22() {
            Ok((humidity, temperature)) => {
                println!("Humidity: {:.1}%, Temperature: {:.1}°C", humidity, temperature);
            }
            Err(e) => println!("Error: {}", e),
        }
        thread::sleep(Duration::from_secs(2));
    }
}