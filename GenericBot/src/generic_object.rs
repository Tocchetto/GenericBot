use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericObject {
    color: String,
    coordinates: [i32; 2],
}

impl GenericObject {
    pub fn new(color: String, coordinates: [i32; 2]) -> GenericObject {
        GenericObject {
            color,
            coordinates,
        }
    }

    pub fn load_generic_object_from_yaml(file_path: &str) -> Result<GenericObject> {
        let yaml_content = fs::read_to_string(file_path)?;
        let generic_object: GenericObject = serde_yaml::from_str(&yaml_content).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Erro ao ler o arquivo YAML e desserializar o objeto GenericObject: {}", e),
            )
        })?;
        Ok(generic_object)
    }    
}
