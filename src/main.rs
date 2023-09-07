fn main() {
  let mut moneda: String = String::new();
  let result = std::io::stdin().read_line(&mut moneda);
  match result {
    Ok(valor) => println!("Nro. de bytes leídos {valor}"),
    Err(error) => println!("Epa! ocurrió un Error: {}", error),
  }
}
