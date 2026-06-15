use std::ffi::OsStr;
use std::path::{Component, Path};
use crate::parser::HAT_EXTENSION;

pub trait ASTKind {
}


pub enum FileKind
{
    Unknown,
    TypeFile,
    FnFile,
    ConstFile,
}


pub fn file_kind(path: &Path) -> FileKind {
    match path.extension() {
        Some(ext) => {
            if let Some(res) = ext.to_str() {
                if res == HAT_EXTENSION {
                    return unwrap_file_kind(path)
                }
            }
            FileKind::Unknown
        },
        None => FileKind::Unknown,
    }
}


// TODO: refactor this function
fn unwrap_file_kind(path: &Path) -> FileKind {
    if let Some(root_path) = path.components().next() {
        if root_path == std::path::Component::Normal(OsStr::new("src")) {

        }
    }
    if is_type_file(path) { return FileKind::TypeFile }
    if is_const_file(path) { return FileKind::ConstFile }
    FileKind::FnFile
}


#[inline]
fn is_type_file(path: &Path) -> bool {
    // skip the root
    path.components().next();
    match path.components().next() {
        None => false,
        Some(c) => match c {
            Component::Normal(x) => { x == OsStr::new("hhat_types") },
            _ => false
        },
    }
}

#[inline]
fn is_const_file(path: &Path) -> bool {
    path.components().next();

    todo!()
}
