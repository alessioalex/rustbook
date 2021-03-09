// This annotation means an instance of ImportantExcerpt
// can’t outlive the reference it holds in its part field
struct ImportantExcerpt<'a> {
    part: &'a str, // need to add lifetime annotations to references
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 1. The first rule is that each parameter that is a reference gets its own lifetime parameter.
    // 2. The second rule is if there is exactly one input lifetime parameter,
    // that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32
    // 3. The third rule is if there are multiple input lifetime parameters, but one of them is &self
    // or &mut self because this is a method, the lifetime of self is assigned to all output
    // lifetime parameters.

    // There are two input lifetimes, so Rust applies the first lifetime elision rule
    // and gives both &self and announcement their own lifetimes.
    // Then, because one of the parameters is &self, the return type gets the
    // lifetime of &self, and all lifetimes have been accounted for.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: &first_sentence,
    };
}
