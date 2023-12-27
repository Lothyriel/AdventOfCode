use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub mod part_1;

fn parse(input: &str) -> Graph {
    let parts: Vec<_> = input.lines().map(get_connections).collect();

    let mut components = parts.iter().map(|(c, _)| (*c, Component::new(c))).collect();

    for (comp, connections) in parts {
        for con in connections {
            connect(&mut components, con, comp);
        }
    }

    components
}

fn connect<'a>(components: &mut HashMap<&'a str, Comp>, con: &'a str, comp: &'a str) {
    let connected = match find_comp(&*components, con) {
        Some(c) => c,
        None => {
            let comp = Component::new(con);

            components.insert(con, comp.to_owned());

            comp
        }
    };

    let component = components
        .entry(comp)
        .or_insert_with(|| Component::new(comp));

    connected
        .borrow_mut()
        .connections
        .push(component.to_owned());

    component.borrow_mut().connections.push(connected);
}

fn find_comp(components: &HashMap<&str, Comp>, con: &str) -> Option<Comp> {
    components.iter().find_map(|(c, comp)| match *c == con {
        true => Some(comp.to_owned()),
        false => None,
    })
}

fn get_connections(input: &str) -> (&str, impl Iterator<Item = &str>) {
    let mut parts = input.split(':');

    let label = parts.next().expect("Expected component label");

    let connections = parts
        .next()
        .expect("Expected connections")
        .split_whitespace();

    (label, connections)
}

pub type Graph<'a> = HashMap<&'a str, Comp>;

pub type Comp = Rc<RefCell<Component>>;

pub struct Component {
    connections: Vec<Rc<RefCell<Component>>>,
    label: String,
}

impl std::fmt::Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} ", &self.label))?;

        let mut list = f.debug_list();

        for c in &self.connections {
            list.entry(&c.borrow().label);
        }

        list.finish()
    }
}

impl Component {
    fn new(label: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            connections: Vec::new(),
            label: label.to_owned(),
        }))
    }
}
