use solver::Alternates;
use tracing::instrument;
use tracing_formal::TracingFormal;

#[derive(Debug)]
struct MyType {
    name: &'static str,
}

// impl a type and define events
impl MyType {
    #[instrument(fields(event = "do_hola"))]
    pub fn hola(&self) {
        println!("hola from {}", self.name);
    }

    #[instrument(fields(event = "do_chau"))]
    pub fn chau(&self) {
        println!("chau from {}", self.name);
    }
}

fn main() {
    // `do_hola`` and `do_chau`` events must satisfy an alternate relation between them
    // this is the spec that should be translated to rust
    let alternates: Alternates = Alternates::new("do_hola", "do_chau");

    let subscriber = TracingFormal::new(vec![alternates]);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let test = MyType {
        name: "MyTypeInstance",
    };

    /* TODO: to support different instances
    let test2 = MyType {
        name: "MyTypeInstance2"
    };
    */

    test.hola();

    test.chau();
    // the following line triggers a violation
    test.chau();
}
