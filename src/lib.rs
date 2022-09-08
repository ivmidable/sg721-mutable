#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, StdError, Empty};

pub mod contract;
pub mod msg;
use sg721::InstantiateMsg;
pub use sg721_base::Extension;

pub type Sg721Base<'a> = sg721_base::Cw721Base<'a>;

pub mod entry {
    use super::*;
    use crate::{
        contract::mutate_metadata,
        msg::{ExecuteMsg, QueryMsg},
    };
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, StdResult};
    use sg721_base::entry::execute as _execute;
    use sg_std::Response;

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> Result<Response, sg721_base::ContractError> {
        unimplemented!()
        //let tract = Sg721Base::default();
        //_instantiate(tract, deps, env, info, msg)
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
            ExecuteMsg::MutateMetadata { token_id, token_uri } => mutate_metadata(tract, deps, env, info, token_id, token_uri),
            _ => _execute(deps, env, info, msg.into())
        }
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Sg721Base::default().query(deps, env, msg.into())
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(
        deps: DepsMut,
        _env: Env,
        _msg: Empty,
    ) -> Result<Response, sg721_base::ContractError> {
        /*let ver = cw2::get_contract_version(deps.storage)?;

        if ver.contract != contract::BASE_CONTRACT_NAME {
            return Err(StdError::generic_err("Can only upgrade from same type").into());
        }
        
        if ver.version >= contract::CONTRACT_VERSION.to_string() {
            return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
        }*/
        
        // set the new version
        cw2::set_contract_version(deps.storage, contract::CONTRACT_NAME, contract::CONTRACT_VERSION)?;
        Ok(Response::default())
    }
}
