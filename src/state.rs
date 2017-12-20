use Value;
use Operation;
use operators;
use operators::Operator;
use serde_json;

#[derive(Serialize, Debug, Clone)]
pub struct InternalState {
  pub register: Option<Value>,
	pub input_tape: Vec<Value>,
	pub output_tape: Vec<Value>,
	pub memory: Vec<Option<Value>>,
	pub instruction_counter: usize
}


macro_rules! apply_operation {
	($self: ident, $operator:expr) => ({
		let op = $operator;
    match op.apply_to($self) {
			Ok(()) => {
				if !op.changes_instruction_counter() {
					$self.instruction_counter += 1;
				}

				Ok(())
			},
			Err(reason) => {
				Err(reason)
			}
		}
	})
}

impl InternalState {

    pub fn new(register: Option<Value>, counter: usize) -> InternalState {
        InternalState {
	    register: register,
	    input_tape: vec!(),
	    output_tape: vec!(),
	    memory: vec!(),
	    instruction_counter: counter
	}
    }

	pub fn apply(&mut self, op: Operation) -> Result<(), String> {
		match op {
			Operation::Add{cell: _cell} => {
				apply_operation!(self, operators::add::AddOp{cell: _cell})
			},
			Operation::Sub{cell: _cell} => {
				apply_operation!(self, operators::sub::SubOp{cell: _cell})
			},
			Operation::Inbox => {
				apply_operation!(self, operators::inbox::InboxOp{})
			},
			Operation::Outbox => {
				apply_operation!(self, operators::outbox::OutboxOp{})
			},
			Operation::CopyFrom{cell: _cell} => {
				apply_operation!(self, operators::copyfrom::CopyFromOp{cell: _cell})
			},
			Operation::CopyTo{cell: _cell} => {
				apply_operation!(self, operators::copyto::CopyToOp{cell: _cell})
			},
			Operation::Label => {
				apply_operation!(self, operators::jump::LabelOp)
			},
			Operation::Jump{next_operation: _next_op} => {
				apply_operation!(self, operators::jump::JumpOp{next_operation: _next_op})
			},
			Operation::JumpEqualsZero{next_operation: _next_op} => {
				apply_operation!(self, operators::jump::JumpEqualsZeroOp{next_operation: _next_op})
			}
			Operation::JumpNegative{next_operation: _next_op} => {
				apply_operation!(self, operators::jump::JumpNegativeOp{next_operation: _next_op})
			}
			Operation::BumpPlus{cell: _cell} => {
				apply_operation!(self, operators::bump::BumpPlusOp{cell: _cell})
			}
			Operation::BumpMinus{cell: _cell} => {
				apply_operation!(self, operators::bump::BumpMinusOp{cell: _cell})
			}
		}
	}
}
