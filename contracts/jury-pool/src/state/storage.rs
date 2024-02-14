use cosmwasm_std::{Addr, Uint64};
use cw_storage_plus::{Item, Map};

use super::models::{Config, SubMsgReplyJob};

pub const CONFIG: Item<Config> = Item::new("config");
pub const ACL_ADDR: Item<Addr> = Item::new("acl_addr");
pub const TABLE_ADDR: Item<Addr> = Item::new("table_addr");
pub const REPLY_JOBS: Map<u64, SubMsgReplyJob> = Map::new("reply_jobs");
pub const JURY_ID_COUNTER: Item<Uint64> = Item::new("jury_id_counter");
pub const JURY_CODE_ID: Item<Uint64> = Item::new("jury_code_id");
pub const JURY_ID_2_JURY: Item<Uint64> = Item::new("jury_code_id");
