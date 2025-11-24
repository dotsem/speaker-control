use super::super::serial_port_manager::SerialPortManager;
use std::sync::Arc;

pub struct Instruction {
    pub attr: &'static str,
    pub description: Option<&'static str>,
    pub manager: Arc<SerialPortManager>,
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
