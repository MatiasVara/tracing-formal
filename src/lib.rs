use std::sync::Arc;
use std::sync::RwLock;

use solver::Alternates;
use tracing::{
    callsite, field::Visit, metadata::LevelFilter, metadata::Metadata, span, span::Attributes,
    span::Id, subscriber::Interest, Event, Subscriber,
};

pub struct TracingFormal {
    pub relations: Arc<RwLock<Vec<Alternates>>>,
}

impl TracingFormal {
    pub fn new(relations: Vec<Alternates>) -> Self {
        TracingFormal {
            relations: Arc::new(RwLock::new(relations)),
        }
    }
}

impl Subscriber for TracingFormal {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        // Enable all events and spans.
        true
    }

    fn new_span(&self, span: &Attributes<'_>) -> Id {
        let mut field_logger = FieldLogger::new(
            self,
            span.metadata().name(),
            span.metadata().file().unwrap(),
            span.metadata().line().unwrap(),
            span.metadata().callsite(),
        );
        span.record(&mut field_logger);

        // IDs are used to uniquely identify spans and events within
        // the context of a subscriber, so span equality will be
        // based on the returned ID. Thus, if the subscriber wishes
        // for all spans with the same metadata to be considered equal,
        // it should return the same ID every time it is given a particular set of metadata.
        // Similarly, if it wishes for two separate instances of a span
        // with the same metadata to not be equal, it should return a
        // distinct ID every time this function is called, regardless of the metadata.
        // for example, in a virtio-devices, same devices should return the same ID
        Id::from_u64(1)
    }

    fn record(&self, _span: &Id, _values: &span::Record<'_>) {
        // Record fields associated with a span
    }

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {
        // Record a relationship between spans
    }

    fn event(&self, event: &Event<'_>) {
        // Handle an event
        let mut field_logger = FieldLogger::new(
            self,
            event.metadata().name(),
            event.metadata().file().unwrap(),
            event.metadata().line().unwrap(),
            event.metadata().callsite(),
        );
        event.record(&mut field_logger);
    }

    fn enter(&self, _span: &Id) {
        // Enter a span
    }

    fn exit(&self, _span: &Id) {
        // Exit a span
    }

    fn register_callsite(&self, _metadata: &'static Metadata<'static>) -> Interest {
        // Decide whether to subscribe to this callsite (metadata).
        Interest::always()
    }

    fn max_level_hint(&self) -> Option<LevelFilter> {
        // Hint to the maximum level of span/event we are interested in
        Some(LevelFilter::TRACE)
    }
}

struct FieldLogger<'a> {
    subscriber: &'a TracingFormal,
    function: &'static str,
    file: &'static str,
    line: u32,
    callsite: callsite::Identifier,
}

impl<'a> FieldLogger<'a> {
    fn new(
        subscriber: &'a TracingFormal,
        function: &'static str,
        file: &'static str,
        line: u32,
        callsite: callsite::Identifier,
    ) -> Self {
        FieldLogger {
            subscriber,
            function,
            file,
            line,
            callsite,
        }
    }
}

impl<'a> Visit for FieldLogger<'a> {
    fn record_u64(&mut self, _field: &tracing::field::Field, _value: u64) {}

    fn record_bool(&mut self, _field: &tracing::field::Field, _value: bool) {}

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "event" {
            for rl in self.subscriber.relations.write().unwrap().iter_mut() {
                if rl.left.name == value && !rl.evaluate(true, false) {
                    println!(
                        "violation: function \"{}()\" defined at {}:{}, call it at {:?}",
                        self.function, self.file, self.line, self.callsite
                    );
                }
                if rl.right.name == value && !rl.evaluate(false, true) {
                    println!(
                        "violation: function \"{}()\" defined at {}:{}, call it at {:?}",
                        self.function, self.file, self.line, self.callsite
                    );
                }
            }
        }
    }

    fn record_debug(&mut self, _field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {}
}
