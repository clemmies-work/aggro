use crate::{cli, types};
use log::error;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct Propagator {
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
            "http://{}:{}/api/{}/{}/_json",
            args.host,
            args.port,
            args.organization.clone().unwrap_or("default".to_string()),
            args.device_name
        );
        let username = args.username.clone();
        let password = args.password.clone();

        let queue = Vec::new();

        Self {
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

        if let Err(e) = self
            .client
            .post(&self.endpoint)
            .basic_auth(&self.username, Some(&self.password))
            .json(&self.queue)
            .timeout(Duration::from_millis(1000))
            .send()
        {
            error!("POSTing to remote failed: {:?}", e);
        }
        self.queue.clear();
    }
}
