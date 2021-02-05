use std::collections::HashMap;

trait Component {
  fn new(registry: &mut impl EventHandler, id: i32, state: String) -> Self where Self: Sized;
  fn id(&self) -> i32;
  fn get_state(&self) -> String;
  fn update(&mut self, msg: &ComponentMsg);
}

#[derive(Clone)]
struct Component1 {
  id: i32,
  state: String,
}

impl Component for Component1 {
  fn new(registry: &mut impl EventHandler, id: i32, state: String) -> Component1 {
    let c1 = Component1 {
      id,
      state,
    };

    registry.register(ComponentType::Component1, Box::new(c1.clone()));
    c1
  }

  fn id(&self) -> i32 {
    self.id
  }

  fn get_state(&self) -> String {
    self.state.to_string()
  }

  fn update(&mut self, msg: &ComponentMsg) {
    if self.id == msg.id {
      match msg.msg {
        Message::Msg1 => println!("Component1 id: {} - handling message 1", self.id),
        Message::Msg2 => println!("Component1 id: {} - handling message 2", self.id),
        Message::Msg3 => println!("Component1 id: {} - handling message 3", self.id),
      }
    }
  }
}

#[derive(Clone)]
struct Component2 {
  id: i32,
  state: String,
}

impl Component for Component2 {
  fn new(registry: &mut impl EventHandler, id: i32, state: String) -> Component2 {
    let c2 = Component2 {
      id,
      state,
    };

    registry.register(ComponentType::Component2, Box::new(c2.clone()));
    
    c2
  }

  fn id(&self) -> i32 {
    self.id
  }

  fn get_state(&self) -> String {
    self.state.to_string()
  }

  fn update(&mut self, msg: &ComponentMsg) {
    if self.id == msg.id {
      match msg.msg {
        Message::Msg1 => println!("Component2 id: {} - handling message 1", self.id),
        Message::Msg2 => println!("Component2 id: {} - handling message 2", self.id),
        Message::Msg3 => println!("Component2 id: {} - handling message 3", self.id),
      }
    }
  }
}

#[derive(std::cmp::PartialEq, std::cmp::Eq, std::hash::Hash)]
enum ComponentType {
  Component1,
  Component2,
}

type ComponentMap = HashMap<ComponentType, Vec<Box<dyn Component + 'static>>>;

struct ComponentRegistry {
  components: ComponentMap
}

impl ComponentRegistry {
  fn new() -> ComponentRegistry {
    ComponentRegistry {
      components: ComponentMap::new()
    }
  }
}

impl EventHandler for ComponentRegistry {
  fn register(&mut self, kind: ComponentType, component: Box<dyn Component>) {
    let items = self.components.entry(kind).or_insert(vec![]);
    items.push(component);
  }

  fn trigger(&mut self, kind: ComponentType, msg: ComponentMsg) {
    let entries = self.components.entry(kind).or_insert(vec![]);
    for entry in entries {
      entry.update(&msg);
    }
  }
}

enum Message {
  Msg1,
  Msg2,
  Msg3,
}

struct ComponentMsg {
  msg: Message,
  id: i32,
}

trait EventHandler {
  fn register(&mut self, kind: ComponentType, component: Box<dyn Component>);
  fn trigger(&mut self, kind: ComponentType, msg: ComponentMsg);
}

fn main() {
  let mut registry = ComponentRegistry::new();

  let _component1_1 = Component1::new(&mut registry, 1, "state1".into());
  let _component1_2 = Component1::new(&mut registry, 2, "state3".into());
  let _component2_1 = Component2::new(&mut registry, 1, "state1".into());
  let _component2_2 = Component2::new(&mut registry, 2, "state5".into());
  let _component2_3 = Component2::new(&mut registry, 3, "state2".into());

  registry.trigger(ComponentType::Component1, ComponentMsg {msg: Message::Msg2, id: 1});
  registry.trigger(ComponentType::Component2, ComponentMsg {msg: Message::Msg3, id: 3});
  registry.trigger(ComponentType::Component2, ComponentMsg {msg: Message::Msg1, id: 1});
}