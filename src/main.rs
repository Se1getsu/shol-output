use std::collections::HashSet;

#[derive(Eq, Hash, Debug, PartialEq)]
enum ResourceType {
    String(String),
    Int(i32)
}

struct _nGen {
    resources: HashSet<ResourceType>
}

impl _nGen {
    fn rule(&mut self) {
        let mut to_replace = Vec::new();
        
        // 変更が必要な要素を特定
        for resource in self.resources.iter() {
            if let ResourceType::Int(value) = resource {
                to_replace.push((ResourceType::Int(*value), ResourceType::Int(*value + 1)));
            }
        }
        
        // 要素の置き換え
        for (old, new) in to_replace {
            self.resources.remove(&old);
            self.resources.insert(new);
        }
    }
}

fn main() {
    let mut nGen = _nGen {
        resources: HashSet::new(),
    };
    nGen.resources.insert(ResourceType::Int(1));
    nGen.resources.insert(ResourceType::String("Hello".to_string()));
    nGen.rule();
    println!("{:?}", nGen.resources);
    nGen.rule();
    println!("{:?}", nGen.resources);
    nGen.rule();
    println!("{:?}", nGen.resources);
    nGen.rule();
    println!("{:?}", nGen.resources);
}
