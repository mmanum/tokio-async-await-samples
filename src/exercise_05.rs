/*
Ejercicio 5 (Nivel 5): timeout + cancelación (sin abort)

Objetivo

Aplicar tokio::time::timeout para limitar una operación async.


Implementa:

    async fn con_timeout(ms_trabajo: u64, ms_timeout: u64) -> Result<String, String>


    Debe ejecutar una tarea que “trabaja” durmiendo ms_trabajo.

    Debe devolver:
        Ok("terminado en ...") si acaba antes del timeout.
        Err("timeout tras ...") si no llega a tiempo.

Restricción:
    Usa tokio::time::timeout(Duration::from_millis(ms_timeout), ...).

*/

use tokio::time;
pub async fn run() {
    println!("--- Exercise_05!!!: Inicio\n\n");

    match with_timeout(1000, 1500).await {
        Ok(rslt) => println!("OK: {:?}", rslt),
        Err(err) => println!("Error: {:?}", err),
    }

    match with_timeout(1000, 500).await {
        Ok(rslt) => println!("OK: {:?}", rslt),
        Err(err) => println!("Error: {:?}", err),
    }

    println!("\n\n--- Exercise_05!!!: Fin");
}

async fn with_timeout(ms: u64, ms_timeout: u64) -> Result<String, String> {
    let work = async {
        time::sleep(time::Duration::from_millis(ms)).await;
        format!("Trabajo finalizado en: ({ms} / {ms_timeout}) ms")
    };

    match time::timeout(time::Duration::from_millis(ms_timeout), work).await {
        Ok(msg) => Ok(msg),
        Err(_) => Err(format!("Timeout!!!!  ({ms} / {ms_timeout}) ms.")),
    }
}
