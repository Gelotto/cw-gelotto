pub mod bond;
pub mod claim;
pub mod evidence;
pub mod lifecycle;
pub mod set_config;
pub mod vote;

use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub struct Context<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}
