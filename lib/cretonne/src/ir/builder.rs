//! Cretonne instruction builder.
//!
//! A `Builder` provides a convenient interface for inserting instructions into a Cretonne
//! function. Many of its methods are generated from the meta language instruction definitions.

use ir::{types, instructions};
use ir::{InstructionData, DataFlowGraph, Cursor};
use ir::{Opcode, Type, Inst, Value, Ebb, JumpTable, VariableArgs, SigRef, FuncRef};
use ir::immediates::{Imm64, Uimm8, Ieee32, Ieee64, ImmVector};
use ir::condcodes::{IntCC, FloatCC};

/// Instruction builder.
///
/// A `Builder` holds mutable references to a data flow graph and a layout cursor. It provides
/// convenience method for creating and inserting instructions at the current cursor position.
pub struct Builder<'a> {
    pub dfg: &'a mut DataFlowGraph,
    pub pos: &'a mut Cursor<'a>,
}

impl<'a> Builder<'a> {
    /// Create a new builder which inserts instructions at `pos`.
    /// The `dfg` and `pos.layout` references should be from the same `Function`.
    pub fn new(dfg: &'a mut DataFlowGraph, pos: &'a mut Cursor<'a>) -> Builder<'a> {
        Builder {
            dfg: dfg,
            pos: pos,
        }
    }

    /// Create and insert an EBB. Further instructions will be inserted into the new EBB.
    pub fn ebb(&mut self) -> Ebb {
        let ebb = self.dfg.make_ebb();
        self.insert_ebb(ebb);
        ebb
    }

    /// Insert an existing EBB at the current position. Further instructions will be inserted into
    /// the new EBB.
    pub fn insert_ebb(&mut self, ebb: Ebb) {
        self.pos.insert_ebb(ebb);
    }

    // Create and insert an instruction.
    // This method is used by the generated format-specific methods.
    fn insert_inst(&mut self, data: InstructionData) -> Inst {
        let inst = self.dfg.make_inst(data);
        self.pos.insert_inst(inst);
        inst
    }
}

// Include code generated by `meta/gen_instr.py`. This file includes `Builder` methods per
// instruction format and per opcode for inserting instructions.
include!(concat!(env!("OUT_DIR"), "/builder.rs"));
