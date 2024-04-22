use std::fs;

pub struct Emitter {
    full_path: String,
    header: String,
    code: String,
}

impl Emitter {

    pub fn new(full_path: &str) -> Self {
        return Emitter {
            full_path: full_path.to_string(),
            header: String::new(),
            code: String::new(),
        };
    }

    pub fn emit(&mut self, code: &str) {
        self.code += code;
    }

    pub fn emit_line(&mut self, code: &str) {
        self.code += &format!("{code}\n");
    }

    pub fn header_line(&mut self, code: &str) {
        self.header += &format!("{code}\n");
    }

    pub fn write_file(&self) {
        let _ = fs::write(self.full_path.clone(), self.header.clone() + &self.code);
    }
}