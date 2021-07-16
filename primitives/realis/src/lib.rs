#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::{Decode, Encode};
use primitive_types::U256;
use sp_std::vec::Vec;

pub use evm::backend::{Basic as Account, Log};

pub type TokenId = U256;
pub type Basic = u8;

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub struct Token {
    pub id: TokenId,
    pub token_type: TokenType,
    // market_type: 	MarketType
}

// #[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
// pub enum MarketType {
// 	Tradeable 	(TradeStatus),
// 	Untradeable
// }

// #[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
// pub enum TradeStatus {
// 	ForSale,
// 	OnHands,
// }

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub enum TokenType {
    // Skin 		,
    Mergeable(Mergeable),
    Stackable(Stackable),
    Basic(Basic),
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub struct Mergeable {
    pub rarity: Rarity,
    pub socket: Socket,
    pub params: Params,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub enum Stackable {
    Silver,
    Gold,
    Diamond,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythical,
    Legendary,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub enum Socket {
    Head,
    Body,
    LegLeft,
    LegRight,
    ArmLeft,
    ArmRight,
    Weapon,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub struct Params {
    pub strength: u8,
    pub agility: u8,
    pub intelligence: u8,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
pub struct OpaqueExtrinsic(Vec<u8>);

impl OpaqueExtrinsic {
    /// Convert an encoded extrinsic to an `OpaqueExtrinsic`.
    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self, codec::Error> {
        OpaqueExtrinsic::decode(&mut bytes)
    }
}