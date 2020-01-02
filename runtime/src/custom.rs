use support::{decl_storage, decl_module, StorageMap, StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as Custom {
		Population: u64;
		People: map u64 => T::AccountId;
		IsRegistered: map T::AccountId => bool;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn register(origin, account: T::AccountId) -> Result {
			if !<IsRegistered<T>>::get(&account) {
				let id = <Population<T>>::get();
				<People<T>>::insert(id, &account);
				<Population<T>>::put(id + 1);
				<IsRegistered<T>>::insert(account, true);
			}
			Ok(())
		}
    }
}
