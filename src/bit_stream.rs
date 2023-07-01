#[derive(Default)]
pub struct BitWriter {
    buffer: Vec<u64>,
    pub(crate) bit_position: u64,
}

impl BitWriter {
    pub fn write_bit(&mut self, bit: bool) {
        if self.bit_position % 64 == 0 {
            self.buffer.push(0);
        }

        if bit {
            let buffer_index = (self.bit_position / 64) as usize;
            let buffer_offset = self.bit_position % 64;
            self.buffer[buffer_index] |= 1 << buffer_offset;
        }

        self.bit_position += 1;
    }

    pub fn write_bits_vec(&mut self, bits: &Vec<bool>) {
        for bit in bits {
            self.write_bit(bit.clone());
        }
    }

    pub fn write_u8(&mut self, n: u8) {
        for i in 0..8 {
            self.write_bit((n >> i) & 1 == 1);
        }
    }

    pub fn write_u16(&mut self, n: u16) {
        for i in 0..16 {
            self.write_bit((n >> i) & 1 == 1);
        }
    }

    pub fn print(&self) {

        for chunk in &self.buffer {
            println!("{chunk:064b}");
        }

    }

    pub fn to_reader(&self) -> BitReader { //Perform a flush
        BitReader::from_writer(&self)
    }
}

#[derive(Default)]
pub struct BitReader {
    pub(crate) buffer: Vec<u64>,
    pub bit_position: usize,
}
impl BitReader {
    pub fn from_writer(writer: &BitWriter) -> BitReader {
        BitReader {
            buffer: writer.buffer.clone(),
            bit_position: 0
        }
    }

    pub fn progress(&mut self, len: usize) {
        self.bit_position += len;
    }

    pub fn read_bit(&mut self) -> Option<bool> {
        let buffer_index = self.bit_position / 64;
        let buffer_offset = self.bit_position % 64;
        self.bit_position += 1;

        if let Some(buffer) = self.buffer.get(buffer_index) {
            return Some((buffer >> buffer_offset & 1) == 1)
        }

        None
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let mut u16: u16 = 0;

        for _ in 0..16 {
            if let Some(bit) = self.read_bit() {
                u16 = (u16 >> 1) | ((bit as u16) << 15);
            } else {
                return None;
            }
        }

        Some(u16)
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        let mut u8: u8 = 0;

        for _ in 0..8 {
            if let Some(bit) = self.read_bit() {
                u8 = (u8 >> 1) | ((bit as u8) << 7);
            } else {
                return None;
            }
        }

        Some(u8)
    }

    pub fn read_bits_vec(&mut self, len: usize) -> Option<Vec<bool>> {
        let mut ret: Vec<bool> = Vec::with_capacity(len);

        for _ in 0..len {
            if let Some(bit) = self.read_bit() {
                ret.push(bit)
            } else {
                return None
            }
        }

        Some(ret)
    }
}