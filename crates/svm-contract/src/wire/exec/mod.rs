//!      Exec Contract Wire Protocol Version 0.0.0.0
//!  -------------------------------------------------------
//!  |   proto    |                                        |
//!  |  version   |          contract address              |
//!  |  (4 bytes) |             (20 bytes)                 |
//!  |____________|________________________________________|
//!  |                                                     |
//!  |                  sender address                     |
//!  |                    (20 bytes)                       |
//!  |_____________________________________________________|
//!  |             |                                       |
//!  |  func name  |                                       |
//!  |   length    |          func name (UTF-8)            |
//!  |  (1 byte)   |                                       |
//!  |_____________|_______________________________________|
//!  |           |              |         |                |
//!  |  #args    |  arg #1 type |  arg #1 |    . . . .     |
//!  | (1 byte)  |  (1 byte)    |  value  |                |
//!  |___________|______________|_________|________________|
//!

mod build;
mod error;
mod field;
mod parse;

pub use build::WireTxBuilder;
pub use error::TransactionBuildError;
pub use parse::parse_transaction;
