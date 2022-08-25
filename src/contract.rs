use sg721_base::contract::create;

use cosmwasm_std::{DepsMut, Env, MessageInfo};
use cw2::set_contract_version;

use sg721::InstantiateMsg;
use sg_std::Response;

use crate::Sg721Base;

use sg721_base::state::COLLECTION_INFO;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:sg721-mutable";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn _instantiate(
    contract: Sg721Base,
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, sg721_base::ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    create(contract, deps, info, msg)
}

pub fn mutate(
    contract: Sg721Base,
    deps:DepsMut,
    _env:Env,
    info:MessageInfo,
    token_id:String,
    token_uri:String
) -> Result<Response, sg721_base::ContractError> {
    //make sure sender is the creator.
    let collection_info = COLLECTION_INFO.load(deps.storage)?;
    if collection_info.creator != info.sender.to_string() {
        return Err(sg721_base::ContractError::Unauthorized { });
    }
    //variable to take ownership of old uri string.
    let mut old_uri:String = "".to_string();

    //update token_uri if token_id is valid and if token_uri is filled in.
    contract.tokens
    .update(deps.storage, &token_id, |token| match token {
        Some(mut token_info) => {
            match token_info.token_uri {
                Some(uri) => {
                    old_uri = uri.clone();
                    token_info.token_uri = Some(token_uri.clone());
                    Ok(token_info)
                },
                None => Err(sg721_base::ContractError::Unauthorized {  })
            }
        },
        None => Err(sg721_base::ContractError::Unauthorized {  }),
    })?;
    Ok(Response::new()
    .add_attribute("action", "mutate")
    .add_attribute("token_id", token_id)
    .add_attribute("old_token_uri", old_uri)
    .add_attribute("token_uri", token_uri))
}
