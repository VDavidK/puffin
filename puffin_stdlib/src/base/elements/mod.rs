use std::collections::HashMap;
use std::str::FromStr;
use puffin_runtime::runtime::Runtime;
use puffin_runtime::RuntimeError;
use puffin_runtime::value::{ComponentNode, DictionaryType, ListType, NodeType, Value};
use puffin_runtime::ratatui::prelude::*;

pub mod text;
pub mod flow;
pub mod frame;
pub mod input;
pub mod image;

pub(crate) type Props = DictionaryType;

pub(crate) fn get_props(runtime: &mut Runtime) -> Result<Props, RuntimeError> {
    Ok(runtime.get_local(-1)?
        .to_owned()
        .take_dictionary()?)
}

pub(crate) fn get_inner(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    Ok(runtime.get_local(-2)?.to_owned())
}

pub(crate) fn get_inner_nodes(runtime: &mut Runtime) -> Result<Vec<NodeType>, RuntimeError> {
    Ok(get_inner_as::<ListType>(runtime)?
        .borrow()
        .iter()
        .cloned()
        .map(|v| v.take_node())
        .collect::<Result<Vec<_>, _>>()?)
}

pub(crate) fn get_inner_as<T: TryFrom<Value, Error = RuntimeError>>(runtime: &mut Runtime) -> Result<T, RuntimeError> {
    T::try_from(get_inner(runtime)?)
}

pub(crate) fn fetch_property<T: TryFrom<Value, Error = RuntimeError>>(props: &Props, name: impl Into<String>) -> Result<T, RuntimeError> {
    let name = name.into();
    let value = props
        .borrow()
        .get(&name.to_owned().into())
        .ok_or_else(|| RuntimeError::NoFieldMatchingName { name })?
        .to_owned();
    T::try_from(value)
}