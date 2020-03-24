#![cfg_attr(not(feature = "std"), no_std)]

// use codec::{Decode, Encode};
use frame_support::{
    decl_event, decl_module, decl_storage,
    dispatch::DispatchResult,
};
use system::{ensure_signed};
use sp_runtime::traits::SaturatedConversion;
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
        fn deposit_event() = default;

        fn verify_deadline(origin, deadline: u64) -> DispatchResult {
            let miner = ensure_signed(origin)?;
            let miner_pub = AsRef::<[u8; 32]>::as_ref(&miner);
            let height = <system::Module<T>>::block_number().saturated_into::<u64>();
            let scoop_data = calculate_scoop(height, miner_pub.as_ref()).to_be_bytes().as_ref();
            let target = shabal256_deadline_fast(&scoop_data, miner_pub.as_ref());
            let base_target = Self::base_target();
            let is_ok = deadline == target/base_target;
            Self::deposit_event(RawEvent::VerifyDeadline(miner, is_ok));
            Ok(())
        }
     }
}