use datastore::{Datastore, Transaction};
use super::sandbox::DatastoreTestSandbox;
use errors::Error;
use traits::Id;

pub fn should_fail_auth_with_a_bad_username<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let auth = sandbox.datastore.auth(I::default(), "foobar".to_string());
    assert!(auth.is_ok());
    assert!(!auth.unwrap());
}

pub fn should_fail_auth_with_a_bad_password<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let auth = sandbox.datastore.auth(sandbox.owner_id, "bad_token".to_string());
    assert!(auth.is_ok());
    assert!(!auth.unwrap());
}

pub fn should_successfully_auth_with_good_credentials<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let auth = sandbox.datastore.auth(sandbox.owner_id, sandbox.owner_secret.clone());
    assert!(auth.is_ok());
    assert!(auth.unwrap());
}

pub fn should_lookup_valid_accounts<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let results = sandbox.datastore.has_account(sandbox.owner_id);
    assert!(results.is_ok());
    assert!(results.unwrap());
}

pub fn should_fail_to_lookup_invalid_accounts<D, T, I>(sandbox: &mut DatastoreTestSandbox<D,
                                                                                          T,
                                                                                          I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let results = sandbox.datastore.has_account(I::default());
    assert!(results.is_ok());
    assert!(!results.unwrap());
}

pub fn should_fail_when_attempting_to_delete_invalid_accounts<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
	where D: Datastore<T, I>,
		  T: Transaction<I>,
		  I: Id {
    let result = sandbox.datastore.delete_account(I::default());
    assert_eq!(result.unwrap_err(), Error::AccountNotFound);
}
