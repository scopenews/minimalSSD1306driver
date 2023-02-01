use super::Display;
use rppal::i2c::I2c;

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
const SPI_OLED_ADDRESS: u16 = 0x3C;

pub struct I2cDisplay {
    pub display: Display,
    pub i2c: I2c,
}

impl I2cDisplay {
    pub fn new() -> I2cDisplay {
        let i2c = I2c::new().unwrap();

        I2cDisplay {
            i2c,
            display: Display::new(),
        }
    }

    pub fn clear_screen(&mut self, clear_char: u8) {
        let mut command_string: Vec<u8> = vec![];
        command_string.push(0x00); // 0x00 = command bytes follow

        command_string.push(SPI_OLED_COLUMNADDR); // 0x21 COMMAND
        command_string.push(0); // Column start address
        command_string.push(SPI_OLED_LCDWIDTH - 1); // Column end address
        command_string.push(SPI_OLED_PAGEADDR); // 0x22 COMMAND
        command_string.push(0); // Start Page address
        command_string.push((SPI_OLED_LCDHEIGHT / 8) - 1); // End Page address

        self.write_data_to_i2c(SPI_OLED_ADDRESS, &command_string);

        let mut screen_clear_array_vec: Vec<u8> = vec![];
        screen_clear_array_vec.push(0x40); // 0x40 = data bytes follow

        for _ in 0..1025 {
            screen_clear_array_vec.push(clear_char);
        }
        self.write_data_to_i2c(SPI_OLED_ADDRESS, &command_string);
    }

    pub fn init_display(&mut self) {
        let mut command_string: Vec<u8> = vec![];
        command_string.push(0x00); // 0x00 = command bytes follow

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
        command_string.push(SPI_OLED_DISPLAYON); //switch on OLED

        self.write_data_to_i2c(SPI_OLED_ADDRESS, &command_string);
    }

    fn write_data_to_i2c(&mut self, slave_address: u16, data: &[u8]) {
        //let mut i2c = I2c::new().unwrap();
        self.i2c.set_slave_address(slave_address).unwrap();
        self.i2c.write(data).unwrap();
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
        command_string.push(0x00);
        command_string.push(SPI_OLED_COLUMNADDR);
        command_string.push(lcd_col * 6);
        command_string.push(127);
        command_string.push(SPI_OLED_PAGEADDR);
        command_string.push(lcd_row);
        command_string.push(lcd_row);

        self.write_data_to_i2c(SPI_OLED_ADDRESS, &command_string);

        let mut data_string: Vec<u8> = vec![];
        data_string.push(0x40);

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

        self.write_data_to_i2c(SPI_OLED_ADDRESS, &data_string);
    }
}
