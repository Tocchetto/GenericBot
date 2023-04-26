mod generic_object;
use generic_object::GenericObject;

fn main() {
    let file_path = "output.yaml";

    match GenericObject::load_generic_object_from_yaml(file_path) {
        Ok(generic_object) => {
            println!("Objeto GenericObject carregado com sucesso: {:?}", generic_object);
        }
        Err(e) => {
            eprintln!("Erro ao carregar o objeto GenericObject do arquivo {}: {}", file_path, e);
        }
    }
}
