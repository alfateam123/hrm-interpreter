use state;

// define the Operator trait: every Operator
// can modify the internal state
pub trait Operator {
	// rust compiler issue 35203.
	fn apply_to(&self, /*mut*/  s: &mut state::InternalState) -> Result<(), String>;
	fn changes_instruction_counter(&self) -> bool;
}

pub mod add;
pub mod sub;
pub mod inbox;
pub mod outbox;
pub mod copyfrom;
pub mod copyto;
pub mod jump;
pub mod bump;
