#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate holochain_collections;

use hdk::{
    utils,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    // entry::Entry,
};
use holochain_collections::bucket_set::{
    self,
    BucketSetStorable,
    BucketIterable,
};


// create an example entry struct and implement BucktSetStorable
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MyEntry {
    content: String,
}

// have entries allocated to buckets according to the first letter of the content string
impl BucketSetStorable for MyEntry {
    fn derive_bucket_id(&self) -> String {
        self.content.chars().next().unwrap_or('\0').to_string()
    }
}

impl BucketIterable for MyEntry {
    fn buckets() -> Box<Iterator<Item = String>> {
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGIJKLMNOPQRSTUVWXYZ"
        .chars().map(|c| {
            c.to_string()
        });
        Box::new(alphabet)
    }
}

fn definition() -> ValidatingEntryType {
    entry!(
        name: "my_entry",
        description: "this is a same entry defintion",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
            Ok(())
        }
    )
}

/////////////////////////////////////////////////////////////////////////
// Zome functions
/////////////////////////////////////////////////////////////////////////

pub fn handle_create_my_entry(entry: MyEntry) -> ZomeApiResult<Address> {
    bucket_set::store("my_entry".into(), entry)
}

pub fn handle_get_my_entry(address: Address) -> ZomeApiResult<MyEntry> {
    utils::get_as_type(address)
}

pub fn handle_get_entries_by_bucket(bucket_id: String) -> ZomeApiResult<Vec<Address>> {
    bucket_set::retrieve("my_entry".into(), bucket_id)
}

pub fn handle_get_all_entries() -> ZomeApiResult<Vec<Address>> {
    bucket_set::retrieve_all::<MyEntry>("my_entry".into())
}

//////////////////////////////////////////////////////////////////////////


define_zome! {
    entries: [
       definition(),
       bucket_set::bucket_entry_def_for("my_entry".into()) // add the special bucket entry
    ]

    genesis: || { Ok(()) }

    functions: [
        create_my_entry: {
            inputs: |entry: MyEntry|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_my_entry
        }
        get_my_entry: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<MyEntry>|,
            handler: handle_get_my_entry
        }
        get_entries_by_bucket: {
            inputs: |bucket_id: String|,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: handle_get_entries_by_bucket
        }
        get_all_entries: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: handle_get_all_entries
        }
    ]

    traits: {
        hc_public [create_my_entry, get_my_entry, get_entries_by_bucket, get_all_entries]
    }
}
