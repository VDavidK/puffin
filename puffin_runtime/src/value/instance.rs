use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use serde_derive::{Deserialize, Serialize};
use crate::event::Event;
use crate::runtime::Runtime;
use crate::RuntimeError;
use crate::value::{Value, ClassType, Reactive, NodeType, LayoutNode, LayoutDirection, Node, ComponentNode};
use crate::value::ops::{ValueDef, ValueTruthy};

pub type InstanceType = Rc<RefCell<Instance>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    class: ClassType,
    fields: HashMap<String, Value>,
    event_handlers: HashMap<String, Value>,
}

impl Instance {
    pub fn new(class: ClassType) -> Self {
        Self {
            class,
            fields: HashMap::new(),
            event_handlers: HashMap::new(),
        }
    }

    pub fn set_field(&mut self, name: impl Into<String>, value: impl Into<Value>) -> Result<(), RuntimeError> {
        let name = name.into();

        if let Some(Value::Reactive(inner)) = self.fields.get(&name) {
            Reactive::set(inner.clone(), value.into())?;
            return Ok(());
        }

        self.fields.insert(name, value.into());
        Ok(())
    }

    pub fn get_field(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.fields.get(name.as_ref())
    }

    pub fn set_event_handler(&mut self, name: impl Into<String>, handler: impl Into<Value>) {
        self.event_handlers.insert(name.into(), handler.into());
    }
    
    pub fn get_class(&self) -> ClassType {
        self.class.clone()
    }

    pub fn get_handler(&self, name: impl AsRef<str>) -> Option<Value> {
        self.event_handlers.get(name.as_ref()).cloned()
    }

    pub fn construct_layout(instance: InstanceType, runtime: &mut Runtime) -> Result<NodeType, RuntimeError> {
        let layout = instance
            .borrow()
            .get_field("<construct>")
            .cloned()
            .unwrap(); // TODO: Remove shitty code

        let res = runtime.call(layout.to_owned(), &[])?;

        let node = match res {
            Value::List(nodes) => {
                let nodes = nodes
                    .borrow()
                    .iter()
                    .cloned()
                    .map(|v| v.take_instance())
                    .collect::<Result<Vec<InstanceType>, RuntimeError>>()?
                    .into_iter()
                    .map(|instance| Node::Component(ComponentNode {
                        instance: instance.to_owned(),
                        root: instance.borrow().get_field("<layout>")
                            .cloned()
                            .unwrap()
                            .take_node()
                            .unwrap(),
                    }))
                    .map(|node| Rc::new(RefCell::new(node)))
                    .collect::<Vec<NodeType>>();

                let root_node = LayoutNode {
                    nodes,
                    direction: LayoutDirection::Vertical,
                    segments: Value::from(Vec::<Value>::new()),
                };

                let root = Rc::new(RefCell::new(Node::Layout(root_node)));

                let component_node = ComponentNode {
                    root,
                    instance: instance.clone(),
                };
                Rc::new(RefCell::new(Node::Component(component_node)))
            }

            Value::Node(node) => node,

            v => Err(RuntimeError::IncorrectType { expected: "node[] or node".to_owned(), ty: v.type_name().to_owned() })?
        };

        Ok(node)
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Object [{}]", self.class.borrow().name))
    }
}

impl TryFrom<Value> for InstanceType {
    type Error = RuntimeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Instance(s) => Ok(s),
            _ => Err(RuntimeError::IncorrectType { ty: value.type_name().to_owned(), expected: "instance".to_owned() }),
        }
    }
}
impl From<InstanceType> for Value {
    fn from(value: InstanceType) -> Self {
        Value::Instance(value)
    }
}

impl From<Instance> for Value {
    fn from(value: Instance) -> Self {
        Value::Instance(Rc::new(RefCell::new(value)))
    }
}

pub fn new_instance(class: ClassType, runtime: &mut Runtime, num_args: usize) -> Result<InstanceType, RuntimeError> {
    let mut instance = Instance::new(class.clone());

    let class = class.borrow();
    let fields = class.get_fields();

    for (k, v) in fields.iter() {
        instance.set_field(k, v.clone())?;
    }

    let instance = Rc::new(RefCell::new(instance));

    for (k, v) in &class.methods {
        match v {
            Value::Function(func) => {
                let method = func.borrow().bound_copy(instance.to_owned());
                instance.borrow_mut().set_field(k, method)?;
            }
            Value::NativeFunction(func) => {
                let method = func.borrow().bound_copy(instance.to_owned());
                instance.borrow_mut().set_field(k, method)?;
            }
            _ => unreachable!("Class method needs to be of type function"),
        }
    }

    for (k, v) in &class.handlers {
        match v {
            Value::Function(func) => {
                let method = func.borrow().bound_copy(instance.to_owned());
                instance.borrow_mut().set_event_handler(k, method);
            }
            Value::NativeFunction(func) => {
                let method = func.borrow().bound_copy(instance.to_owned());
                instance.borrow_mut().set_event_handler(k, method);
            }
            _ => unreachable!("Class event handler needs to be of type function"),
        }
    }

    class.run_constructor(instance.clone(), num_args, runtime)?;

    let inst = instance.borrow();
    let construct_fn = inst
        .get_field("<construct>")
        .cloned();

    drop(inst);

    if construct_fn.is_some() {
        let node = Instance::construct_layout(instance.to_owned(), runtime)?;
        instance.borrow_mut().set_field("<layout>", node)?;
    }

    Ok(instance)
}

impl ValueTruthy for InstanceType {
    fn truthy(&self) -> bool {
        true
    }
}

impl ValueDef for InstanceType {
    const TYPE_NAME: &'static str = "instance";
}