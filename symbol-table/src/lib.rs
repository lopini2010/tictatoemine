// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! The symbol table for a Leo program.
//!
//! This module contains the [`SymbolTable`] type, an abstract data type that tracks the current
//! bindings for functions and circuits in a Leo program.
//!
//! A new [`Symbol Table`] type can be created from a reference to a [`LeoAst`].
//! A [`Symbol Table`] type can be used to create a new [`TypeInference`] type.

#[macro_use]
extern crate thiserror;

pub mod attributes;
pub use self::attributes::*;

pub mod errors;
pub use self::errors::*;

pub mod imports;
pub use self::imports::*;

pub mod symbol_table;
pub use self::symbol_table::*;

pub mod types;
pub use self::types::*;
