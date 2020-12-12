use crate::frontend::*;

pub trait Backend {
    /// Returns what protocol this `Backend` uses.
    fn protocol_name(&self) -> &str;
    /// Returns an array with requested info for auth.
    ///
    /// The array takes tuples with a string and a boolean.
    /// The boolean, if true, indicates that this information is sensitive, and
    /// the frontend should treat it like so.
    fn auth_req(&self) -> [(&str, bool)];
    fn auth_status(&self) -> AuthStatus;
    fn root_node(&self) -> Node;
    /// Sends this `Backend` an `Action` to perform.
    fn queue_action(&mut self, action: impl Into<PendingAction>);
    fn should_update_ui(&mut self) -> bool;
}
