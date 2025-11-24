use std::sync::{Arc, LazyLock};
use super::serial_port_manager::SerialPortManager;
use super::instructions::{bool_instruction::BoolInstruction, int_instruction::IntInstruction, string_instruction::StringInstruction};

pub struct ControllerBinding {
    pub standby: BoolInstruction,
    pub source_select: StringInstruction,
    pub volume: IntInstruction,
    pub balance: IntInstruction,
    pub eq_bass: IntInstruction,
    pub eq_treble: IntInstruction
}

impl ControllerBinding {
    pub fn init() -> ControllerBinding {
        ControllerBinding {
            standby: BoolInstruction::new("STANDBY", Some("Standby"), Arc::clone(&MANAGER)),
            source_select: StringInstruction::new("SELECT", Some("Select a source"), &["A", "B", "AB", "WL"], Arc::clone(&MANAGER)),
            volume: IntInstruction::new("SDQLVL", Some("Volume level"), -64, 0, 1, Arc::clone(&MANAGER)),
            balance: IntInstruction::new("BALANCE", Some("Left - right balance"), -31, 31, 1, Arc::clone(&MANAGER)),
            eq_bass: IntInstruction::new("EQBASS", Some("Equalizer bass"), -14, 14, 2, Arc::clone(&MANAGER)),
            eq_treble: IntInstruction::new("EQTREB", Some("Equalizer treble"), -14, 14, 2, Arc::clone(&MANAGER))
        }
    }
}

static MANAGER: LazyLock<Arc<SerialPortManager>> = LazyLock::new(|| {
    Arc::new(SerialPortManager::new("/dev/ttyUSB0", 19200).unwrap())
});

pub static VOLUME: LazyLock<IntInstruction> = LazyLock::new(|| {
    IntInstruction::new("SDQLVL", Some("Volume level"), -64, 0, 1, Arc::clone(&MANAGER))
});

pub static BALANCE: LazyLock<IntInstruction> = LazyLock::new(|| {
    IntInstruction::new("BALANCE", Some("Left - right balance"), -31, 31, 1, Arc::clone(&MANAGER))
});

pub static SOURCE_SELECT: LazyLock<StringInstruction> = LazyLock::new(|| {
    StringInstruction::new("SELECT", Some("Select a source"), &["A", "B", "AB", "WL"], Arc::clone(&MANAGER))
});

pub static EQ_BASS: LazyLock<IntInstruction> = LazyLock::new(|| {
    IntInstruction::new("EQBASS", Some("Equalizer bass"), -14, 14, 2, Arc::clone(&MANAGER))
});

pub static EQ_TREBLE: LazyLock<IntInstruction> = LazyLock::new(|| {
    IntInstruction::new("EQTREB", Some("Equalizer treble"), -14, 14, 2, Arc::clone(&MANAGER))
});

pub static STANDBY: LazyLock<BoolInstruction> = LazyLock::new(|| {
    BoolInstruction::new("STANDBY", Some("Standby"), Arc::clone(&MANAGER))
});

pub static AUTO_POWER_OFF: LazyLock<IntInstruction> = LazyLock::new(|| {
    IntInstruction::new("AUTOPW", Some("Auto power off"), 0, 30, 1, Arc::clone(&MANAGER))
});

pub static AUTO_POWER_TRIGGER_LEVEL: LazyLock<IntInstruction> = LazyLock::new(|| {
    IntInstruction::new("AUPTRG", Some("Auto power trigger level"), 0, 9, 1, Arc::clone(&MANAGER))
});
