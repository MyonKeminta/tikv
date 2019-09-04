// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use super::{Callback, Result};
use kvproto::kvrpcpb::Context;

pub trait GCWorker: Send {
    fn async_gc(&self, ctx: Context, safe_point: u64, callback: Callback<()>) -> Result<()>;
}

#[derive(Clone)]
pub struct DummyGCWorker;

impl GCWorker for DummyGCWorker {
    fn async_gc(&self, _: Context, _: u64, callback: Callback<()>) -> Result<()> {
        callback(Ok(()));
        Ok(())
    }
}
