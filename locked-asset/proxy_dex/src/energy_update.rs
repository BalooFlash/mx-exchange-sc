elrond_wasm::imports!();

use common_structs::Nonce;
use energy_factory::locked_token_transfer::ProxyTrait as _;
use energy_query::Energy;
use simple_lock::locked_token::LockedTokenAttributes;

static LOCKED_TOKEN_ID_STORAGE_KEY: &[u8] = b"lockedTokenId";

#[elrond_wasm::module]
pub trait EnergyUpdateModule:
    energy_query::EnergyQueryModule
    + utils::UtilsModule
    + crate::proxy_common::ProxyCommonModule
    + legacy_token_decode_module::LegacyTokenDecodeModule
{
    fn burn_locked_tokens_and_update_energy(
        &self,
        token_id: &TokenIdentifier,
        token_nonce: Nonce,
        token_amount: &BigUint,
        user_address: &ManagedAddress,
    ) {
        if token_amount == &0u64 {
            return;
        }

        self.deduct_energy_from_user(user_address, token_id, token_nonce, token_amount);
        self.send()
            .esdt_local_burn(token_id, token_nonce, token_amount);
    }

    fn deduct_energy_from_user(
        &self,
        user: &ManagedAddress,
        token_id: &TokenIdentifier,
        token_nonce: u64,
        token_amount: &BigUint,
    ) {
        let current_epoch = self.blockchain().get_block_epoch();
        let mut energy = self.get_energy_entry(user);

        let energy_factory_addr = self.energy_factory_address().get();
        let new_locked_token_id = self.get_locked_token_id(&energy_factory_addr);
        let is_new_token = token_id == &new_locked_token_id;
        let is_old_token = !self.factory_address_for_locked_token(token_id).is_empty();
        if is_new_token {
            let attributes: LockedTokenAttributes<Self::Api> =
                self.get_token_attributes(token_id, token_nonce);
            energy.update_after_unlock_any(token_amount, attributes.unlock_epoch, current_epoch);
        } else if is_old_token {
            let attributes = self.decode_legacy_token(token_id, token_nonce);
            let epoch_amount_pairs = attributes.get_unlock_amounts_per_epoch(token_amount);
            for pair in epoch_amount_pairs.pairs {
                energy.update_after_unlock_any(&pair.amount, pair.epoch, current_epoch);
            }
        } else {
            sc_panic!("Invalid token for energy update");
        }

        self.set_energy_in_factory(user.clone(), energy, energy_factory_addr);
    }

    fn set_energy_in_factory(
        &self,
        user: ManagedAddress,
        energy: Energy<Self::Api>,
        energy_factory_addr: ManagedAddress,
    ) {
        let _: () = self
            .energy_factory_proxy(energy_factory_addr)
            .set_user_energy_after_locked_token_transfer(user, energy)
            .execute_on_dest_context();
    }

    fn get_locked_token_id(&self, energy_factory_addr: &ManagedAddress) -> TokenIdentifier {
        self.storage_raw().read_from_address(
            energy_factory_addr,
            ManagedBuffer::new_from_bytes(LOCKED_TOKEN_ID_STORAGE_KEY),
        )
    }
}
