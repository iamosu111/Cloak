use super::*;
use anyhow::Ok;
use howlong::Duration;
use log::info;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QueryParam{
    #[serde(rename = "blk_height")]
    pub block_id:IdType,
    #[serde(rename = "query_slice")]
    pub slice: Vec<Vec<i32>>,
    #[serde(rename = "coefficient_p")]
    pub p: usize,
    #[serde(rename = "coefficient_lambda")]
    pub lambda:usize
}

/// res_txs for block query transactions, and boundary check.
/// res_sigs for aggregate_sinatures of each block
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverallResult{
    #[serde(rename = "result")]
    pub res_txs: Vec<i32>,
    pub query_param: QueryParam,
    pub query_time_ms: u64,
}

impl OverallResult {
    // pub async fn verify(
    //     &self,
    //     chain: &impl LightNodeInterface
    // )
    // -> Result<(VerifyResult, Duration)>{
    //     let cpu_timer = howlong::ProcessCPUTimer::new();
    //     let timer = howlong::HighResolutionTimer::new();
    //     // let res = self.inner_verify(chain).await?;
    //     let res = self.aggre_verify(chain).await?;
    //     let time = timer.elapsed();
    //     info!("verify used time {}",cpu_timer.elapsed());
        
    //     Ok((res, time))
    // }

    // // async fn inner_verify(&self, chain: &impl LightNodeInterface) -> Result<VerifyResult>{
    // //     let mut result = VerifyResult::default();
    // //     let mut signature: Option<Signature>;
    // //     let mut block_header: BlockHeader;
    // //     let ctx = signing_context(b"");
    // //     for (id, txs) in self.res_txs.0.iter(){
    // //         signature = self.res_sigs.0.get(id).unwrap().to_owned();
    // //         block_header = chain.lightnode_read_block_header(id.to_owned()).await?;
    // //         if signature.eq(&Option::None){
    // //             //this means no satisfying txs in block(id)
    // //             //and the Vec stores boundary conditions 
    // //             continue;
    // //         }
    // //         let mut aggre_string_txs: String = String::from("");
    // //         let public_key = PublicKey::recover(block_header.public_key);
    // //         for tx in txs {
    // //             aggre_string_txs += &serde_json::to_string(&tx).unwrap();
    // //         }
    // //         //verify failed, malicious actions exist
    // //         if public_key.verify(ctx.bytes(aggre_string_txs.as_bytes()), &signature.unwrap()).is_err() {
    // //             result.add(InvalidReason::InvalidSignature);
    // //         }
    // //     }

    // //     Ok(result)
    // // }

    // async fn aggre_verify(&self, chain: &impl LightNodeInterface) -> Result<VerifyResult>{
    //     let mut result = VerifyResult::default();
    //     Ok(result)
    // }
}



// #[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
// pub struct TimeRange([Option<TsType>; 2]);


pub fn historical_query(q_param: &QueryParam, chain: &impl ReadInterface) 
 -> Result<OverallResult>{
    info!("process query {:?}", q_param);
    
    let cpu_timer = howlong::ProcessCPUTimer::new();
    let mut block_data=chain.read_block_data(q_param.block_id)?;
    let mut res_txs=Vec::new();
    let mut result = OverallResult {
        res_txs: res_txs.clone(),
        query_param: q_param.clone(),
        query_time_ms: 0,
    };
    let (q,u,v)=Coefficient_server(q_param.p, block_data.tx_ids.len());
    let mut txs_value=Vec::new();
    for tx_id in 0..block_data.tx_ids.len() as IdType {
        txs_value.push(chain.read_transaction(tx_id)?.value.trans_value);
    }
    res_txs=Response(&q_param.slice, q_param.p, q, u, v, q_param.lambda, block_data.tx_ids.len(), &txs_value);


    result.res_txs = res_txs.clone();
    info!("used time: {:?}", cpu_timer.elapsed());
    Ok(result)
}


