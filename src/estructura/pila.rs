// Definición de la estructura para la Piluse 
use super::traza::Traza;
use serde::{Deserialize,Serialize};

#[derive(Debug, Clone,Deserialize,Serialize)]
pub struct Pila {
    pub elementos: Vec<Traza>,
}


impl Pila {
    // Crear una nueva Pila vacía
    pub fn nueva() -> Self {
        Pila {
            elementos: Vec::new(),
        }
    }

    // Agregar un elemento a la Pila (push)
    pub fn push(&mut self, elemento: Traza) {
        self.elementos.push(elemento);
    }

    // Eliminar y devolver el último elemento de la Pila (pop)
    pub fn pop(&mut self) -> Traza {
        if self.elementos.is_empty() {

            println!("La pila esta vacia");

           panic!("Error")

        } else {
            self.elementos.pop().unwrap()
        }
    }

    // Obtener una referencia al último elemento de la Pila sin eliminarlo (top)
    pub fn top(&self) -> Traza {
        self.elementos.last().unwrap().clone()
    }

    // Verificar si la Pila está vacía
    pub fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }

    // Obtener el tamaño actual de la Pila
    pub fn tamano(&self) -> usize {
        self.elementos.len()
    }

    pub fn vaciar(&mut self) {
        self.elementos = Vec::new();
    }
}
