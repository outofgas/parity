// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Generic trait for trie node encoding/decoding. Takes a `hashdb::Hasher` 
//! to parametrize the hashes used in the codec; takes a `stream_encoder::Stream` 
//! implementation to do streaming encoding.

use bytes::Bytes;
use hashdb::Hasher;
use node::Node;
use stream_encoder::Stream;

/// Trait for trie node encoding/decoding
pub trait NodeCodec<H: Hasher>: Sized {
	type E: ::std::error::Error;
	type S: Stream;

	/// Encode a Node to bytes (`Vec<u8>`).
	fn encode(&Node) -> Bytes;

	/// Decode bytes to a `Node`. Returns `Result` on failure.
	fn decode(data: &[u8]) -> Result<Node, Self::E>;

	/// Decode bytes to the `Hasher`s output type. Assumes 32 bytes long hashes! Returns `None` on failure.
	fn try_decode_hash(data: &[u8]) -> Option<H::Out>;

	// Check if the provided bytes correspond to the codecs "empty" node.
	fn is_empty_node(data: &[u8]) -> bool;
}