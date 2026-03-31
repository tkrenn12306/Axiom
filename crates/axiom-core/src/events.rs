use std::any::{Any, TypeId};
use std::collections::HashMap;

/// An event that can be emitted and consumed via the EventBus.
pub trait Event: Send + Sync + 'static {}

/// Blanket implementation: every Send+Sync+'static type is an Event.
impl<T: Send + Sync + 'static> Event for T {}

type BoxedEvent = Box<dyn Any + Send + Sync>;
type HandlerFn = Box<dyn Fn(&dyn Any) + Send + Sync>;

/// A simple event bus with immediate and deferred (next-tick) delivery.
///
/// Events are stored by TypeId. At the start of each tick, `flush()` is called
/// to deliver deferred events to registered handlers.
pub struct EventBus {
    /// Events queued for delivery at next flush.
    deferred: Vec<(TypeId, BoxedEvent)>,
    /// Registered handlers, keyed by event TypeId.
    handlers: HashMap<TypeId, Vec<HandlerFn>>,
    /// Events from the previous flush available for reading this tick.
    current: HashMap<TypeId, Vec<BoxedEvent>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            deferred: Vec::new(),
            handlers: HashMap::new(),
            current: HashMap::new(),
        }
    }

    /// Emit an event immediately (calls handlers right now).
    pub fn emit_immediate<E: Event>(&mut self, event: E) {
        let type_id = TypeId::of::<E>();
        if let Some(handlers) = self.handlers.get(&type_id) {
            for handler in handlers {
                handler(&event);
            }
        }
    }

    /// Emit an event to be delivered on the next `flush()`.
    pub fn emit<E: Event>(&mut self, event: E) {
        let type_id = TypeId::of::<E>();
        self.deferred.push((type_id, Box::new(event)));
    }

    /// Register a handler for events of type `E`.
    pub fn subscribe<E: Event, F: Fn(&E) + Send + Sync + 'static>(&mut self, handler: F) {
        let type_id = TypeId::of::<E>();
        let boxed: HandlerFn = Box::new(move |any_event| {
            if let Some(e) = any_event.downcast_ref::<E>() {
                handler(e);
            }
        });
        self.handlers.entry(type_id).or_default().push(boxed);
    }

    /// Flush deferred events: deliver them to handlers and make them available via `read`.
    /// Called at the start of each tick by `TickEngine`.
    pub fn flush(&mut self) {
        self.current.clear();

        let events = std::mem::take(&mut self.deferred);
        for (type_id, event) in events {
            // Call registered handlers
            if let Some(handlers) = self.handlers.get(&type_id) {
                for handler in handlers {
                    handler(event.as_ref());
                }
            }
            // Store for `read` access
            self.current.entry(type_id).or_default().push(event);
        }
    }

    /// Read all events of type `E` delivered in the current tick.
    pub fn read<E: Event>(&self) -> impl Iterator<Item = &E> {
        let type_id = TypeId::of::<E>();
        self.current
            .get(&type_id)
            .into_iter()
            .flat_map(|events| events.iter())
            .filter_map(|e| e.downcast_ref::<E>())
    }

    /// Returns true if there are any pending deferred events.
    pub fn has_pending(&self) -> bool {
        !self.deferred.is_empty()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Clone)]
    struct TestEvent {
        value: i32,
    }

    #[test]
    fn test_emit_and_flush() {
        let mut bus = EventBus::new();
        bus.emit(TestEvent { value: 42 });
        assert!(bus.has_pending());
        bus.flush();
        assert!(!bus.has_pending());

        let events: Vec<&TestEvent> = bus.read::<TestEvent>().collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].value, 42);
    }

    #[test]
    fn test_subscribe_and_flush() {
        let received = Arc::new(Mutex::new(Vec::<i32>::new()));
        let received_clone = received.clone();

        let mut bus = EventBus::new();
        bus.subscribe::<TestEvent, _>(move |e| {
            received_clone.lock().unwrap().push(e.value);
        });

        bus.emit(TestEvent { value: 1 });
        bus.emit(TestEvent { value: 2 });
        bus.flush();

        let vals = received.lock().unwrap();
        assert_eq!(*vals, vec![1, 2]);
    }

    #[test]
    fn test_flush_clears_previous_events() {
        let mut bus = EventBus::new();
        bus.emit(TestEvent { value: 10 });
        bus.flush();

        // No new events this tick
        bus.flush();
        let events: Vec<&TestEvent> = bus.read::<TestEvent>().collect();
        assert_eq!(events.len(), 0);
    }

    #[test]
    fn test_emit_immediate() {
        let received = Arc::new(Mutex::new(Vec::<i32>::new()));
        let received_clone = received.clone();

        let mut bus = EventBus::new();
        bus.subscribe::<TestEvent, _>(move |e| {
            received_clone.lock().unwrap().push(e.value);
        });

        bus.emit_immediate(TestEvent { value: 99 });
        let vals = received.lock().unwrap();
        assert_eq!(*vals, vec![99]);
    }
}
