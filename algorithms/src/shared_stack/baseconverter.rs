use super::stack::Stack;

pub struct BaseConverter {
    array_elemets: u32,
}

impl BaseConverter {
    pub fn new() -> Self {
        Self { array_elemets: 16 }
    }
    fn set_max_base_size(&mut self, value: u32) {
        self.array_elemets = value;
    }
    fn generate_array(&mut self) -> Vec<String> {
        let vec = (0..self.array_elemets)
            .map(|n| format!("{:X}", n))
            .collect::<Vec<_>>();
        vec
    }
    pub fn base_converter(&mut self, mut value: u32, base: u32) -> String {
        //value Should be decimal only
        self.set_max_base_size(base);
        let digits = self.generate_array();
        let mut my_stack: Stack<String> = Stack::new();
        while value > 0 {
            my_stack.push((value % self.array_elemets).to_string());
            value = value / self.array_elemets;
        }

        let mut base_str = "".to_string();
        while !my_stack.is_empty() {
            let rem = my_stack.pop().unwrap();
            let index: usize = rem.parse().unwrap();
            base_str += &digits[index];
        }
        return base_str;
    }
}
