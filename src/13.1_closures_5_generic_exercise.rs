use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<F, K, V> {
    calculation: F,
    cache: HashMap<K, V>,
}

impl<F, K, V> Cacher<F, K, V>
where
    F: Fn(&K) -> V,
    K: Hash + Eq,
{
    fn new(calculation: F) -> Cacher<F, K, V> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> &V {
        match self.cache.entry(arg) {
            Entry::Occupied(occupied) => occupied.into_mut(),
            Entry::Vacant(vacant) => {
                let value = (self.calculation)(vacant.key());
                vacant.insert(value)
            }
        }
    }
}

fn main() {
    let mut c = Cacher::new(|a: &u32| -> u32 {
        println!("doing some calculation for {}", a);
        a + 1
    });

    let v1 = c.value(1);
    println!("{:?}", v1);

    let v2 = c.value(2);
    println!("{:?}", v2);
}
