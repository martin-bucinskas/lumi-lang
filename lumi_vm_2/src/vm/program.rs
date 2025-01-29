use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub id: Uuid,
    pub bytecode: Vec<u8>,
//    pub symbols: HashMap<usize, Symbol>,
    pub program_size: usize,
    pub debug: bool,
}

impl Program {
    pub fn new() -> Self {
        Program {
            id: Uuid::new_v4(),
            bytecode: vec![],
            program_size: 0,
            debug: false,
        }
    }

    pub fn load(&mut self) -> Self {
        self.program_size = self.bytecode.len();
        self.clone()
    }
}