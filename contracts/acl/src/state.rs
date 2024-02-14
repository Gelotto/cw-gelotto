use crate::error::ContractError;
use crate::msg::{BlacklistEntry, InstantiateMsg};
use crate::{client::Acl, util::split_path_str};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Order, Storage};
use cw_storage_plus::{Item, Map};
use gelotto_core::models::owner::Owner;

pub const MAX_PATH_LEN: usize = 1000;

pub const PRINCIPAL_TYPE_ADDRESS: u8 = 0;
pub const PRINCIPAL_TYPE_ROLE: u8 = 1;

pub const OWNER: Item<Owner> = Item::new("owner");
pub const CONFIG_NAME: Item<Option<String>> = Item::new("name");
pub const CONFIG_DESCRIPTION: Item<Option<String>> = Item::new("description");
pub const UNRESTRICTED_RESOURCES: Map<&String, bool> = Map::new("unrestricted_resources");
pub const IX_TREE: Map<(&String, &String), bool> = Map::new("ix_tree");
pub const IX_BLACKLIST: Map<(u8, &String), BlacklistEntry> = Map::new("ix_blacklist");
pub const IX_PRINCIPAL_RES: Map<(u8, &String, &String), bool> = Map::new("ix_principal_resource");
pub const IX_RES_PRINCIPAL: Map<(u8, &String, &String), bool> = Map::new("ix_resource_principal");
pub const IX_PRINCIPAL_ROLE: Map<(u8, &String, &String), bool> = Map::new("ix_principal_role");

/// Initialize contract state data.
pub fn initialize(
    deps: DepsMut,
    _env: &Env,
    info: &MessageInfo,
    msg: &InstantiateMsg,
) -> Result<(), ContractError> {
    OWNER.save(
        deps.storage,
        &msg.owner
            .clone()
            .unwrap_or_else(|| Owner::Address(info.sender.clone())),
    )?;

    CONFIG_NAME.save(deps.storage, &msg.name)?;
    CONFIG_DESCRIPTION.save(deps.storage, &msg.description)?;

    // perform initial authorizations
    if let Some(authorizations) = msg.authorizations.clone() {
        for auth in authorizations.iter() {
            let principal_type = auth.principal.as_u8();
            let principal_id = auth.principal.to_string();
            for res in auth.resources.iter() {
                IX_PRINCIPAL_RES.save(deps.storage, (principal_type, &principal_id, res), &true)?;
            }
        }
    }

    Ok(())
}

pub fn ensure_can_execute(
    deps: &DepsMut,
    principal: &Addr,
    path: &str,
) -> Result<(), ContractError> {
    if !match OWNER.load(deps.storage)? {
        Owner::Address(addr) => *principal == addr,
        Owner::Acl(acl_addr) => {
            let acl = Acl::new(&acl_addr);
            acl.is_allowed(&deps.querier, principal, path)?
        }
    } {
        Err(ContractError::NotAuthorized {})
    } else {
        Ok(())
    }
}

pub fn is_principal_allowed(
    storage: &dyn Storage,
    principal_type: u8,
    principal_id: &String,
    cannonical_path: &String,
) -> Result<bool, ContractError> {
    // NOTE: The order of the checks matters.
    // Is principal blacklisted?
    if IX_BLACKLIST.has(storage, (principal_type, principal_id)) {
        return Ok(false);
    }
    // If not specifically denied, is the resource permitted?
    if UNRESTRICTED_RESOURCES.has(storage, &cannonical_path) {
        return Ok(true);
    }
    // Is resource specifically allowed to principal?
    if let Some(is_allowed) =
        is_principal_allowed_by_path(storage, principal_type, principal_id, cannonical_path)?
    {
        return Ok(is_allowed);
    }
    // Check if principal is denied through one of its roles
    let mut is_allowed_thru_roles = false;
    for maybe_role in IX_PRINCIPAL_ROLE
        .prefix((principal_type, principal_id))
        .keys(storage, None, None, Order::Ascending)
    {
        let role = maybe_role?;
        if let Some(is_allowed) = is_role_allowed(storage, &role, cannonical_path)? {
            is_allowed_thru_roles = is_allowed;
            if !is_allowed {
                break;
            }
        }
    }

    Ok(is_allowed_thru_roles)
}

fn is_role_allowed(
    storage: &dyn Storage,
    role: &String,
    cannonical_path: &String,
) -> Result<Option<bool>, ContractError> {
    // NOTE: The order of the checks matters.
    // Is principal blacklisted?
    if IX_BLACKLIST.has(storage, (PRINCIPAL_TYPE_ROLE, role)) {
        return Ok(Some(false));
    }
    // Is resource specifically allowed to principal?
    is_principal_allowed_by_path(storage, PRINCIPAL_TYPE_ROLE, role, cannonical_path)
}

fn is_principal_allowed_by_path(
    storage: &dyn Storage,
    principal_type: u8,
    principal_id: &String,
    cannonical_path: &String,
) -> Result<Option<bool>, ContractError> {
    let mut path: String = cannonical_path.clone();
    let mut is_allowed = None;
    loop {
        if let Some(b) =
            IX_PRINCIPAL_RES.may_load(storage, (principal_type, principal_id, &path))?
        {
            if !b {
                is_allowed = Some(false);
                break;
            } else {
                is_allowed = Some(true);
            }
        }

        if path == "/" {
            break;
        } else {
            path = split_path_str(&path).0;
        }
    }
    Ok(is_allowed)
}

pub fn create_resource_if_not_exists(
    storage: &mut dyn Storage,
    child_path_str: &String,
) -> Result<(), ContractError> {
    let (parent_path, maybe_child) = split_path_str(child_path_str);
    if let Some(child) = maybe_child {
        // Save a mapping from parent path to the child resource
        if !IX_TREE.has(storage, (&parent_path, &child)) {
            IX_TREE.save(storage, (&parent_path, &child), &true)?;
        }
    }
    Ok(())
}
