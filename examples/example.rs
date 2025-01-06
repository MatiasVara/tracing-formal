use solver::Alternates;
use tracing::event;
use tracing::instrument;
use tracing::Level;
use tracing_formal::TracingFormal;

#[instrument(fields(event = "do_hola"))]
fn hola() {
    println!("do hola()");
}

// TODO: this does not have context
// TODO: fields can store parameters to the functions and the result
#[instrument(fields(event = "do_chau"))]
fn chau() {
    println!("do chau()");
}

fn main() {
    // `do_hola`` and `do_chau`` events must satisfy an alternate relation between them
    // this is the spec that should be translated to rust
    let alternates: Alternates = Alternates::new("do_hola", "do_chau");

    let subscriber = TracingFormal::new(vec![alternates]);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    // just an event as example
    // events provide better granularity
    // fields is used to trigger the `do_hola` event
    event!(Level::INFO, event = "do_hola");

    hola(); // this is a violation of the partial order
    chau();
    chau(); // this is a violation in the partial order
}
