use super::Display;

use rppal::gpio::{Gpio, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::thread::sleep;
use std::time::Duration;

const SPI_OLED_LCDWIDTH: u8 = 128;
const SPI_OLED_LCDHEIGHT: u8 = 64;
const SPI_OLED_SETCONTRAST: u8 = 0x81;
const SPI_OLED_DISPLAYALLON_RESUME: u8 = 0xA4;
const SPI_OLED_NORMALDISPLAY: u8 = 0xA6;
const SPI_OLED_DISPLAYOFF: u8 = 0xAE;
const SPI_OLED_DISPLAYON: u8 = 0xAF;
const SPI_OLED_SETDISPLAYOFFSET: u8 = 0xD3;
const SPI_OLED_SETCOMPINS: u8 = 0xDA;
const SPI_OLED_SETVCOMDETECT: u8 = 0xDB;
const SPI_OLED_SETDISPLAYCLOCKDIV: u8 = 0xD5;
const SPI_OLED_SETPRECHARGE: u8 = 0xD9;
const SPI_OLED_SETMULTIPLEX: u8 = 0xA8;
const SPI_OLED_SETSTARTLINE: u8 = 0x40;
const SPI_OLED_MEMORYMODE: u8 = 0x20;
const SPI_OLED_COLUMNADDR: u8 = 0x21;
const SPI_OLED_PAGEADDR: u8 = 0x22;
const SPI_OLED_COMSCANDEC: u8 = 0xC8;
const SPI_OLED_SEGREMAP: u8 = 0xA0;
const SPI_OLED_CHARGEPUMP: u8 = 0x8D;

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

        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode0)?;
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

    pub fn clear_screen(&mut self, clear_char: u8) {
        let mut command_string: Vec<u8> = vec![];

        command_string.push(SPI_OLED_COLUMNADDR); // 0x21 COMMAND
        command_string.push(0); // Column start address
        command_string.push(SPI_OLED_LCDWIDTH - 1); // Column end address
        command_string.push(SPI_OLED_PAGEADDR); // 0x22 COMMAND
        command_string.push(0); // Start Page address
        command_string.push((SPI_OLED_LCDHEIGHT / 8) - 1); // End Page address

        self.dc.set_low();
        self.write_bytes(&command_string);
        self.dc.set_high();

        let mut screen_clear_array_vec: Vec<u8> = vec![];
        screen_clear_array_vec.push(0x40); // 0x40 = data bytes follow

        for _ in 0..1025 {
            screen_clear_array_vec.push(clear_char);
        }

        self.dc.set_high();
        self.write_bytes(&screen_clear_array_vec);
    }

    pub fn init_display(&mut self) {
        let mut command_string: Vec<u8> = vec![];

        // Init sequence for 128x64 OLED module
        command_string.push(SPI_OLED_DISPLAYOFF); // 0xAE
        command_string.push(SPI_OLED_SETDISPLAYCLOCKDIV); // 0xD5
        command_string.push(0x80); // the suggested ratio 0x80
        command_string.push(SPI_OLED_SETMULTIPLEX); // 0xA8
        command_string.push(0x3F);
        command_string.push(SPI_OLED_SETDISPLAYOFFSET); // 0xD3
        command_string.push(0x0); // no offset
        command_string.push(SPI_OLED_SETSTARTLINE); // | 0x0);        // line #0
        command_string.push(SPI_OLED_CHARGEPUMP); // 0x8D
        command_string.push(0x14); // using internal VCC
        command_string.push(SPI_OLED_MEMORYMODE); // 0x20
        command_string.push(0x01); // 0x00 horizontal addressing
        command_string.push(SPI_OLED_SEGREMAP | 0x1); // rotate screen 180
        command_string.push(SPI_OLED_COMSCANDEC); // rotate screen 180
        command_string.push(SPI_OLED_SETCOMPINS); // 0xDA
        command_string.push(0x12);
        command_string.push(SPI_OLED_SETCONTRAST); // 0x81
        command_string.push(0xCF);
        command_string.push(SPI_OLED_SETPRECHARGE); // 0xd9
        command_string.push(0xF1);
        command_string.push(SPI_OLED_SETVCOMDETECT); // 0xDB
        command_string.push(0x40);
        command_string.push(SPI_OLED_DISPLAYALLON_RESUME); // 0xA4
        command_string.push(SPI_OLED_NORMALDISPLAY); // 0xA6
        command_string.push(SPI_OLED_DISPLAYON); // switch on OLED

        self.dc.set_low();
        self.write_bytes(&command_string);
        self.dc.set_high();
    }

    pub fn display_text_18x8_fast(
        &mut self,
        lcd_row: u8,
        lcd_col: u8,
        inverse_text: u8,
        the_text: &[u8],
    ) {
        // Set Display Char/Row Address

        let mut command_string: Vec<u8> = vec![];
        command_string.push(SPI_OLED_COLUMNADDR);
        command_string.push(lcd_col * 6);
        command_string.push(127);
        command_string.push(SPI_OLED_PAGEADDR);
        command_string.push(lcd_row);
        command_string.push(lcd_row);

        self.dc.set_low();
        self.write_bytes(&command_string);
        self.dc.set_high();

        let mut data_string: Vec<u8> = vec![];

        for c in the_text.iter() {
            let mut nc = *c;
            nc -= 32;
            let ncu = nc as usize;

            if inverse_text == 1 {
                data_string.push(Display::font_table(ncu)[0]);
                data_string.push(Display::font_table(ncu)[1]);
                data_string.push(Display::font_table(ncu)[2]);
                data_string.push(Display::font_table(ncu)[3]);
                data_string.push(Display::font_table(ncu)[4]);
                data_string.push(0x00);
            } else {
                data_string.push(!Display::font_table(ncu)[0]);
                data_string.push(!Display::font_table(ncu)[1]);
                data_string.push(!Display::font_table(ncu)[2]);
                data_string.push(!Display::font_table(ncu)[3]);
                data_string.push(!Display::font_table(ncu)[4]);
                data_string.push(0xFF);
            }
        }

        self.dc.set_high();
        self.write_bytes(&data_string);
        self.dc.set_high();
    }
}
