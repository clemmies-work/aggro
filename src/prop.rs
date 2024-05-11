use log::error;
use crate::{cli, types};

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
        let endpoint = format!("http://{}:{}/api/default/default/_json", args.host, args.port);
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
        if let Err(e) = self.client.post(&self.endpoint)
            .basic_auth(&self.username, &self.password)
            .json(&self.queue)
            .send() {
            error!("POSTing to remote failed: {:?}", e);
        }
        self.queue.clear();
    }
}
