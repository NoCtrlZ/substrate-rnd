use support::{decl_storage, decl_module, StorageMap, StorageValue, dispatch::Result};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{As, Hash};

pub trait Trait: system::Trait {}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
// 提案
pub struct Proposal <AccountId, Hash> {
	// 提案者のEOA
	proposer: AccountId,
	// 提案のHash値
	proposal_id: Hash,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
// 裁判所変更提案
pub struct Impeachment <AccountId, Hash> {
	// 提案者のEOA
	proposer: AccountId,
	// 提案のHash値
	impeachment_id: Hash,
	// 変更先裁判所アドレス
	new_court: AccountId,
}

decl_storage! {
    trait Store for Module<T: Trait> as Custom {
		// 人口
		Population: u64;
		// DAO構成員のIDとEOA
		People: map u64 => T::AccountId;
		// EOAがDAOに登録されているか
		IsRegistered: map T::AccountId => bool;

		// 提案の構造体の現在のHash(Debug用)
		PrevProposalId: T::Hash;
		// 提案のHashと提案の構造体のマッピング
		Proposals: map T::Hash => Proposal<T::AccountId, T::Hash>;

		// 裁判所アドレス
		Court: T::AccountId;
		// 裁判所変更提案の直近のHash(Debug用)
		PrevImpeachmentId: T::Hash;
		// 裁判所変更提案
		Impeachments: map T::Hash => Impeachment<T::AccountId, T::Hash>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// DAOの構成員として登録する処理
        fn register(origin, account: T::AccountId) -> Result {
			if !<IsRegistered<T>>::get(&account) {
				let id = <Population<T>>::get();
				<People<T>>::insert(id, &account);
				<Population<T>>::mutate(|n| *n += 1);
				<IsRegistered<T>>::insert(account, true);
			}
			Ok(())
		}

		// 提案を作成する処理
		fn propose(origin) -> Result {
			let sender = ensure_signed(origin)?;
			if <IsRegistered<T>>::get(&sender) {
				let proposal_id = (<system::Module<T>>::random_seed(), &sender, <PrevProposalId<T>>::get()).using_encoded(<T as system::Trait>::Hashing::hash);
				<PrevProposalId<T>>::put(&proposal_id);
				let new_proposal = Proposal {
					proposer: sender,
					proposal_id: proposal_id,
				};
				<Proposals<T>>::insert(proposal_id, new_proposal);
			}
			Ok(())
		}

		// 裁判所のアドレスを変える提案を作成する処理
		fn impeachment(origin, new_court: T::AccountId) -> Result {
			let sender = ensure_signed(origin)?;
			if <IsRegistered<T>>::get(&sender) {
				let impeachment_id = (<system::Module<T>>::random_seed(), &sender, <PrevImpeachmentId<T>>::get()).using_encoded(<T as system::Trait>::Hashing::hash);
				<PrevImpeachmentId<T>>::put(&impeachment_id);
				let new_impeachment = Impeachment {
					proposer: sender,
					impeachment_id: impeachment_id,
					new_court: new_court,
				};
				<Impeachments<T>>::insert(impeachment_id, new_impeachment);
			}
			Ok(())
		}
    }
}
