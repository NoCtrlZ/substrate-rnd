use support::{decl_storage, decl_module, StorageMap, StorageValue, dispatch::Result};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{As, Hash};

pub trait Trait: system::Trait {}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Proposal <AccountId, Hash> {
	proposer: AccountId,
	proposal_id: Hash,
}

decl_storage! {
    trait Store for Module<T: Trait> as Custom {
		Population: u64;
		People: map u64 => T::AccountId;
		IsRegistered: map T::AccountId => bool;
		PrevProposalId: T::Hash;
		Proposals: map T::Hash => Proposal<T::AccountId, T::Hash>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn register(origin, account: T::AccountId) -> Result {
			if !<IsRegistered<T>>::get(&account) {
				let id = <Population<T>>::get();
				<People<T>>::insert(id, &account);
				<Population<T>>::mutate(|n| *n += 1);
				<IsRegistered<T>>::insert(account, true);
			}
			Ok(())
		}

		fn propose(origin) -> Result {
			let sender = ensure_signed(origin)?;
			if <IsRegistered<T>>::get(&sender) {
				let proposal_id = (<system::Module<T>>::random_seed(), &sender, <Population<T>>::get()).using_encoded(<T as system::Trait>::Hashing::hash);
				<PrevProposalId<T>>::put(&proposal_id);
				let new_proposal = Proposal {
					proposer: sender,
					proposal_id: proposal_id,
				};
				<Proposals<T>>::insert(proposal_id, new_proposal);
			}
			Ok(())
		}
    }
}
