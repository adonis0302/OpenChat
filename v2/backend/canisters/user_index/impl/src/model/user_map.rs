use crate::model::account_billing::AccountCharge;
use crate::model::user::{PhoneStatus, UnconfirmedPhoneNumber, User};
use crate::{CONFIRMATION_CODE_EXPIRY_MILLIS, CONFIRMED_PHONE_NUMBER_STORAGE_ALLOWANCE};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use types::{CyclesTopUp, Milliseconds, PhoneNumber, TimestampMillis, Timestamped, UserId, Version};
use utils::case_insensitive_hash_map::CaseInsensitiveHashMap;
use utils::time::{DAY_IN_MS, HOUR_IN_MS, MINUTE_IN_MS, WEEK_IN_MS};

const FIVE_MINUTES_IN_MS: Milliseconds = MINUTE_IN_MS * 5;
const THIRTY_DAYS_IN_MS: Milliseconds = DAY_IN_MS * 30;
const PRUNE_UNCONFIRMED_PHONE_NUMBERS_INTERVAL_MS: Milliseconds = MINUTE_IN_MS * 15;

#[derive(Serialize, Deserialize, Default)]
pub struct UserMap {
    users_by_principal: HashMap<Principal, User>,
    #[serde(skip)]
    phone_number_to_principal: HashMap<PhoneNumber, Principal>,
    #[serde(skip)]
    username_to_principal: CaseInsensitiveHashMap<Principal>,
    #[serde(skip)]
    user_id_to_principal: HashMap<UserId, Principal>,
    cached_metrics: Timestamped<Metrics>,
    #[serde(skip)]
    users_with_unconfirmed_phone_numbers: HashSet<Principal>,
    unconfirmed_phone_numbers_last_pruned: TimestampMillis,
    reserved_usernames: HashSet<String>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct Metrics {
    pub users_created: u64,
    pub users_online_5_minutes: u32,
    pub users_online_1_hour: u32,
    pub users_online_1_week: u32,
    pub users_online_1_month: u32,
}

impl UserMap {
    pub fn rehydrate(&mut self) {
        for (principal, user) in self.users_by_principal.iter() {
            match &user.phone_status {
                PhoneStatus::Confirmed(p) => {
                    self.phone_number_to_principal.insert(p.clone(), *principal);
                }
                PhoneStatus::Unconfirmed(p) => {
                    self.phone_number_to_principal.insert(p.phone_number.clone(), *principal);
                    self.users_with_unconfirmed_phone_numbers.insert(*principal);
                }
                _ => {}
            };

            self.username_to_principal.insert(&user.username, *principal);
            self.user_id_to_principal.insert(user.user_id, *principal);
        }
    }

    // Returns true if the username can be reserved or false if the username is taken
    pub fn reserve_username(&mut self, username: &str) -> bool {
        !self.username_to_principal.contains_key(username) && self.reserved_usernames.insert(username.to_owned())
    }

    pub fn release_username(&mut self, username: &str) {
        self.reserved_usernames.remove(username);
    }

    pub fn register(
        &mut self,
        principal: Principal,
        user_id: UserId,
        wasm_version: Version,
        username: String,
        now: TimestampMillis,
    ) {
        self.username_to_principal.insert(&username, principal);
        self.user_id_to_principal.insert(user_id, principal);

        let user = User::new(principal, user_id, username, now, wasm_version);
        self.users_by_principal.insert(principal, user);
    }

    pub fn update(&mut self, user: User) -> UpdateUserResult {
        let principal = user.principal;

        if let Some(previous) = self.users_by_principal.get(&principal) {
            let previous_phone_number = previous.phone_status.phone_number();
            let phone_number = user.phone_status.phone_number();
            let phone_number_changed = previous_phone_number != phone_number;

            let previous_username = &previous.username;
            let username = &user.username;
            let username_case_insensitive_changed = previous_username.to_uppercase() != username.to_uppercase();

            let previous_user_id = previous.user_id;
            let user_id = user.user_id;
            let user_id_changed = previous_user_id != user_id;

            if phone_number_changed {
                if let Some(phone_number) = phone_number {
                    if self.phone_number_to_principal.contains_key(phone_number) {
                        return UpdateUserResult::PhoneNumberTaken;
                    }
                }
            }

            if username_case_insensitive_changed && self.username_to_principal.contains_key(username) {
                return UpdateUserResult::UsernameTaken;
            }

            // Checks are complete, now update the data

            if phone_number_changed {
                if let Some(previous_phone_number) = previous_phone_number {
                    self.phone_number_to_principal.remove(previous_phone_number);
                }
                if let Some(phone_number) = phone_number {
                    self.phone_number_to_principal.insert(phone_number.clone(), principal);
                }
            }

            if username_case_insensitive_changed {
                self.username_to_principal.remove(previous_username);
                self.username_to_principal.insert(username, principal);
            }

            if user_id_changed {
                self.user_id_to_principal.remove(&previous_user_id);
                self.user_id_to_principal.insert(user_id, principal);
            }

            self.users_by_principal.insert(principal, user);
            UpdateUserResult::Success
        } else {
            UpdateUserResult::UserNotFound
        }
    }

    pub fn submit_phone_number(
        &mut self,
        principal: Principal,
        phone_number: PhoneNumber,
        confirmation_code: String,
        now: TimestampMillis,
    ) -> SubmitPhoneNumberResult {
        if let Some(user) = self.users_by_principal.get_mut(&principal) {
            match &mut user.phone_status {
                PhoneStatus::Confirmed(_) => return SubmitPhoneNumberResult::AlreadyConfirmed,
                PhoneStatus::Unconfirmed(p) => {
                    if p.phone_number != phone_number {
                        if self.phone_number_to_principal.contains_key(&phone_number) {
                            return SubmitPhoneNumberResult::PhoneNumberTaken;
                        }
                        self.phone_number_to_principal.remove(&p.phone_number);
                        self.phone_number_to_principal.insert(phone_number.clone(), principal);
                    }
                    p.phone_number = phone_number;
                    p.sms_messages_sent += 1;
                    p.confirmation_code = confirmation_code;
                    p.valid_until = now + CONFIRMATION_CODE_EXPIRY_MILLIS;
                }
                PhoneStatus::None => {
                    if self.phone_number_to_principal.contains_key(&phone_number) {
                        return SubmitPhoneNumberResult::PhoneNumberTaken;
                    }
                    self.phone_number_to_principal.insert(phone_number.clone(), principal);
                    user.phone_status = PhoneStatus::Unconfirmed(UnconfirmedPhoneNumber {
                        phone_number,
                        confirmation_code,
                        valid_until: now + CONFIRMATION_CODE_EXPIRY_MILLIS,
                        sms_messages_sent: 1,
                    });
                }
            }

            self.users_with_unconfirmed_phone_numbers.insert(principal);
            SubmitPhoneNumberResult::Success
        } else {
            SubmitPhoneNumberResult::UserNotFound
        }
    }

    pub fn confirm_phone_number(
        &mut self,
        principal: Principal,
        confirmation_code: String,
        test_mode: bool,
        now: TimestampMillis,
    ) -> ConfirmPhoneNumberResult {
        if let Some(user) = self.users_by_principal.get_mut(&principal) {
            match &user.phone_status {
                PhoneStatus::Confirmed(_) => ConfirmPhoneNumberResult::AlreadyConfirmed,
                PhoneStatus::Unconfirmed(p) => {
                    let test_code = test_mode && confirmation_code == "123456";

                    if now > p.valid_until {
                        ConfirmPhoneNumberResult::CodeExpired
                    } else if (confirmation_code != p.confirmation_code) && !test_code {
                        ConfirmPhoneNumberResult::CodeIncorrect
                    } else {
                        user.phone_status = PhoneStatus::Confirmed(p.phone_number.clone());
                        user.open_storage_limit_bytes += CONFIRMED_PHONE_NUMBER_STORAGE_ALLOWANCE;
                        ConfirmPhoneNumberResult::Success(Some(user.open_storage_limit_bytes))
                    }
                }
                // We remove unconfirmed phone numbers once their confirmation codes expire, so we
                // are not able to distinguish between a user who never submitted a phone number and
                // a user who's confirmation code has expired.
                PhoneStatus::None => ConfirmPhoneNumberResult::CodeExpired,
            }
        } else {
            ConfirmPhoneNumberResult::UserNotFound
        }
    }

    pub fn mark_online(&mut self, principal: &Principal, now: TimestampMillis) -> bool {
        if let Some(user) = self.users_by_principal.get_mut(principal) {
            user.last_online = now;
            true
        } else {
            false
        }
    }

    pub fn get_by_principal(&self, principal: &Principal) -> Option<&User> {
        self.users_by_principal.get(principal)
    }

    pub fn get_by_user_id(&self, user_id: &UserId) -> Option<&User> {
        self.user_id_to_principal
            .get(user_id)
            .map(|p| self.users_by_principal.get(p))
            .flatten()
    }

    pub fn get_by_username(&self, username: &str) -> Option<&User> {
        self.username_to_principal
            .get(username)
            .map(|p| self.users_by_principal.get(p))
            .flatten()
    }

    pub fn is_valid_caller(&self, caller: Principal) -> bool {
        self.users_by_principal.contains_key(&caller) || self.user_id_to_principal.contains_key(&caller.into())
    }

    pub fn mark_cycles_top_up(&mut self, user_id: &UserId, top_up: CyclesTopUp) -> bool {
        if let Some(user) = self.get_by_user_id_mut_internal(user_id) {
            user.mark_cycles_top_up(top_up);
            true
        } else {
            false
        }
    }

    pub fn set_avatar_id(&mut self, user_id: &UserId, avatar_id: Option<u128>, now: TimestampMillis) -> bool {
        if let Some(user) = self.get_by_user_id_mut_internal(user_id) {
            user.set_avatar_id(avatar_id, now);
            true
        } else {
            false
        }
    }

    pub fn record_account_charge(&mut self, user_id: &UserId, charge: AccountCharge) -> bool {
        if let Some(user) = self.get_by_user_id_mut_internal(user_id) {
            user.account_billing.add_charge(charge);
            true
        } else {
            false
        }
    }

    pub fn set_storage_limit(&mut self, user_id: &UserId, bytes: u64) -> bool {
        if let Some(user) = self.get_by_user_id_mut_internal(user_id) {
            user.open_storage_limit_bytes = bytes;
            true
        } else {
            false
        }
    }

    pub fn search(&self, term: &str) -> impl Iterator<Item = &User> {
        self.username_to_principal
            .search(term)
            .filter_map(move |p| self.users_by_principal.get(p))
    }

    // Remove unconfirmed phone numbers whose confirmation codes have expired, this frees up memory
    // and also allows the phone numbers to be reused.
    // This will only execute once every 15 minutes.
    pub fn prune_unconfirmed_phone_numbers_if_required(&mut self, now: TimestampMillis) -> Option<usize> {
        if now > self.unconfirmed_phone_numbers_last_pruned + PRUNE_UNCONFIRMED_PHONE_NUMBERS_INTERVAL_MS {
            let mut removed = Vec::new();

            for principal in self.users_with_unconfirmed_phone_numbers.iter() {
                if let Some(user) = self.users_by_principal.get_mut(principal) {
                    if let PhoneStatus::Unconfirmed(p) = &mut user.phone_status {
                        if now > p.valid_until {
                            self.phone_number_to_principal.remove(&p.phone_number);
                            user.phone_status = PhoneStatus::None;
                            removed.push(user.principal);
                        }
                    }
                }
            }

            for principal in removed.iter() {
                self.users_with_unconfirmed_phone_numbers.remove(principal);
            }
            self.unconfirmed_phone_numbers_last_pruned = now;
            Some(removed.len())
        } else {
            None
        }
    }

    pub fn metrics(&self) -> Metrics {
        self.cached_metrics.value.clone()
    }

    pub fn calculate_metrics(&mut self, now: TimestampMillis) {
        // Throttle to once every 5 minutes
        if now < self.cached_metrics.timestamp + FIVE_MINUTES_IN_MS {
            return;
        }

        let mut metrics = Metrics::default();

        for user in self.users_by_principal.values() {
            metrics.users_created += 1;
            if user.last_online > now - FIVE_MINUTES_IN_MS {
                metrics.users_online_5_minutes += 1;
            }
            if user.last_online > now - HOUR_IN_MS {
                metrics.users_online_1_hour += 1;
            }
            if user.last_online > now - WEEK_IN_MS {
                metrics.users_online_1_week += 1;
            }
            if user.last_online > now - THIRTY_DAYS_IN_MS {
                metrics.users_online_1_month += 1;
            }
        }

        self.cached_metrics = Timestamped::new(metrics, now);
    }

    pub fn iter(&self) -> impl Iterator<Item = &User> {
        self.users_by_principal.values()
    }

    pub fn len(&self) -> usize {
        self.users_by_principal.len()
    }

    fn get_by_user_id_mut_internal(&mut self, user_id: &UserId) -> Option<&mut User> {
        let principal = self.user_id_to_principal.get(user_id)?;
        self.users_by_principal.get_mut(principal)
    }

    #[cfg(test)]
    pub fn add_test_user(&mut self, user: User) {
        self.register(
            user.principal,
            user.user_id,
            user.wasm_version,
            user.username.clone(),
            user.date_created,
        );
        self.update(user);
    }
}

#[derive(Debug)]
pub enum UpdateUserResult {
    Success,
    PhoneNumberTaken,
    UsernameTaken,
    UserNotFound,
}

pub enum SubmitPhoneNumberResult {
    Success,
    PhoneNumberTaken,
    AlreadyConfirmed,
    UserNotFound,
}

pub enum ConfirmPhoneNumberResult {
    Success(Option<u64>),
    CodeExpired,
    CodeIncorrect,
    AlreadyConfirmed,
    UserNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::user::{PhoneStatus, User};
    use itertools::Itertools;

    #[test]
    fn register_with_no_clashes() {
        let mut user_map = UserMap::default();
        let principal1 = Principal::from_slice(&[1]);
        let principal2 = Principal::from_slice(&[2]);
        let principal3 = Principal::from_slice(&[3]);

        let phone_number3 = PhoneNumber::new(44, "3333 333 333".to_owned());

        let username1 = "1".to_string();
        let username2 = "2".to_string();
        let username3 = "3".to_string();

        let user_id1: UserId = Principal::from_slice(&[3, 1]).into();
        let user_id2: UserId = Principal::from_slice(&[3, 2]).into();
        let user_id3: UserId = Principal::from_slice(&[3, 3]).into();

        user_map.register(principal1, user_id1, Version::new(0, 0, 0), username1.clone(), 1);
        user_map.register(principal2, user_id2, Version::new(0, 0, 0), username2.clone(), 2);
        user_map.register(principal3, user_id3, Version::new(0, 0, 0), username3.clone(), 3);
        user_map.submit_phone_number(principal3, phone_number3.clone(), "123".to_string(), 4);

        let phone_number_to_principal: Vec<_> = user_map
            .phone_number_to_principal
            .iter()
            .map(|(ph, p)| (ph.clone(), p.clone()))
            .sorted_by_key(|(_, p)| *p)
            .collect();
        let user_id_to_principal: Vec<_> = user_map
            .user_id_to_principal
            .iter()
            .map(|(u, p)| (u.clone(), p.clone()))
            .sorted_by_key(|(_, p)| *p)
            .collect();
        let username_to_principal: Vec<_> = user_map
            .username_to_principal
            .iter()
            .map(|(u, p)| (u.clone(), p.clone()))
            .sorted_by_key(|(_, p)| *p)
            .collect();

        assert_eq!(user_map.users_by_principal.len(), 3);

        assert_eq!(phone_number_to_principal, vec!((phone_number3, principal3)));
        assert_eq!(
            username_to_principal,
            vec!((username1, principal1), (username2, principal2), (username3, principal3))
        );
        assert_eq!(
            user_id_to_principal,
            vec!((user_id1, principal1), (user_id2, principal2), (user_id3, principal3))
        );
    }

    #[test]
    fn submit_phone_number_with_clashing_phone_number() {
        let mut user_map = UserMap::default();
        let principal1 = Principal::from_slice(&[1]);
        let principal2 = Principal::from_slice(&[2]);

        let user_id1 = Principal::from_slice(&[1, 1]).into();
        let user_id2 = Principal::from_slice(&[2, 2]).into();

        let phone_number = PhoneNumber::new(44, "1111 111 111".to_owned());

        user_map.register(principal1, user_id1, Version::new(0, 0, 0), "1".to_string(), 1);
        user_map.submit_phone_number(principal1, phone_number.clone(), "123".to_string(), 2);
        user_map.register(principal2, user_id2, Version::new(0, 0, 0), "2".to_string(), 3);

        assert!(matches!(
            user_map.submit_phone_number(principal2, phone_number, "123".to_string(), 4),
            SubmitPhoneNumberResult::PhoneNumberTaken
        ));
    }

    #[test]
    fn update_with_no_clashes() {
        let mut user_map = UserMap::default();
        let principal = Principal::from_slice(&[1]);

        let phone_number1 = PhoneNumber::new(44, "1111 111 111".to_owned());
        let phone_number2 = PhoneNumber::new(44, "2222 222 222".to_owned());

        let username1 = "1".to_string();
        let username2 = "2".to_string();

        let user_id = Principal::from_slice(&[1, 1]).into();

        user_map.register(principal, user_id, Version::new(0, 0, 0), username1, 1);
        user_map.submit_phone_number(principal, phone_number1, "123".to_string(), 2);

        if let Some(original) = user_map.get_by_principal(&principal) {
            let mut updated = original.clone();
            updated.username = username2.clone();
            updated.phone_status = PhoneStatus::Confirmed(phone_number2.clone());

            assert!(matches!(user_map.update(updated), UpdateUserResult::Success));

            assert_eq!(user_map.users_by_principal.keys().collect_vec(), vec!(&principal));
            assert_eq!(user_map.phone_number_to_principal.keys().collect_vec(), vec!(&phone_number2));
            assert_eq!(user_map.username_to_principal.len(), 1);
            assert!(user_map.username_to_principal.contains_key(&username2));
            assert_eq!(user_map.user_id_to_principal.keys().collect_vec(), vec!(&user_id));
        }
    }

    #[test]
    fn update_with_clashing_phone_number() {
        let mut user_map = UserMap::default();
        let principal1 = Principal::from_slice(&[1]);
        let principal2 = Principal::from_slice(&[2]);

        let phone_number1 = PhoneNumber::new(44, "1111 111 111".to_owned());
        let phone_number2 = PhoneNumber::new(44, "2222 222 222".to_owned());

        let username1 = "1".to_string();
        let username2 = "2".to_string();

        let user_id1 = Principal::from_slice(&[1, 1]).into();
        let user_id2 = Principal::from_slice(&[2, 2]).into();

        let original = User {
            principal: principal1,
            phone_status: PhoneStatus::Confirmed(phone_number1),
            user_id: user_id1,
            username: username1.clone(),
            date_created: 1,
            date_updated: 1,
            last_online: 1,
            ..Default::default()
        };

        let other = User {
            principal: principal2,
            phone_status: PhoneStatus::Confirmed(phone_number2.clone()),
            user_id: user_id2,
            username: username2.clone(),
            date_created: 2,
            date_updated: 2,
            last_online: 2,
            ..Default::default()
        };

        let mut updated = original.clone();
        updated.phone_status = PhoneStatus::Confirmed(phone_number2);

        user_map.add_test_user(original);
        user_map.add_test_user(other);
        assert!(matches!(user_map.update(updated), UpdateUserResult::PhoneNumberTaken));
    }

    #[test]
    fn update_with_clashing_username() {
        let mut user_map = UserMap::default();
        let principal1 = Principal::from_slice(&[1]);
        let principal2 = Principal::from_slice(&[2]);

        let phone_number1 = PhoneNumber::new(44, "1111 111 111".to_owned());
        let phone_number2 = PhoneNumber::new(44, "2222 222 222".to_owned());

        let username1 = "1".to_string();
        let username2 = "2".to_string();

        let user_id1 = Principal::from_slice(&[1, 1]).into();
        let user_id2 = Principal::from_slice(&[2, 2]).into();

        let original = User {
            principal: principal1,
            phone_status: PhoneStatus::Confirmed(phone_number1),
            user_id: user_id1,
            username: username1.clone(),
            date_created: 1,
            date_updated: 1,
            last_online: 1,
            ..Default::default()
        };

        let other = User {
            principal: principal2,
            phone_status: PhoneStatus::Confirmed(phone_number2.clone()),
            user_id: user_id2,
            username: username2.clone(),
            date_created: 2,
            date_updated: 2,
            last_online: 2,
            ..Default::default()
        };

        let mut updated = original.clone();
        updated.username = username2;

        user_map.add_test_user(original);
        user_map.add_test_user(other);
        assert!(matches!(user_map.update(updated), UpdateUserResult::UsernameTaken));
    }

    #[test]
    fn update_username_change_casing() {
        let mut user_map = UserMap::default();
        let principal = Principal::from_slice(&[1]);
        let phone_number = PhoneNumber::new(44, "1111 111 111".to_owned());
        let username = "abc".to_string();
        let user_id = Principal::from_slice(&[1, 1]).into();

        let original = User {
            principal,
            phone_status: PhoneStatus::Confirmed(phone_number),
            user_id: user_id,
            username: username.clone(),
            date_created: 1,
            date_updated: 1,
            last_online: 1,
            ..Default::default()
        };

        let mut updated = original.clone();

        user_map.add_test_user(original);
        updated.username = "ABC".to_string();

        assert!(matches!(user_map.update(updated), UpdateUserResult::Success));
    }

    #[test]
    fn prune_unconfirmed_phone_numbers_runs_once_every_specified_interval() {
        let mut now = 1_000_000;
        let mut user_map = UserMap::default();
        assert_eq!(user_map.prune_unconfirmed_phone_numbers_if_required(now), Some(0));
        now += PRUNE_UNCONFIRMED_PHONE_NUMBERS_INTERVAL_MS;
        assert_eq!(user_map.prune_unconfirmed_phone_numbers_if_required(now), None);
        now += 1;
        assert_eq!(user_map.prune_unconfirmed_phone_numbers_if_required(now), Some(0));
    }

    #[test]
    fn prune_unconfirmed_phone_numbers_only_removes_users_with_expired_codes() {
        let mut now = 1_000_000;
        let mut user_map = UserMap::default();

        let principal1 = Principal::from_slice(&[1]);
        let principal2 = Principal::from_slice(&[2]);

        let user_id1 = Principal::from_slice(&[1, 1]).into();
        let user_id2 = Principal::from_slice(&[2, 2]).into();

        let phone_number1 = PhoneNumber::new(44, "1111 111 111".to_owned());
        let phone_number2 = PhoneNumber::new(44, "2222 222 222".to_owned());

        let user1 = User {
            principal: principal1,
            user_id: user_id1,
            username: "1".to_string(),
            ..Default::default()
        };

        let user2 = User {
            principal: principal2,
            user_id: user_id2,
            username: "2".to_string(),
            ..Default::default()
        };

        user_map.add_test_user(user1);
        user_map.add_test_user(user2);

        user_map.submit_phone_number(principal1, phone_number1.clone(), "1".to_string(), now);

        now += 1;

        user_map.submit_phone_number(principal2, phone_number2.clone(), "2".to_string(), now);

        now += CONFIRMATION_CODE_EXPIRY_MILLIS;

        assert_eq!(user_map.prune_unconfirmed_phone_numbers_if_required(now), Some(1));
        assert_eq!(
            user_map.users_with_unconfirmed_phone_numbers.iter().copied().collect_vec(),
            vec![principal2]
        );
        assert!(user_map
            .users_by_principal
            .get(&principal1)
            .unwrap()
            .phone_status
            .phone_number()
            .is_none());
        assert_eq!(
            user_map.phone_number_to_principal.keys().cloned().collect_vec(),
            vec![phone_number2]
        );
    }
}
