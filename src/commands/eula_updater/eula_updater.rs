use std::{ fs::File, io::{ Read, Write } };

pub fn update_eula(loc: &str) {
    let mut updated_eula = String::new();

    {
        let mut eula = File::open(loc).expect("Unable to find the file");
        eula.read_to_string(&mut updated_eula).expect("Error reading file");
    }

    assert!(updated_eula.contains("false") || updated_eula.contains("true"));

    if updated_eula.contains("false") {
        updated_eula = updated_eula.replace("false", "true");
    } else {
        updated_eula = updated_eula.replace("true", "false");
    }

    {
        let mut eula = File::create(loc).expect("Unable to create the file");
        eula.write(updated_eula.as_bytes()).expect("Error writing to file");
    }
}

#[macro_export]
macro_rules! update {
    (path:String) => {
        use crate::eula_updater::update_eula;
        update_eula(path)
    };
    () => {
        use crate::eula_updater::update_eula;
        update_eula("eula.txt")
    };
}

pub use update;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eula_update() {
        const LOC: &str = "tests/eula.txt";
        let flag: bool;
        {
            let mut updated_eula = String::new();
            let mut eula = File::open(LOC).expect("Unable to find the file");
            eula.read_to_string(&mut updated_eula).expect("Error reading file");
            println!("Before Update: {}", updated_eula.contains("eula=false"));
            flag = updated_eula.contains("eula=false");
        }

        let _ = update_eula(LOC);

        {
            let mut updated_eula = String::new();
            let mut eula = File::open(LOC).expect("Unable to find the file");
            eula.read_to_string(&mut updated_eula).expect("Error reading file");
            println!("After Update: {}", updated_eula.contains("eula=false"));
            assert!(updated_eula.contains("eula=false") ^ flag);
        }
        
    }
}