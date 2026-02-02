/*
Ejercicio 6 (Nivel 6): select! (primero que termine gana)

Objetivo

    Competir dos operaciones async y quedarte con la primera que finalice.

Enunciado

    Implementa async fn carrera() -> String que:
    Lanza dos futures:
        esperar_y_devolver(300, "A")
        esperar_y_devolver(100, "B")

Usa tokio::select! para devolver solo el resultado del primero que termine.

Reutiliza tu esperar_y_devolver(ms, etiqueta) -> String con sleep.
 */

use tokio::time;

pub async fn run() {
    println!("--- Exercise_06!!!: Inicio\n\n");

    println!("Futures:\n");

    let the_winner = carrera_futures().await;
    println!("The winner is: {the_winner}\n\n");

    println!("Task:\n");

    let the_winner = carrera_tasks().await;
    println!("The winner is: {the_winner}\n\n");

    println!("\n\n--- Exercise_06!!!: Fin");
}

async fn wait_return(delay: u64, label: String) -> String {
    println!("{label} tras {delay}ms");
    time::sleep(time::Duration::from_millis(delay)).await;
    format!("{label} tras {delay}ms")
}

async fn carrera_futures() -> String {
    let fut_a = wait_return(300, "A".to_string());
    let fut_b = wait_return(299, "B".to_string());

    tokio::select! {
        a = fut_a => a,
        b = fut_b => b,
    }
}

async fn carrera_tasks() -> String {
    let task_a = tokio::spawn(async {
        wait_return(300, "A".to_string());
        wait_return(300, "AA".to_string());
        wait_return(300, "AAA".to_string())
    });
    let task_b = tokio::spawn(async {
        wait_return(299, "B".to_string());
        wait_return(299, "BBB".to_string());
        wait_return(299, "BBB".to_string())
    });

    tokio::select! {
        a = task_a => {
            match a {
                Ok(rslt) => rslt.await,
                Err(err) => format!("Fail A: {err}"),
            }
        },
        b = task_b => {
            match b {
                Ok(rslt) => rslt.await,
                Err(err) => format!("Fail B: {err}"),
            }
        },
    }
}
