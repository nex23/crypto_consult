fn main() {
  let mut moneda: String = String::new();
  let _ = std::io::stdin()
    .read_line(&mut moneda)
    .expect("Ha ocurrido un error leyendo el input");
  let result_precio = get_precio(&moneda);
      match result_precio {
          Ok(precio) => println!("El precio es: {}", precio),
          Err(error) => println!("Error al buscar el precio {}", error),
      }
}

fn get_precio(moneda: &str) -> Result<String, ureq::Error> {
  let body: String = ureq::get(&format!(
    "https://api.coingecko.com/api/v3/coins/{}?localization=false",
    moneda))
    .call()?
    .into_string()?;
    Ok(body)
}
