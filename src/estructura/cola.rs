use std::collections::VecDeque;
use  super::traza::Traza;

#[derive(Debug,Clone)]
pub struct Cola {
   pub items: VecDeque<Traza>,
}


impl  Cola {
    // Crear una cola vacía
    pub fn nueva() -> Self {
        Cola {
            items: VecDeque::new(),
        }
    }

    // Agregar un elemento al final de la cola
    pub fn encolar(&mut self, elemento: Traza) {
        self.items.push_back(elemento);
    }

    // Obtener y eliminar el primer elemento de la cola
    pub fn desencolar(&mut self) -> Traza {
        if self.esta_vacia() {
            

            panic!("Error,la cola vacia")
        } else {
            self.items.pop_front().unwrap()
        }
    }

    // Obtener el primer elemento de la cola sin eliminarlo
    pub fn frente(&self) -> Traza {
        self.items.front().unwrap_or(&Traza{nombre:"vacio".to_string(),traza:"vacio".to_string()}).clone()
    }

    // Verificar si la cola está vacía
    pub fn esta_vacia(&self) -> bool {
        self.items.is_empty()
    }

    // Obtener el tamaño de la cola
    pub fn tamano(&self) -> usize {
        self.items.len()
    }
}
