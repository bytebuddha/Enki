use crate::enki::ActionReactor;
use crate::enki::Configuration;
use crate::xi::client::Client;

pub struct Context<'a, 'b, 'c> {
    pub client: &'a mut Client,
    pub config: &'b mut Configuration,
    pub reactor: &'c mut ActionReactor,
}
