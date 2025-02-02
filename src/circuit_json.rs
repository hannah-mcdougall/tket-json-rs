//! Contains structs for serializing and deserializing TKET circuits to and from
//! JSON.

use crate::opbox::OpBox;
use crate::optype::OpType;
use serde::{Deserialize, Serialize};

/// A register of locations sharing the same name.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Register(pub String, pub Vec<i64>);

/// A gate defined by a circuit.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CompositeGate {
    /// Name of the composite gate.
    pub name: String,
    /// Expressions corresponding to parameter values of the composite gate, if it has parameters.
    pub args: Vec<String>,
    /// The circuit defining the gate.
    pub definition: Box<SerialCircuit>,
}

/// A classical bit register.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitRegister {
    /// Name of the bit register.
    pub name: String,
    /// Number of bits in the register.
    pub size: u32,
}

/// A vector of booleans.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Bitstring {
    /// Vector of booleans.
    pub vec: Vec<bool>,
}

/// A 2D matrix.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Matrix<T = f64> {
    /// A 2D vector of complex numbers.
    pub data: Vec<Vec<(T, T)>>,
}

/// The units used in a [`ClassicalExp`].
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ClassicalExpUnit {
    /// Unsigned 32-bit integer.
    U32(u32),
    /// Register of locations.
    Register(Register),
    /// Register of bits.
    BitRegister(BitRegister),
    /// A nested classical expression.
    ClassicalExpUnit(ClassicalExp),
}

/// A box for holding classical expressions on Bits.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassicalExp {
    /// Arguments to the expression.
    pub args: Vec<ClassicalExpUnit>,
    /// The expression.
    pub op: String,
}

/// Decorates another op, adding a QASM-style classical condition.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Conditional {
    /// Internal operation to conditionally apply.
    pub op: Box<Operation>,
    /// Number of bits in the classical condition.
    pub width: u32,
    /// Value to compare against.
    pub value: u32,
}

/// Serializable operation descriptor.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Operation<P = String> {
    /// The type of operation.
    #[serde(rename = "type")]
    pub op_type: OpType,
    /// Number of input and output qubits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_qb: Option<u32>,
    /// Expressions for the parameters of the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<P>>,
    /// Internal box for the operation.
    #[serde(rename = "box")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_box: Option<OpBox>,
    /// The pre-computed signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Vec<String>>,
    /// A QASM-style classical condition for the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional: Option<Conditional>,
}

/// Operation applied in a circuit, with defined arguments.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Command<P = String> {
    /// The operation to be applied.
    pub op: Operation<P>,
    /// The arguments to the operation.
    pub args: Vec<Register>,
    /// Operation group identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opgroup: Option<String>,
}

/// A permutation of the elements of a register.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Permutation(pub Register, pub Register);

/// Pytket canonical serialized circuit
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SerialCircuit<P = String> {
    /// The name of the circuit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The global phase, as a symengine expression.
    pub phase: P,
    /// List of commands in the circuit.
    pub commands: Vec<Command<P>>,
    /// Input qubit registers.
    pub qubits: Vec<Register>,
    /// Input bit registers.
    pub bits: Vec<Register>,
    /// Implicit permutation of the output qubits.
    pub implicit_permutation: Vec<Permutation>,
}

impl<P> Operation<P> {
    /// Returns a default-initialized Operation with the given type.
    ///
    /// For optypes that require additional parameters, this may generate
    /// invalid operations.
    pub fn from_optype(op_type: OpType) -> Self {
        Self {
            op_type,
            n_qb: None,
            params: None,
            op_box: None,
            signature: None,
            conditional: None,
        }
    }
}
