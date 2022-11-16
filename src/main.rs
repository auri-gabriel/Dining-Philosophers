use core::time;
use semaphore::Semaphore;
use std::sync::{Arc, Mutex};
use std::{thread, vec};

struct Filosofo {
    nome: String,
    esquerda: usize,
    direita: usize,
}

struct Mesa {
    garfos: Vec<Mutex<()>>,
}

impl Filosofo {
    fn new(nome: String, esquerda: usize, direita: usize) -> Self {
        Self {
            nome,
            esquerda,
            direita,
        }
    }

    fn come(&self, mesa: &Mesa) {
        let _esquerda = mesa.garfos[self.esquerda].lock().unwrap();
        let _direita = mesa.garfos[self.direita].lock().unwrap();

        println!("{} está comendo!", self.nome);

        thread::sleep(time::Duration::from_millis(1000));

        println!("{} parou de comer!", self.nome);
    }
}
fn main() {
    let mesa = Arc::new(Mesa {
        garfos: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let filosofos = vec![
        Filosofo::new("Filosofo 1".to_string(), 0, 1),
        Filosofo::new("Filosofo 2".to_string(), 1, 2),
        Filosofo::new("Filosofo 3".to_string(), 2, 3),
        Filosofo::new("Filosofo 4".to_string(), 3, 4),
        Filosofo::new("Filosofo 5".to_string(), 0, 4),
    ];

    let tamanho_menos_um = filosofos.len() - 1;

    let semaforo = Arc::new(Semaphore::new(tamanho_menos_um, ()));

    let passos: Vec<_> = filosofos
        .into_iter()
        .map(|f| {
            let mesa = mesa.clone();
            let semaforo = semaforo.clone();

            thread::spawn(move || {
                if semaforo.try_access().is_ok() {
                    f.come(&mesa);
                }
            })
        })
        .collect();

    for p in passos {
        p.join().unwrap();
    }
}
