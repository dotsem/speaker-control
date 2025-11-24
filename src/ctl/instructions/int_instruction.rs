use std::sync::Arc;
use super::instruction::Instruction;
use super::super::serial_port_manager::SerialPortManager;


macro_rules! out_of_range_error {
    ($self:expr, $value:expr) => {
        Err(format!(
            "Value {} out of range [{}, {}]",
            $value, $self.min, $self.max
        )
        .into())
    };
}

pub struct IntInstruction {
    value: i32,
    base: Instruction,
    min: i32,
    max: i32,
    step: i32,
}

impl IntInstruction {
    pub fn new(
        attr: &'static str,
        description: Option<&'static str>,
        min: i32,
        max: i32,
        step: i32,
        manager: Arc<SerialPortManager>,
    ) -> IntInstruction {
        let base = Instruction::new(attr, description, manager);
        IntInstruction {
            value: base.get().unwrap().parse().unwrap(),
            base,
            min,
            max,
            step,
        }
    }

    pub fn get(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let cmd = format!("GET {}", self.base.attr);
        let response = self.base.manager.send_command(&cmd)?;
        Ok(response.trim().parse()?)
    }

    pub fn set(&mut self, value: i32) -> Result<String, Box<dyn std::error::Error>> {
        if value < self.min || value > self.max {
            return out_of_range_error!(self, value);
        }
        let cmd = format!("SET {} {}", self.base.attr, value);
        let response = self.base.manager.send_command(&cmd)?;
        self.value = value;
        Ok(response)
    }

    pub fn inc(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        if (self.value + self.step) > self.max {
            return out_of_range_error!(self, self.value + self.step);
        }
        let cmd = format!("SET {} {}", self.base.attr, self.value + self.step);
        let response = self.base.manager.send_command(&cmd)?;
        self.value += self.step;
        Ok(response)
    }

    pub fn dec(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        if (self.value - self.step) < self.min {
            return out_of_range_error!(self, self.value - self.step);
        }
        let cmd = format!("SET {} {}", self.base.attr, self.value - self.step);
        let response = self.base.manager.send_command(&cmd)?;
        self.value -= self.step;
        Ok(response)
    }
}