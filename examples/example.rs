use solver::Alternates;
use tracing::instrument;
use tracing_formal::TracingFormal;

#[instrument(fields(event = "do_hola"))]
fn hola() {
    println!("do hola()");
}

// TODO: this does not have context
#[instrument(fields(event = "do_chau"))]
fn chau() {
    println!("do chau()");
}

fn main() {
    let alternates: Alternates = Alternates::new("do_hola", "do_chau");

    let subscriber = TracingFormal::new(vec![alternates]);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    hola();
    chau();
    chau(); // this is a violation in the partial order
}
