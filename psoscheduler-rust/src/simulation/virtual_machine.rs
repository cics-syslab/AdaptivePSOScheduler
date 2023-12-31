use core::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::hash::{Hash, Hasher};

pub static VIRTUAL_MACHINE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct VirtualMachine {
  pub id: usize,
  pub data_center_id: usize,
  pub millions_of_instructions_per_second: u32,
  pub active_state_joules_per_million_instructions: f32,
}

impl VirtualMachine {
  pub fn new(millions_of_instructions_per_second: u32, active_state_watts: f32) -> VirtualMachine {
    VirtualMachine { 
      id: VIRTUAL_MACHINE_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
      data_center_id: 0,
      millions_of_instructions_per_second: millions_of_instructions_per_second,
      active_state_joules_per_million_instructions: active_state_watts / (millions_of_instructions_per_second as f32),
    }
  }

  pub fn get_idle_state_joules_per_million_instructions(&self) -> f32 {
    self.active_state_joules_per_million_instructions * 0.6
  }
}

impl fmt::Display for VirtualMachine {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "(VM{}: {} mips, {} J/MI)", self.id, self.millions_of_instructions_per_second, self.active_state_joules_per_million_instructions)
  }
}

impl Hash for VirtualMachine {
    fn hash<H: Hasher>(&self, state: &mut H) {
      self.id.hash(state);
    }
}

impl PartialEq for VirtualMachine {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl Eq for VirtualMachine { }