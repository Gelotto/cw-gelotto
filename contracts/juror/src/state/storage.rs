use cosmwasm_std::Addr;
use cw_storage_plus::{Deque, Item};
use gelotto_jury_lib::juror::models::DomainExpertise;

pub const CREATED_BY: Item<Addr> = Item::new("created_by");
pub const TABLE_ADDR: Item<Addr> = Item::new("table_addr");
pub const ACL_ADDR: Item<Addr> = Item::new("acl_addr");

pub const JUROR_ID: Item<String> = Item::new("id");
pub const JUROR_EXPERTISE: Deque<DomainExpertise> = Deque::new("expertise");
pub const JUROR_SCORE_EXPERIENCE: Item<u32> = Item::new("z_exp");
pub const JUROR_SCORE_SPEED: Item<u8> = Item::new("z_speed");
pub const JUROR_SCORE_INITIATIVE: Item<u8> = Item::new("z_initiative");
pub const JUROR_SCORE_IDENTITY: Item<u8> = Item::new("z_identity");
pub const JUROR_SCORE_PRECISION: Item<u8> = Item::new("z_precision");
pub const JUROR_SCORE_RESEARCH: Item<u8> = Item::new("z_research");
