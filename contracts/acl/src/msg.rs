use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use gelotto_core::models::owner::Owner;

use crate::state::{PRINCIPAL_TYPE_ADDRESS, PRINCIPAL_TYPE_ROLE};

#[cw_serde]
pub struct Authorization {
    pub principal: Principal,
    pub resources: Vec<String>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<Owner>,
    pub authorizations: Option<Vec<Authorization>>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[cw_serde]
pub enum PrincipalMsg {
    Allow {
        principal: Principal,
        resources: Vec<String>,
    },
    Deny {
        principal: Principal,
        resources: Vec<String>,
        clear: Option<bool>,
    },
    GrantRole {
        principal: Principal,
        roles: Vec<String>,
    },
    RevokeRole {
        principal: Principal,
        roles: Vec<String>,
    },
    Ban {
        principal: Principal,
        reason: Option<String>,
    },
    Unban {
        principal: Principal,
    },
}

#[cw_serde]
pub enum ResourcesMsg {
    Open { resources: Vec<String> },
    Close { resources: Vec<String> },
}

#[cw_serde]
pub enum AdminMsg {
    SetOwner(Owner),
}

#[cw_serde]
pub enum ExecuteMsg {
    Resources(ResourcesMsg),
    Principal(PrincipalMsg),
    Admin(AdminMsg),
}

#[cw_serde]
pub enum Principal {
    Address(Addr),
    Role(String),
}

impl Principal {
    pub fn as_u8(&self) -> u8 {
        match self {
            Principal::Address(..) => PRINCIPAL_TYPE_ADDRESS,
            Principal::Role(..) => PRINCIPAL_TYPE_ROLE,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Principal::Address(addr) => addr.to_string(),
            Principal::Role(role) => role.clone(),
        }
    }
}

#[cw_serde]
pub enum QueryMsg {
    Principal(PrincipalQueryMsg),
    Resources(ResourcesQueryMsg),
}

#[cw_serde]
pub enum PrincipalQueryMsg {
    IsAllowed {
        principal: Principal,
        resources: Vec<String>,
    },
    HasRoles {
        principal: Principal,
        roles: Vec<String>,
    },
    Resources {
        principal: Principal,
        cursor: Option<String>,
    },
    Roles {
        principal: Principal,
        cursor: Option<String>,
    },
}

#[cw_serde]
pub enum ResourcesQueryMsg {
    Get {
        path: String,
        cursor: Option<String>,
        principal: Option<Principal>,
    },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct BlacklistEntry {
    pub reason: Option<String>,
}

#[cw_serde]
pub struct ResourceNode {
    pub path: String,
    pub children: Vec<ResourceNode>,
    pub is_allowed: Option<bool>,
}

#[cw_serde]
pub struct LsResponse {
    pub resource: ResourceNode,
    pub cursor: Option<String>,
}
