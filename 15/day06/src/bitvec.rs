use core::fmt;

pub struct Bitvec {
    size: u32,
    data: Vec<u32>,
}

impl Bitvec {
    pub fn new(size: u32) -> Bitvec {
        let n = size / 32 + (if size % 32 == 0 { 0 } else { 1 });
        Bitvec {
            size,
            data: vec![0; n.try_into().unwrap()],
        }
    }

    pub fn set(&mut self, idx: u32) {
        let (array_idx, offset) = array_offset_pair(idx);

        self.data[array_idx] = self.data[array_idx] | (1u32 << (31 - offset))
    }

    pub fn unset(&mut self, idx: u32) {
        let (array_idx, offset) = array_offset_pair(idx);

        self.data[array_idx] = self.data[array_idx] & !(1u32 << (31 - offset))
    }

    pub fn toggle(&mut self, idx: u32) {
        if self.is_set(idx) {
            self.unset(idx)
        } else {
            self.set(idx)
        }
    }

    pub fn is_set(&self, idx: u32) -> bool {
        let (array_idx, offset) = array_offset_pair(idx);

        self.data[array_idx] & (1u32 << (31 - offset)) > 0
    }
}

fn array_offset_pair(idx: u32) -> (usize, u32) {
    ((idx / 32).try_into().unwrap(), idx % 32)
}

impl fmt::Debug for Bitvec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut binary_vec = String::with_capacity(self.data.len() * 9);
        binary_vec.push_str("[\n");
        for v in &self.data {
            binary_vec.push_str(format!("\t{:#034b}\n", v).as_str());
        }
        binary_vec.push(']');
        f.debug_struct("Bitvec")
            .field("size", &self.size)
            .field("data", &format_args!("{}", binary_vec))
            .finish()
    }
}
