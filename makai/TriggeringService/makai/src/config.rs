use serde_json::{from_reader, Value};
use std::fs::File;
use std::path::Path;

///Representation of the configuration file's required fields.
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    /// zmq endpoint for the triggering broker.
    pub zmq_trigger_endpoint: String,
    ///zmq endpoint for the acquisition broker.
    pub zmq_acquisition_endpoint: String,
    ///Mongo endpoint.
    pub mongo_host: String,
    ///Mongo port.
    pub mongo_port: u16,
    ///How long the measurements live in mongo before they expire.
    pub mongo_measurement_expiration_seconds: u64,
    ///Window width for the trend calculation.
    pub mongo_trends_update_interval_seconds: u64,
    ///How long the overlapping intervals structure keeps data.
    pub event_request_expiration_window_ms: u64,
    ///Plugin specific settings.
    pub plugins: Vec<Value>,
}

impl Settings {
    /// Load the settings file from disk.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Settings, String> {
        let file = File::open(path).or(Err("No such file"))?;
        let u = from_reader(file).or(Err("Could not parse config file."))?;
        Ok(u)
    }
}
