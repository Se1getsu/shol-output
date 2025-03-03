use std::collections::HashMap;

#[derive(Eq, Debug, PartialEq, Clone)]
enum ResourceType {
    String(String),
    Int(i32)
}

struct _kemo {
    resources: Vec<ResourceType>
}

impl _kemo {
    fn rule(&mut self) {

        /*
        . $ < 4 # $ + 1
        | $ % 2 = 0 # $ * 10
        */
        // let mut buf = Vec::new();
        // for resource in self.resources.iter() {
        //     let mut どの条件にも当てはまらない = true;
        //     if let ResourceType::Int(value) = resource {
        //         if *value < 4 {
        //             buf.push(ResourceType::Int(*value + 1));
        //             どの条件にも当てはまらない = false;
        //         }
        //         if *value % 2 == 0 {
        //             buf.push(ResourceType::Int(*value * 10));
        //             どの条件にも当てはまらない = false;

        //         }
        //     }
        //     if どの条件にも当てはまらない {
        //         buf.push(resource.clone());
        //     }
        // }
        // self.resources = buf;

        /*
        . $1:int, $2:int, $3 # $1 + $2
        */
        let mut insertions: HashMap<usize, Vec<ResourceType>> = HashMap::new();
        let mut 使用済み: Vec<bool> = vec![false; self.resources.len()];
        let mut capture_process: Vec<usize> = vec![0;3];
        let mut 探索中のキャプチャ: usize = 0; // 0 or 1 or 2
        while capture_process[探索中のキャプチャ] < self.resources.len() {
            if 使用済み[capture_process[探索中のキャプチャ]] {
                capture_process[探索中のキャプチャ] += 1;
                continue;
            }
            match 探索中のキャプチャ {
                0 => {
                    if let ResourceType::Int(value) = &self.resources[capture_process[0]] {
                        探索中のキャプチャ = 1;
                        使用済み[capture_process[0]] = true;
                    }
                    capture_process[0] += 1;
                },
                1 => {
                    if let ResourceType::Int(value) = &self.resources[capture_process[1]] {
                        探索中のキャプチャ = 2;
                        使用済み[capture_process[1]] = true;
                    }
                    capture_process[1] += 1;
                },
                2 => {
                    探索中のキャプチャ = 0;
                    使用済み[capture_process[2]] = true;
                    if let ResourceType::Int(value1) = &self.resources[capture_process[0] - 1] {
                    if let ResourceType::Int(value2) = &self.resources[capture_process[1] - 1] {
                        insertions
                            .entry(capture_process[0] - 1)
                            .or_default()
                            .push(ResourceType::Int(value1 + value2));
                    }}
                    capture_process[2] += 1;
                },
                _ => {}
            }
        }
        for i in 0..探索中のキャプチャ {
            使用済み[capture_process[i] - 1] = false;
        }
        let mut buf = Vec::new();
        for (i, resource) in self.resources.iter().enumerate() {
            let mut どの条件にも当てはまらない = true;

            if 使用済み[i] { どの条件にも当てはまらない = false; }
            if let Some(insertion) = insertions.get(&i) {
                for resource in insertion {
                    buf.push(resource.clone());
                }
            }

            if どの条件にも当てはまらない {
                buf.push(resource.clone());
            }
        }
        self.resources = buf;
    }
}

// struct _nGen {
//     resources: Vec<ResourceType>
// }

// struct _fizzbuzz {
//     resources: Vec<ResourceType>
// }

// struct _print {
//     resources: Vec<ResourceType>
// }

// impl _nGen {
//     fn rule(&mut self, _fizzbuzz_resources: &mut Vec<ResourceType>) {
//         let mut to_replace = Vec::new();
        
//         /*
//         . $ < 100 # $ + 1 
//         | $ #fizzbuzz $
//         */
//         for resource in self.resources.iter() {
//             if let ResourceType::Int(value) = resource {
//                 if *value < 100 {
//                     to_replace.push((ResourceType::Int(*value), ResourceType::Int(*value + 1)));
//                 }
//                 _fizzbuzz_resources.push(ResourceType::Int(*value));
//             }
//         }
        
//         // 要素の置き換え
//         for (old, new) in to_replace {
//             self.resources.remove(&old);
//             self.resources.push(new);
//         }        
//     }
// }

// impl _fizzbuzz {
//     fn rule(&mut self, _print_resources: &mut Vec<ResourceType>) {
//         let mut to_replace = Vec::new();
        
//         /*
//         . $ < 100 # $ + 1 
//         | $ #fizzbuzz $
//         */
//         for resource in self.resources.iter() {
//             if let ResourceType::Int(value) = resource {
//                 if value % 3 == 0  {
//                     to_replace.push((ResourceType::Int(*value), ResourceType::String(std::string::String::from("fizz"))));
//                 }
//                 if value % 5 == 0 {
//                     to_replace.push((ResourceType::Int(*value), ResourceType::String(std::string::String::from("buzz"))));
//                 }
//             }
//         }
//         for (old, new) in to_replace {
//             self.resources.remove(&old);
//             self.resources.push(new);
//         }

//         // . $ = Fizz, $ = Buzz # FizzBuzz
//         let mut to_replace = Vec::new();
//         if self.resources.contains(&ResourceType::String(String::from("fizz"))) &&
//            self.resources.contains(&ResourceType::String(String::from("buzz"))) {

//             self.resources.remove(&ResourceType::String(String::from("fizz")));
//             self.resources.remove(&ResourceType::String(String::from("buzz")));

//             self.resources.push(ResourceType::String(String::from("fizzbuzz")));
//         }

//         // 要素の置き換え
//         for (old, new) in to_replace {
//             self.resources.remove(&old);
//             self.resources.push(new);
//         }
        
//         let mut to_remove = Vec::new();
//         for resource in self.resources.iter() {
//             if let ResourceType::Int(value) = resource {
//                 _print_resources.push(ResourceType::Int(*value));
//                 to_remove.push(ResourceType::Int(*value));
//             }
//             if let ResourceType::String(value) = resource {
//                 _print_resources.push(ResourceType::String(value.to_string()));
//                 to_remove.push(ResourceType::String(value.to_string()));
//             }
//         }
//         for resource in to_remove {
//             self.resources.remove(&resource);
//         }
//     }
// }

// impl _print {
//     fn rule(&mut self) {
//         let mut to_remove = Vec::new();
//         for resource in self.resources.iter() {
//             match resource {
//                 ResourceType::String(value) => {
//                     println!("{}", value);
//                     to_remove.push(ResourceType::String(value.to_string()));
//                 },
//                 ResourceType::Int(value) => {
//                     println!("{}", value);
//                     to_remove.push(ResourceType::Int(*value));
//                 }
//             }
//         }
//         for resource in to_remove {
//             self.resources.remove(&resource);
//         }
//     }
// }

fn main() {
    let mut kemo = _kemo {
        resources: Vec::new()
    };
    kemo.resources.push(ResourceType::Int(1));
    kemo.resources.push(ResourceType::String("x".to_string()));
    kemo.resources.push(ResourceType::String("y".to_string()));
    kemo.resources.push(ResourceType::Int(3));
    kemo.resources.push(ResourceType::Int(10));
    kemo.resources.push(ResourceType::Int(11));
    println!("{:?}", kemo.resources);

    kemo.rule();
    println!("{:?}", kemo.resources);

    // let mut nGen = _nGen {
    //     resources: Vec::new()
    // };
    // let mut fizzbuzz = _fizzbuzz {
    //     resources: Vec::new()
    // };
    // let mut print = _print {
    //     resources: Vec::new()
    // };
    // nGen.resources.push(ResourceType::Int(1));
    // nGen.rule(&mut fizzbuzz.resources);
    // fizzbuzz.rule(&mut print.resources);
    // print.rule();
    // println!("{:?}", nGen.resources);
    // println!("{:?}", fizzbuzz.resources);

    // nGen.rule(&mut fizzbuzz.resources);
    // fizzbuzz.rule(&mut print.resources);
    // print.rule();
    // println!("{:?}", nGen.resources);
    // println!("{:?}", fizzbuzz.resources);

    // nGen.rule(&mut fizzbuzz.resources);
    // fizzbuzz.rule(&mut print.resources);
    // print.rule();
    // println!("{:?}", nGen.resources);
    // println!("{:?}", fizzbuzz.resources);
}
