use crate::generated::instruction::*;
use crate::STORE;
use massbit_solana_sdk::entity::{Attribute, Entity, Value};
use massbit_solana_sdk::types::SolanaBlock;
use serde_json;
use solana_program::pubkey::Pubkey;
use solana_transaction_status::TransactionWithStatusMeta;
use std::collections::HashMap;
use uuid::Uuid;

pub trait EntityExt {
    fn save(&self, entity_name: &str);
}
impl EntityExt for Entity {
    fn save(&self, entity_name: &str) {
        unsafe {
            STORE
                .as_mut()
                .unwrap()
                .save(String::from(entity_name), self.clone());
        }
    }
}

pub struct Handler {}
impl Handler {
    pub fn process(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        input: &[u8],
    ) {
        println!(
            "Process block {} with input {:?}",
            block.block_number, input
        );
        if let Some(instruction) = MarketInstruction::unpack(input) {
            match instruction {
                MarketInstruction::InitializeMarket(arg) => {
                    self.process_initialize_market(block, transaction, program_id, accounts, arg);
                }
                MarketInstruction::NewOrder(arg) => {
                    self.process_new_order(block, transaction, program_id, accounts, arg);
                }
                MarketInstruction::MatchOrders(arg) => {
                    self.process_match_orders(block, transaction, program_id, accounts, arg);
                }
                MarketInstruction::ConsumeEvents(arg) => {
                    self.process_consume_events(block, transaction, program_id, accounts, arg);
                }
                MarketInstruction::CancelOrder(arg) => {
                    self.process_cancel_order(block, transaction, program_id, accounts, arg);
                }
                MarketInstruction::SettleFunds => {
                    self.process_settle_funds(block, transaction, program_id, accounts);
                }
                MarketInstruction::CancelOrderByClientId(arg) => {
                    self.process_cancel_order_by_client_id(
                        block,
                        transaction,
                        program_id,
                        accounts,
                        arg,
                    );
                }
                MarketInstruction::DisableMarket => {
                    self.process_disable_market(block, transaction, program_id, accounts);
                }
                MarketInstruction::SweepFees => {
                    self.process_sweep_fees(block, transaction, program_id, accounts);
                }
                MarketInstruction::NewOrderV2(arg) => {
                    self.process_new_order_v2(block, transaction, program_id, accounts, arg);
                }
                MarketInstruction::NewOrderV3(arg) => {
                    self.process_new_order_v3(block, transaction, program_id, accounts, arg);
                }
                MarketInstruction::CancelOrderV2(arg) => {
                    self.process_cancel_order_v2(block, transaction, program_id, accounts, arg);
                }
                MarketInstruction::CancelOrderByClientIdV2(arg) => {
                    self.process_cancel_order_by_client_id_v2(
                        block,
                        transaction,
                        program_id,
                        accounts,
                        arg,
                    );
                }
                MarketInstruction::SendTake(arg) => {
                    self.process_send_take(block, transaction, program_id, accounts, arg);
                }
                MarketInstruction::CloseOpenOrders => {
                    self.process_close_open_orders(block, transaction, program_id, accounts);
                }
                MarketInstruction::InitOpenOrders => {
                    self.process_init_open_orders(block, transaction, program_id, accounts);
                }
                MarketInstruction::Prune(arg) => {
                    self.process_prune(block, transaction, program_id, accounts, arg);
                }
                MarketInstruction::ConsumeEventsPermissioned(arg) => {
                    self.process_consume_events_permissioned(
                        block,
                        transaction,
                        program_id,
                        accounts,
                        arg,
                    );
                }
            }
        }
    }
    pub fn process_initialize_market(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: InitializeMarketInstruction,
    ) -> Result<(), anyhow::Error> {
        println!("call function process_initialize_market for handle incoming block {} with argument {:?}", block.block_number, &arg);

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "request_queue".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "event_queue".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "bids".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "asks".to_string(),
            Value::from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "coin_currency".to_string(),
            Value::from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "price_currency".to_string(),
            Value::from(
                accounts
                    .get(6)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "coin_currency_mint".to_string(),
            Value::from(
                accounts
                    .get(7)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "price_currency_mint".to_string(),
            Value::from(
                accounts
                    .get(8)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "rent_sysvar".to_string(),
            Value::from(
                accounts
                    .get(9)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders_market_authority".to_string(),
            Value::from(
                accounts
                    .get(10)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "prune_authority".to_string(),
            Value::from(
                accounts
                    .get(11)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "crank_authority".to_string(),
            Value::from(
                accounts
                    .get(12)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert("coin_lot_size".to_string(), Value::from(arg.coin_lot_size));
        map.insert("pc_lot_size".to_string(), Value::from(arg.pc_lot_size));
        map.insert("fee_rate_bps".to_string(), Value::from(arg.fee_rate_bps));
        map.insert(
            "vault_signer_nonce".to_string(),
            Value::from(arg.vault_signer_nonce),
        );
        map.insert(
            "pc_dust_threshold".to_string(),
            Value::from(arg.pc_dust_threshold),
        );
        Entity::from(map).save("InitializeMarket");
        Ok(())
    }
    pub fn process_new_order(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: NewOrderInstructionV1,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_new_order for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "request_queue".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "account_paying".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "owner_openOrders_account".to_string(),
            Value::from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "coin_vault".to_string(),
            Value::from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pc_vault".to_string(),
            Value::from(
                accounts
                    .get(6)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_program".to_string(),
            Value::from(
                accounts
                    .get(7)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "rent_sysvar".to_string(),
            Value::from(
                accounts
                    .get(8)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "SRM_account".to_string(),
            Value::from(
                accounts
                    .get(9)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "side".to_string(),
            Value::from(serde_json::to_string(&arg.side).unwrap_or(Default::default())),
        );
        map.insert(
            "limit_price".to_string(),
            Value::from(arg.limit_price.get()),
        );
        map.insert("max_qty".to_string(), Value::from(arg.max_qty.get()));
        map.insert(
            "order_type".to_string(),
            Value::from(serde_json::to_string(&arg.order_type).unwrap_or(Default::default())),
        );
        map.insert("client_id".to_string(), Value::from(arg.client_id));
        Entity::from(map).save("NewOrder");
        Ok(())
    }
    pub fn process_match_orders(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: u16,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_match_orders for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "request_queue".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "event_queue".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "bids".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "asks".to_string(),
            Value::from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "coin_fee".to_string(),
            Value::from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pc_fee".to_string(),
            Value::from(
                accounts
                    .get(6)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert("value".to_string(), Value::from(arg));
        Entity::from(map).save("MatchOrders");
        Ok(())
    }
    pub fn process_consume_events(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: u16,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_consume_events for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert("value".to_string(), Value::from(arg));
        Entity::from(map).save("ConsumeEvents");
        Ok(())
    }
    pub fn process_cancel_order(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: CancelOrderInstruction,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_cancel_order for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "request_queue".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders_owner".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "side".to_string(),
            Value::from(serde_json::to_string(&arg.side).unwrap_or(Default::default())),
        );
        map.insert(
            "order_id".to_string(),
            Value::from(serde_json::to_string(&arg.order_id).unwrap_or(Default::default())),
        );
        map.insert(
            "owner".to_string(),
            Value::from(
                arg.owner
                    .iter()
                    .map(|&owner| Value::from(owner))
                    .collect::<Vec<Value>>(),
            ),
        );
        map.insert("owner_slot".to_string(), Value::from(arg.owner_slot));
        Entity::from(map).save("CancelOrder");
        Ok(())
    }
    pub fn process_settle_funds(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_settle_funds for handle incoming block {}",
            block.block_number
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders_owner".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "coin_vault".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pc_vault".to_string(),
            Value::from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "coin_wallet".to_string(),
            Value::from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pc_wallet".to_string(),
            Value::from(
                accounts
                    .get(6)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "vault_signer".to_string(),
            Value::from(
                accounts
                    .get(7)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_program".to_string(),
            Value::from(
                accounts
                    .get(8)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "referrer_pc_wallet".to_string(),
            Value::from(
                accounts
                    .get(9)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        Entity::from(map).save("SettleFunds");
        Ok(())
    }
    pub fn process_cancel_order_by_client_id(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: u64,
    ) -> Result<(), anyhow::Error> {
        println!("call function process_cancel_order_by_client_id for handle incoming block {} with argument {:?}", block.block_number, &arg);

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "request_queue".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders_owner".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert("value".to_string(), Value::from(arg));
        Entity::from(map).save("CancelOrderByClientId");
        Ok(())
    }
    pub fn process_disable_market(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_disable_market for handle incoming block {}",
            block.block_number
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "disable_authority".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        Entity::from(map).save("DisableMarket");
        Ok(())
    }
    pub fn process_sweep_fees(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_sweep_fees for handle incoming block {}",
            block.block_number
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pc_vault".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "fee_sweeping_authority".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "fee_receivable_account".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "vault_signer".to_string(),
            Value::from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_program".to_string(),
            Value::from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        Entity::from(map).save("SweepFees");
        Ok(())
    }
    pub fn process_new_order_v2(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: NewOrderInstructionV2,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_new_order_v2 for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "request_queue".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "account_paying_for_the_order".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders_owner".to_string(),
            Value::from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "coin_vault".to_string(),
            Value::from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pc_vault".to_string(),
            Value::from(
                accounts
                    .get(6)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_program".to_string(),
            Value::from(
                accounts
                    .get(7)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "rent_sysvar".to_string(),
            Value::from(
                accounts
                    .get(8)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "SRM_account".to_string(),
            Value::from(
                accounts
                    .get(9)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "side".to_string(),
            Value::from(serde_json::to_string(&arg.side).unwrap_or(Default::default())),
        );
        map.insert(
            "limit_price".to_string(),
            Value::from(arg.limit_price.get()),
        );
        map.insert("max_qty".to_string(), Value::from(arg.max_qty.get()));
        map.insert(
            "order_type".to_string(),
            Value::from(serde_json::to_string(&arg.order_type).unwrap_or(Default::default())),
        );
        map.insert("client_id".to_string(), Value::from(arg.client_id));
        map.insert(
            "self_trade_behavior".to_string(),
            Value::from(
                serde_json::to_string(&arg.self_trade_behavior).unwrap_or(Default::default()),
            ),
        );
        Entity::from(map).save("NewOrderV2");
        Ok(())
    }
    pub fn process_new_order_v3(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: NewOrderInstructionV3,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_new_order_v3 for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "request_queue".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "event_queue".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "bids".to_string(),
            Value::from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "asks".to_string(),
            Value::from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "account_paying_for_the_order".to_string(),
            Value::from(
                accounts
                    .get(6)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders_owner".to_string(),
            Value::from(
                accounts
                    .get(7)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "coin_vault".to_string(),
            Value::from(
                accounts
                    .get(8)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "pc_vault".to_string(),
            Value::from(
                accounts
                    .get(9)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "token_program".to_string(),
            Value::from(
                accounts
                    .get(10)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "rent_sysvar".to_string(),
            Value::from(
                accounts
                    .get(11)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "side".to_string(),
            Value::from(serde_json::to_string(&arg.side).unwrap_or(Default::default())),
        );
        map.insert(
            "limit_price".to_string(),
            Value::from(arg.limit_price.get()),
        );
        map.insert(
            "max_coin_qty".to_string(),
            Value::from(arg.max_coin_qty.get()),
        );
        map.insert(
            "max_native_pc_qty_including_fees".to_string(),
            Value::from(arg.max_native_pc_qty_including_fees.get()),
        );
        map.insert(
            "self_trade_behavior".to_string(),
            Value::from(
                serde_json::to_string(&arg.self_trade_behavior).unwrap_or(Default::default()),
            ),
        );
        map.insert(
            "order_type".to_string(),
            Value::from(serde_json::to_string(&arg.order_type).unwrap_or(Default::default())),
        );
        map.insert(
            "client_order_id".to_string(),
            Value::from(arg.client_order_id),
        );
        map.insert("limit".to_string(), Value::from(arg.limit));
        Entity::from(map).save("NewOrderV3");
        Ok(())
    }
    pub fn process_cancel_order_v2(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: CancelOrderInstructionV2,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_cancel_order_v2 for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "bids".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "asks".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders_owner".to_string(),
            Value::from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "event_queue".to_string(),
            Value::from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "side".to_string(),
            Value::from(serde_json::to_string(&arg.side).unwrap_or(Default::default())),
        );
        map.insert(
            "order_id".to_string(),
            Value::from(serde_json::to_string(&arg.order_id).unwrap_or(Default::default())),
        );
        Entity::from(map).save("CancelOrderV2");
        Ok(())
    }
    pub fn process_cancel_order_by_client_id_v2(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: u64,
    ) -> Result<(), anyhow::Error> {
        println!("call function process_cancel_order_by_client_id_v2 for handle incoming block {} with argument {:?}", block.block_number, &arg);

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "bids".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "asks".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert("value".to_string(), Value::from(arg));
        Entity::from(map).save("CancelOrderByClientIdV2");
        Ok(())
    }
    pub fn process_send_take(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: SendTakeInstruction,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_send_take for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "bids".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "asks".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "side".to_string(),
            Value::from(serde_json::to_string(&arg.side).unwrap_or(Default::default())),
        );
        map.insert(
            "limit_price".to_string(),
            Value::from(arg.limit_price.get()),
        );
        map.insert(
            "max_coin_qty".to_string(),
            Value::from(arg.max_coin_qty.get()),
        );
        map.insert(
            "max_native_pc_qty_including_fees".to_string(),
            Value::from(arg.max_native_pc_qty_including_fees.get()),
        );
        map.insert("min_coin_qty".to_string(), Value::from(arg.min_coin_qty));
        map.insert(
            "min_native_pc_qty".to_string(),
            Value::from(arg.min_native_pc_qty),
        );
        map.insert("limit".to_string(), Value::from(arg.limit));
        Entity::from(map).save("SendTake");
        Ok(())
    }
    pub fn process_close_open_orders(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_close_open_orders for handle incoming block {}",
            block.block_number
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "open_orders".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders_owner".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "destination_to_send_rent_exemption_sol".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        Entity::from(map).save("CloseOpenOrders");
        Ok(())
    }
    pub fn process_init_open_orders(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_init_open_orders for handle incoming block {}",
            block.block_number
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "open_orders".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders_owner".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "rent_sysvar".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders_market_authority".to_string(),
            Value::from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        Entity::from(map).save("InitOpenOrders");
        Ok(())
    }
    pub fn process_prune(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: u16,
    ) -> Result<(), anyhow::Error> {
        println!(
            "call function process_prune for handle incoming block {} with argument {:?}",
            block.block_number, &arg
        );

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert(
            "market".to_string(),
            Value::from(
                accounts
                    .get(0)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "bids".to_string(),
            Value::from(
                accounts
                    .get(1)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "asks".to_string(),
            Value::from(
                accounts
                    .get(2)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "prune_authority".to_string(),
            Value::from(
                accounts
                    .get(3)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders".to_string(),
            Value::from(
                accounts
                    .get(4)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "open_orders_owner".to_string(),
            Value::from(
                accounts
                    .get(5)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert(
            "event_queue".to_string(),
            Value::from(
                accounts
                    .get(6)
                    .and_then(|pubkey| Some(pubkey.to_string()))
                    .unwrap_or_default(),
            ),
        );
        map.insert("value".to_string(), Value::from(arg));
        Entity::from(map).save("Prune");
        Ok(())
    }
    pub fn process_consume_events_permissioned(
        &self,
        block: &SolanaBlock,
        transaction: &TransactionWithStatusMeta,
        program_id: &Pubkey,
        accounts: &Vec<Pubkey>,
        arg: u16,
    ) -> Result<(), anyhow::Error> {
        println!("call function process_consume_events_permissioned for handle incoming block {} with argument {:?}", block.block_number, &arg);

        let mut map: HashMap<Attribute, Value> = HashMap::default();
        map.insert(
            "id".to_string(),
            Value::from(Uuid::new_v4().to_simple().to_string()),
        );
        map.insert("value".to_string(), Value::from(arg));
        Entity::from(map).save("ConsumeEventsPermissioned");
        Ok(())
    }
}
