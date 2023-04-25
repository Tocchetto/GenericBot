pub struct Objeto {
  nome: String,
  cor: String,
  coordenadas: [i32; 4],
  precisao_click: i16,
}

impl Objeto {
  pub fn new(nome: String, cor: String, coordenadas: [i32; 4], precisao_click: i16) -> Objeto {
    Objeto {
      nome,
      cor,
      coordenadas,
      precisao_click: 0,
    }
  }
}
