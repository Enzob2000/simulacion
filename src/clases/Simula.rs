use crate::estructura;
use estructura::cola::Cola;
use estructura::pila::Pila;
use estructura::traza::Traza;

#[derive(Debug, Clone)]
pub struct Procesos {
    activas:usize,
    nombre: String,
    trazas: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Simulacion {
    
    pub ejecucion: Vec<String>,
    pub proceso: Vec<Procesos>,
    pub cola_listos: Cola,
    pub cola_ejecucion: Cola,
    pub cola_pendiente: Cola,
    pub cola_terminados: Cola,
    pub pila_ejecicion: Pila,
}

impl Simulacion {
    pub fn nuevo()->Simulacion{

        Simulacion{
        ejecucion:vec![],
        proceso:vec![],
        cola_listos:Cola::nueva(),
        cola_ejecucion:Cola::nueva(),
        cola_pendiente:Cola::nueva(),
        cola_terminados:Cola::nueva(),
        pila_ejecicion:Pila::nueva(),



        }

    }

   pub fn cargar_proceso(&mut self, nombre: String, trazas: Vec<String>) {
        self.proceso.push(Procesos {
            nombre: nombre.clone(),
            trazas: trazas.clone(),
            activas:trazas.len()
        });
        
    }

    pub fn cargador(&mut self, orden: Vec<String>) {
        let aux = self.proceso.clone();
        self.proceso.clear();
        let mut cont = 0;

        while (cont != orden.len()-1) {
            for i in aux.iter() {
                if *i.nombre == orden[cont] {
                    self.proceso.push(i.clone());
                    cont += 1;
                }
            }
        }

        for i in self.proceso.iter() {
            for j in i.trazas.iter() {
                self.cola_listos.encolar(
                    (Traza {
                        nombre: i.nombre.clone(),
                        traza: j.clone(),
                    }),
                );
            }
        }
    }

   pub fn atender_proceso(&mut self) {
          
        let mut activo=self.cola_listos.frente();
         
        for i in 0..4 {
            if self.cola_listos.esta_vacia() {
                break;
            }
            activo = self.cola_listos.desencolar();
            self.ejecucion.push(activo.nombre.clone());
            self.cola_ejecucion.encolar(activo.clone());
        }

        let mut nombre2 = self.cola_listos.frente();

        while (nombre2.nombre ==activo.nombre ) {
            self.cola_pendiente.encolar(self.cola_listos.desencolar());
            nombre2=self.cola_listos.frente();
        }
    }

    pub fn terminar_proceso(&mut self) {
        self.ejecucion = vec![];

        for i in 0..4 {
            if self.cola_ejecucion.esta_vacia() {
                break;
            }
            let aux = self.cola_ejecucion.desencolar();
            ////

            for j in self.proceso.iter_mut() {

                if aux.nombre==*j.nombre{

               j.activas-=1;

                }
                
            }

        }

        while (!self.cola_pendiente.esta_vacia()) {
            self.cola_listos.encolar(self.cola_pendiente.desencolar());
        } 

        let aux=self.proceso.clone();

        for (i,j) in aux.iter().enumerate(){


            if j.activas==0{
            
            self.proceso.remove(i);

            self.pila_ejecicion.push(Traza { nombre: j.nombre.clone(), traza: "".to_string() })

            }
        }
        
    }

   pub fn cancelar(&mut self, nombrecan: String) {
        let mut cont = 0;

        for i in self.ejecucion.iter() {
            if *i == nombrecan {
                cont = 1;
            }
        }

        if cont == 0 {

            //return false
        }

        let mut aux = self.cola_listos.clone();

        self.cola_listos.items.clear();

        while (!aux.esta_vacia()) {
            let nom = aux.desencolar();

            if nom.nombre != nombrecan {
                self.cola_listos.encolar(nom.clone())
            }
        }
    }
}



