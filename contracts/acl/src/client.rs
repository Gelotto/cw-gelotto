use cosmwasm_std::{Addr, Empty, QuerierWrapper, StdResult};

use crate::msg::{Principal, PrincipalQueryMsg, QueryMsg};

pub struct Acl {
    acl_addr: Addr,
}

impl Acl {
    pub fn new(address: &Addr) -> Self {
        Self {
            acl_addr: address.clone(),
        }
    }

    pub fn is_allowed(
        &self,
        querier: &QuerierWrapper<Empty>,
        addresss: &Addr,
        path: &str,
    ) -> StdResult<bool> {
        Ok(querier.query_wasm_smart::<bool>(
            self.acl_addr.clone(),
            &QueryMsg::Principal(PrincipalQueryMsg::IsAllowed {
                principal: Principal::Address(addresss.clone()),
                resources: vec![path.to_string()],
            }),
        )?)
    }
}
