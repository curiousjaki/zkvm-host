use std::str::FromStr;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}
impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Operation::Add),
            "sub" => Ok(Operation::Sub),
            "mul" => Ok(Operation::Mul),
            "div" => Ok(Operation::Div),
            _ => Err(()),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct OperationRequest {
    pub a: f64,
    pub b: f64,
    pub operation: Operation,
}

impl OperationRequest {
    pub fn compute(&self) -> f64 {
        match self.operation {
            Operation::Add => self.a + self.b,
            Operation::Sub => self.a - self.b,
            Operation::Mul => self.a * self.b,
            Operation::Div => self.a / self.b,
        }
    }
}
