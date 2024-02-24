use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};
use cw_utils::Duration;
use gelotto_jury_lib::models::{Bond, JurorQualifications, JuryTask, Verdict};

use super::models::{JurorVoteMetadata, VotingPeriod};

pub const CREATED_BY: Item<Addr> = Item::new("created_by");
pub const TABLE_ADDR: Item<Addr> = Item::new("table_addr");
pub const ACL_ADDR: Item<Addr> = Item::new("acl_addr");

pub const JURY_ID: Item<String> = Item::new("id");
pub const JURY_TITLE: Item<String> = Item::new("title");
pub const JURY_TASK: Item<JuryTask> = Item::new("task");
pub const JURY_TAGS: Map<&String, bool> = Map::new("tags");
pub const JURY_VOTING_PERIOD: Item<VotingPeriod> = Item::new("voting_period");
pub const JURY_ALLOW_APPEALS: Item<bool> = Item::new("allow_appeals");
pub const JURY_MIN_VOTE_COUNT: Item<u32> = Item::new("min_vote_count");
pub const JURY_MIN_CONSENSUS_PCT: Item<u32> = Item::new("min_consensus_pct");

pub const JUROR_QUALIFICATIONS: Item<JurorQualifications> = Item::new("juror_qualifications");
pub const JUROR_BOND_REQUIREMENTS: Item<Vec<Bond>> = Item::new("juror_bond_requirements");
pub const JUROR_BONDS: Map<(&Addr, &String), bool> = Map::new("juror_bonds");
pub const JUROR_VOTES: Map<(u8, &Addr, &String), bool> = Map::new("juror_votes");
pub const JUROR_VOTE_TOTALS: Map<(u8, &String), u32> = Map::new("juror_vote_totals");
pub const JUROR_VOTE_METADATA: Map<&Addr, JurorVoteMetadata> = Map::new("juror_vote_meta");
pub const JUROR_VOTE_RATIONALES: Map<&Addr, String> = Map::new("juror_vote_rationales");
pub const JUROR_VOTE_PROPOSERS: Map<&String, Addr> = Map::new("juror_vote_proposers");
pub const JUROR_SPEED_SCORES: Map<&Addr, u8> = Map::new("juror_speed_scores");

// The sum of these two equal the grand total vote count:
pub const TOTAL_QUALIFIED_VOTE_COUNT: Item<u32> = Item::new("total_qualified_vote_count");
pub const TOTAL_UNQUALIFIED_VOTE_COUNT: Item<u32> = Item::new("total_unqualified_vote_count");

pub const VERDICT: Item<Verdict> = Item::new("verdict");
