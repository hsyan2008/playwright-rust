use crate::imp::{core::*, prelude::*};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct Browser {
    channel: ChannelOwner
}

impl RemoteObject for Browser {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}
