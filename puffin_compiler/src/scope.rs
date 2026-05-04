use std::collections::HashMap;
use puffin_runtime::chunk::LocalOffset;

#[derive(Default)]
pub struct Scope<'a> {
    local_table: HashMap<&'a str, LocalOffset>,
    local_count: usize,
    parent: Option<Box<Scope<'a>>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self {
            local_table: HashMap::new(),
            local_count: 0,
            parent: None,
        }
    }

    pub fn set_parent(&mut self, parent: Box<Scope<'a>>) {
        self.parent = Some(parent);
    }

    pub fn remove_parent(&mut self) -> Option<Box<Scope<'a>>> {
        self.parent.take()
    }

    pub fn define_local(&mut self, name: &'a str) -> LocalOffset {
        let local_count = self.total_local_count() as LocalOffset;
        self.local_table.insert(name, local_count);
        self.local_count += 1;
        local_count
    }

    pub fn replace_local(&mut self, name: &'a str) -> LocalOffset {
        let local_count = self.total_local_count() as LocalOffset;
        self.local_table.insert(name, local_count - 1);
        local_count
    }

    pub fn define_unnamed_local(&mut self) -> LocalOffset {
        let local_count = self.total_local_count() as LocalOffset;
        self.local_count += 1;
        local_count
    }

    pub fn remove_top_local(&mut self) {
        self.local_count -= 1;
    }

    pub fn remove_top_n_locals(&mut self, n: usize) {
        self.local_count -= n;
    }

    pub fn local_count(&self) -> usize {
        self.local_count
    }

    pub fn total_local_count(&self) -> usize {
        self.local_count + self.parent
            .as_ref()
            .map(|parent| parent.local_count())
            .unwrap_or(0)
    }

    pub fn lookup_local(&self, name: &str) -> Option<LocalOffset> {
        if let Some(offset) = self.local_table.get(name) {
            Some(*offset)
        } else if let Some(parent) = &self.parent {
            parent.lookup_local(name)
        } else {
            None
        }
    }
}
