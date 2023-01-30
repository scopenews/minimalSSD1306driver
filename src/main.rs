use display::Display;
use i2c_display::I2cDisplay;
use spi_display::SpiDisplay;

use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

mod display;
mod i2c_display;
mod spi_display;

// Generate random 22 byte strings to show on display.
fn random_stringz() -> String {
    let mut rng = rand::thread_rng();
    (0..22)
        .map(|_| (rng.gen_range(32, 128) as u8) as char)
        .collect()
}

fn main() {
    // Take a handle to our I2c display object
    let mut small_oled_display = I2cDisplay::new();
    let mut big_oled_display = SpiDisplay::new().unwrap();

    sleep(Duration::from_millis(100));
    small_oled_display.init_display();
    big_oled_display.init_display();
    sleep(Duration::from_millis(100));
    small_oled_display.clear_screen(0xAA);
    big_oled_display.clear_screen(0xAA);

    sleep(Duration::from_millis(5000));

    // display random chars at random rows with random invert
    let mut rng = rand::thread_rng();
    let mut random_row: u8 = rng.gen_range(0, 8);
    let mut random_inv: u8 = rng.gen_range(1, 20);
    let mut random_string = random_stringz();
    let mut random_slice: &[u8] = random_string.as_bytes();
    loop {
        small_oled_display.display_text_18x8(random_row as u8, 0, random_inv % 2, random_slice);
        big_oled_display.display_text_18x8(random_row as u8, 0, random_inv % 2, random_slice);
        random_string = random_stringz();
        random_slice = random_string.as_bytes();
        random_row = rng.gen_range(0, 8);
        random_inv = rng.gen_range(1, 20);

        // dump cache to console so we can verify cache works

        // println!("{}", "------------------------");
        // println!();
        // for row in 0..7 {
        //     for col in 0..21 {
        //         let ascii_char = (small_oled_display.screen_char_cache[row][col] + 32) as char;

        //         if small_oled_display.screen_inverse_cache[row][col] == 1 {
        //             print!("\x1B[7m{}\x1B[27m", ascii_char);
        //         } else {
        //             print!("{}", ascii_char);
        //         }
        //     }
        //     println!();
        // }

        //sleep(Duration::from_millis(1000));
    }
}
