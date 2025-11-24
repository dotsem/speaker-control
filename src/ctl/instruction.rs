use super::serial_port_mananger::SerialPortManager;
use std::sync::Arc;

pub struct Instruction {
    attr: &'static str,
    description: Option<&'static str>,
    manager: Arc<SerialPortManager>,
}

impl Instruction {
    pub fn new(
        attr: &'static str,
        description: Option<&'static str>,
        manager: Arc<SerialPortManager>,
    ) -> Instruction {
        Instruction {
            attr,
            description,
            manager,
        }
    }
    pub fn attr(&self) -> &str {
        &self.attr
    }

    pub fn description(&self) -> &str {
        self.description.as_deref().unwrap_or("No description")
    }

    pub fn get(&self) -> Result<String, Box<dyn std::error::Error>> {
        let cmd = format!("GET {}", self.attr);
        let response = self.manager.send_command(&cmd)?;
        Ok(response.replace(&(self.attr.to_string() + " "), ""))
    }
}

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

pub struct BoolInstruction {
    value: bool,
    base: Instruction,
}

impl BoolInstruction {
    pub fn new(
        attr: &'static str,
        description: Option<&'static str>,
        manager: Arc<SerialPortManager>,
    ) -> BoolInstruction {
        let base = Instruction::new(attr, description, manager);
        BoolInstruction {
            value: BoolInstruction::state_to_bool(base.get().unwrap().as_str()).unwrap(),
            base,
        }
    }

    fn bool_to_state(value: bool) -> String {
        if value {
            "ON".to_string()
        } else {
            "OFF".to_string()
        }
    }

    fn state_to_bool(state: &str) -> Result<bool, Box<dyn std::error::Error>> {
        match state.trim() {
            "ON" => Ok(true),
            "OFF" => Ok(false),
            _ => Err(format!("Invalid boolean state: {}", state).into()),
        }
    }

    pub fn get(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let cmd = format!("GET {}", self.base.attr);
        let response = self.base.manager.send_command(&cmd)?;
        Ok(BoolInstruction::state_to_bool(response.as_str())?)
    }

    pub fn set(&mut self, value: bool) -> Result<String, Box<dyn std::error::Error>> {
        let cmd = format!(
            "SET {} {}",
            self.base.attr,
            BoolInstruction::bool_to_state(value)
        );
        let response = self.base.manager.send_command(&cmd)?;
        self.value = value;
        Ok(response)
    }

    pub fn toggle(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        if self.value {
            self.set(false)
        } else {
            self.set(true)
        }
    }
}

pub struct StringInstruction {
    value: String,
    base: Instruction,
    allowed_values: &'static [&'static str],
}

impl StringInstruction {
    pub fn new(
        attr: &'static str,
        description: Option<&'static str>,
        allowed_values: &'static [&'static str],
        manager: Arc<SerialPortManager>,
    ) -> StringInstruction {
        let base = Instruction::new(attr, description, manager);
        StringInstruction {
            value: base.get().unwrap(),
            base,
            allowed_values,
        }
    }

    pub fn set(&mut self, value: &str) -> Result<String, Box<dyn std::error::Error>> {
        if !self.allowed_values.contains(&value) {
            return Err(format!("Value {} not allowed", value).into());
        }

        let cmd = format!("SET {} {}", self.base.attr, value);
        let response = self.base.manager.send_command(&cmd)?;
        self.value = value.to_string();
        Ok(response)
    }
}
