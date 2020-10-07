// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Cumulus related primitive types and traits.

#![cfg_attr(not(feature = "std"), no_std)]

pub use polkadot_core_primitives as relay_chain;
pub use polkadot_core_primitives::DownwardMessage;
/// A generic upward message from a Parachain to the Relay Chain.
///
/// It is "generic" in such a way, that the actual message is encoded in the `data` field.
/// Besides the `data` it also holds the `origin` of the message.
pub use polkadot_parachain::primitives::UpwardMessage as GenericUpwardMessage;
pub use polkadot_parachain::primitives::{
	Id as ParaId, ParachainDispatchOrigin as UpwardMessageOrigin,
};

#[cfg(feature = "std")]
pub mod genesis;
pub mod validation_function_params;
pub mod xcmp;

use codec::{Decode, Encode};
use sp_runtime::traits::Block as BlockT;

/// Identifiers and types related to Cumulus Inherents
pub mod inherents {
	use sp_inherents::InherentIdentifier;

	/// Inherent identifier for downward messages.
	pub const DOWNWARD_MESSAGES_IDENTIFIER: InherentIdentifier = *b"cumdownm";

	/// The type of the inherent downward messages.
	pub type DownwardMessagesType = sp_std::vec::Vec<crate::DownwardMessage>;

	/// The identifier for the `validation_function_params` inherent.
	pub const VALIDATION_FUNCTION_PARAMS_IDENTIFIER: InherentIdentifier = *b"valfunp0";
	/// The type of the inherent.
	pub type ValidationFunctionParamsType =
		crate::validation_function_params::ValidationFunctionParams;
}

/// Well known keys for values in the storage.
pub mod well_known_keys {
	/// The storage key for the upward messages.
	///
	/// The upward messages are stored as SCALE encoded `Vec<GenericUpwardMessage>`.
	pub const UPWARD_MESSAGES: &'static [u8] = b":cumulus_upward_messages:";

	/// Current validation function parameters.
	pub const VALIDATION_FUNCTION_PARAMS: &'static [u8] = b":cumulus_validation_function_params";

	/// Code upgarde (set as appropriate by a pallet).
	pub const NEW_VALIDATION_CODE: &'static [u8] = b":cumulus_new_validation_code";

	/// The storage key for the processed downward messages.
	///
	/// The value is stored as SCALE encoded `u32`.
	pub const PROCESSED_DOWNWARD_MESSAGES: &'static [u8] = b":cumulus_processed_downward_messages:";
}

/// Something that should be called when a downward message is received.
#[impl_trait_for_tuples::impl_for_tuples(30)]
pub trait DownwardMessageHandler {
	/// Handle the given downward message.
	fn handle_downward_message(msg: &DownwardMessage);
}

/// Something that can send upward messages.
pub trait UpwardMessageSender<UpwardMessage> {
	/// Send an upward message to the relay chain.
	///
	/// Returns an error if sending failed.
	fn send_upward_message(msg: &UpwardMessage, origin: UpwardMessageOrigin) -> Result<(), ()>;
}

/// The head data of the parachain, stored in the relay chain.
#[derive(Decode, Encode, Debug)]
pub struct HeadData<Block: BlockT> {
	pub header: Block::Header,
}
