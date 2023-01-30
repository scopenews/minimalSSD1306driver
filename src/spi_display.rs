use super::Display;

use rppal::gpio::{Gpio, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::thread::sleep;
use std::time::Duration;

const SPI_OLED_LCDWIDTH: u8 = 128;
const SPI_OLED_LCDHEIGHT: u8 = 64;
const SPI_OLED_SETCONTRAST: u8 = 0x81;
const SPI_OLED_DISPLAYALLON_RESUME: u8 = 0xA4;
//const SPI_OLED_DISPLAYALLON:        u8 = 0xA5;
const SPI_OLED_NORMALDISPLAY: u8 = 0xA6;
//const SPI_OLED_INVERTDISPLAY:       u8 = 0xA7;
const SPI_OLED_DISPLAYOFF: u8 = 0xAE;
const SPI_OLED_DISPLAYON: u8 = 0xAF;
const SPI_OLED_SETDISPLAYOFFSET: u8 = 0xD3;
const SPI_OLED_SETCOMPINS: u8 = 0xDA;
const SPI_OLED_SETVCOMDETECT: u8 = 0xDB;
const SPI_OLED_SETDISPLAYCLOCKDIV: u8 = 0xD5;
const SPI_OLED_SETPRECHARGE: u8 = 0xD9;
const SPI_OLED_SETMULTIPLEX: u8 = 0xA8;
//const SPI_OLED_SETLOWCOLUMN:        u8 = 0x00;
//const SPI_OLED_SETHIGHCOLUMN:       u8 = 0x10;
const SPI_OLED_SETSTARTLINE: u8 = 0x40;
const SPI_OLED_MEMORYMODE: u8 = 0x20;
const SPI_OLED_COLUMNADDR: u8 = 0x21;
const SPI_OLED_PAGEADDR: u8 = 0x22;
//const SPI_OLED_COMSCANINC:          u8 = 0xC0;
const SPI_OLED_COMSCANDEC: u8 = 0xC8;
const SPI_OLED_SEGREMAP: u8 = 0xA0;
const SPI_OLED_CHARGEPUMP: u8 = 0x8D;
//const SPI_OLED_EXTERNALVCC:         u8 = 0x1;
//const SPI_OLED_SWITCHCAPVCC:        u8 = 0x2;
//const SPI_OLED_ADDRESS: u16 = 0x3C;

pub struct SpiDisplay {
    pub display: Display,
    pub spi: Spi,
    pub dc: OutputPin,
    pub reset: OutputPin,
}

impl SpiDisplay {
    pub fn new() -> Result<SpiDisplay, rppal::spi::Error> {
        let gpio = Gpio::new().unwrap();

        let mut dc = gpio.get(24).unwrap().into_output();
        dc.set_high();

        let mut reset = gpio.get(23).unwrap().into_output();
        reset.set_low();
        sleep(Duration::from_millis(100));
        reset.set_high();

        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0)?;
        Ok(SpiDisplay {
            spi,
            dc,
            reset,
            display: Display::new(),
        })
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.spi.write(bytes).unwrap();
    }

    pub fn oled_command(&mut self, c: u8) {
        self.dc.set_low();
        let cmd = [c];
        self.write_bytes(&cmd);
        self.dc.set_high();
    }
    pub fn oled_data(&mut self, c: u8) {
        self.dc.set_high();
        let cmd = [c];
        self.write_bytes(&cmd);
        self.dc.set_high();
    }

    pub fn clear_screen(&mut self, clear_char: u8) {
        self.oled_command(SPI_OLED_COLUMNADDR); // 0x21 COMMAND
        self.oled_command(0); // Column start address
        self.oled_command(SPI_OLED_LCDWIDTH - 1); // Column end address
        self.oled_command(SPI_OLED_PAGEADDR); // 0x22 COMMAND
        self.oled_command(0); // Start Page address
        self.oled_command((SPI_OLED_LCDHEIGHT / 8) - 1); // End Page address

        for _ in 0..1024 {
            self.oled_data(clear_char);
        }

        for row in 0..7 {
            for col in 0..21 {
                self.display.screen_char_cache[row][col] = 0;
                self.display.screen_inverse_cache[row][col] = 0;
            }
        }
    }

    pub fn init_display(&mut self) {
        // Init sequence for 128x64 OLED module
        self.oled_command(SPI_OLED_DISPLAYOFF); // 0xAE
        self.oled_command(SPI_OLED_SETDISPLAYCLOCKDIV); // 0xD5
        self.oled_command(0x80); // the suggested ratio 0x80
        self.oled_command(SPI_OLED_SETMULTIPLEX); // 0xA8
        self.oled_command(0x3F);
        self.oled_command(SPI_OLED_SETDISPLAYOFFSET); // 0xD3
        self.oled_command(0x0); // no offset
        self.oled_command(SPI_OLED_SETSTARTLINE); // | 0x0);        // line #0
        self.oled_command(SPI_OLED_CHARGEPUMP); // 0x8D
        self.oled_command(0x14); // using internal VCC
        self.oled_command(SPI_OLED_MEMORYMODE); // 0x20
        self.oled_command(0x01); // 0x00 horizontal addressing
        self.oled_command(SPI_OLED_SEGREMAP | 0x1); // rotate screen 180
        self.oled_command(SPI_OLED_COMSCANDEC); // rotate screen 180
        self.oled_command(SPI_OLED_SETCOMPINS); // 0xDA
        self.oled_command(0x12);
        self.oled_command(SPI_OLED_SETCONTRAST); // 0x81
        self.oled_command(0xCF);
        self.oled_command(SPI_OLED_SETPRECHARGE); // 0xd9
        self.oled_command(0xF1);
        self.oled_command(SPI_OLED_SETVCOMDETECT); // 0xDB
        self.oled_command(0x40);
        self.oled_command(SPI_OLED_DISPLAYALLON_RESUME); // 0xA4
        self.oled_command(SPI_OLED_NORMALDISPLAY); // 0xA6
        self.oled_command(SPI_OLED_DISPLAYON); // switch on OLED
    }

    fn show_font57_12864(&mut self, the_char: u8, row: u8, col: u8, inv: u8) {
        // if this char is already at this position, skip the write
        if (self.display.screen_char_cache[row as usize][col as usize] == the_char)
            && (self.display.screen_inverse_cache[row as usize][col as usize] == inv)
        {
            return;
        }

        // otherwise cache the written char and inverse setting
        self.display.screen_char_cache[row as usize][col as usize] = the_char;
        self.display.screen_inverse_cache[row as usize][col as usize] = inv;

        // Set Display Char/Row Address
        self.oled_command(SPI_OLED_COLUMNADDR); // 0x21 COMMAND
        self.oled_command(col * 6); // Column start address
        self.oled_command(127); // Column end address
        self.oled_command(SPI_OLED_PAGEADDR); // 0x22 COMMAND
        self.oled_command(row); // Start Page address
        self.oled_command(row); // End Page address

        if inv == 1 {
            // Show Character
            self.oled_data(!Display::font_table(the_char.into())[0]);
            self.oled_data(!Display::font_table(the_char.into())[1]);
            self.oled_data(!Display::font_table(the_char.into())[2]);
            self.oled_data(!Display::font_table(the_char.into())[3]);
            self.oled_data(!Display::font_table(the_char.into())[4]);
            self.oled_data(0xFF);
        } else {
            // Show Character
            self.oled_data(Display::font_table(the_char.into())[0]);
            self.oled_data(Display::font_table(the_char.into())[1]);
            self.oled_data(Display::font_table(the_char.into())[2]);
            self.oled_data(Display::font_table(the_char.into())[3]);
            self.oled_data(Display::font_table(the_char.into())[4]);
            self.oled_data(0x00);
        }
    }
    pub fn display_text_18x8(
        &mut self,
        lcd_row: u8,
        lcd_col: u8,
        inverse_text: u8,
        the_text: &[u8],
    ) {
        let mut ctr = 0;

        for c in the_text.iter() {
            let mut nc = *c;
            nc -= 32;
            self.show_font57_12864(nc, lcd_row, ctr + lcd_col, inverse_text);
            ctr += 1;
        }
    }
}
