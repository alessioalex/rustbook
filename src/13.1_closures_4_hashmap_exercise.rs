use std::collections::HashMap;
// use std::thread;
// use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    map: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let value = self.map.get(&arg);

        match value {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.map.insert(arg, v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| {
        println!("doing some calculation for {}", a);
        a
    });

    let v1 = c.value(1);
    let _v11 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
    assert_eq!(v1, 1);
}

fn main() {}
