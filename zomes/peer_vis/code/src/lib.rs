extern crate hdk;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::prelude::*;
use hdk::{
    AGENT_ADDRESS,
};

const ANCHOR: &str = "simple_anchor";
fn definition() -> ValidatingEntryType {
    entry!(
        name: ANCHOR,
        description: "this is a same entry defintion",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<String>| {
            Ok(())
        },
        links: [
            to!(
                "%agent_id",
                link_type: "peers",
                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PeerList {
    me: bool,
    address: Address
}

impl PeerList {
    pub fn new(agent: &Address) -> PeerList {
        if agent.to_string() == AGENT_ADDRESS.to_string() {
            PeerList {
                me: true,
                address: agent.to_owned(),
            }
        } else {
            PeerList {
                me: false,
                address: agent.to_owned(),
            }
        }

    }
}

pub fn ping(agent: &Address) -> ZomeApiResult<PeerList> {
    let message: String = "Hi".to_string();
    hdk::send(agent.to_owned(), message, 60000.into())?;
    Ok(PeerList::new(&agent))
}

fn get_anchor() -> ZomeApiResult<Address>{
    let entry = Entry::App(ANCHOR.into(), "PeerVis".into());
    hdk::commit_entry(&entry)
}
pub fn handle_get_peers() -> ZomeApiResult<Vec<PeerList>> {
    let anchor_address: Address = get_anchor()?;
    let agents: Vec<PeerList> = hdk::get_links(
        &anchor_address,
        LinkMatch::Exactly("peers"),
        LinkMatch::Any
    )?.addresses()
    .iter()
    .map(|agent| {
        ping(agent)
    })
    .filter_map(Result::ok)
    .collect::<Vec<PeerList>>();
    Ok(agents)
}


fn genesis() -> Result < (), String > {
    let anchor_address: Address = get_anchor()?;
    hdk::link_entries(
        &anchor_address,
        &AGENT_ADDRESS,
        "peers",
        "friend"
    ).map_err(|e| String::from(e))?;
    Ok(())
}

define_zome! {
    entries: [
        definition()
    ]

    init: || {
        genesis()
    }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    receive: |_from, _msg_json| {
        "Hi!".to_string()
    }

    functions: [
        get_peers: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<PeerList>>|,
            handler: handle_get_peers
        }
    ]

    traits: {
        hc_public [get_peers]
    }
}
