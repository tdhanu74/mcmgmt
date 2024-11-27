use std::{ fs::File, io::{ Read, Write } };
use regex::{Captures, Regex};

#[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard
}

impl Difficulty {
    pub fn as_str(&self) -> &'static str {
        match self {
            &Difficulty::Peaceful => "peaceful",
            &Difficulty::Easy => "easy",
            &Difficulty::Normal => "normal",
            &Difficulty::Hard => "hard"
        }
    }
}


#[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator
}

impl GameMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            &GameMode::Survival => "survival",
            &GameMode::Creative => "creative",
            &GameMode::Adventure => "adventure",
            &GameMode::Spectator => "spectator"
        }
    }
}


pub fn set_property(loc:&str, name:&str, value: &str) {
    let mut server_properties_txt = String::new();
    let re = Regex::new(format!(r"({}=)([\S]\w+)", name).as_str()).unwrap();
    {
        let mut server_properties = File::open(loc).expect("Unable to find the file");
        server_properties.read_to_string(&mut server_properties_txt).expect("Error reading file");
    }

    assert!(re.is_match(&server_properties_txt));

    let out = re.replace(&server_properties_txt, |caps: &Captures| {format!("{}{}", &caps[1], value) });

    {
        let mut server_properties = File::create(loc).expect("Unable to create the file");
        server_properties.write(out.as_ref().as_ref()).expect("Error writing to file");
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_property() {
        const LOC: &str = "tests/server.properties";

        let _ = set_property(LOC, "difficulty", Difficulty::Hard.as_str());
        let _ = set_property(LOC, "view-distance", 16.to_string().as_str());
        let _ = set_property(LOC, "simulation-distance", 16.to_string().as_str());
        let _ = set_property(LOC, "gamemode", GameMode::Survival.as_str());
        let _ = set_property(LOC, "hardcore", true.to_string().as_str());

        {
            let diff_re = Regex::new(r"(difficulty=)([\s\S]\w+)").unwrap();
            let view_re = Regex::new(r"(view-distance=)([\s\S]\w+)").unwrap();
            let sim_re = Regex::new(r"(simulation-distance=)([\s\S]\w+)").unwrap();
            let game_re = Regex::new(r"(gamemode=)([\s\S]\w+)").unwrap();
            let hard_re = Regex::new(r"(hardcore=)([\s\S]\w+)").unwrap();

            let mut server_properties_txt = String::new();
            let mut server_properties = File::open(LOC).expect("Unable to find the file");
            server_properties.read_to_string(&mut server_properties_txt).expect("Error reading file");

            let diff_caps = diff_re.captures(server_properties_txt.as_str()).unwrap();
            let view_caps = view_re.captures(server_properties_txt.as_str()).unwrap();
            let sim_caps = sim_re.captures(server_properties_txt.as_str()).unwrap();
            let game_caps = game_re.captures(server_properties_txt.as_str()).unwrap();
            let hard_caps = hard_re.captures(server_properties_txt.as_str()).unwrap();

            assert!(&diff_caps[2].eq(Difficulty::Hard.as_str()));
            assert!(&view_caps[2].eq(16.to_string().as_str()));
            assert!(&sim_caps[2].eq(16.to_string().as_str()));
            assert!(&game_caps[2].eq(GameMode::Survival.as_str()));
            assert!(&hard_caps[2].eq(true.to_string().as_str()));
        }

        let _ = set_property(LOC, "difficulty", Difficulty::Peaceful.as_str());
        let _ = set_property(LOC, "view-distance", 20.to_string().as_str());
        let _ = set_property(LOC, "simulation-distance", 20.to_string().as_str());
        let _ = set_property(LOC, "gamemode", GameMode::Creative.as_str());
        let _ = set_property(LOC, "hardcore", false.to_string().as_str());
        
        {
            let diff_re = Regex::new(r"(difficulty=)([\s\S]\w+)").unwrap();
            let view_re = Regex::new(r"(view-distance=)([\s\S]\w+)").unwrap();
            let sim_re = Regex::new(r"(simulation-distance=)([\s\S]\w+)").unwrap();
            let game_re = Regex::new(r"(gamemode=)([\s\S]\w+)").unwrap();
            let hard_re = Regex::new(r"(hardcore=)([\s\S]\w+)").unwrap();
            
            let mut server_properties_txt = String::new();
            let mut server_properties = File::open(LOC).expect("Unable to find the file");
            server_properties.read_to_string(&mut server_properties_txt).expect("Error reading file");

            let diff_caps = diff_re.captures(server_properties_txt.as_str()).unwrap();
            let view_caps = view_re.captures(server_properties_txt.as_str()).unwrap();
            let sim_caps = sim_re.captures(server_properties_txt.as_str()).unwrap();
            let game_caps = game_re.captures(server_properties_txt.as_str()).unwrap();
            let hard_caps = hard_re.captures(server_properties_txt.as_str()).unwrap();

            assert!(&diff_caps[2].eq(Difficulty::Peaceful.as_str()));
            assert!(&view_caps[2].eq(20.to_string().as_str()));
            assert!(&sim_caps[2].eq(20.to_string().as_str()));
            assert!(&game_caps[2].eq(GameMode::Creative.as_str()));
            assert!(&hard_caps[2].eq(false.to_string().as_str()));
        }

    }
}