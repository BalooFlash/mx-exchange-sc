#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub type Week = usize;
pub type Epoch = u64;
pub type Nonce = u64;

pub type TokenAmountPairsVec<M> = ManagedVec<M, TokenAmountPair<M>>;
pub type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;

#[derive(
    TypeAbi,
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    ManagedVecItem,
    PartialEq,
    Debug,
    Clone,
)]
pub struct TokenAmountPair<M: ManagedTypeApi> {
    pub token: TokenIdentifier<M>,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> TokenAmountPair<M> {
    #[inline]
    pub fn new(token: TokenIdentifier<M>, amount: BigUint<M>) -> Self {
        TokenAmountPair { token, amount }
    }
}
