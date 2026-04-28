use ratatui::DefaultTerminal;
use crate::runtime::Runtime;
use crate::RuntimeError;
use crate::value::{new_instance, ClassType, InstanceType, LayoutDirection, LayoutNode, Node, NodeType, Value};

pub struct Dom {
    tree: Vec<InstanceType>,
    term: DefaultTerminal,
}

impl Dom {
    pub fn new() -> Self {
        let term = ratatui::init();

        Self {
            tree: vec![],
            term,
        }
    }

    pub fn construct_layout(&mut self, root: Value) -> Result<(), RuntimeError> {
        let layout = root
            .take_list()?
            .borrow()
            .iter()
            .cloned()
            .map(|v| v.take_instance())
            .collect::<Result<Vec<_>, _>>()?;

        self.tree = layout;
        Ok(())
    }

    pub fn run_component(&mut self, runtime: &mut Runtime, component: Value) -> Result<(), RuntimeError> {
        let component = component.clone().take_class()?;
        let instance = new_instance(component.clone());

        if let Some(constructor) = component.borrow().get_constructor() {
            let constructor = constructor
                .to_owned()
                .take_function()?;

            let constructor = constructor
                .borrow_mut()
                .bound_copy(instance.to_owned());

            runtime.call_val(constructor.into(), 0)?;
        }

        let main = instance.borrow()
            .get_field("<layout>")
            .ok_or(RuntimeError::GlobalNotFound { name: "<layout>".to_string() })?
            .clone();

        runtime.push_value(component.clone());
        let ret = runtime.call_val(main, 1)?;

        self.construct_layout(ret)?;

        loop {
            self.render(runtime)?;
            self.poll()?;
        }
    }

    pub fn render(&mut self, runtime: &mut Runtime) -> Result<(), RuntimeError> {
        let nodes = self.tree.iter()
            .map(|component| -> Result<NodeType, RuntimeError> {
                let component = component.borrow();
                let layout_fn = component.get_field("<layout>")
                    .expect("TODO: REMOVE PLS");

                let res = runtime.call_val(layout_fn.to_owned(), 0)?;

                res.take_node()
            })
            .collect::<Result<Vec<_>, _>>()?;

        self.term.draw(|frame| {
            let layout = LayoutNode {
                direction: LayoutDirection::Vertical,
                nodes,
            };
            frame.render_stateful_widget(&layout, frame.area(), runtime);
        })?;

        Ok(())
    }

    pub fn poll(&self) -> Result<(), RuntimeError> {
        ratatui::crossterm::event::read()?;
        Ok(())
    }
}
