use log::info;
use serde_json::to_string;
use sqlx::Row;

use crate::db_handler::DBHandler;
use crate::graph_communicator::GraphCommunicator;
use crate::models::*;

pub struct Logic {
    pub db_conn: DBHandler,
    pub graph_conn: GraphCommunicator,
}

impl Logic {
    pub async fn healthcheck(&self) -> Result<String, serde_json::Error> {
        info!("Healthcheck");
        let payload = to_string(&Message {
            status: 200,
            msg: "This works!".to_string(),
        });
        return payload;
    }

    pub async fn graph_get_self(&self) -> Result<String, serde_json::Error> {
        let res = self.graph_conn.get_self().await;
        if res.is_ok() {
            let payload = to_string(&GraphResponse {
                status: 200,
                resp: res.unwrap(),
            });
            return payload;
        } else {
            let payload = to_string(&GraphResponse {
                status: 500,
                resp: res.unwrap(),
            });
            return payload;
        }
    }

    pub async fn db_get_all_notes(&self) -> Result<String, serde_json::Error> {
        let res = self.db_conn.get_all_notes().await;
        if res.is_ok() {
            // get the vector out of the Result
            let res = res.unwrap();
            let mut vec_of_notes: Vec<String> = vec![];
            for row in res {
                // parse the result of the db per row
                let recieved_note = to_string(&Note {
                    id: row.get("id"),
                    title: row.get("title"),
                    content: row.get("content"),
                    time_added: row.get("time_added"),
                    details: row.get("details"),
                });
                // add to vector
                vec_of_notes.push(recieved_note.unwrap());
            }
            // first stringified with SetOfNotes as it takes a Vector of Strings, then add it to a DbResponse as the payload
            let struct_of_notes = to_string(&SetOfNotes {
                notes: vec_of_notes
            });
            let payload = to_string(&DbResponse {
                status: 200,
                resp: struct_of_notes.unwrap(),
            })
            .unwrap();
            Ok(payload)
        } else {
            let payload = to_string(&Message {
                status: 500,
                msg: res.err().unwrap().to_string(),
            })
            .unwrap_err();
            Err(payload)
        }
    }
}
