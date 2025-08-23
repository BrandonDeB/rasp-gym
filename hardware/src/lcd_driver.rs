/// Basic commands for HD44780 LCD
const LCD_CLEARDISPLAY: u8 = 0x01;
const LCD_RETURNHOME: u8 = 0x02;
const LCD_ENTRYMODESET: u8 = 0x04;
const LCD_DISPLAYCONTROL: u8 = 0x08;
const LCD_FUNCTIONSET: u8 = 0x20;

const LCD_ENTRYLEFT: u8 = 0x02;
const LCD_DISPLAYON: u8 = 0x04;
const LCD_2LINE: u8 = 0x08;
const LCD_5x8DOTS: u8 = 0x00;

// Backlight flag
const LCD_BACKLIGHT: u8 = 0x08;

// Enable bit
const ENABLE: u8 = 0b00000100;

use embedded_hal::i2c::I2c;
use core::fmt::Debug;

pub struct I2cLcd<I2C> {
    i2c: I2C,
    address: u8,
    backlight: u8,
}

impl<I2C, E> I2cLcd<I2C>
where
    I2C: I2c<Error = E>,
    E: Debug,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        let mut lcd = Self {
            i2c,
            address,
            backlight: LCD_BACKLIGHT,
        };
        lcd.init().unwrap();
        lcd
    }

    fn init(&mut self) -> Result<(), E> {
        // Wait >40ms after power on (assumed done by your delay or startup)

        // Function set: 4-bit mode, 2 line, 5x8 dots
        self.command(LCD_FUNCTIONSET | LCD_2LINE | LCD_5x8DOTS)?;

        // Display control: display on, cursor off, blink off
        self.command(LCD_DISPLAYCONTROL | LCD_DISPLAYON)?;

        // Clear display
        self.command(LCD_CLEARDISPLAY)?;

        // Entry mode set: cursor moves right, no display shift
        self.command(LCD_ENTRYMODESET | LCD_ENTRYLEFT)?;

        Ok(())
    }

    fn command(&mut self, value: u8) -> Result<(), E> {
        self.send(value, 0)
    }

    fn write_char(&mut self, value: u8) -> Result<(), E> {
        self.send(value, 1)
    }

    fn send(&mut self, value: u8, mode: u8) -> Result<(), E> {
        let high_nibble = value & 0xF0;
        let low_nibble = (value << 4) & 0xF0;

        self.write4bits(high_nibble | mode | self.backlight)?;
        self.write4bits(low_nibble | mode | self.backlight)?;
        Ok(())
    }

    fn write4bits(&mut self, value: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[value | ENABLE])?;
        cortex_m::asm::delay(1000); // short delay (~ >450ns)
        self.i2c.write(self.address, &[value & !ENABLE])?;
        cortex_m::asm::delay(1000);
        Ok(())
    }

    pub fn write_str(&mut self, s: &str) -> Result<(), E> {
        for &b in s.as_bytes() {
            self.write_char(b)?;
        }
        Ok(())
    }
}
