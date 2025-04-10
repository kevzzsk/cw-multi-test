//! # Handler for `CosmosMsg::Stargate`, `CosmosMsg::Any`, `QueryRequest::Stargate` and `QueryRequest::Grpc` messages

use std::fmt::Debug;
use std::marker::PhantomData;
use crate::error::AnyResult;
use crate::{AppResponse, CosmosRouter};
use anyhow::bail;
use cosmwasm_std::{
    to_json_binary, Addr, AnyMsg, Api, Binary, BlockInfo, CustomMsg, CustomQuery, Empty, GrpcQuery,
    Querier, Storage,
};
use serde::de::DeserializeOwned;

/// Interface of handlers for processing `Stargate`/`Any` message variants
/// and `Stargate`/`Grpc` queries.
pub trait Stargate {
    /// Type of messages processed by the stargate instance.
    type ExecC;
    /// Type of queries processed by the stargate instance.
    type QueryC;

    /// Processes `CosmosMsg::Stargate` message variant.
    fn execute_stargate(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = Self::ExecC, QueryC = Self::QueryC>,
        _block: &BlockInfo,
        sender: Addr,
        type_url: String,
        value: Binary,
    ) -> AnyResult<AppResponse>
    where
        Self::ExecC: CustomMsg + DeserializeOwned + 'static,
        Self::QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        bail!(
            "Unexpected stargate execute: type_url={}, value={} from {}",
            type_url,
            value,
            sender,
        )
    }

    /// Processes `QueryRequest::Stargate` query.
    fn query_stargate(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        path: String,
        data: Binary,
    ) -> AnyResult<Binary> {
        bail!("Unexpected stargate query: path={}, data={}", path, data)
    }

    /// Processes `CosmosMsg::Any` message variant.
    fn execute_any(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = Self::ExecC, QueryC = Self::QueryC>,
        _block: &BlockInfo,
        sender: Addr,
        msg: AnyMsg,
    ) -> AnyResult<AppResponse>
    where
        Self::ExecC: CustomMsg + DeserializeOwned + 'static,
        Self::QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        bail!("Unexpected any execute: msg={:?} from {}", msg, sender)
    }

    /// Processes `QueryRequest::Grpc` query.
    fn query_grpc(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        request: GrpcQuery,
    ) -> AnyResult<Binary> {
        bail!("Unexpected grpc query: request={:?}", request)
    }
}

/// Always failing handler for `Stargate`/`Any` message variants and `Stargate`/`Grpc` queries.
pub struct StargateFailing<ExecT = Empty,QueryT = Empty>(PhantomData<(ExecT, QueryT)>);

impl<ExecT, QueryT> StargateFailing<ExecT, QueryT> {
    /// Creates an instance of a failing stargate.
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<ExecC, QueryC> Stargate for StargateFailing<ExecC,QueryC>
where
    ExecC: Debug,
    QueryC: Debug
{
    type ExecC = ExecC;
    type QueryC = QueryC;
}

/// Always accepting handler for `Stargate`/`Any` message variants and `Stargate`/`Grpc` queries.
pub struct StargateAccepting<ExecT,QueryT>(PhantomData<(ExecT, QueryT)>);

impl<ExecT, QueryT> StargateAccepting<ExecT, QueryT> {
    /// Creates an instance of an accepting stargate.
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<ExecC, QueryC> Stargate for StargateAccepting<ExecC, QueryC>
where
    ExecC: Debug,
    QueryC: Debug
{
    type ExecC = ExecC;
    type QueryC = QueryC;

    fn execute_stargate(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        _type_url: String,
        _value: Binary,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        Ok(AppResponse::default())
    }

    fn query_stargate(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        _path: String,
        _data: Binary,
    ) -> AnyResult<Binary> {
        to_json_binary(&Empty {}).map_err(Into::into)
    }

    fn execute_any(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        _msg: AnyMsg,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        Ok(AppResponse::default())
    }

    fn query_grpc(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        _request: GrpcQuery,
    ) -> AnyResult<Binary> {
        Ok(Binary::default())
    }
}
