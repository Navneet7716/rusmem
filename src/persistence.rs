use std::io::{self, Read};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    net::TcpStream,
    sync::{Arc, Mutex, RwLock},
    time::SystemTime,
};
struct Transaction {}

struct SortedSetMember {
    score: f64,
    member: String,
}
pub struct KeyValueStore {
    strings: HashMap<String, String>,
    lists: HashMap<String, Vec<String>>,
    hashes: HashMap<String, HashMap<String, String>>,
    sets: HashMap<String, HashSet<String>>,
    sorted_sets: HashMap<String, Vec<SortedSetMember>>,
    expirations: HashMap<String, SystemTime>,
    mu: Arc<RwLock<()>>,             // Using Arc<RwLock<()>> for synchronization
    current_tx: Option<Transaction>, // Using Option for nullable current transaction
    total_commands_processed: i32,
    connected_clients: HashMap<String, TcpStream>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore {
            strings: HashMap::new(),
            lists: HashMap::new(),
            hashes: HashMap::new(),
            sets: HashMap::new(),
            sorted_sets: HashMap::new(),
            expirations: HashMap::new(),
            total_commands_processed: 0,
            connected_clients: HashMap::new(),
            mu: todo!(),
            current_tx: todo!(),
        }
    }
}

pub struct Persistence {
    kv: Arc<KeyValueStore>,
    data_file: String,
    mu: Mutex<()>,
    should_save: bool,
}

impl Persistence {
    pub fn new(kv: Arc<KeyValueStore>, data_file: String) -> Self {
        let persistence = Persistence {
            kv,
            data_file: data_file.clone(),
            mu: Mutex::new(()),
            should_save: true,
        };

        if let Err(err) = persistence.load_data() {
            println!("Error loading data: {:?}", err);
        }

        persistence
    }

    fn load_data(&self) -> io::Result<()> {
        let mut file = File::open(&self.data_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(())
    }

    fn save_data(&self) -> io::Result<()> {
        let mut file = File::create(&self.data_file)?;
        Ok(())
    }
}
