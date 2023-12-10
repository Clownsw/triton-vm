use std::ops::Index;
use std::ops::IndexMut;

use color_eyre::eyre::bail;
use color_eyre::eyre::Result;

use triton_vm::instruction::*;
use triton_vm::op_stack::NumberOfWords::*;
use triton_vm::op_stack::*;

/// A helper “shadow stack” mimicking the behavior of the actual stack. Helps debugging programs
/// written for Triton VM by (manually set) type hints next to stack elements.
#[derive(Debug, Clone)]
pub(crate) struct TypeHintStack {
    pub type_hints: Vec<Option<ElementTypeHint>>,
}

/// A hint about the type of a single stack element. Helps debugging programs written for Triton VM.
/// **Does not enforce types.**
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementTypeHint {
    /// The name of the type. See [`TypeHint`][type_hint] for details.
    ///
    /// [type_hint]: TypeHint
    pub type_name: Option<String>,

    /// The name of the variable. See [`TypeHint`][type_hint] for details.
    ///
    /// [type_hint]: TypeHint
    pub variable_name: String,

    /// The index of the element within the type. For example, if the type is `Digest`, then this
    /// could be `0` for the first element, `1` for the second element, and so on.
    ///
    /// Does not apply to types that are not composed of multiple [`BFieldElement`][bfe]s, like `u32` or
    /// [`BFieldElement`][bfe] itself.
    ///
    /// [bfe]: triton_vm::BFieldElement
    pub index: Option<usize>,
}

impl TypeHintStack {
    pub fn new() -> Self {
        let type_hints = vec![None; NUM_OP_STACK_REGISTERS];
        let mut stack = Self { type_hints };

        let program_hash_type_hint = TypeHint {
            type_name: Some("Digest".to_string()),
            variable_name: "program_digest".to_string(),
            starting_index: 11,
            length: triton_vm::Digest::default().0.len(),
        };
        stack.apply_type_hint(&program_hash_type_hint).unwrap();
        stack
    }

    pub fn len(&self) -> usize {
        self.type_hints.len()
    }

    pub fn apply_type_hint(&mut self, type_hint: &TypeHint) -> Result<()> {
        let type_hint_range_end = type_hint.starting_index + type_hint.length;
        if type_hint_range_end > self.type_hints.len() {
            bail!("the op stack is not large enough to apply the given type hint");
        }

        let element_type_hint_template = ElementTypeHint {
            type_name: type_hint.type_name.clone(),
            variable_name: type_hint.variable_name.clone(),
            index: None,
        };

        if type_hint.length <= 1 {
            let insertion_index = self.len() - type_hint.starting_index - 1;
            self[insertion_index] = Some(element_type_hint_template);
            return Ok(());
        }

        let stack_indices = type_hint.starting_index..type_hint_range_end;
        for (index_in_variable, stack_index) in stack_indices.enumerate() {
            let mut element_type_hint = element_type_hint_template.clone();
            element_type_hint.index = Some(index_in_variable);
            let insertion_index = self.len() - stack_index - 1;
            self[insertion_index] = Some(element_type_hint);
        }
        Ok(())
    }

    pub fn mimic_instruction(&mut self, instruction: Option<Instruction>) {
        let Some(instruction) = instruction else {
            return;
        };
        match instruction {
            Instruction::Pop(n) => _ = self.pop_n(n),
            Instruction::Push(_) => self.push(None),
            Instruction::Divine(n) => self.extend_by(n),
            Instruction::Dup(st) => self.dup(st),
            Instruction::Swap(st) => self.swap_top_with(st),
            Instruction::Halt => (),
            Instruction::Nop => (),
            Instruction::Skiz => _ = self.pop(),
            Instruction::Call(_) => (),
            Instruction::Return => (),
            Instruction::Recurse => (),
            Instruction::Assert => _ = self.pop(),
            Instruction::ReadMem(n) => self.read_mem(n),
            Instruction::WriteMem(n) => self.write_mem(n),
            Instruction::Hash => _ = self.pop_n(N5),
            Instruction::DivineSibling => self.divine_sibling(),
            Instruction::AssertVector => _ = self.pop_n(N5),
            Instruction::SpongeInit => (),
            Instruction::SpongeAbsorb => self.sponge_absorb(),
            Instruction::SpongeSqueeze => self.sponge_squeeze(),
            Instruction::Add => self.binop(),
            Instruction::Mul => self.binop(),
            Instruction::Invert => self.unop(),
            Instruction::Eq => self.binop(),
            Instruction::Split => self.split(),
            Instruction::Lt => self.binop(),
            Instruction::And => self.binop(),
            Instruction::Xor => self.binop(),
            Instruction::Log2Floor => self.unop(),
            Instruction::Pow => self.binop(),
            Instruction::DivMod => self.div_mod(),
            Instruction::PopCount => self.unop(),
            Instruction::XxAdd => _ = self.pop_n(N3),
            Instruction::XxMul => _ = self.pop_n(N3),
            Instruction::XInvert => self.x_invert(),
            Instruction::XbMul => self.xb_mul(),
            Instruction::ReadIo(n) => self.extend_by(n),
            Instruction::WriteIo(n) => _ = self.pop_n(n),
        }
    }

    fn push(&mut self, element_type_hint: Option<ElementTypeHint>) {
        self.type_hints.push(element_type_hint);
    }

    fn extend_by(&mut self, n: NumberOfWords) {
        self.type_hints.extend(vec![None; n.into()]);
    }

    fn swap_top_with(&mut self, index: OpStackElement) {
        let top_index = self.len() - 1;
        let other_index = self.len() - usize::from(index) - 1;
        self.type_hints.swap(top_index, other_index);
    }

    fn pop(&mut self) -> Option<ElementTypeHint> {
        self.type_hints.pop().flatten()
    }

    fn pop_n(&mut self, n: NumberOfWords) -> Vec<Option<ElementTypeHint>> {
        let start_index = self.len() - usize::from(n);
        self.type_hints.drain(start_index..).collect()
    }

    fn dup(&mut self, st: OpStackElement) {
        let dup_index = self.len() - usize::from(st) - 1;
        self.push(self[dup_index].clone());
    }

    fn read_mem(&mut self, n: NumberOfWords) {
        let ram_pointer = self.pop();
        self.extend_by(n);
        self.push(ram_pointer);
    }

    fn write_mem(&mut self, n: NumberOfWords) {
        let ram_pointer = self.pop();
        let _ = self.pop_n(n);
        self.push(ram_pointer);
    }

    fn divine_sibling(&mut self) {
        self.pop_n(N5);
        self.extend_by(N5);
        self.extend_by(N5);
    }

    fn sponge_absorb(&mut self) {
        self.pop_n(N5);
        self.pop_n(N5);
    }

    fn sponge_squeeze(&mut self) {
        self.extend_by(N5);
        self.extend_by(N5);
    }

    fn unop(&mut self) {
        self.pop();
        self.push(None);
    }

    fn binop(&mut self) {
        self.pop_n(N2);
        self.push(None);
    }

    fn split(&mut self) {
        self.pop();
        self.extend_by(N2);
    }

    fn div_mod(&mut self) {
        self.pop_n(N2);
        self.extend_by(N2);
    }

    fn x_invert(&mut self) {
        self.pop_n(N3);
        self.extend_by(N3);
    }

    fn xb_mul(&mut self) {
        self.pop_n(N4);
        self.extend_by(N3);
    }
}

impl Default for TypeHintStack {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for TypeHintStack {
    type Output = Option<ElementTypeHint>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.type_hints[index]
    }
}

impl IndexMut<usize> for TypeHintStack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.type_hints[index]
    }
}

#[cfg(test)]
mod tests {
    use assert2::assert;
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn default_type_hint_stack_is_as_long_as_default_actual_stack() {
        let actual_stack_length = TypeHintStack::default().len();
        let expected_stack_length = OpStack::new(Default::default()).stack.len();
        assert!(expected_stack_length == actual_stack_length);
    }

    #[test]
    fn type_hint_stack_grows_and_shrinks_like_actual_stack() {
        let initial_length = TypeHintStack::default().len() as i32;
        for instruction in ALL_INSTRUCTIONS {
            'arg: for argument in OpStackElement::iter() {
                let Ok(instruction) = instruction.change_arg(argument.into()) else {
                    continue 'arg;
                };
                let mut type_hint_stack = TypeHintStack::default();
                type_hint_stack.mimic_instruction(Some(instruction));

                let expected_stack_delta = instruction.op_stack_size_influence();
                let actual_stack_delta = type_hint_stack.len() as i32 - initial_length;
                assert!(expected_stack_delta == actual_stack_delta, "{instruction}");
            }
        }
    }
}
