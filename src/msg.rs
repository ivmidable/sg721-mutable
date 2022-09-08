use cosmwasm_std::Binary;
use cw_utils::Expiration;
use cw721_base::msg::QueryMsg as Cw721QueryMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sg721::MintMsg;
use sg721::ExecuteMsg as Sg721ExecuteMsg;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg<T> {
    _Ready {},
    TransferNft {
        recipient: String,
        token_id: String,
    },
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    Revoke {
        spender: String,
        token_id: String,
    },
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    RevokeAll {
        operator: String,
    },
    Mint(MintMsg<T>),
    Burn {
        token_id: String,
    },
    MutateMetadata {
        token_id: String,
        token_uri: String
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    NumTokens {},
    ContractInfo {},
    NftInfo {
        token_id: String,
    },
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    Minter {},
    CollectionInfo {},
}

impl<T> From<ExecuteMsg<T>> for Sg721ExecuteMsg<T>
{
    fn from(msg: ExecuteMsg<T>) -> Sg721ExecuteMsg<T> {
        match msg {
            ExecuteMsg::_Ready {} => Sg721ExecuteMsg::_Ready {},
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => Sg721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            },
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => Sg721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            },
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => Sg721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            },
            ExecuteMsg::ApproveAll { operator, expires } => {
                Sg721ExecuteMsg::ApproveAll { operator, expires }
            }
            ExecuteMsg::Revoke { spender, token_id } => {
                Sg721ExecuteMsg::Revoke { spender, token_id }
            }
            ExecuteMsg::RevokeAll { operator } => Sg721ExecuteMsg::RevokeAll { operator },
            ExecuteMsg::Mint(MintMsg {
                token_id,
                owner,
                token_uri,
                extension,
            }) => Sg721ExecuteMsg::Mint(MintMsg {
                token_id,
                owner,
                token_uri,
                extension,
            }),
            ExecuteMsg::Burn { token_id } => Sg721ExecuteMsg::Burn { token_id },
            _ => unreachable!("Invalid ExecuteMsg"),
        }
    }
}


impl From<QueryMsg> for Cw721QueryMsg {
    fn from(msg: QueryMsg) -> Cw721QueryMsg {
        match msg {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            },
            QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
            QueryMsg::ContractInfo {} => Cw721QueryMsg::ContractInfo {},
            QueryMsg::NftInfo { token_id } => Cw721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllTokens { start_after, limit } => {
                Cw721QueryMsg::AllTokens { start_after, limit }
            }
            QueryMsg::Minter {} => Cw721QueryMsg::Minter {},
            _ => unreachable!("cannot convert {:?} to Cw721QueryMsg", msg),
        }
    }
}
