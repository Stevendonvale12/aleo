// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

pub mod account;
pub use account::*;

#[cfg(not(feature = "wasm"))]
pub mod api;
#[cfg(not(feature = "wasm"))]
pub use api::*;

#[cfg(not(feature = "wasm"))]
pub mod program;
#[cfg(not(feature = "wasm"))]
pub use program::*;

#[cfg(test)]
#[cfg(not(feature = "wasm"))]
pub mod test_utils;
#[cfg(test)]
#[cfg(not(feature = "wasm"))]
pub use test_utils::*;
