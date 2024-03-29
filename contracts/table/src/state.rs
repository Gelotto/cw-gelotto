use crate::context::Context;
use crate::models::{
    ContractMetadataView, ContractMetadataViewDetails, Details, DynamicContractMetadata, ReplyJob,
};
use crate::msg::{
    Config, ContractRecord, GroupCreationParams, GroupMetadata, IndexCreationParams, IndexMetadata,
    IndexType, InstantiateMsg, PartitionCreationParams, PartitionMetadata, PartitionSelector,
    TableInfo,
};
use crate::{error::ContractError, models::ContractMetadata};
use cosmwasm_std::{
    to_json_binary, Addr, Binary, DepsMut, Env, MessageInfo, Order, Storage, Timestamp, Uint128,
    Uint64,
};
use cw_acl::client::Acl;
use cw_storage_plus::{Item, Map};
use gelotto_core::models::owner::Owner;

// TODO: store size of each partition Map<u16, Uint64>
// TODO: add str prefix to custom index names

pub type PartitionID = u32;
pub type GroupID = u32;
pub type ContractID = u64;
pub type IndexMap<K> = Map<'static, K, u8>;
pub type CustomIndexMap<'a, T> = Map<'a, (PartitionID, T, ContractID), u8>;

// Marker/dummy value for IndexMap values
pub const X: u8 = 1;

// Table contract config settings:
pub const CONFIG_OWNER: Item<Owner> = Item::new("owner");
pub const CONFIG_CODE_ID_ALLOWLIST_ENABLED: Item<bool> = Item::new("code_id_allowlist_enabled");
pub const CONFIG_BACKUP: Item<Binary> = Item::new("config_backup");
pub const CONFIG_STR_MAX_LEN: Item<u16> = Item::new("config_indexed_str_max_len");

// Top-level metadata describing what this cw-table is and contains.
pub const TABLE_INFO: Item<TableInfo> = Item::new("table_info");

// Contract ID-related data. Each new contract ID increments the counter, and
// the two maps map Addr <-> u64 ID.
pub const CONTRACT_ID_COUNTER: Item<Uint64> = Item::new("contract_id_counter");
pub const CONTRACT_ID_2_ADDR: Map<ContractID, Addr> = Map::new("contract_id_2_addr");
pub const CONTRACT_ADDR_2_ID: Map<&Addr, Uint64> = Map::new("contract_addr_2_id");

// Created contract metadata generated through the create API, like its ID,
// block info at the time, and the initiator (i.e. the account that called
// create()).
pub const CONTRACT_METADATA: Map<ContractID, ContractMetadata> = Map::new("contract_meta");

// Contract metadata that changes on each update to any of its indices.
pub const CONTRACT_DYN_METADATA: Map<ContractID, DynamicContractMetadata> =
    Map::new("contract_dyn_meta");

// Flags indicating that a given contract is suspended
pub const CONTRACT_SUSPENSIONS: Map<ContractID, bool> = Map::new("contract_suspensions");

pub const CONTRACT_GROUP_IDS: IndexMap<(ContractID, GroupID)> = Map::new("contract_groups");

// Lookup table for finding all tags associated with a contract ID
pub const CONTRACT_TAGS: IndexMap<(ContractID, String)> = Map::new("contract_tags");

// Flag indicating that a given contract uses implements the lifecycle interface
pub const CONTRACT_USES_LIFECYCLE_HOOKS: Map<u64, bool> = Map::new("lifecycle_hooks_toggles");

// Jobs for processing in the cw reply entrypoint
pub const REPLY_JOBS: Map<u64, ReplyJob> = Map::new("reply_jobs");
pub const REPLY_JOB_ID_COUNTER: Item<Uint64> = Item::new("reply_job_id_counter");

// Allow list, where the keys are the Code ID's that can be instantiated through
// the create() API. Only used if the allowlist is enabled through config.
pub const CODE_ID_ALLOWLIST: IndexMap<u64> = Map::new("code_id_allowlist");

pub const PARTITION_ID_COUNTER: Item<PartitionID> = Item::new("partition_id_counter");
pub const PARTITION_NAME_2_ID: Map<String, PartitionID> = Map::new("partition_name_2_id");

// Partition metadata
pub const PARTITION_METADATA: Map<PartitionID, PartitionMetadata> = Map::new("partition_metadata");

// Number of contracts in each partition.
pub const PARTITION_SIZES: Map<PartitionID, Uint64> = Map::new("partition_sizes");

// Each contract can be associated with many tags. TAG_COUNTS records the total
// number of contracts with which each tag is associated.
pub const PARTITION_TAG_COUNTS: Map<(PartitionID, &String), u32> = Map::new("partition_tag_counts");

// Lookup table for finding names/keys of indexed values for a given contract ID
pub const CONTRACT_INDEX_TYPES: Map<(ContractID, &String), IndexType> =
    Map::new("contract_index_types");

// Metadata for custom indices.
pub const INDEX_METADATA: Map<String, IndexMetadata> = Map::new("index_metadata");

// INDEX_* are built-in index maps owned and managed by this contract.
pub const IX_CONTRACT_ID: IndexMap<(PartitionID, u64, ContractID)> = Map::new("ix_contract_id");
pub const IX_CODE_ID: IndexMap<(PartitionID, u64, ContractID)> = Map::new("ix_code_id");
pub const IX_CREATED_BY: IndexMap<(PartitionID, String, ContractID)> = Map::new("ix_created_by");
pub const IX_CREATED_AT: IndexMap<(PartitionID, u64, ContractID)> = Map::new("ix_created_at");
pub const IX_UPDATED_AT: IndexMap<(PartitionID, u64, ContractID)> = Map::new("ix_updated");
pub const IX_UPDATED_BY: IndexMap<(PartitionID, String, ContractID)> = Map::new("ix_updated_by");
pub const IX_REV: IndexMap<(PartitionID, u64, ContractID)> = Map::new("ix_created_by");
pub const IX_TAG: IndexMap<(PartitionID, &String, ContractID)> = Map::new("ix_tag");

// Groups transcend partitions, i.e. two contracts may belong to the same group
// despite beloning to separate partitions.
pub const IX_GROUP: IndexMap<(GroupID, ContractID)> = Map::new("ix_group");

// Lookup tables for current value of a given key, indexed for a given contract
// ID. For example, if a contract's "color" string is indexed, supposing that
// the contract ID is 1, We'd expect that the VALUES_STRING map contains the
// entry: (1, "color") => "red".
pub const VALUES_STRING: Map<(ContractID, &String), String> = Map::new("values_string");
pub const VALUES_BOOL: Map<(ContractID, &String), bool> = Map::new("values_bool");
pub const VALUES_TIME: Map<(ContractID, &String), Timestamp> = Map::new("values_time");
pub const VALUES_I32: Map<(ContractID, &String), i32> = Map::new("values_i32");
pub const VALUES_U8: Map<(ContractID, &String), u8> = Map::new("values_u8");
pub const VALUES_U16: Map<(ContractID, &String), u16> = Map::new("values_u16");
pub const VALUES_U32: Map<(ContractID, &String), u32> = Map::new("values_u32");
pub const VALUES_U64: Map<(ContractID, &String), Uint64> = Map::new("values_u64");
pub const VALUES_U128: Map<(ContractID, &String), Uint128> = Map::new("values_u128");
pub const VALUES_BINARY: Map<(ContractID, &String), Binary> = Map::new("values_binary");

/// Relationships define an arbitrary M-N named relationship between a contract
/// ID and an arbitrary Addr, like (contract_id, "winner", user_addr)

pub const UNIQUE: u8 = 1;
pub const NOT_UNIQUE: u8 = 2;

pub const REL_ADDR_2_ID: Map<(String, String, String), u8> = Map::new("rel_addr_2_contract_id");
pub const REL_ID_2_ADDR: Map<(ContractID, String, String), u8> = Map::new("rel_contract_id_2_addr");

// Group state:
pub const GROUP_METADATA: Map<GroupID, GroupMetadata> = Map::new("group_metadata");
pub const GROUP_ID_COUNTER: Item<GroupID> = Item::new("group_id_counter");
pub const GROUP_IX_NAME: IndexMap<(String, GroupID)> = Map::new("group_ix_name");
pub const GROUP_IX_CREATED_AT: IndexMap<(u64, GroupID)> = Map::new("group_ix_created_at");

pub fn initialize(ctx: Context, msg: InstantiateMsg) -> Result<(), ContractError> {
    let Context { deps, env, info } = ctx;

    deps.api
        .addr_validate(msg.config.owner.to_addr().as_str())?;

    CONFIG_OWNER.save(deps.storage, &msg.config.owner)?;
    CONFIG_CODE_ID_ALLOWLIST_ENABLED.save(deps.storage, &msg.config.code_id_allowlist_enabled)?;
    CONFIG_STR_MAX_LEN.save(deps.storage, &msg.config.max_str_len)?;

    CONTRACT_ID_COUNTER.save(deps.storage, &Uint64::zero())?;
    REPLY_JOB_ID_COUNTER.save(deps.storage, &Uint64::zero())?;
    GROUP_ID_COUNTER.save(deps.storage, &0)?;
    PARTITION_ID_COUNTER.save(deps.storage, &0)?;
    TABLE_INFO.save(deps.storage, &msg.info)?;

    for params in msg.partitions.unwrap_or_else(|| {
        vec![PartitionCreationParams {
            name: Some("Partition 1".to_owned()),
            description: None,
        }]
    }) {
        create_partition(deps.storage, env.block.time, &params)?;
    }

    for params in msg.indices.unwrap_or_default() {
        create_index(deps.storage, params)?;
    }

    for params in msg.groups.unwrap_or_default() {
        create_group(deps.storage, params, &info, &env)?;
    }

    Ok(())
}

pub fn create_group(
    storage: &mut dyn Storage,
    params: GroupCreationParams,
    info: &MessageInfo,
    env: &Env,
) -> Result<GroupMetadata, ContractError> {
    let group_id = increment_next_group_id(storage)?;
    let name = params.name.unwrap_or_else(|| group_id.to_string());
    let metadata = GroupMetadata {
        name: name.clone(),
        description: params.description,
        created_by: info.sender.clone(),
        created_at: env.block.time,
        size: Uint64::zero(),
    };

    GROUP_METADATA.save(storage, group_id, &metadata)?;
    GROUP_IX_NAME.save(storage, (name.clone(), group_id), &X)?;
    GROUP_IX_CREATED_AT.save(storage, (env.block.time.nanos(), group_id), &X)?;

    Ok(metadata)
}

fn increment_next_group_id(storage: &mut dyn Storage) -> Result<GroupID, ContractError> {
    GROUP_ID_COUNTER.update(storage, |n| -> Result<_, ContractError> {
        n.checked_add(1)
            .ok_or_else(|| ContractError::UnexpectedError {
                reason: "unexpected overflow incrementing group ID counter".to_owned(),
            })
    })
}

pub fn create_index(
    storage: &mut dyn Storage,
    params: IndexCreationParams,
) -> Result<IndexMetadata, ContractError> {
    INDEX_METADATA.update(
        storage,
        params.name.clone(),
        |maybe_meta| -> Result<_, ContractError> {
            if maybe_meta.is_some() {
                Err(ContractError::NotAuthorized {
                    reason: format!("index {} already exists", params.name),
                })
            } else {
                Ok(IndexMetadata {
                    size: Uint64::zero(),
                    index_type: params.index_type,
                    name: params.name,
                })
            }
        },
    )
}

pub fn build_contract_metadata_view(
    storage: &dyn Storage,
    id: ContractID,
    with_details: bool,
) -> Result<ContractMetadataView, ContractError> {
    let meta = CONTRACT_METADATA.load(storage, id)?;
    let (updated_at, rev, updated_at_height, updated_by) =
        match CONTRACT_DYN_METADATA.may_load(storage, id)? {
            Some(meta) => (
                Some(meta.updated_at),
                Some(meta.rev),
                Some(meta.updated_at_height),
                Some(meta.updated_by),
            ),
            None => (None, None, None, None),
        };

    Ok(ContractMetadataView {
        is_suspended: is_suspended(storage, id)?,
        partition: meta.partition,
        created_at: meta.created_at,
        updated_at,
        rev,
        details: if with_details {
            Some(ContractMetadataViewDetails {
                groups: load_contract_group_ids(storage, id)?,
                code_id: meta.code_id,
                created_at_height: meta.created_at_height,
                created_by: meta.created_by,
                is_managed: meta.is_managed,
                updated_at_height,
                updated_by,
                id: id.into(),
            })
        } else {
            None
        },
    })
}

pub fn ensure_allowed_by_acl(
    deps: &DepsMut,
    principal: &Addr,
    action: &str,
) -> Result<(), ContractError> {
    if !match CONFIG_OWNER.load(deps.storage)? {
        Owner::Address(addr) => *principal == addr,
        Owner::Acl(acl_addr) => {
            let acl = Acl::new(&acl_addr);
            acl.is_allowed(&deps.querier, principal, action)?
        }
    } {
        Err(ContractError::NotAuthorized {
            reason: "Owner authorization required".to_owned(),
        })
    } else {
        Ok(())
    }
}

pub fn ensure_partition_exists(
    storage: &dyn Storage,
    partition_id: PartitionID,
) -> Result<(), ContractError> {
    if !PARTITION_METADATA.has(storage, partition_id) {
        return Err(ContractError::PartitionNotFound {
            reason: format!("Partition ID {} does not exist", partition_id),
        });
    }
    Ok(())
}

pub fn is_suspended(storage: &dyn Storage, contract_id: ContractID) -> Result<bool, ContractError> {
    if let Some(is_suspended) = CONTRACT_SUSPENSIONS.may_load(storage, contract_id)? {
        return Ok(is_suspended);
    }
    Ok(false)
}

pub fn ensure_contract_not_suspended(
    storage: &dyn Storage,
    contract_id: ContractID,
) -> Result<(), ContractError> {
    if let Some(is_suspended) = CONTRACT_SUSPENSIONS.may_load(storage, contract_id)? {
        if is_suspended {
            return Err(ContractError::ContractSuspended { contract_id });
        }
    }
    Ok(())
}

pub fn save_config(storage: &mut dyn Storage, config: &Config) -> Result<(), ContractError> {
    // Load and save existing config as backup. This can be restored by the
    // updated owner by executing the Restore msg.
    let prev_config = load_config(storage)?;

    CONFIG_BACKUP.save(storage, &to_json_binary(&prev_config)?)?;

    // Overwrite existing config settings with new ones
    CONFIG_OWNER.save(storage, &config.owner)?;
    CONFIG_CODE_ID_ALLOWLIST_ENABLED.save(storage, &config.code_id_allowlist_enabled)?;
    Ok(())
}

pub fn load_config(storage: &dyn Storage) -> Result<Config, ContractError> {
    Ok(Config {
        owner: CONFIG_OWNER.load(storage)?,
        code_id_allowlist_enabled: CONFIG_CODE_ID_ALLOWLIST_ENABLED.load(storage)?,
        max_str_len: CONFIG_STR_MAX_LEN.load(storage)?,
    })
}

pub fn load_reply_job(storage: &dyn Storage, job_id: u64) -> Result<ReplyJob, ContractError> {
    if let Some(job) = REPLY_JOBS.may_load(storage, job_id)? {
        Ok(job)
    } else {
        Err(ContractError::JobNotFound {
            reason: format!("Create msg job {} not found", job_id),
        })
    }
}

pub fn load_contract_addr(
    storage: &dyn Storage,
    contract_id: ContractID,
) -> Result<Addr, ContractError> {
    if let Some(addr) = CONTRACT_ID_2_ADDR.may_load(storage, contract_id)? {
        Ok(addr)
    } else {
        Err(ContractError::NotAuthorized {
            reason: format!("Unrecognized contract ID: {}", contract_id),
        })
    }
}

pub fn load_contract_id(
    storage: &dyn Storage,
    contract_addr: &Addr,
) -> Result<ContractID, ContractError> {
    if let Some(id) = CONTRACT_ADDR_2_ID.may_load(storage, contract_addr)? {
        Ok(id.into())
    } else {
        Err(ContractError::NotAuthorized {
            reason: "Unrecognized contract address".to_owned(),
        })
    }
}

pub fn load_next_contract_id(
    storage: &mut dyn Storage,
    contract_addr: &Addr,
) -> Result<u64, ContractError> {
    // Make sure that the contract doesn't already exist.
    if CONTRACT_ADDR_2_ID.has(storage, contract_addr) {
        return Err(ContractError::NotAuthorized {
            reason: "address already exists".to_owned(),
        });
    }
    // Increment and return the ID counter. This is the new Id.
    let contract_id: ContractID = CONTRACT_ID_COUNTER
        .update(storage, |counter| -> Result<_, ContractError> {
            Ok(counter + Uint64::one())
        })?
        .into();

    CONTRACT_ADDR_2_ID.save(storage, contract_addr, &contract_id.into())?;
    CONTRACT_ID_2_ADDR.save(storage, contract_id.into(), contract_addr)?;

    Ok(contract_id)
}

pub fn create_relationship(
    storage: &mut dyn Storage,
    contract_id: ContractID,
    addr: &Addr,
    name: &String,
) -> Result<(), ContractError> {
    REL_ADDR_2_ID.save(
        storage,
        (addr.into(), name.clone(), contract_id.to_string()),
        &X,
    )?;
    REL_ID_2_ADDR.save(storage, (contract_id, name.clone(), addr.to_string()), &X)?;
    Ok(())
}

pub fn delete_relationship(
    storage: &mut dyn Storage,
    contract_id: ContractID,
    addr: &Addr,
    cannonical_name: &String,
) -> Result<(), ContractError> {
    REL_ADDR_2_ID.remove(
        storage,
        (
            addr.into(),
            cannonical_name.clone(),
            contract_id.to_string(),
        ),
    );
    REL_ID_2_ADDR.remove(
        storage,
        (contract_id, cannonical_name.clone(), addr.to_string()),
    );
    Ok(())
}

pub fn increment_tag_count(
    storage: &mut dyn Storage,
    partition: PartitionID,
    cannonical_tag: &String,
) -> Result<u32, ContractError> {
    PARTITION_TAG_COUNTS.update(
        storage,
        (partition, &cannonical_tag),
        |n| -> Result<_, ContractError> {
            n.unwrap_or_default()
                .checked_add(1)
                .ok_or_else(|| ContractError::UnexpectedError {
                    reason: format!(
                        "Overflow incrementing count for tag '{}' in partition {}",
                        cannonical_tag, partition
                    ),
                })
        },
    )
}

pub fn decrement_tag_count(
    storage: &mut dyn Storage,
    partition: PartitionID,
    cannonical_tag: &String,
) -> Result<u32, ContractError> {
    PARTITION_TAG_COUNTS.update(
        storage,
        (partition, &cannonical_tag),
        |n| -> Result<_, ContractError> {
            n.unwrap_or_default()
                .checked_sub(1)
                .ok_or_else(|| ContractError::UnexpectedError {
                    reason: format!(
                        "error subtracting tag count of 0 for tag '{}' in partition {}",
                        cannonical_tag, partition
                    ),
                })
        },
    )
}

pub fn load_one_contract_record(
    storage: &dyn Storage,
    id: u64,
    maybe_detail_level: Option<Details>,
) -> Result<ContractRecord, ContractError> {
    let record = ContractRecord {
        address: load_contract_addr(storage, id)?,
        meta: if let Some(level) = &maybe_detail_level {
            Some(build_contract_metadata_view(
                storage,
                id,
                *level == Details::Full,
            )?)
        } else {
            None
        },
    };
    Ok(record)
}

pub fn load_contract_records(
    storage: &dyn Storage,
    contract_ids: &Vec<u64>,
    maybe_detail_level: Option<Details>,
) -> Result<Vec<ContractRecord>, ContractError> {
    let mut contracts: Vec<ContractRecord> = Vec::with_capacity(contract_ids.len());

    for id in contract_ids.iter() {
        let record = ContractRecord {
            address: load_contract_addr(storage, *id)?,
            meta: if let Some(level) = &maybe_detail_level {
                Some(build_contract_metadata_view(
                    storage,
                    *id,
                    *level == Details::Full,
                )?)
            } else {
                None
            },
        };
        contracts.push(record);
    }

    Ok(contracts)
}

pub fn resolve_partition_id(
    storage: &dyn Storage,
    selector: PartitionSelector,
) -> Result<PartitionID, ContractError> {
    Ok(match selector {
        PartitionSelector::Id(id) => id,
        PartitionSelector::Name(name) => {
            PARTITION_NAME_2_ID
                .load(storage, name.clone())
                .map_err(|_| ContractError::PartitionNotFound {
                    reason: format!("Partition '{}' does not exist", name),
                })?
        }
    })
}

pub fn append_group(
    storage: &mut dyn Storage,
    group_id: GroupID,
    contract_id: ContractID,
) -> Result<(), ContractError> {
    if IX_GROUP.has(storage, (group_id, contract_id)) {
        return Ok(());
    }

    IX_GROUP.save(storage, (group_id, contract_id), &X)?;
    CONTRACT_GROUP_IDS.save(storage, (contract_id, group_id), &X)?;

    GROUP_METADATA.update(
        storage,
        group_id,
        |maybe_meta| -> Result<_, ContractError> {
            if let Some(mut meta) = maybe_meta {
                meta.size = meta.size.checked_add(Uint64::one()).map_err(|e| {
                    ContractError::UnexpectedError {
                        reason: format!(
                            "Error incrementing group {} size: {}",
                            group_id,
                            e.to_string()
                        ),
                    }
                })?;
                Ok(meta)
            } else {
                Err(ContractError::UnexpectedError {
                    reason: format!("Group {} not found", group_id),
                })
            }
        },
    )?;
    Ok(())
}

pub fn remove_from_group(
    storage: &mut dyn Storage,
    group_id: GroupID,
    contract_id: ContractID,
) -> Result<(), ContractError> {
    if !IX_GROUP.has(storage, (group_id, contract_id)) {
        return Ok(());
    }

    IX_GROUP.remove(storage, (group_id, contract_id));
    CONTRACT_GROUP_IDS.remove(storage, (contract_id, group_id));

    GROUP_METADATA.update(
        storage,
        group_id,
        |maybe_meta| -> Result<_, ContractError> {
            if let Some(mut meta) = maybe_meta {
                meta.size = meta.size.checked_sub(Uint64::one()).map_err(|e| {
                    ContractError::UnexpectedError {
                        reason: format!(
                            "Error decrementing group {} size: {}",
                            group_id,
                            e.to_string()
                        ),
                    }
                })?;
                Ok(meta)
            } else {
                Err(ContractError::UnexpectedError {
                    reason: format!("Group {} not found", group_id),
                })
            }
        },
    )?;
    Ok(())
}

pub fn load_contract_group_ids(
    storage: &dyn Storage,
    contract_id: ContractID,
) -> Result<Vec<GroupID>, ContractError> {
    let mut group_ids: Vec<GroupID> = Vec::with_capacity(2);
    for result in CONTRACT_GROUP_IDS
        .prefix(contract_id)
        .keys(storage, None, None, Order::Ascending)
    {
        let group_id = result.map_err(|e| ContractError::UnexpectedError {
            reason: format!(
                "error loading contract {} group ids: {}",
                contract_id,
                e.to_string()
            ),
        })?;
        group_ids.push(group_id);
    }
    Ok(group_ids)
}

pub fn create_partition(
    storage: &mut dyn Storage,
    time: Timestamp,
    params: &PartitionCreationParams,
) -> Result<PartitionID, ContractError> {
    let partition_id = increment_next_partition_id(storage)?;
    let name = params
        .name
        .clone()
        .unwrap_or_else(|| partition_id.to_string());

    // Save id into name -> ID lookup table.
    PARTITION_NAME_2_ID.save(storage, name.clone(), &partition_id)?;

    // Init partition metadata state.
    PARTITION_METADATA.update(
        storage,
        partition_id,
        |maybe_meta| -> Result<_, ContractError> {
            if maybe_meta.is_some() {
                Err(ContractError::NotAuthorized {
                    reason: format!("partition {} already exists", partition_id),
                })
            } else {
                Ok(PartitionMetadata {
                    description: params.description.clone(),
                    created_at: time,
                    name,
                })
            }
        },
    )?;

    Ok(partition_id)
}

fn increment_next_partition_id(storage: &mut dyn Storage) -> Result<PartitionID, ContractError> {
    PARTITION_ID_COUNTER.update(storage, |n| -> Result<_, ContractError> {
        n.checked_add(1)
            .ok_or_else(|| ContractError::UnexpectedError {
                reason: "unexpected overflow incrementing partition ID counter".to_owned(),
            })
    })
}

pub fn exists_contract_address(storage: &dyn Storage, addr: &Addr) -> bool {
    CONTRACT_ADDR_2_ID.has(storage, addr)
}

pub fn incr_decr_index_size(
    storage: &mut dyn Storage,
    index_name: &String,
    is_positive: bool,
) -> Result<(), ContractError> {
    INDEX_METADATA.update(
        storage,
        index_name.clone(),
        |maybe_meta| -> Result<_, ContractError> {
            if let Some(mut meta) = maybe_meta {
                if is_positive {
                    meta.size = meta.size.checked_add(Uint64::one()).map_err(|_| {
                        ContractError::UnexpectedError {
                            reason: format!("Overflow incrementing index {} size", index_name),
                        }
                    })?;
                } else {
                    meta.size = meta.size.checked_sub(Uint64::one()).map_err(|_| {
                        ContractError::UnexpectedError {
                            reason: format!("Overflow subtracting index {} size", index_name),
                        }
                    })?;
                }
                Ok(meta)
            } else {
                Err(ContractError::UnexpectedError {
                    reason: format!("Index {} not found", index_name),
                })
            }
        },
    )?;
    Ok(())
}
