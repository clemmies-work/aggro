use crate::{cli, types};
use log::error;
use std::time::Duration;
use serde::{Deserialize, Serialize};

pub struct Propagator {
    device_name: String,

    client: reqwest::blocking::Client,
    endpoint: String,
    username: String,
    password: String,

    queue: Vec<types::Log>,
}

impl Propagator {
    pub fn new(args: &cli::Args) -> Self {
        let client = reqwest::blocking::Client::new();
        let endpoint = format!(
            "http://{}:{}/api/default/default/_json",
            args.host, args.port
        );
        let username = args.username.clone();
        let password = args.password.clone();
        let device_name = args.device_name.clone();

        let queue = Vec::new();

        Self {
            device_name,
            client,
            endpoint,
            username,
            password,
            queue,
        }
    }

    pub fn add(&mut self, msg: types::Log) {
        self.queue.push(msg);
        if self.queue.len() > 100 {
            self.propagate();
        }
    }

    pub fn propagate(&mut self) {
        if self.queue.is_empty() {
            return;
        }

        #[derive(Serialize, Debug, Clone)]
        struct Entry {
            device_name: String,
            #[serde(flatten)]
            log: types::Log,
        }
        let entries = self.queue.iter()
            .map(|msg| Entry {
                device_name: self.device_name.clone(),
                log: msg.clone()
            })
            .collect::<Vec<_>>();
        if let Err(e) = self
            .client
            .post(&self.endpoint)
            .basic_auth(&self.username, Some(&self.password))
            .json(&entries)
            .timeout(Duration::from_millis(1000))
            .send()
        {
            error!("POSTing to remote failed: {:?}", e);
        }
        self.queue.clear();
    }
}
