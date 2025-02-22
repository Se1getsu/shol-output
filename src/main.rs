use std::{collections::HashSet, hash::Hash};

#[derive(Eq, Hash, Debug, PartialEq)]
enum ResourceType {
    String(String),
    Int(i32)
}

struct _kemo {
    resources: HashSet<ResourceType>
}

impl _kemo {
    fn rule(&mut self) {
        let mut to_replace = Vec::new();
        
        /*
        . $ < 4 # $ + 1 
        | $ % 2 = 0 # $ * 10
        */
        for resource in self.resources.iter() {
            if let ResourceType::Int(value) = resource {
                if *value < 4 {
                    to_replace.push((ResourceType::Int(*value), ResourceType::Int(*value + 1)));
                }
                if value % 2 == 0 {
                    to_replace.push((ResourceType::Int(*value), ResourceType::Int(*value * 10)));
                }
            }
        }
        
        // 要素の置き換え
        for (old, new) in to_replace {
            self.resources.remove(&old);
            self.resources.insert(new);
        }
    }
}

struct _nGen {
    resources: HashSet<ResourceType>
}

struct _fizzbuzz {
    resources: HashSet<ResourceType>
}

impl _nGen {
    fn rule(&mut self, _fizzbuzz_resources: &mut HashSet<ResourceType>) {
        let mut to_replace = Vec::new();
        
        /*
        . $ < 100 # $ + 1 
        | $ #fizzbuzz $
        */
        for resource in self.resources.iter() {
            if let ResourceType::Int(value) = resource {
                if *value < 100 {
                    to_replace.push((ResourceType::Int(*value), ResourceType::Int(*value + 1)));
                }
                _fizzbuzz_resources.insert(ResourceType::Int(*value));
            }
        }
        
        // 要素の置き換え
        for (old, new) in to_replace {
            self.resources.remove(&old);
            self.resources.insert(new);
        }        
    }
}

impl _fizzbuzz {
    fn rule(&mut self) {
        println!("{:?}", self.resources);
    }
}

fn main() {
    let mut nGen = _nGen {
        resources: HashSet::new()
    };
    let mut fizzbuzz = _fizzbuzz {
        resources: HashSet::new()
    };
    nGen.resources.insert(ResourceType::Int(1));
    nGen.rule(&mut fizzbuzz.resources);
    fizzbuzz.rule();
    nGen.rule(&mut fizzbuzz.resources);
    fizzbuzz.rule();
    nGen.rule(&mut fizzbuzz.resources);
    fizzbuzz.rule();
}
