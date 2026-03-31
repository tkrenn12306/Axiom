/// Marker trait for all ECS components.
/// Any type that is `Send + Sync + 'static` can be a component.
///
/// This is satisfied automatically by hecs — all components just need to
/// implement `Send + Sync + 'static`.
pub trait Component: Send + Sync + 'static {}

/// Blanket implementation: every Send+Sync+'static type is a Component.
impl<T: Send + Sync + 'static> Component for T {}
