use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CustomQuery, QuerierWrapper, StdResult, Coin};
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator, DisplayFromStr};

/// TODO: Maybe we need to deserialize as well.
/// A number of Custom messages that can call into the CoinMaster bindings
#[serde_as]
#[cw_serde]
pub enum CoinMasterMsg {
    Mint { 
        #[serde_as(serialize_as = "DisplayFromStr")]
        amount: Coin
    },
    Burn { 
        #[serde_as(serialize_as = "DisplayFromStr")]
        amount: Coin
    },
}

/// Coinmaster-specific queries
#[cw_serde]
#[derive(QueryResponses)]
pub enum CoinMasterQuery {
    #[returns(CoinMasterParamsResponse)]
    Params {},
}

/// TODO: look for a way to convert this String into an Addr.
#[serde_as]
#[cw_serde]
pub struct CoinMasterParams {
    #[serde_as(deserialize_as = "StringWithSeparator::<CommaSeparator, String>")]
    #[serde_as(serialize_as = "StringWithSeparator::<CommaSeparator, String>")]
    pub minters: Vec<String>,
    #[serde_as(deserialize_as = "StringWithSeparator::<CommaSeparator, String>")]
    #[serde_as(serialize_as = "StringWithSeparator::<CommaSeparator, String>")]
    pub denoms: Vec<String>,
}

#[cw_serde]
pub struct CoinMasterParamsResponse {
    pub params: CoinMasterParams,
}

pub trait CoinMasterQuerier {
    fn query_coin_master_params(&self) -> StdResult<CoinMasterParamsResponse>;
}

impl<'a, T> CoinMasterQuerier for QuerierWrapper<'a, T>
where
    T: CustomQuery + From<CoinMasterQuery>,
{
    fn query_coin_master_params(&self) -> StdResult<CoinMasterParamsResponse> {
        let custom_query: T = CoinMasterQuery::Params {}.into();
        self.query(&custom_query.into())
    }
}

pub trait CreateCoinMasterMsg {
    type Msg: From<CoinMasterMsg>;
    fn coin_master_mint(amount: Coin) -> StdResult<Self::Msg>;
    fn coin_master_burn(amount: Coin) -> StdResult<Self::Msg>;
}

impl<T> CreateCoinMasterMsg for T
where
    T: From<CoinMasterMsg>,
{
    type Msg = T;
    fn coin_master_mint(amount: Coin) -> StdResult<Self::Msg> {
        Ok(CoinMasterMsg::Mint {
            amount,
        }
        .into())
    }
    fn coin_master_burn(amount: Coin) -> StdResult<Self::Msg> {
        Ok(CoinMasterMsg::Burn {
            amount,
        }
        .into())
    }
}

// This export is added to all contracts that import this package, signifying that they require
// "coinmaster" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_coinmaster() {}
