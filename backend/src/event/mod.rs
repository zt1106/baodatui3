pub trait Event {
    fn event_type(&self) -> &'static EventType;
}

pub struct EventType {
    debounce_policy: EventDebouncePolicy,
}

enum EventDebouncePolicy {}
