use crate::bindings::exports::component::lambda_golem_bot::api::Guest;

mod bindings;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}
