
#[derive(Debug, Default)]
pub struct Lockfile {
    pub port: String,
    pub password: String,
}

// Reads lockfile data from "C:\Users\User1\AppData\Local\Riot Games\Riot Client\Config" which contains the port and password to access local api
pub fn get_lockfile() -> Option<Lockfile> {
    if let Ok(path) = std::env::var("LOCALAPPDATA") {
        let lockfile_path = format!{"{}{}", path, "\\Riot Games\\Riot Client\\Config\\lockfile"};
        
        let content = match std::fs::read_to_string(&lockfile_path) {
            Ok(text) => text,
            Err(_) => return None,
        };

        let split_content: Vec<&str> = content.split(":").collect();
        let mut lockfile = Lockfile::default();

        if let Some(port) = split_content.get(2) {
            lockfile.port = port.to_string();
        } else {
            return None;
        }

        if let Some(password) = split_content.get(3) {
            lockfile.password = password.to_string();
        } else {
            return None;
        }
        
        return Some(lockfile);
    }
    return None;
}