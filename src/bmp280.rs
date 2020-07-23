use embedded_hal::blocking::i2c::{Write, WriteRead};

const I2C_BASE_ADDR: u8 = 0x76;
const BMP280_ID: u8 = 0x58;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Register {
    ID         = 0xD0,
    RESET      = 0xE0,
    CTRL_MEAS  = 0xF4,
    CONFIG     = 0xF5,
    PRESS_MSB  = 0xF7,
    PRESS_LSB  = 0xF8,
    PRESS_XLSB = 0xF9,
}

impl Register {
    fn addr(&self) -> u8 {
        *self as u8
    }
}

pub struct Bmp280<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> Bmp280<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>
{
    pub fn new(i2c: I2C, sdo_pin: bool) -> Result<Self, E> {
        let address = I2C_BASE_ADDR | sdo_pin as u8;
        let mut bmp280 = Bmp280 {i2c, address};

        let id = bmp280.read_register(Register::ID)?;
        assert_eq!(id, BMP280_ID, "Invalid bmp280 ID: expected 0x{:02x}, get 0x{:02x}", BMP280_ID, id);

        bmp280.write_register(Register::CTRL_MEAS, 0xFF)?;

        Ok(bmp280)
    }

    pub fn write_register(&mut self, reg: Register, data: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[reg.addr(), data])
    }

    pub fn read_register(&mut self, reg: Register) -> Result<u8, E> {
        let mut buff: [u8; 1] = [0];
        self.i2c.write_read(self.address, &[reg.addr()], &mut buff)?;
        Ok(buff[0])
    }

    pub fn get_pressure(&mut self) -> Result<u32, E> {
        let mut buff: [u8; 4] = [0; 4];
        self.i2c.write_read(self.address, &[Register::PRESS_MSB.addr()], &mut buff[0..3])?;
        Ok(u32::from_be_bytes(buff) >> 12)
    }
}
