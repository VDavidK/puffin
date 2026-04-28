use ratatui::DefaultTerminal;
use crate::runtime::Runtime;
use crate::RuntimeError;
use crate::value::{new_instance, ClassType, LayoutDirection, LayoutNode, Node, Value};

pub struct Dom {
    tree: Node,
    term: DefaultTerminal,
}

impl Dom {
    pub fn new() -> Self {
        let term = ratatui::init();

        Self {
            tree: Node::Layout(LayoutNode {
                direction: LayoutDirection::Vertical,
                nodes: vec![],
            }),
            term,
        }
    }

    pub fn construct_node_tree(&mut self, root: Value) -> Result<(), RuntimeError> {
        let node = root
            .take_list()?
            .borrow()
            .iter()
            .cloned()
            .map(|v| v.take_node())
            .collect::<Result<Vec<_>, _>>()?;

        let layout = LayoutNode {
            direction: LayoutDirection::Vertical,
            nodes: node,
        };

        self.tree = Node::Layout(layout);
        Ok(())
    }

    pub fn run_component(&mut self, runtime: &mut Runtime, component: Value) -> Result<(), RuntimeError> {
        let component = component.clone().take_class()?;
        let instance = new_instance(component.clone());

        if let Some(constructor) = component.borrow().get_constructor() {
            runtime.push_value(instance.clone());
            runtime.call_fn(constructor.clone().take_function()?)?;
        }

        let main = instance.borrow()
            .get_field("<layout>")
            .ok_or(RuntimeError::GlobalNotFound { name: "<layout>".to_string() })?
            .clone();

        runtime.push_value(component.clone());
        let ret = runtime.call_val(main, 1)?;

        self.construct_node_tree(ret)?;

        loop {
            self.render(runtime)?;
            self.poll()?;
        }
    }

    pub fn render(&mut self, runtime: &mut Runtime) -> Result<(), RuntimeError> {
        self.term.draw(|frame| {
            frame.render_stateful_widget(&self.tree, frame.area(), runtime);
        })?;

        Ok(())
    }

    pub fn poll(&self) -> Result<(), RuntimeError> {
        ratatui::crossterm::event::read()?;
        Ok(())
    }
}
