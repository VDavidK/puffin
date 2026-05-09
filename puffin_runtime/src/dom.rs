use ratatui::crossterm::event::{DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste, EnableFocusChange, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::DefaultTerminal;
use crate::event::Event;
use crate::runtime::Runtime;
use crate::RuntimeError;
use crate::value::{new_instance, ComponentNode, Instance, InstanceType, LayoutDirection, LayoutNode, Node, NodeType, Value};

pub struct Dom {
    tree: NodeType,
    term: DefaultTerminal,
}

impl Dom {
    pub fn new(component: Value, runtime: &mut Runtime) -> Result<Self, RuntimeError> {
        let term = ratatui::init();

        execute!(std::io::stdout(), EnableMouseCapture)?;
        execute!(std::io::stdout(), EnableBracketedPaste)?;
        execute!(std::io::stdout(), EnableFocusChange)?;

        let component = component
            .clone()
            .take_class()?;

        let instance = new_instance(component.clone(), runtime, 0)?;

        let root = instance
            .borrow()
            .get_field("<layout>")
            .expect("<layout> does not contain a node") // TODO: Better
            .to_owned()
            .take_node()?;

        let tree = ComponentNode {
            instance,
            root,
        }.into();

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
        self.tree
            .borrow()
            .dispatch(runtime, &event)?;

        Ok(())
    }

    pub fn poll(&self, runtime: &mut Runtime) -> Result<(), RuntimeError> {
        use ratatui::crossterm::event::{
            Event as CrosstermEvent,
        };

        let event = ratatui::crossterm::event::read()?;

        match event {
            CrosstermEvent::Key(evt) => {
                self.dispatch_event(runtime, Event::Key(evt))?;
            }
            CrosstermEvent::Mouse(evt) => {
                self.dispatch_event(runtime, Event::Mouse(evt))?;
            }
            CrosstermEvent::FocusGained => {
                self.dispatch_event(runtime, Event::FocusGained)?;
            }
            CrosstermEvent::FocusLost => {
                self.dispatch_event(runtime, Event::FocusLost)?;
            }
            CrosstermEvent::Paste(buffer) => {
                self.dispatch_event(runtime, Event::Paste(buffer))?;
            }
            CrosstermEvent::Resize(width, height) => {
                self.dispatch_event(runtime, Event::Resize(width, height))?;
            }
        }

        Ok(())
    }
}

impl Drop for Dom {
    fn drop(&mut self) {
        _ = execute!(std::io::stdout(), DisableMouseCapture);
        _ = execute!(std::io::stdout(), DisableBracketedPaste);
        _ = execute!(std::io::stdout(), DisableFocusChange);
    }
}
