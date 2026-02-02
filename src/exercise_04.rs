/*
Ejercicio 4 (Nivel 4): spawn + JoinHandle + propagación de errores

Objetivo
    Crear tareas independientes y recoger sus resultados con .await sobre JoinHandle.

Enunciado

    1. Implementa:
        async fn tarea(ms: u64, etiqueta: &str) -> Result<String, &'static str>

        Si ms == 0, devuelve Err("delay inválido").
        Si no, duerme ms y devuelve Ok("<etiqueta> tras <ms>ms").


    2. En run():

        Crea dos tareas con tokio::spawn:
            tarea(300, "A")
            tarea(0, "B") ← esta debe fallar por tu lógica

        Espera ambas con .await sobre los JoinHandle.

        Imprime un resultado por línea en formato:

            OK: ... si todo fue bien
            ERR: ... si falló (ya sea por tu Err o por JoinError)

Restricciones

    1. Debes manejar dos niveles de resultado:

        JoinHandle<T> al await devuelve Result<T, JoinError>
        y T en tu caso es Result<String, &'static str>


Pista mínima (estructura)
    Algo como: match handle.await { Ok(inner) => match inner { ... } , Err(join_err) => ... }
*/

use tokio::{task::JoinHandle, time};

pub async fn run() {
    println!("---Exercise_04!!!: Inicio\n\n");

    let a = tokio::task::spawn(my_tasks(100, "A"));
    let b = tokio::task::spawn(my_tasks(0, "B"));

    tokio::join!(show("A".to_string(), a), show("B".to_string(), b));

    println!("\n\n---Exercise_04!!!: Fin");
}

async fn my_tasks(ms: u64, label: &str) -> Result<String, &'static str> {
    if ms == 0 {
        time::sleep(time::Duration::from_millis(50)).await;
        Err("delay invalido")
    } else {
        time::sleep(time::Duration::from_millis(ms)).await;
        Ok(format!("{label} tras {ms}ms"))
    }
}

async fn show(label: String, handle: JoinHandle<Result<String, &'static str>>) {
    match handle.await {
        Ok(Ok(msg)) => println!("{label}: All Right! {msg}"),
        Ok(Err(msg)) => println!("{label}: Only JoinHandle is right! {msg}"),
        Err(join_err) => println!("ERROR: {join_err}"),
    }
}
