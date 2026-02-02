/*
Ejercicio 2 (Nivel 1):

Objetivo
Entender await como punto de suspensión y comprobar el orden real de ejecución.

Enunciado
Implementa una función:

rust
￼Copy code
async fn esperar_y_devolver(ms: u64, etiqueta: &str) -> String
que:

Espere ms milisegundos usando tokio::time::sleep.

Devuelva un String con el formato: "<etiqueta> tras <ms>ms".

Luego, en tu run():

Imprime "Inicio".

Llama secuencialmente (una después de otra, con .await) a:

esperar_y_devolver(300, "A")

esperar_y_devolver(100, "B")

Imprime los resultados en el orden en que los recibes.

Imprime "Fin".

Restricciones
Debe ser secuencial (todavía no join!, todavía no spawn).

Debes importar tokio::time::{sleep, Duration}.

Resultado esperado (conceptualmente)
Aunque B “duerme” menos, si llamas secuencial, verás primero A... y luego B....

Cuando lo implementes, pega el código y reviso. */

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Exercise_02 !!!: Inicio");
    let a = esperar_y_devolver(300, "A".to_string()).await;
    let b = esperar_y_devolver(100, "B".to_string()).await;
    println!("A:{a} -- B: {b}");

    println!("Exercise_02 !!!: Fin");
    Ok(())
}

async fn esperar_y_devolver(delay: u64, etiqueta: String) -> String {
    use tokio::time::{Duration, sleep};
    sleep(Duration::from_millis(delay)).await; // importante el .await, sino no se ejecuta la función.
    format!("{etiqueta} tras {delay}ms")
}
