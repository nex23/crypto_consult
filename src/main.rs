fn main() {
  let mut moneda: String = String::new();
  let result = std::io::stdin().read_line(&mut moneda);
  match result {
    Ok(_) => {
      let precio: String = get_precio(&moneda);
      println!("Precio: {precio}");
    }
    Err(error) => println!("Epa! ocurrió un Error: {}", error),
  }
}

fn get_precio(_moneda: &str) -> String {
  String::from("Probando..")
}
