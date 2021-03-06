#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_event, decl_module, decl_storage,
    dispatch::DispatchResult,
};
use frame_system::{ensure_signed};
use sp_runtime::traits::SaturatedConversion;
use sp_std::vec::Vec;

use poc::{poc_hashing::{calculate_scoop, find_best_deadline_rust}, nonce::noncegen_rust};

pub trait Trait: frame_system::Trait + timestamp::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Bhd {
        BaseTarget get(base_target): u64 = 10;
    }
}

decl_event! {
pub enum Event<T>
    where
    AccountId = <T as frame_system::Trait>::AccountId
    {
        VerifyDeadline(AccountId, bool),
    }
}

decl_module! {
     pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn verify_deadline(origin, account_id: u64, sig: [u8; 32], nonce: u64, deadline: u64) -> DispatchResult {
            let miner = ensure_signed(origin)?;
            let height = <frame_system::Module<T>>::block_number().saturated_into::<u64>();
            let scoop_data = calculate_scoop(height, &sig) as u64;

            let mut cache = Vec::with_capacity(262144);
            noncegen_rust(cache.as_mut(), account_id, nonce, 1);
            let mirror_scoop_data = Self::gen_mirror_scoop_data(scoop_data, cache);

            let (target, _) = find_best_deadline_rust(mirror_scoop_data.as_ref(), 1, &sig);
            let base_target = Self::base_target();
            let is_ok = deadline == target/base_target;
            Self::deposit_event(RawEvent::VerifyDeadline(miner, is_ok));
            Ok(())
        }
     }
}

impl<T: Trait> Module<T> {
    fn gen_mirror_scoop_data(scoop_data: u64, cache: Vec<u8>) -> Vec<u8>{
        let addr = 64 * scoop_data as usize;
        let mirror_scoop = 4095 - scoop_data as usize;
        let mirror_addr = 64 * mirror_scoop as usize;
        let mut mirror_scoop_data = Vec::<u8>::with_capacity(64);
        mirror_scoop_data[0..32].clone_from_slice(&cache[addr..addr + 32]);
        mirror_scoop_data[32..64].clone_from_slice(&cache[mirror_addr + 32..mirror_addr + 64]);
        mirror_scoop_data
    }
}