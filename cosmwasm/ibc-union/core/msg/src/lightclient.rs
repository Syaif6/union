use unionlabs_primitives::Bytes;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Status {
    Active,
    Expired,
    Frozen,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct VerifyClientMessageUpdate {
    pub height: u64,
    pub consensus_state: Bytes,
    pub client_state: Bytes,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct MisbehaviourResponse {
    pub client_state: Bytes,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub ty: String,
    pub attributes: Vec<Attribute>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateLensClient {
    pub client_id: u32,
    pub l1_client_id: u32,
    pub l2_client_id: u32,
    pub l2_chain_id: String,
}

impl CreateLensClient {
    pub fn into_event(self) -> Event {
        Event {
            ty: "create_lens_client".to_string(),
            attributes: vec![
                Attribute {
                    key: "client_id".to_string(),
                    value: self.client_id.to_string(),
                },
                Attribute {
                    key: "l1_client_id".to_string(),
                    value: self.l1_client_id.to_string(),
                },
                Attribute {
                    key: "l2_client_id".to_string(),
                    value: self.l2_client_id.to_string(),
                },
                Attribute {
                    key: "l2_chain_id".to_string(),
                    value: self.l2_chain_id,
                },
            ],
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct VerifyCreationResponse {
    pub latest_height: u64,
    pub counterparty_chain_id: String,
    pub events: Option<Vec<Event>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum QueryMsg {
    GetTimestamp {
        client_id: u32,
        height: u64,
    },
    GetLatestHeight {
        client_id: u32,
    },
    GetStatus {
        client_id: u32,
    },
    VerifyCreation {
        client_id: u32,
        client_state: Bytes,
        consensus_state: Bytes,
    },
    VerifyMembership {
        client_id: u32,
        height: u64,
        proof: Bytes,
        path: Bytes,
        value: Bytes,
    },
    VerifyNonMembership {
        client_id: u32,
        height: u64,
        proof: Bytes,
        path: Bytes,
    },
    VerifyClientMessage {
        client_id: u32,
        caller: String,
    },
    Misbehaviour {
        client_id: u32,
        message: Bytes,
    },
}
