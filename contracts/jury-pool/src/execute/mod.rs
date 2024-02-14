pub mod request_jury;
pub mod set_config;

use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub struct Context<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}

pub struct ReplyContext<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
}
