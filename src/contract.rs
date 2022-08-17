use cosmwasm_std::{to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult, Storage};
use std::convert::TryFrom;
use crate::msg::{HandleMsg, InitMsg, QueryMsg, HandleAnswer, QueryAnswer};
use crate::state::{load, may_load, save, State, Reminder, CONFIG_KEY, Nodecount};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let s=State{
        nodename:msg.nodename.clone(),
        website:msg.website.clone(),
        address:env.message.sender.to_string(),
    };
    let c:Nodecount=Nodecount { count: 1 };

    save(&mut deps.storage, CONFIG_KEY,&c)?;    
    let con:&[u8]=msg.nodename.as_bytes();

    save(&mut deps.storage, con,&s)?;
    Ok(InitResponse::default())
    // add init constructor functionality here
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        // add handle transaction execution code here 
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        // add query execution code here
    }
}