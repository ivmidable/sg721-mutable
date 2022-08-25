#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

pub mod contract;
pub mod msg;
use sg721::InstantiateMsg;
pub use sg721_base::Extension;

pub type Sg721Base<'a> = sg721_base::Cw721Base<'a>;

pub mod entry {
    use super::*;
    use crate::{
        contract::{_instantiate, mutate},
        msg::{ExecuteMsg, QueryMsg},
    };
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, StdResult};
    use sg721_base::entry::execute as _execute;
    use sg_std::Response;

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, sg721_base::ContractError> {
        let tract = Sg721Base::default();
        _instantiate(tract, deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, sg721_base::ContractError> {
        let tract = Sg721Base::default();
        match msg {
            ExecuteMsg::Mutate { token_id, token_uri } => mutate(tract, deps, env, info, token_id, token_uri),
            _ => _execute(deps, env, info, msg.into())
        }
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Sg721Base::default().query(deps, env, msg.into())
    }
}
