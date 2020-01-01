use support::{decl_storage, decl_module, StorageMap, dispatch::Result};
use system::ensure_signed;

pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as Custom {
		Vote: map T::AccountId => bool;
		Voted: map T::AccountId => bool;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn vote(origin, value: bool) -> Result {
			let sender = ensure_signed(origin)?;
			let is_voted = <Voted<T>>::get(&sender);

			if !is_voted {
				<Vote<T>>::insert(&sender, value);
				<Voted<T>>::insert(sender, true);
			}

			Ok(())
        }
    }
}
