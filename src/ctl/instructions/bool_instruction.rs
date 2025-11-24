use std::sync::Arc;
use super::instruction::Instruction;
use super::super::serial_port_mananger::SerialPortManager;


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