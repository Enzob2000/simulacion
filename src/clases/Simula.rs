use crate::estructura;
use estructura::traza::Traza;
use serde::{Deserialize, Serialize};
use std::collections::LinkedList;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{self, Path},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Procesos {
    pub activas: usize,
    pub nombre: String,
    pub trazas: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Simulacion {
    pub procesoV: bool,
    pub cargaV: bool,
    pub atenderV: bool,
    pub terminarV: bool,
    pub activos: Vec<String>,
    pub ejecucion: Vec<String>,
    pub proceso: Vec<Procesos>,
    pub cola_listos: LinkedList<Traza>,
    pub cola_ejecucion: LinkedList<Traza>,
    pub cola_pendiente: LinkedList<Traza>,
    pub cola_terminados: LinkedList<Traza>,
    pub pila_ejecicion: LinkedList<Traza>,
}

impl Simulacion {
    pub fn nuevo() -> Simulacion {
        let di = Path::new("Data.json");

        if di.exists() {
            let mut archivo = File::open("Data.json").unwrap();

            let mut contenido = String::new();

            archivo.read_to_string(&mut contenido).unwrap();

            let datos: Simulacion = serde_json::from_str(&contenido).unwrap();

            return Simulacion {
                procesoV: datos.procesoV,
                cargaV: datos.cargaV,
                atenderV: datos.atenderV,
                terminarV: datos.atenderV,
                activos: datos.activos,
                ejecucion: datos.ejecucion,
                proceso: datos.proceso,
                cola_listos: datos.cola_listos,
                cola_ejecucion: datos.cola_ejecucion,
                cola_pendiente: datos.cola_pendiente,
                cola_terminados: datos.cola_terminados,
                pila_ejecicion: datos.pila_ejecicion,
            };
        } else {
            return Simulacion {
                procesoV: false,
                cargaV: false,
                atenderV: false,
                terminarV: false,
                activos: vec![],
                ejecucion: vec![],
                proceso: vec![],
                cola_listos: LinkedList::new(),
                cola_ejecucion: LinkedList::new(),
                cola_pendiente: LinkedList::new(),
                cola_terminados: LinkedList::new(),
                pila_ejecicion: LinkedList::new(),
            };
        }
    }

    pub fn reset(&mut self) {
        fs::remove_file("Data.json");
        self.procesoV = false;
        self.cargaV = false;
        self.atenderV = false;
        self.terminarV = false;
        self.cola_ejecucion.clear();
        self.cola_listos.clear();
        self.cola_pendiente.clear();
        self.cola_terminados.clear();
        self.pila_ejecicion.clear();
        self.ejecucion.clear();
        self.proceso.clear();
        self.activos.clear();
    }
    fn archivo(&self) {
        let simu = Simulacion {
            procesoV: self.procesoV.clone(),
            cargaV: self.cargaV.clone(),
            atenderV: self.atenderV.clone(),
            terminarV: self.terminarV.clone(),
            activos: self.activos.clone(),
            ejecucion: self.ejecucion.clone(),
            proceso: self.proceso.clone(),
            cola_listos: self.cola_listos.clone(),
            cola_ejecucion: self.cola_ejecucion.clone(),
            cola_pendiente: self.cola_pendiente.clone(),
            cola_terminados: self.cola_terminados.clone(),
            pila_ejecicion: self.pila_ejecicion.clone(),
        };

        let json = serde_json::to_string(&simu).unwrap();

        let mut archivo = File::create("Data.json").unwrap();

        archivo.write_all(json.as_bytes()).unwrap()
    }

    pub fn cargar_proceso(&mut self, nombre: String, trazas: Vec<String>) {
        self.proceso.push(Procesos {
            nombre: nombre.clone(),
            trazas: trazas.clone(),
            activas: trazas.len(),
        });

        self.activos.push(nombre);
        self.archivo();
    }

    pub fn cargador(&mut self, orden: Vec<String>) {
        let aux = self.proceso.clone();
        self.proceso.clear();
        let mut cont = 0;

        while cont != orden.len() {
            for i in aux.iter() {
                if i.nombre == orden[cont] {
                    self.proceso.push(i.clone());
                    cont += 1;
                    break;
                }
            }
        }

        for i in self.proceso.iter() {
            for j in i.trazas.iter() {
                self.cola_listos.push_back(Traza {
                    nombre: i.nombre.clone(),
                    traza: j.clone(),
                });
            }
        }

        self.archivo();
    }

    pub fn atender_proceso(&mut self) {
        let mut activo = self.cola_listos.front().unwrap().clone();

        for i in 0..4 {
            if self.cola_listos.is_empty() {
                break;
            }
            activo = self.cola_listos.pop_front().unwrap();
            self.ejecucion.push(activo.nombre.clone());
            self.cola_ejecucion.push_back(activo.clone());
        }

        while !self.cola_listos.is_empty()
            && self.cola_listos.front().unwrap().nombre == activo.nombre
        {
            self.cola_pendiente
                .push_back(self.cola_listos.pop_front().unwrap());
        }
        self.archivo();
    }

    pub fn terminar_proceso(&mut self) {
        self.ejecucion = vec![];

        for i in 0..4 {
            if self.cola_ejecucion.is_empty() {
                break;
            }
            let aux = self.cola_ejecucion.pop_front().unwrap();

            self.pila_ejecicion.push_front(aux.clone());

            for j in self.proceso.iter_mut() {
                if aux.nombre == *j.nombre {
                    j.activas -= 1;
                }
            }
        }

        while !self.cola_pendiente.is_empty() {
            self.cola_listos
                .push_back(self.cola_pendiente.pop_front().unwrap());
        }

        let aux = self.proceso.clone();

        for (i, j) in aux.iter().enumerate() {
            if self.proceso.len() == 1 && j.activas == 0 {
                self.cola_terminados.push_back(Traza {
                    nombre: j.nombre.clone(),
                    traza: "".to_string(),
                });
                self.proceso.clear();
                self.activos.clear();

                fs::remove_file("Data.json");
            }

            if j.activas == 0 && self.proceso.len() != 0 {
                self.proceso.remove(i);
                self.cola_terminados.push_back(Traza {
                    nombre: j.nombre.clone(),
                    traza: "".to_string(),
                });
                self.activos.remove(i);
            }
        }
        self.archivo();
    }

    pub fn cancelar(&mut self, nombrecan: String) {
        let mut cont = 0;

        for i in self.ejecucion.iter() {
            if *i == nombrecan {
                cont = 1;
            }
        }

        if cont == 1 {
            return;
        }

        let mut aux = self.cola_listos.clone();

        self.cola_listos.clear();

        while !aux.is_empty() {
            let nom = aux.pop_back().unwrap();

            if nom.nombre != nombrecan {
                self.cola_listos.push_back(nom.clone())
            }
        }

        let auxeje = self.activos.clone();

        for (i, j) in auxeje.iter().enumerate() {
            if self.proceso.len() == 1 && *j == nombrecan {
                self.activos.clear();
            }

            if *j == nombrecan && self.activos.len() != 0 {
                self.activos.remove(i);
            }
        }

        self.archivo()
    }

    pub fn insertar(&mut self, proceso: String, trazas: String) {
        self.proceso
            .iter_mut()
            .filter(|x| *x.nombre == proceso && !(x.trazas.contains(&trazas.clone())))
            .for_each(|x| {
                x.trazas.push(trazas.clone());
                x.activas += 1;
                self.cola_listos.push_back(Traza {
                    nombre: proceso.clone(),
                    traza: trazas.clone(),
                });
            });
        self.archivo();
    }

    pub fn eliminar(&mut self, proceso: String, trazas: String) {
        
        self.cola_listos=self.cola_listos
           .clone()
           .into_iter()
            .filter(|x| !(*x.nombre == proceso && *x.traza == trazas))
            .collect();

        self.proceso
        .iter_mut()
        .filter(|x| *x.nombre==proceso)
        .for_each(|x|x.trazas.retain(|x| *x!=trazas));

        self.archivo();
    }
}
