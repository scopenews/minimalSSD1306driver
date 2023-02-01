// this class returns the next ASCII letter, loops back once it gets
// to the end. just a way to cough up ASCII letters for testing
// the display

pub struct AsciiLetter {
    ascii_letter: u8,
}

impl AsciiLetter {
    pub fn new() -> AsciiLetter {
        AsciiLetter { ascii_letter: 65 }
    }

    pub fn get_and_increment(&mut self) -> u8 {
        let result = self.ascii_letter;
        self.ascii_letter += 1;
        if self.ascii_letter >= 122 {
            self.ascii_letter = 32;
        }
        result
    }
}
