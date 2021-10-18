// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) use self::lock::Lock;

mod lock {
  use alloc::sync::Arc;
  use spin::mutex::{
    Mutex,
    MutexGuard,
  };

  /// Thread safe lock: just wraps a `Mutex`.
  pub(crate) struct Lock<A> {
    lock: Arc<Mutex<A>>,
  }

  impl<A> Lock<A> {
    pub(crate) fn new(value: A) -> Self {
      Lock { lock: Arc::new(Mutex::new(value)) }
    }

    #[inline]
    pub(crate) fn lock(&mut self) -> Option<MutexGuard<'_, A>> {
      Some(self.lock.lock())
    }
  }

  impl<A> Clone for Lock<A> {
    fn clone(&self) -> Self { Lock { lock: self.lock.clone() } }
  }
}
