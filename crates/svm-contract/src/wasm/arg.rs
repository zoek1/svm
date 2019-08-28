use std::convert::TryFrom;

/// `WasmArgType` defines 4 types:
/// * I32 - Represents a 4-byte integer argument.
///
/// * I64 - Represents a 8-byte integer argument.
///
/// * Fixed - Represents a fixed-size array of bytes.
///   For example: `Address` is a 32 bytes fixed-array.
///
/// * Slice - Represents a blob of data, not known ahead.
///   For example: `String`
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WasmArgType {
    I32,
    I64,
    Fixed,
    Slice,
}

/// The actual value of `wasm` argument.
///
/// * I32 - A 4-byte integer.
///
/// * I64 - A 8-byte integer.
///
/// * Fixed - A tuple of `(WasmIntType, Vec<u8>)`
///     * `WasmIntType` - Represents the integer type of start offset in wasm linear-memory the copied fixed-array starts.
///
///     * `Vec<u8>` - The bytes of the fixed-array.
///
/// * Slice - A tuple of `(WasmInt, WasmInt, Vec<u8>)`
///     * `WasmIntType` (the left one) - Represents the integer type of start offset of wasm linear-memory the slice starts.
///
///     * `WasmIntType` (the right one) - Represents the integer type of slice bytes-length.
///
///     * `Vec<u8>` - The bytes of the slice.
///
///  The actual values of these `WasmArgType` **aren't** part of the execution transaction raw data.
///  These values will be inferred as part of preparing the wasm instance for execution,
///  When initializing the wasm instance memory and passing the call arguments to the invoked function.
#[derive(Clone, PartialEq, Debug)]
pub enum WasmArgValue {
    I32(u32),
    I64(u64),
    Fixed(WasmIntType, Vec<u8>),
    Slice(WasmIntType, WasmIntType, Vec<u8>),
}

/// Represents a `wasm` Integer type
/// * I32 - Represents a 4-byte integer argument.
/// * I64 - Represents a 8-byte integer argument.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WasmIntType {
    I32,
    I64,
}

/// Converts `WasmArgType` to its numeric representation
impl Into<u8> for WasmArgType {
    fn into(self) -> u8 {
        match self {
            WasmArgType::I32 => 0,
            WasmArgType::I64 => 1,
            WasmArgType::Fixed => 2,
            WasmArgType::Slice => 3,
        }
    }
}

/// Converts `WasmIntType` to its numeric representation
impl Into<u8> for &WasmIntType {
    fn into(self) -> u8 {
        match *self {
            WasmIntType::I32 => 0,
            WasmIntType::I64 => 1,
        }
    }
}

pub enum WasmArgTypeError {
    UnsupportedType(u8),
}

/// Converts `WasmArgType` to its numeric representation
impl TryFrom<u8> for WasmArgType {
    type Error = WasmArgTypeError;

    fn try_from(value: u8) -> Result<WasmArgType, WasmArgTypeError> {
        match value {
            0 => Ok(WasmArgType::I32),
            1 => Ok(WasmArgType::I64),
            2 => Ok(WasmArgType::Fixed),
            3 => Ok(WasmArgType::Slice),
            _ => Err(WasmArgTypeError::UnsupportedType(value)),
        }
    }
}
