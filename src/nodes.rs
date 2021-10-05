// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) mod btree;
pub(crate) mod rrb;

pub(crate) mod chunk {
  use crate::config::VECTOR_CHUNK_SIZE;
  use sp_sized_chunks as sc;

  pub(crate) type Chunk<A> = sc::sized_chunk::Chunk<A, VECTOR_CHUNK_SIZE>;
  pub(crate) const CHUNK_SIZE: usize = VECTOR_CHUNK_SIZE;
}
