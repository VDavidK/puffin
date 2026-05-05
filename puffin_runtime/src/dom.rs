use ratatui::DefaultTerminal;
use crate::event::Event;
use crate::runtime::Runtime;
use crate::RuntimeError;
use crate::value::{new_instance, InstanceType, LayoutDirection, LayoutNode, NodeType, Value};

pub struct Dom {
    tree: NodeType,
    term: DefaultTerminal,
}

impl Dom {
    pub fn new(component: Value, runtime: &mut Runtime) -> Result<Self, RuntimeError> {
        let term = ratatui::init();

        let component = component
            .clone()
            .take_class()?;

        let instance = new_instance(component.clone(), runtime, 0)?;

        let tree = instance
            .borrow()
            .get_field("<layout>")
            .expect("<layout> does not contain a node") // TODO: Better
            .to_owned()
            .take_node()?;

        Ok(Self {
            tree,
            term,
        })
    }

    pub fn run(&mut self, runtime: &mut Runtime) -> Result<(), RuntimeError> {
        loop {
            self.render(runtime)?;
            self.poll(runtime)?;

            if runtime.exit_requested() {
                break;
            }
        }

        Ok(())
    }

    pub fn render(&mut self, runtime: &mut Runtime) -> Result<(), RuntimeError> {
        let node = self.tree.clone();

        self.term.draw(|frame| {
            frame.render_stateful_widget(&*node.borrow(), frame.area(), runtime);
        })?;

        Ok(())
    }

    pub fn dispatch_event(&self, runtime: &mut Runtime, event: Event) -> Result<(), RuntimeError> {
        // event.dispatch(runtime, elem.to_owned())?;

        self.tree
            .borrow()
            .dispatch(runtime, &event)?;

        Ok(())
    }

    pub fn poll(&self, runtime: &mut Runtime) -> Result<(), RuntimeError> {
        use ratatui::crossterm::event::{
            Event as CrosstermEvent,
            KeyEvent,
            KeyCode,
        };

        let event = ratatui::crossterm::event::read()?;

        if let CrosstermEvent::Key(KeyEvent { code: KeyCode::Char(c), .. }) = event {
            self.dispatch_event(runtime, Event::KeyPress(c))?;
        }

        Ok(())
    }
}
