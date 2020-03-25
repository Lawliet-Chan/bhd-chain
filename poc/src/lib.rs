
use sp_runtime_interface::runtime_interface;

mod shabal256;
mod poc_hashing;

#[runtime_interface]
pub trait Poc {
    fn calculate_scoop(&self, height: u64, gensig: &[u8; 32]) -> u32 {
        poc_hashing::calculate_scoop(height, gensig)
    }

    fn shabal256_deadline_fast(&self, data: &[u8], gensig: &[u8; 32]) -> u64 {
        shabal256::shabal256_deadline_fast(data, gensig)
    }
}
