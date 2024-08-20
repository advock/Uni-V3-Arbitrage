use ethers::prelude::*;
use log::error;
use log::info;
use std::{
    collections::{btree_map::Entry, BTreeMap},
    sync::Arc,
    time::Duration,
};
use tokio::time::timeout;

pub async fn get_state_diffs(
    client: &Arc<Provider<Ws>>,
    meats: &Vec<Transaction>,
    block_num: BlockNumber,
) -> Option<BTreeMap<Address, AccountDiff>> {
    // add statediff trace to each transaction
    let req = meats
        .iter()
        .map(|tx| (tx, vec![TraceType::StateDiff]))
        .collect();

    let block_traces = match client.trace_call_many(req, Some(block_num)).await {
        Ok(x) => x,
        Err(_) => {
            return None;
        }
    };

    let mut merged_state_diffs = BTreeMap::new();

    block_traces
        .into_iter()
        .flat_map(|bt| bt.state_diff.map(|sd| sd.0.into_iter()))
        .flatten()
        .for_each(|(address, account_diff)| {
            match merged_state_diffs.entry(address) {
                Entry::Vacant(entry) => {
                    entry.insert(account_diff);
                }
                Entry::Occupied(_) => {
                    // Do nothing if the key already exists
                    // we only care abt the starting state
                }
            }
        });

    Some(merged_state_diffs)
}

pub async fn get_logs(
    client: &Arc<Provider<Ws>>,
    tx: &Transaction,
    block_num: BlockNumber,
) -> Option<Vec<CallLogFrame>> {
    // add statediff trace to each transaction

    let mut trace_ops = GethDebugTracingCallOptions::default();
    let mut call_config = CallConfig::default();
    call_config.with_log = Some(true);
    info!("before trace_ops");

    trace_ops.tracing_options.tracer = Some(GethDebugTracerType::BuiltInTracer(
        GethDebugBuiltInTracerType::CallTracer,
    ));
    trace_ops.tracing_options.tracer_config = Some(GethDebugTracerConfig::BuiltInTracer(
        GethDebugBuiltInTracerConfig::CallTracer(call_config),
    ));
    let block_num = Some(BlockId::Number(block_num));
    info!("before call_frame ");

    let mut retries = 3;
    while retries > 0 {
        match timeout(
            Duration::from_secs(5),
            client.debug_trace_call(tx, block_num, trace_ops.clone()),
        )
        .await
        {
            Ok(Ok(GethTrace::Known(GethTraceFrame::CallTracer(call_frame)))) => {
                let mut logs = Vec::new();
                extract_logs(&call_frame, &mut logs);
                return Some(logs);
            }
            Ok(Ok(_)) => {
                error!("Unexpected GethTraceFrame variant");
                return None;
            }
            Ok(Err(e)) => {
                error!("debug_trace_call error: {:?}", e);
            }
            Err(_) => {
                error!("debug_trace_call timed out");
            }
        }
        retries -= 1;
        tokio::time::sleep(Duration::from_secs(2)).await; // Wait before retrying
    }

    None
}

fn extract_logs(call_frame: &CallFrame, logs: &mut Vec<CallLogFrame>) {
    if let Some(ref logs_vec) = call_frame.logs {
        logs.extend(logs_vec.iter().cloned());
    }

    if let Some(ref calls_vec) = call_frame.calls {
        for call in calls_vec {
            extract_logs(call, logs);
        }
    }
}
