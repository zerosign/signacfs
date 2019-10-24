use fuse::{
    FileAttr, FileSystem, FileType, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use std::ffi::OsStr;

pub trait AccessControl {
    fn check(&self, action: UserAction) -> Option<Policy>;
}
