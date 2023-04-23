use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use bitbadges_cosmwasm::{BitBadgesMsg, BitBadgesQuery};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(BitBadgesMsg), &out_dir);
    export_schema(&schema_for!(BitBadgesQuery), &out_dir);
    // export_schema(&schema_for!(DenomResponse), &out_dir);
}
