use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cosmwasm_std::Coin;

use cw_juror::msg::ExecuteMsg;
use gelotto_jury_lib::{msg::JurorInstantiateMsg, query::JurorQueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(JurorInstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(JurorQueryMsg), &out_dir);
    export_schema(&schema_for!(Coin), &out_dir);
}