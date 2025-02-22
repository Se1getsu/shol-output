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

struct _print {
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
    fn rule(&mut self, _print_resources: &mut HashSet<ResourceType>) {
        let mut to_replace = Vec::new();
        
        /*
        . $ < 100 # $ + 1 
        | $ #fizzbuzz $
        */
        for resource in self.resources.iter() {
            if let ResourceType::Int(value) = resource {
                if value % 3 == 0  {
                    to_replace.push((ResourceType::Int(*value), ResourceType::String(std::string::String::from("fizz"))));
                }
                if value % 5 == 0 {
                    to_replace.push((ResourceType::Int(*value), ResourceType::String(std::string::String::from("buzz"))));
                }
            }
        }
        for (old, new) in to_replace {
            self.resources.remove(&old);
            self.resources.insert(new);
        }

        // . $ = Fizz, $ = Buzz # FizzBuzz
        let mut to_replace = Vec::new();
        if self.resources.contains(&ResourceType::String(String::from("fizz"))) &&
           self.resources.contains(&ResourceType::String(String::from("buzz"))) {

            self.resources.remove(&ResourceType::String(String::from("fizz")));
            self.resources.remove(&ResourceType::String(String::from("buzz")));

            self.resources.insert(ResourceType::String(String::from("fizzbuzz")));
        }

        // 要素の置き換え
        for (old, new) in to_replace {
            self.resources.remove(&old);
            self.resources.insert(new);
        }
        
        let mut to_remove = Vec::new();
        for resource in self.resources.iter() {
            if let ResourceType::Int(value) = resource {
                _print_resources.insert(ResourceType::Int(*value));
                to_remove.push(ResourceType::Int(*value));
            }
            if let ResourceType::String(value) = resource {
                _print_resources.insert(ResourceType::String(value.to_string()));
                to_remove.push(ResourceType::String(value.to_string()));
            }
        }
        for resource in to_remove {
            self.resources.remove(&resource);
        }
    }
}

impl _print {
    fn rule(&mut self) {
        let mut to_remove = Vec::new();
        for resource in self.resources.iter() {
            match resource {
                ResourceType::String(value) => {
                    println!("{}", value);
                    to_remove.push(ResourceType::String(value.to_string()));
                },
                ResourceType::Int(value) => {
                    println!("{}", value);
                    to_remove.push(ResourceType::Int(*value));
                }
            }
        }
        for resource in to_remove {
            self.resources.remove(&resource);
        }
    }
}

fn main() {
    let mut nGen = _nGen {
        resources: HashSet::new()
    };
    let mut fizzbuzz = _fizzbuzz {
        resources: HashSet::new()
    };
    let mut print = _print {
        resources: HashSet::new()
    };
    nGen.resources.insert(ResourceType::Int(1));
    nGen.rule(&mut fizzbuzz.resources);
    fizzbuzz.rule(&mut print.resources);
    print.rule();
    println!("{:?}", nGen.resources);
    println!("{:?}", fizzbuzz.resources);

    nGen.rule(&mut fizzbuzz.resources);
    fizzbuzz.rule(&mut print.resources);
    print.rule();
    println!("{:?}", nGen.resources);
    println!("{:?}", fizzbuzz.resources);

    nGen.rule(&mut fizzbuzz.resources);
    fizzbuzz.rule(&mut print.resources);
    print.rule();
    println!("{:?}", nGen.resources);
    println!("{:?}", fizzbuzz.resources);
}
