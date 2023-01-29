use rand::Rng;
use rppal::i2c::I2c;
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
const SPI_OLED_ADDRESS: u16 = 0x3C;

struct Display {
    i2c: I2c,
    screen_char_cache: [[u8; 22]; 8],
    screen_inverse_cache: [[u8; 22]; 8],
}

impl Display {
    fn new() -> Display {
        let i2c = I2c::new().unwrap();
        Display {
            i2c,
            screen_char_cache: [[0xFF; 22]; 8],
            screen_inverse_cache: [[0xFF; 22]; 8],
        }
    }

    fn font_table(character: usize) -> [u8; 5] {
        let the_fonts: [[u8; 5]; 97] = [
            [0x00, 0x00, 0x00, 0x00, 0x00], // (space)
            [0x00, 0x00, 0x5F, 0x00, 0x00], // !
            [0x00, 0x07, 0x00, 0x07, 0x00], // "
            [0x14, 0x7F, 0x14, 0x7F, 0x14], // #
            [0x24, 0x2A, 0x7F, 0x2A, 0x12], // $
            [0x23, 0x13, 0x08, 0x64, 0x62], // %
            [0x36, 0x49, 0x55, 0x22, 0x50], // &
            [0x00, 0x05, 0x03, 0x00, 0x00], // '
            [0x00, 0x1C, 0x22, 0x41, 0x00], // (
            [0x00, 0x41, 0x22, 0x1C, 0x00], // )
            [0x08, 0x2A, 0x1C, 0x2A, 0x08], // *
            [0x08, 0x08, 0x3E, 0x08, 0x08], // +
            [0x00, 0x50, 0x30, 0x00, 0x00], // ,
            [0x08, 0x08, 0x08, 0x08, 0x08], // -
            [0x00, 0x30, 0x30, 0x00, 0x00], // .
            [0x20, 0x10, 0x08, 0x04, 0x02], // /
            [0x3E, 0x51, 0x49, 0x45, 0x3E], // 0
            [0x00, 0x42, 0x7F, 0x40, 0x00], // 1
            [0x42, 0x61, 0x51, 0x49, 0x46], // 2
            [0x21, 0x41, 0x45, 0x4B, 0x31], // 3
            [0x18, 0x14, 0x12, 0x7F, 0x10], // 4
            [0x27, 0x45, 0x45, 0x45, 0x39], // 5
            [0x3C, 0x4A, 0x49, 0x49, 0x30], // 6
            [0x01, 0x71, 0x09, 0x05, 0x03], // 7
            [0x36, 0x49, 0x49, 0x49, 0x36], // 8
            [0x06, 0x49, 0x49, 0x29, 0x1E], // 9
            [0x00, 0x36, 0x36, 0x00, 0x00], // :
            [0x00, 0x56, 0x36, 0x00, 0x00], // ;
            [0x00, 0x08, 0x14, 0x22, 0x41], // <
            [0x14, 0x14, 0x14, 0x14, 0x14], // =
            [0x41, 0x22, 0x14, 0x08, 0x00], // >
            [0x02, 0x01, 0x51, 0x09, 0x06], // ?
            [0x32, 0x49, 0x79, 0x41, 0x3E], // @
            [0x7E, 0x11, 0x11, 0x11, 0x7E], // A
            [0x7F, 0x49, 0x49, 0x49, 0x36], // B
            [0x3E, 0x41, 0x41, 0x41, 0x22], // C
            [0x7F, 0x41, 0x41, 0x22, 0x1C], // D
            [0x7F, 0x49, 0x49, 0x49, 0x41], // E
            [0x7F, 0x09, 0x09, 0x01, 0x01], // F
            [0x3E, 0x41, 0x41, 0x51, 0x32], // G
            [0x7F, 0x08, 0x08, 0x08, 0x7F], // H
            [0x00, 0x41, 0x7F, 0x41, 0x00], // I
            [0x20, 0x40, 0x41, 0x3F, 0x01], // J
            [0x7F, 0x08, 0x14, 0x22, 0x41], // K
            [0x7F, 0x40, 0x40, 0x40, 0x40], // L
            [0x7F, 0x02, 0x04, 0x02, 0x7F], // M
            [0x7F, 0x04, 0x08, 0x10, 0x7F], // N
            [0x3E, 0x41, 0x41, 0x41, 0x3E], // O
            [0x7F, 0x09, 0x09, 0x09, 0x06], // P
            [0x3E, 0x41, 0x51, 0x21, 0x5E], // Q
            [0x7F, 0x09, 0x19, 0x29, 0x46], // R
            [0x46, 0x49, 0x49, 0x49, 0x31], // S
            [0x01, 0x01, 0x7F, 0x01, 0x01], // T
            [0x3F, 0x40, 0x40, 0x40, 0x3F], // U
            [0x1F, 0x20, 0x40, 0x20, 0x1F], // V
            [0x7F, 0x20, 0x18, 0x20, 0x7F], // W
            [0x63, 0x14, 0x08, 0x14, 0x63], // X
            [0x03, 0x04, 0x78, 0x04, 0x03], // Y
            [0x61, 0x51, 0x49, 0x45, 0x43], // Z
            [0x00, 0x00, 0x7F, 0x41, 0x41], // [
            [0x02, 0x04, 0x08, 0x10, 0x20], // "\"
            [0x41, 0x41, 0x7F, 0x00, 0x00], // ]
            [0x04, 0x02, 0x01, 0x02, 0x04], // ^
            [0x40, 0x40, 0x40, 0x40, 0x40], // _
            [0x00, 0x01, 0x02, 0x04, 0x00], // `
            [0x20, 0x54, 0x54, 0x54, 0x78], // a
            [0x7F, 0x48, 0x44, 0x44, 0x38], // b
            [0x38, 0x44, 0x44, 0x44, 0x20], // c
            [0x38, 0x44, 0x44, 0x48, 0x7F], // d
            [0x38, 0x54, 0x54, 0x54, 0x18], // e
            [0x08, 0x7E, 0x09, 0x01, 0x02], // f
            [0x08, 0x14, 0x54, 0x54, 0x3C], // g
            [0x7F, 0x08, 0x04, 0x04, 0x78], // h
            [0x00, 0x44, 0x7D, 0x40, 0x00], // i
            [0x20, 0x40, 0x44, 0x3D, 0x00], // j
            [0x00, 0x7F, 0x10, 0x28, 0x44], // k
            [0x00, 0x41, 0x7F, 0x40, 0x00], // l
            [0x7C, 0x04, 0x18, 0x04, 0x78], // m
            [0x7C, 0x08, 0x04, 0x04, 0x78], // n
            [0x38, 0x44, 0x44, 0x44, 0x38], // o
            [0x7C, 0x14, 0x14, 0x14, 0x08], // p
            [0x08, 0x14, 0x14, 0x18, 0x7C], // q
            [0x7C, 0x08, 0x04, 0x04, 0x08], // r
            [0x48, 0x54, 0x54, 0x54, 0x20], // s
            [0x04, 0x3F, 0x44, 0x40, 0x20], // t
            [0x3C, 0x40, 0x40, 0x20, 0x7C], // u
            [0x1C, 0x20, 0x40, 0x20, 0x1C], // v
            [0x3C, 0x40, 0x30, 0x40, 0x3C], // w
            [0x44, 0x28, 0x10, 0x28, 0x44], // x
            [0x0C, 0x50, 0x50, 0x50, 0x3C], // y
            [0x44, 0x64, 0x54, 0x4C, 0x44], // z
            [0x00, 0x08, 0x36, 0x41, 0x00], // {
            [0x00, 0x00, 0x7F, 0x00, 0x00], // |
            [0x06, 0x09, 0x09, 0x06, 0x00], // }
            [0x08, 0x08, 0x2A, 0x1C, 0x08], // ->
            [0x08, 0x1C, 0x2A, 0x08, 0x08], // <-
            [0x00, 0x02, 0x05, 0x02, 0x00], //  deg
        ];

        the_fonts[character]
    }

    fn clear_screen(&mut self, clear_char: u8) {
        self.oled_command(SPI_OLED_COLUMNADDR); // 0x21 COMMAND
        self.oled_command(0); // Column start address
        self.oled_command(SPI_OLED_LCDWIDTH - 1); // Column end address
        self.oled_command(SPI_OLED_PAGEADDR); // 0x22 COMMAND
        self.oled_command(0); // Start Page address
        self.oled_command((SPI_OLED_LCDHEIGHT / 8) - 1); // End Page address

        let mut screen_clear_array: [u8; 1025] = [clear_char; 1025];
        screen_clear_array[0] = 0x40; // 1st byte of all OLED commands
        self.write_data_to_i2c(SPI_OLED_ADDRESS, &screen_clear_array);

        for row in 0..7 {
            for col in 0..21 {
                self.screen_char_cache[row][col] = 0;
                self.screen_inverse_cache[row][col] = 0;
            }
        }
    }

    fn init_display(&mut self) {
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
        self.oled_command(SPI_OLED_DISPLAYON); //switch on OLED
    }

    fn display_text_18x8(&mut self, lcd_row: u8, lcd_col: u8, inverse_text: u8, the_text: &[u8]) {
        let mut ctr = 0;

        for c in the_text.iter() {
            let mut nc = *c;
            nc -= 32;
            self.show_font57_12864(nc, lcd_row, ctr + lcd_col, inverse_text);
            ctr += 1;
        }
    }

    fn show_font57_12864(&mut self, the_char: u8, row: u8, col: u8, inv: u8) {
        // if this char is already at this position, skip the write
        if (self.screen_char_cache[row as usize][col as usize] == the_char)
            && (self.screen_inverse_cache[row as usize][col as usize] == inv)
        {
            return;
        }

        // otherwise cache the written char and inverse setting
        self.screen_char_cache[row as usize][col as usize] = the_char;
        self.screen_inverse_cache[row as usize][col as usize] = inv;

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

    fn oled_command(&mut self, c: u8) {
        // commands start with a 0x00
        let data = [0x00, c as u8];
        self.write_data_to_i2c(SPI_OLED_ADDRESS, &data);
    }

    fn oled_data(&mut self, c: u8) {
        // data starts with a 0x40
        let data = [0x40, c as u8];
        self.write_data_to_i2c(SPI_OLED_ADDRESS, &data);
    }

    fn write_data_to_i2c(&mut self, slave_address: u16, data: &[u8]) {
        //let mut i2c = I2c::new().unwrap();
        self.i2c.set_slave_address(slave_address).unwrap();
        self.i2c.write(data).unwrap();
    }
}

fn random_stringz() -> String {
    let mut rng = rand::thread_rng();
    (0..20)
        .map(|_| (rng.gen_range(32, 128) as u8) as char)
        .collect()
}

fn main() {
    let mut small_oled_display = Display::new();
    sleep(Duration::from_millis(1000));
    small_oled_display.init_display();
    sleep(Duration::from_millis(1000));
    small_oled_display.clear_screen(0x00);

    loop {
        let mut rng = rand::thread_rng();
        let mut random_row: u8 = rng.gen_range(0, 8);
        let mut random_inv: u8 = rng.gen_range(2, 6);

        let mut random_string = random_stringz();
        let mut random_slice: &[u8] = random_string.as_bytes();

        small_oled_display.display_text_18x8(random_row as u8, 0, random_inv % 2, random_slice);

        random_string = random_stringz();
        random_slice = random_string.as_bytes();
        random_row = rng.gen_range(0, 8);
        random_inv = rng.gen_range(2, 6);
        small_oled_display.display_text_18x8(random_row as u8, 0, random_inv % 2, random_slice);

        random_string = random_stringz();
        random_slice = random_string.as_bytes();
        random_row = rng.gen_range(0, 8);
        random_inv = rng.gen_range(2, 6);
        small_oled_display.display_text_18x8(random_row as u8, 0, random_inv % 2, random_slice);

        random_string = random_stringz();
        random_slice = random_string.as_bytes();
        random_row = rng.gen_range(0, 8);
        random_inv = rng.gen_range(2, 6);
        small_oled_display.display_text_18x8(random_row as u8, 0, random_inv % 2, random_slice);

        random_string = random_stringz();
        random_slice = random_string.as_bytes();
        random_row = rng.gen_range(0, 8);
        random_inv = rng.gen_range(2, 6);
        small_oled_display.display_text_18x8(random_row as u8, 0, random_inv % 2, random_slice);

        random_string = random_stringz();
        random_slice = random_string.as_bytes();
        random_row = rng.gen_range(0, 8);
        random_inv = rng.gen_range(2, 6);
        small_oled_display.display_text_18x8(random_row as u8, 0, random_inv % 2, random_slice);

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
