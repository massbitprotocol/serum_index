use arrayref::{array_ref, array_refs};
use serde::{Deserialize, Serialize};
use std::num::*;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CancelOrderInstruction {
    pub side: Side,
    pub order_id: u128,
    pub owner: Vec<u64>,
    pub owner_slot: u8,
}
impl CancelOrderInstruction {
    pub fn unpack(input: &[u8; 53]) -> Option<Self> {
        let (side, order_id, owner, owner_slot) = array_refs![input, 4, 16, 32, 1];
        let side = Side::unpack(side);
        if side.is_some() {
            Some(CancelOrderInstruction {
                side: side.unwrap(),
                order_id: u128::from_le_bytes(*order_id),
                owner: {
                    let arr = array_refs![owner, 8, 8, 8, 8];
                    vec![
                        u64::from_le_bytes(*arr.0),
                        u64::from_le_bytes(*arr.1),
                        u64::from_le_bytes(*arr.2),
                        u64::from_le_bytes(*arr.3),
                    ]
                },
                owner_slot: u8::from_le_bytes(*owner_slot),
            })
        } else {
            None
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CancelOrderInstructionV2 {
    pub side: Side,
    pub order_id: u128,
}
impl CancelOrderInstructionV2 {
    pub fn unpack(input: &[u8; 20]) -> Option<Self> {
        let (side, order_id) = array_refs![input, 4, 16];
        let side = Side::unpack(side);
        if side.is_some() {
            Some(CancelOrderInstructionV2 {
                side: side.unwrap(),
                order_id: u128::from_le_bytes(*order_id),
            })
        } else {
            None
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct InitializeMarketInstruction {
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub fee_rate_bps: u16,
    pub vault_signer_nonce: u64,
    pub pc_dust_threshold: u64,
}
impl InitializeMarketInstruction {
    pub fn unpack(input: &[u8; 34]) -> Option<Self> {
        let (coin_lot_size, pc_lot_size, fee_rate_bps, vault_signer_nonce, pc_dust_threshold) =
            array_refs![input, 8, 8, 2, 8, 8];

        Some(InitializeMarketInstruction {
            coin_lot_size: u64::from_le_bytes(*coin_lot_size),
            pc_lot_size: u64::from_le_bytes(*pc_lot_size),
            fee_rate_bps: u16::from_le_bytes(*fee_rate_bps),
            vault_signer_nonce: u64::from_le_bytes(*vault_signer_nonce),
            pc_dust_threshold: u64::from_le_bytes(*pc_dust_threshold),
        })
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NewOrderInstructionV1 {
    pub side: Side,
    pub limit_price: NonZeroU64,
    pub max_qty: NonZeroU64,
    pub order_type: OrderType,
    pub client_id: u64,
}
impl NewOrderInstructionV1 {
    pub fn unpack(input: &[u8; 32]) -> Option<Self> {
        let (side, limit_price, max_qty, order_type, client_id) = array_refs![input, 4, 8, 8, 4, 8];
        let side = Side::unpack(side);
        let limit_price = NonZeroU64::new(u64::from_le_bytes(*limit_price));
        let max_qty = NonZeroU64::new(u64::from_le_bytes(*max_qty));
        let order_type = OrderType::unpack(order_type);
        if side.is_some() && limit_price.is_some() && max_qty.is_some() && order_type.is_some() {
            Some(NewOrderInstructionV1 {
                side: side.unwrap(),
                limit_price: limit_price.unwrap(),
                max_qty: max_qty.unwrap(),
                order_type: order_type.unwrap(),
                client_id: u64::from_le_bytes(*client_id),
            })
        } else {
            None
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NewOrderInstructionV2 {
    pub side: Side,
    pub limit_price: NonZeroU64,
    pub max_qty: NonZeroU64,
    pub order_type: OrderType,
    pub client_id: u64,
    pub self_trade_behavior: SelfTradeBehavior,
}
impl NewOrderInstructionV2 {
    pub fn unpack(input: &[u8; 33]) -> Option<Self> {
        let (side, limit_price, max_qty, order_type, client_id, self_trade_behavior) =
            array_refs![input, 4, 8, 8, 4, 8, 1];
        let side = Side::unpack(side);
        let limit_price = NonZeroU64::new(u64::from_le_bytes(*limit_price));
        let max_qty = NonZeroU64::new(u64::from_le_bytes(*max_qty));
        let order_type = OrderType::unpack(order_type);
        let self_trade_behavior = SelfTradeBehavior::unpack(self_trade_behavior);
        if side.is_some()
            && limit_price.is_some()
            && max_qty.is_some()
            && order_type.is_some()
            && self_trade_behavior.is_some()
        {
            Some(NewOrderInstructionV2 {
                side: side.unwrap(),
                limit_price: limit_price.unwrap(),
                max_qty: max_qty.unwrap(),
                order_type: order_type.unwrap(),
                client_id: u64::from_le_bytes(*client_id),
                self_trade_behavior: self_trade_behavior.unwrap(),
            })
        } else {
            None
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NewOrderInstructionV3 {
    pub side: Side,
    pub limit_price: NonZeroU64,
    pub max_coin_qty: NonZeroU64,
    pub max_native_pc_qty_including_fees: NonZeroU64,
    pub self_trade_behavior: SelfTradeBehavior,
    pub order_type: OrderType,
    pub client_order_id: u64,
    pub limit: u16,
}
impl NewOrderInstructionV3 {
    pub fn unpack(input: &[u8; 46]) -> Option<Self> {
        let (
            side,
            limit_price,
            max_coin_qty,
            max_native_pc_qty_including_fees,
            self_trade_behavior,
            order_type,
            client_order_id,
            limit,
        ) = array_refs![input, 4, 8, 8, 8, 4, 4, 8, 2];
        let side = Side::unpack(side);
        let limit_price = NonZeroU64::new(u64::from_le_bytes(*limit_price));
        let max_coin_qty = NonZeroU64::new(u64::from_le_bytes(*max_coin_qty));
        let max_native_pc_qty_including_fees =
            NonZeroU64::new(u64::from_le_bytes(*max_native_pc_qty_including_fees));
        let self_trade_behavior = SelfTradeBehavior::unpack(self_trade_behavior);
        let order_type = OrderType::unpack(order_type);
        if side.is_some()
            && limit_price.is_some()
            && max_coin_qty.is_some()
            && max_native_pc_qty_including_fees.is_some()
            && self_trade_behavior.is_some()
            && order_type.is_some()
        {
            Some(NewOrderInstructionV3 {
                side: side.unwrap(),
                limit_price: limit_price.unwrap(),
                max_coin_qty: max_coin_qty.unwrap(),
                max_native_pc_qty_including_fees: max_native_pc_qty_including_fees.unwrap(),
                self_trade_behavior: self_trade_behavior.unwrap(),
                order_type: order_type.unwrap(),
                client_order_id: u64::from_le_bytes(*client_order_id),
                limit: u16::from_le_bytes(*limit),
            })
        } else {
            None
        }
    }
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum OrderType {
    Limit,
    ImmediateOrCancel,
    PostOnly,
}
impl OrderType {
    pub fn unpack(input: &[u8]) -> Option<Self> {
        let (&tag_slice, data) = array_refs![input, 4; ..;];
        let tag_val = u32::from_le_bytes(tag_slice) as u32;
        match tag_val {
            0 => Some(OrderType::Limit),
            1 => Some(OrderType::ImmediateOrCancel),
            2 => Some(OrderType::PostOnly),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum SelfTradeBehavior {
    DecrementTake,
    CancelProvide,
    AbortTransaction,
}
impl SelfTradeBehavior {
    pub fn unpack(input: &[u8]) -> Option<Self> {
        let (&tag_slice, data) = array_refs![input, 4; ..;];
        let tag_val = u32::from_le_bytes(tag_slice) as u32;
        match tag_val {
            0 => Some(SelfTradeBehavior::DecrementTake),
            1 => Some(SelfTradeBehavior::CancelProvide),
            2 => Some(SelfTradeBehavior::AbortTransaction),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SendTakeInstruction {
    pub side: Side,
    pub limit_price: NonZeroU64,
    pub max_coin_qty: NonZeroU64,
    pub max_native_pc_qty_including_fees: NonZeroU64,
    pub min_coin_qty: u64,
    pub min_native_pc_qty: u64,
    pub limit: u16,
}
impl SendTakeInstruction {
    pub fn unpack(input: &[u8; 46]) -> Option<Self> {
        let (
            side,
            limit_price,
            max_coin_qty,
            max_native_pc_qty_including_fees,
            min_coin_qty,
            min_native_pc_qty,
            limit,
        ) = array_refs![input, 4, 8, 8, 8, 8, 8, 2];
        let side = Side::unpack(side);
        let limit_price = NonZeroU64::new(u64::from_le_bytes(*limit_price));
        let max_coin_qty = NonZeroU64::new(u64::from_le_bytes(*max_coin_qty));
        let max_native_pc_qty_including_fees =
            NonZeroU64::new(u64::from_le_bytes(*max_native_pc_qty_including_fees));
        if side.is_some()
            && limit_price.is_some()
            && max_coin_qty.is_some()
            && max_native_pc_qty_including_fees.is_some()
        {
            Some(SendTakeInstruction {
                side: side.unwrap(),
                limit_price: limit_price.unwrap(),
                max_coin_qty: max_coin_qty.unwrap(),
                max_native_pc_qty_including_fees: max_native_pc_qty_including_fees.unwrap(),
                min_coin_qty: u64::from_le_bytes(*min_coin_qty),
                min_native_pc_qty: u64::from_le_bytes(*min_native_pc_qty),
                limit: u16::from_le_bytes(*limit),
            })
        } else {
            None
        }
    }
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Side {
    Bid,
    Ask,
}
impl Side {
    pub fn unpack(input: &[u8]) -> Option<Self> {
        let (&tag_slice, data) = array_refs![input, 4; ..;];
        let tag_val = u32::from_le_bytes(tag_slice) as u32;
        match tag_val {
            0 => Some(Side::Bid),
            1 => Some(Side::Ask),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum MarketInstruction {
    InitializeMarket(InitializeMarketInstruction),
    NewOrder(NewOrderInstructionV1),
    MatchOrders(u16),
    ConsumeEvents(u16),
    CancelOrder(CancelOrderInstruction),
    SettleFunds,
    CancelOrderByClientId(u64),
    DisableMarket,
    SweepFees,
    NewOrderV2(NewOrderInstructionV2),
    NewOrderV3(NewOrderInstructionV3),
    CancelOrderV2(CancelOrderInstructionV2),
    CancelOrderByClientIdV2(u64),
    SendTake(SendTakeInstruction),
    CloseOpenOrders,
    InitOpenOrders,
    Prune(u16),
    ConsumeEventsPermissioned(u16),
}
impl MarketInstruction {
    pub fn unpack(input: &[u8]) -> Option<Self> {
        let (&[offset], &tag_slice, data) = array_refs![input, 1, 4; ..;];
        let tag_val = u32::from_le_bytes(tag_slice) as u32;
        match tag_val {
            0 => {
                let field_slice = array_ref![data, 0, 34];
                let inner = InitializeMarketInstruction::unpack(field_slice);
                inner.and_then(|inner_val| Some(MarketInstruction::InitializeMarket(inner_val)))
            }
            1 => {
                let field_slice = array_ref![data, 0, 32];
                let inner = NewOrderInstructionV1::unpack(field_slice);
                inner.and_then(|inner_val| Some(MarketInstruction::NewOrder(inner_val)))
            }
            2 => {
                let field_slice = array_ref![data, 0, 2];
                Some(MarketInstruction::MatchOrders(u16::from_le_bytes(
                    *field_slice,
                )))
            }
            3 => {
                let field_slice = array_ref![data, 0, 2];
                Some(MarketInstruction::ConsumeEvents(u16::from_le_bytes(
                    *field_slice,
                )))
            }
            4 => {
                let field_slice = array_ref![data, 0, 53];
                let inner = CancelOrderInstruction::unpack(field_slice);
                inner.and_then(|inner_val| Some(MarketInstruction::CancelOrder(inner_val)))
            }
            5 => Some(MarketInstruction::SettleFunds),
            6 => {
                let field_slice = array_ref![data, 0, 8];
                Some(MarketInstruction::CancelOrderByClientId(
                    u64::from_le_bytes(*field_slice),
                ))
            }
            7 => Some(MarketInstruction::DisableMarket),
            8 => Some(MarketInstruction::SweepFees),
            9 => {
                let field_slice = array_ref![data, 0, 33];
                let inner = NewOrderInstructionV2::unpack(field_slice);
                inner.and_then(|inner_val| Some(MarketInstruction::NewOrderV2(inner_val)))
            }
            10 => {
                let field_slice = array_ref![data, 0, 46];
                let inner = NewOrderInstructionV3::unpack(field_slice);
                inner.and_then(|inner_val| Some(MarketInstruction::NewOrderV3(inner_val)))
            }
            11 => {
                let field_slice = array_ref![data, 0, 20];
                let inner = CancelOrderInstructionV2::unpack(field_slice);
                inner.and_then(|inner_val| Some(MarketInstruction::CancelOrderV2(inner_val)))
            }
            12 => {
                let field_slice = array_ref![data, 0, 8];
                Some(MarketInstruction::CancelOrderByClientIdV2(
                    u64::from_le_bytes(*field_slice),
                ))
            }
            13 => {
                let field_slice = array_ref![data, 0, 46];
                let inner = SendTakeInstruction::unpack(field_slice);
                inner.and_then(|inner_val| Some(MarketInstruction::SendTake(inner_val)))
            }
            14 => Some(MarketInstruction::CloseOpenOrders),
            15 => Some(MarketInstruction::InitOpenOrders),
            16 => {
                let field_slice = array_ref![data, 0, 2];
                Some(MarketInstruction::Prune(u16::from_le_bytes(*field_slice)))
            }
            17 => {
                let field_slice = array_ref![data, 0, 2];
                Some(MarketInstruction::ConsumeEventsPermissioned(
                    u16::from_le_bytes(*field_slice),
                ))
            }
            _ => None,
        }
    }
}
