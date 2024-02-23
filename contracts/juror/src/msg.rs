use cosmwasm_schema::cw_serde;
use cw_table::lifecycle::LifecycleExecuteMsg;

use crate::state::models::Config;

#[cw_serde]
pub enum ExecuteMsg {
    Lifecycle(LifecycleExecuteMsg),
    SetConfig(Config),
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);
