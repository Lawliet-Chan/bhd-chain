#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::DispatchResult,
    ensure,
    weights::{
        ClassifyDispatch, DispatchClass, DispatchInfo, PaysFee, SimpleDispatchInfo, WeighData,
        Weight,
    },
};
use keccak_hasher::KeccakHasher;
use sp_core::ed25519::{Pair, Public, Signature};
use system::{ensure_signed};
use poc::{poc_hashing::calculate_scoop, shabal256::shabal256_deadline_fast};

pub trait Trait: system::Trait + timestamp::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Bhd {
        BaseTarget get(base_target): u64 = 10;
    }
}

decl_event! {
pub enum Event<T>
    where
    AccountId = <T as system::Trait>::AccountId
    {
        VerifyDeadline(AccountId, bool),
    }
}

decl_module! {
     pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;

        fn verify_deadline(origin, deadline: u64) -> DispatchResult {
            let miner = ensure_signed(origin)?;
            let height = <system::Module<T>>::block_number().saturated_into::<u64>();
            let scoop_data = calculate_scoop(height, &miner.0);
            let target = shabal256_deadline_fast(&scoop_data, &miner.0);
            let base_target = Self::base_target();
            let is_ok = ( deadline == target/base_target );
            Self::deposit_event(RawEvent::VerifyDeadline(miner, is_ok));
            Ok(())
        }
     }
}