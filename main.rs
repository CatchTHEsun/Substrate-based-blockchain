// Import required libraries
use substrate_runtime::{BuildStorage, RuntimeDefinition};
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};
use sc_service::{ChainType, Properties};
use node_template_runtime::{self, GenesisConfig, opaque::Block, BlockBuilder};
use sc_telemetry::{TelemetryHandle, TelemetryWorker};
use std::sync::{Arc, Mutex};
use sp_api::ConstructRuntimeApi;
use sc_rpc::RpcHandlers;

fn main() {
    // Set up Substrate node configuration
    let mut properties = Properties::new();
    properties.insert("chain.base_path", "/tmp/substrate-chain".into());
    properties.insert("chain.type", ChainType::Local.toString().into());
    
    // Create GenesisConfig with custom parameters
    let genesis_config = GenesisConfig {
        /* add your custom parameters here */
        ..
    };
    
    // Define the runtime API and register the necessary RPC handlers
    let mut telemetry = TelemetryHandle::spawn_default("node-template/1.0.0");
    let telemetry_worker = telemetry.create_telemetry_worker().unwrap();
    let mut rpc_handlers = RpcHandlers::new();
    node_template_runtime::rpc::register_all(&mut rpc_handlers);
    
    // Create the Substrate service and run it
    let service = sc_service::new_full::<Block, node_template_runtime::RuntimeApi>(
        &properties,
        &genesis_config,
        node_template_runtime::opaque::FullBlock::import,
        node_template_runtime::RuntimeApi::new(),
        Some(rpc_handlers),
        None,
        telemetry_worker,
        &sp_core::traits::NoStatusInstantiation,
    ).unwrap();
    
    // Start the Substrate service
    service.start();
}
