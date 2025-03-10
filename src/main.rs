// Generated by Shol compiler.
// DO NOT EDIT MANUALLY.
//
// Original source: example/fizzbuzz.shol
// Shol version: 0.1.0

#![allow(
  non_snake_case,
  non_camel_case_types,
  dead_code,
  unused_variables,
  unused_imports,
  unused_mut,
  unreachable_patterns,
  unused_parens,
  unused_assignments,
)]
use std::collections::HashMap;

#[derive(Eq,Debug,PartialEq,Clone)]
enum ResourceType {
  Int(i32),
  String(String),
  Bool(bool),
  Tuple(Vec<ResourceType>),
}
trait Colony {
  fn debug_print(&mut self);
  fn receive(&mut self, g: Vec<ResourceType>);
  fn rule(&mut self) -> HashMap<usize, Vec<ResourceType>>;
}

struct Colony_nGen {
  resources: Vec<ResourceType>,
}
impl Colony for Colony_nGen {
  fn debug_print(&mut self) { println!("{:?}", self.resources); }
  fn receive(&mut self, g: Vec<ResourceType>) { self.resources.extend(g); }
  fn rule(&mut self) -> HashMap<usize, Vec<ResourceType>> {
    let mut gifts: HashMap<usize, Vec<ResourceType>> = HashMap::new();
    let mut buf = Vec::new();
    for (i, resource) in self.resources.iter().enumerate() {
      let mut no_match = true;
      match resource {
        ResourceType::Bool(v) => {
          {
            let entry = gifts.entry(1).or_default();
            entry.push(ResourceType::Bool(v.clone()));
            no_match = false;
          }
        }
        ResourceType::Int(v) => {
          if (v.clone()<100) {
            buf.push(ResourceType::Int((v.clone()+1)));
            no_match = false;
          }
          {
            let entry = gifts.entry(1).or_default();
            entry.push(ResourceType::Int(v.clone()));
            no_match = false;
          }
        }
        ResourceType::String(v) => {
          {
            let entry = gifts.entry(1).or_default();
            entry.push(ResourceType::String(v.clone()));
            no_match = false;
          }
        }
        ResourceType::Tuple(v) => {
        }
      }
      if no_match {
        buf.push(resource.clone());
      }
    }
    self.resources = buf;
    gifts
  }
}

struct Colony_fizzBuzz {
  resources: Vec<ResourceType>,
}
impl Colony for Colony_fizzBuzz {
  fn debug_print(&mut self) { println!("{:?}", self.resources); }
  fn receive(&mut self, g: Vec<ResourceType>) { self.resources.extend(g); }
  fn rule(&mut self) -> HashMap<usize, Vec<ResourceType>> {
    let mut gifts: HashMap<usize, Vec<ResourceType>> = HashMap::new();
    let mut buf = Vec::new();
    for (i, resource) in self.resources.iter().enumerate() {
      let mut no_match = true;
      match resource {
        ResourceType::String(v) => {
        }
        ResourceType::Int(v) => {
          if ((v.clone()%3)==0) {
            buf.push(ResourceType::String("Fizz".to_owned()));
            no_match = false;
          }
          if ((v.clone()%5)==0) {
            buf.push(ResourceType::String("Buzz".to_owned()));
            no_match = false;
          }
        }
        ResourceType::Bool(v) => {
        }
        ResourceType::Tuple(v) => {
        }
      }
      if no_match {
        buf.push(resource.clone());
      }
    }
    self.resources = buf;
    let mut insertions: HashMap<usize, Vec<ResourceType>> = HashMap::new();
    let mut some_used: Vec<bool> = vec![false; self.resources.len()];
    let mut used: Vec<bool> = vec![false; self.resources.len()];
    let mut capt_prog: Vec<usize> = vec![0;2];
    let mut capt_idx: usize = 0;
    while capt_prog[capt_idx] < self.resources.len() {
      if used[capt_prog[capt_idx]] {
        capt_prog[capt_idx] += 1;
        continue;
      }
      match capt_idx {
        0 => {
          if match &self.resources[capt_prog[0]] {
            ResourceType::String(v) => v.clone() == "Fizz".to_owned(),
            _ => false
          } {
            capt_idx = 1;
            used[capt_prog[0]] = true;
            some_used[capt_prog[0]] = true;
          }
          capt_prog[0] += 1;
        },
        1 => {
          if match &self.resources[capt_prog[1]] {
            ResourceType::String(v) => v.clone() == "Buzz".to_owned(),
            _ => false
          } {
            capt_idx = 0;
            used[capt_prog[1]] = true;
            some_used[capt_prog[1]] = true;
            capt_prog[1] += 1;
            let entry = insertions.entry(capt_prog[0]-1).or_default();
            entry.push(ResourceType::String("FizzBuzz".to_owned()));
          } else {
            capt_prog[1] += 1;
          }
        },
        _ => unreachable!()
      }
    }
    for i in 0..capt_idx {
      used[capt_prog[i]-1] = false;
      some_used[capt_prog[i]-1] = false;
    }
    let mut buf = Vec::new();
    for (i, resource) in self.resources.iter().enumerate() {
      let mut no_match = true;
      no_match &= !some_used[i];
      if let Some(insertion) = insertions.get(&i) {
        buf.extend(insertion.clone());
      }
      if no_match {
        buf.push(resource.clone());
      }
    }
    self.resources = buf;
    let mut buf = Vec::new();
    for (i, resource) in self.resources.iter().enumerate() {
      let mut no_match = true;
      match resource {
        ResourceType::Bool(v) => {
          {
            let entry = gifts.entry(2).or_default();
            entry.push(ResourceType::Bool(v.clone()));
            no_match = false;
          }
        }
        ResourceType::Int(v) => {
          {
            let entry = gifts.entry(2).or_default();
            entry.push(ResourceType::Int(v.clone()));
            no_match = false;
          }
        }
        ResourceType::String(v) => {
          {
            let entry = gifts.entry(2).or_default();
            entry.push(ResourceType::String(v.clone()));
            no_match = false;
          }
        }
        ResourceType::Tuple(v) => {
        }
      }
      if no_match {
        buf.push(resource.clone());
      }
    }
    self.resources = buf;
    gifts
  }
}

struct Colony_print {
  resources: Vec<ResourceType>,
}
impl Colony for Colony_print {
  fn debug_print(&mut self) { println!("{:?}", self.resources); }
  fn receive(&mut self, g: Vec<ResourceType>) { self.resources.extend(g); }
  fn rule(&mut self) -> HashMap<usize, Vec<ResourceType>> {
    let mut gifts: HashMap<usize, Vec<ResourceType>> = HashMap::new();
    for resource in &self.resources {
      match resource {
        ResourceType::String(v) => println!("{v}"),
        ResourceType::Bool(v) => println!("{v}"),
        ResourceType::Int(v) => println!("{v}"),
        ResourceType::Tuple(v) => println!("{:?}", v),
      }
    }
    self.resources = vec![];
    gifts
  }
}

fn main() {
  let mut colonies: Vec<Box<dyn Colony>> = Vec::new();
  colonies.push(Box::new(Colony_nGen {
    resources: vec![
      ResourceType::Int(1),
    ],
  }));
  colonies.push(Box::new(Colony_fizzBuzz {
    resources: vec![
    ],
  }));
  colonies.push(Box::new(Colony_print {
    resources: vec![
    ],
  }));
  for _ in 0..8 {
    for i in 0..colonies.len() {
    // for debugging
    // println!(""); for i in 0..colonies.len() { colonies[i].debug_print(); }
      for (d, gv) in colonies[i].rule() {
        colonies[d].receive(gv);
      }
    }
  }
}
