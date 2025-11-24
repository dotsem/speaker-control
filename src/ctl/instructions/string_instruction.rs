use std::sync::Arc;
use super::instruction::Instruction;
use super::super::serial_port_mananger::SerialPortManager;

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
