use ascii_letter::AsciiLetter;
use display::Display;
use i2c_display::I2cDisplay;
use spi_display::SpiDisplay;

use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

mod ascii_letter;
mod display;
mod i2c_display;
mod spi_display;

fn generate_string(letter: &mut AsciiLetter) -> String {
    (0..21)
        .map(|_| letter.get_and_increment() as char)
        .collect()
}

fn main() {
    // Take a handle to our I2c display object
    let mut small_oled_display = I2cDisplay::new();
    let mut big_oled_display = SpiDisplay::new().unwrap();

    // this guy feeds an ascii table slowly
    let mut letter1 = AsciiLetter::new();

    small_oled_display.init_display();
    big_oled_display.init_display();
    sleep(Duration::from_millis(100));
    small_oled_display.clear_screen(0);
    big_oled_display.clear_screen(0);

    //sleep(Duration::from_millis(22000));

    // display ascii chars at random rows with random invert
    let mut rng = rand::thread_rng();
    let mut random_row: u8 = rng.gen_range(1, 7);
    let mut random_inv: u8 = rng.gen_range(1, 20);
    let mut random_string = generate_string(&mut letter1);
    let mut random_slice: &[u8] = random_string.as_bytes();

    big_oled_display.display_text_18x8_fast(0, 0, 1, b"    MULTI DISPLAY    ");
    big_oled_display.display_text_18x8_fast(7, 0, 0, b"[ TEST ]");
    big_oled_display.display_text_18x8_fast(7, 10, 1, b"[ ACTIVE ]");
    small_oled_display.display_text_18x8_fast(0, 0, 1, b"    MULTI DISPLAY    ");
    small_oled_display.display_text_18x8_fast(7, 0, 0, b"[ TEST ]");
    small_oled_display.display_text_18x8_fast(7, 10, 1, b"[ ACTIVE ]");

    loop {
        small_oled_display.display_text_18x8_fast(random_row, 0, random_inv % 2, random_slice);
        big_oled_display.display_text_18x8_fast(random_row, 0, random_inv % 2, random_slice);

        random_row = rng.gen_range(1, 7);
        random_inv = rng.gen_range(1, 20);
        random_string = generate_string(&mut letter1);
        random_slice = random_string.as_bytes();

        sleep(Duration::from_millis(1));
    }
}
