use timekeeper::TimeKeeper;
use interrupt::InterruptState;
use debugger::Debugger;

/// State shared between various modules
pub struct SharedState {
    tk: TimeKeeper,
    debugger: Debugger,
    irq_state: InterruptState,
}

impl SharedState {
    pub fn new() -> SharedState {
        SharedState {
            tk: TimeKeeper::new(),
            debugger: Debugger::new(),
            irq_state: InterruptState::new(),
        }
    }

    pub fn tk(&mut self) -> &mut TimeKeeper {
        &mut self.tk
    }

    pub fn debugger(&mut self) -> &mut Debugger {
        &mut self.debugger
    }

    pub fn irq_state(&mut self) -> &mut InterruptState {
        &mut self.irq_state
    }
}
