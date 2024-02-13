use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub enum Owner {
    Address(Addr),
    Acl(Addr),
}

impl Owner {
    pub fn to_addr(&self) -> Addr {
        match self {
            Owner::Address(addr) => addr.clone(),
            Owner::Acl(addr) => addr.clone(),
        }
    }
}
