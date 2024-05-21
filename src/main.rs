mod estructura;
mod clases;
use std::clone;
use std::process::exit;

use clases::Simula::Simulacion;
use estructura::cola::Cola;
use estructura::pila::Pila;
use iced::color;

use iced::alignment::Horizontal::Right;
use iced::command::Command;
use iced::executor::Default;
use iced::overlay::menu::State;
use iced::theme::palette::Background;
use iced::theme::Text::Color as colore;
use iced::theme::{self, Text};
use iced::widget::image::Image as extraer;
use iced::widget::text::Appearance;
use iced::widget::text_input::StyleSheet;
use iced::widget::{
    button, column, combo_box::ComboBox, container, keyed_column, row, scrollable, text, Button,
    Column, Scrollable, TextInput,
};
use iced::widget::{combo_box, text_input};

use iced::Application;
use iced::Sandbox;
use iced::Settings;
use iced::{Border, Color, Element, Length, Padding, Shadow, Vector};
use iced::{Renderer, Theme};


#[derive(Debug, Clone)]
enum Pagina {
    menu,
    crear,
    cargar,
    cancelar
    
}


#[derive(Debug, Clone)]
enum Message {

     proceso(String),

     traza(String),
    

    guardar,

    eliminar(String),

    pala(String),

    orden(String),
    eliminarorden(String),
    menu,
    crear,
    cargar,
    cancelar,
    guardar_proceso,
    guardar_orden,
    atender_proceso,
    terminar_proceso,
    salir,
    reset,
    cancelar_proceso(String)


}

struct Interfas {
    procesoV:bool,
    cargaV:bool,
    atenderV:bool,
    terminarV:bool,
   // value: i32,
    ve: Vec<String>,
    proceso: String,
    traza: String,
    cant: usize,
    procesos:Vec<String>,
    ordenamiento:Vec<String>,
    simula:Simulacion,
    pagina:Pagina
}

pub fn main() -> iced::Result {
    Interfas::run(Settings::default())
}

impl Application for Interfas {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(fla: ()) -> (Interfas, Command<Message>) {
        (
            Interfas {
               // value: 0,
                ve: vec![],
                proceso: "".to_string(),
                traza: "".to_string(),
                cant: 0,
                procesos:vec![],
                ordenamiento:vec![],
                simula:Simulacion::nuevo(),
                pagina:Pagina::menu,
                procesoV:false,
                cargaV:false,
                atenderV:false,
                terminarV:false
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Calculadora")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::cancelar_proceso(proceso)=>{

                self.simula.cancelar(proceso.clone());

                
                
            }

            Message::reset=>{

            self.simula.reset()

            }
            Message::salir=>{

                self.simula.reset();

                exit(0x0100);
            }
            Message::atender_proceso=>{
                
                if self.simula.procesoV && self.simula.cargaV && !self.simula.terminarV{
                
                self.simula.terminarV=true;
                self.simula.atenderV=false;
                self.simula.atender_proceso()
               
               
               }
            }
            Message::terminar_proceso=>{

                 if self.simula.procesoV && self.simula.cargaV && !self.simula.atenderV{
                 
                    self.simula.terminarV=false;
                    self.simula.atenderV=true;

                self.simula.terminar_proceso()}
            }
            Message::guardar_orden=>{

                if self.simula.procesoV{
                self.simula.cargaV=true;
                self.simula.cargador(self.ordenamiento.clone() );
                self.ordenamiento=vec![];
                self.pagina=Pagina::menu;}
                
            }
            Message::guardar_proceso=>{
                if !self.cargaV{
                self.procesoV=true;
                self.simula.cargar_proceso(self.proceso.clone(), self.ve.clone());
                self.ve=vec![];
                self.procesos.push(self.proceso.clone());
                self.proceso="".to_string();
                self.pagina=Pagina::menu;}
                

            }
            Message::menu=>{
                self.pagina=Pagina::menu
            }
            Message::cargar=>{
                if self.procesoV && !self.cargaV{
                    
                    self.pagina=Pagina::cargar
                }
                
            }
            Message::cancelar=>{

                self.pagina=Pagina::cancelar
            }
            Message::crear=>{
                if !self.simula.cargaV{
                    self.simula.procesoV=true;
                self.pagina=Pagina::crear;}
            }
    
            Message::traza(tra)=>{

                self.traza=tra;
            }

            Message::proceso(proce)=>{

                self.proceso=proce;
            }

            Message::pala(palabra) => {}

            Message::guardar => {

                self.ve.push(self.traza.clone());
                self.traza="".to_string();
            }
            Message::eliminar(eliminar) => {
                self.simula.cancelar(eliminar)
            }
            Message::orden(pala)=>{

                self.ordenamiento.push(pala);
            }
            Message::eliminarorden(eliminar)=>{

                self.ordenamiento = self
                    .ordenamiento
                    .clone()
                    .into_iter()
                    .filter(|x| *x != eliminar)
                    .collect();
            }
        }

        Command::none()
    }
        
    fn view(&self) -> Element<Message> {

        let actual= match self.pagina {
            Pagina::menu=>login(),
            Pagina::crear=>cargar(self.traza.clone(),self.proceso.clone(), self.ve.clone()),
            Pagina::cargar=>ordenador(self.procesos.clone(), self.ordenamiento.clone()),
            Pagina::cancelar=>cancel(self.simula.activos.clone())
        };

        row!(
        actual,
        pilas("Pila de ejecucion", self.simula.pila_ejecicion.clone()),
        fila("Cola de listos", self.simula.cola_listos.clone()),
        fila("Cola de ejecucion", self.simula.cola_ejecucion.clone()),
        fila("Cola de pendientes", self.simula.cola_pendiente.clone()), 
        fila("Cola de terminados", self.simula.cola_terminados.clone()),
        ).spacing(10)
        .into()
        
    }
}
fn login() -> Element<'static, Message> {

    let imagen = extraer::new("imagenes/Atom.png");

    let a = column!(
        Button::new(
            text("Crear procesos")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(15)
        )
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
        .on_press(Message::crear),
        Button::new(
            text("Cargar Procesos")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(15)
        )
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
        .on_press(Message::cargar),
        Button::new(
            text("Atender procesos")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(15)
        )
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
        .on_press(Message::atender_proceso),
        Button::new(
            text("Terminar proceso")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(15)
        )
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
        .on_press(Message::terminar_proceso),
        Button::new(
            text("Cancelar proceso")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(15)
        )
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
        .on_press(Message::cancelar),
        row!(Button::new(
            text("Salir")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(15)
        )
        .width(Length::Fixed(150.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
        .on_press(Message::salir),

        Button::new(
            text("reset")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(15)
        )
        .width(Length::Fixed(70.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
        .on_press(Message::reset),
    ).spacing(10)
    )
    .width(Length::Fill)
    .spacing(20)
    .align_items(iced::Alignment::Center);

    let c = column!(
        
        
        imagen.width(150).height(150), a)
        .spacing(30)
        .align_items(iced::Alignment::Center);

    let b = container(c)
        .width(300)
        .padding(Padding::from(40))
        .center_x()
        .center_y()
        .style(iced::theme::Container::Custom(Box::new(Containestyle::menu)));

        container(b)
        .width(350)
        .height(700)
        .center_x()
        .center_y()
        .into()

    
}

fn cargar(traza: String, proceso: String, trazas: Vec<String>) -> Element<'static, Message> {
   
    let mut b = column!()
        .width(Length::Fill)
        .spacing(20)
        .align_items(iced::Alignment::Center);

    for i in trazas.iter() {
        b = b.push(
            
            row!(
                text(i).size(25).style(colore(color!(244, 246, 244))),
                container(
                    Button::new(
                        text("Eliminar")
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .size(15)
                    )
                    .width(Length::Fixed(100.0))
                    .height(Length::Fixed(30.0))
                    .style(iced::theme::Button::Custom(Box::new(Buttonstyless::eliminar)))
                    .on_press(Message::eliminar(i.to_string())),
                ),
            )
            .spacing(15)
            .align_items(iced::Alignment::Start)
        );
    }

    let c=container(Scrollable::new(b))
    .width(300)
    .height(400)
    .padding(Padding::from(10))
    .center_x()
    .center_x()
    .style(iced::theme::Container::Custom(Box::new(Containestyle::cargar)));
   


    let mut a = column!(
        text_input("Nombre del proceso", &proceso)
            .width(Length::Fixed(500.0))
            .on_input(|pala| { Message::proceso(pala) })
            .style(iced::theme::TextInput::Custom(Box::new(Text_inputstyle))),
        text_input("Nombre de la traza  (ENTER)", &traza)
            .width(Length::Fixed(500.0))
            .on_input(|pala| { Message::traza(pala) })
            .on_submit(Message::guardar)
            .style(iced::theme::TextInput::Custom(Box::new(Text_inputstyle))),

            c,
            row!(

              
            Button::new(
                text("salir")
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .vertical_alignment(iced::alignment::Vertical::Center)
                    .size(15)
            )
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(45.0))
            .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
            .on_press(Message::menu), 

            Button::new(
                text("Agregar proceso")
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .vertical_alignment(iced::alignment::Vertical::Center)
                    .size(15)
            )
            .width(Length::Fixed(150.0))
            .height(Length::Fixed(45.0))
            .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
            .on_press(Message::guardar_proceso),

        
        
        
         ).spacing(10) 

    )
    .width(Length::Fill)
    .spacing(20)
    .align_items(iced::Alignment::Center);


   let d= container(a)
    .width(300)
    .padding(Padding::from(20))
    .center_x()
    .center_y()
    .style(iced::theme::Container::Custom(Box::new(Containestyle::menu)));

    container(d)
            .width(350)
            .height(700)
            .center_x()
            .center_y()
            .into()

   
    
}

fn ordenador(procesos:Vec<String>,orden:Vec<String>)->Element<'static,Message>{

    let mut b = column!()
    .width(Length::Fill)
    .spacing(20)
    .align_items(iced::Alignment::Center);

for i in procesos.iter() {
   
    if orden.contains(i){

        continue;
    }

    b = b.push(
        
        row!(

            container(
                Button::new(
                    text(i)
   
                     .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .vertical_alignment(iced::alignment::Vertical::Center)
                        .size(15)
                )
                .width(Length::Fixed(200.0))
                .height(Length::Fixed(30.0))
                .style(iced::theme::Button::Custom(Box::new(Buttonstyless::eliminar)))
                .on_press(Message::orden(i.to_string())),
            ),
        )
        .spacing(15)
        .align_items(iced::Alignment::Start)
    );
}
let c=container(container(Scrollable::new(b))
.width(300)
.height(190)
.padding(Padding::from(10))
.center_x()
.center_x()
.style(iced::theme::Container::Custom(Box::new(Containestyle::cargar)))).width(Length::Fill).center_x().center_y();



let mut a = column!(
    
).width(Length::Fill)
.spacing(20)
.align_items(iced::Alignment::Center);

for i in orden.iter() {
    a= a.push(
        
        row!(

            container(
                Button::new(
                    text(i)
   
                     .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .vertical_alignment(iced::alignment::Vertical::Center)
                        .size(15)
                )
                .width(Length::Fixed(200.0))
                .height(Length::Fixed(30.0))
                .style(iced::theme::Button::Custom(Box::new(Buttonstyless::eliminar)))
                .on_press(Message::eliminarorden(i.to_string())),
            ),
        )
        .spacing(15)
        .align_items(iced::Alignment::Start)
    );
}



let d= container(container(Scrollable::new(a))
.width(300)
.height(190)
.padding(Padding::from(20))
.center_x()
.center_x()
.style(iced::theme::Container::Custom(Box::new(Containestyle::cargar)))).width(Length::Fill).center_x().center_y();

let mut ul=container(column!(
    
    container(text("Indique el orden").size(30).style(colore(color!(244, 246, 244)))).width(Length::Fill).center_x().center_y(),
    
    c,
    d,
    row!(
    Button::new(
        text("salir")
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .size(15)
    )
    .width(Length::Fixed(100.0))
    .height(Length::Fixed(30.0))
    .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
    .on_press(Message::menu),
    container(
    Button::new(
        text("Listo")

         .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .size(15)
    )
    .width(Length::Fixed(150.0))
    .height(Length::Fixed(30.0))
    .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
    .on_press(Message::guardar_orden)
   ).width(Length::Fill)
    .center_x().center_y())


      ).spacing(30))
        .padding(Padding::from(20))
        .width(300)
        .height(600)
        .center_x()
        .center_y()
        .style(iced::theme::Container::Custom(Box::new(Containestyle::menu)));

    container(ul)
    .width(330)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
    
}

fn cancel(orden:Vec<String>)->Element<'static,Message>{
  
    let mut a = column!(
    
    ).width(Length::Fill)
    .spacing(20)
    .align_items(iced::Alignment::Center);
    
    for i in orden.iter() {
        a= a.push(
            
            row!(
    
                container(
                    Button::new(
                        text(i)
       
                         .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center)
                            .size(15)
                    )
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(30.0))
                    .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
                    .on_press(Message::cancelar_proceso(i.to_string())),
                ),
            )
            .spacing(110)
            .align_items(iced::Alignment::Start)
        );
    }
    
    
    
    let d= container(container(Scrollable::new(a))
    .width(300)
    .height(400)
    .padding(Padding::from(20))
    .center_x()
    .center_x()
    .style(iced::theme::Container::Custom(Box::new(Containestyle::menu)))).width(Length::Fill).center_x().center_y();

  let c= container( column!(
        container(text("Cancelar proceso").size(30).style(colore(color!(244, 246, 244)))).width(Length::Fill).center_x().center_y(),
     d,
     Button::new(
        text("Salir")

         .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .size(25)
    )
    .width(Length::Fixed(200.0))
    .height(Length::Fixed(50.0))
    .style(iced::theme::Button::Custom(Box::new(Buttonstyless::eliminar)))
    .on_press(Message::menu)

    

    ).width(Length::Fill)
    .spacing(20)
    .align_items(iced::Alignment::Center)

) .width(300)
//.height(800)
.padding(Padding::from(20))
.center_x()
.center_x()
.style(iced::theme::Container::Custom(Box::new(Containestyle::cargar)));

container(

    c
).width(350)
.height(900)
.center_x()
.center_y()
.into()

}

fn fila(texto:&str,mut cola: Cola)->Element<'static,Message> {


    let mut a = column!(

        container(
            text(texto).size(15).style(colore(color!(244, 246, 244)))
        ).width(Length::Fill).center_x().center_y(),

    
    ).width(Length::Fill)
    .spacing(20)
    .align_items(iced::Alignment::Center);
  
    
   while !cola.esta_vacia() {
        
        let mut nombre=cola.desencolar();
        
        let nom=format!("{}[{}]",nombre.nombre,nombre.traza);

        a= a.push(
            
            row!(
    
                container(

                    container(Scrollable::new(text(nom).size(20).style(colore(color!(244, 246, 244)))))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                )
                .width(100)
                .height(50)
                .style(iced::theme::Container::Custom(Box::new(Containestyle::menu)))
            )
            .spacing(110)
            .align_items(iced::Alignment::Center)
        );
    }
    
    let d=container(Scrollable::new(a))
    .width(170)
    .height(610)
    .padding(Padding::from(20))
    .center_x()
    .center_x()
    .style(iced::theme::Container::Custom(Box::new(Containestyle::cargar)));

    container(d)
    //.width(220)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
    
}

fn pilas(texto:&str,mut cola:Pila)->Element<'static,Message> {

   

    let mut a = column!(

        container(
            text(texto).size(15).style(colore(color!(244, 246, 244)))
        ).width(Length::Fill).center_x().center_y(),

    
    ).width(Length::Fill)
    .spacing(20)
    .align_items(iced::Alignment::Center);
    
    while !cola.esta_vacia() {
        
        let  nombre=cola.pop();
        let nom=format!("{}[{}]",nombre.nombre,nombre.traza);
        a= a.push(
            
            row!(
    
                container(
                    container(Scrollable::new(text(nom).size(20).style(colore(color!(244, 246, 244)))))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                )
                .width(100)
                .height(50)
                .style(iced::theme::Container::Custom(Box::new(Containestyle::menu)))
            )
            .spacing(110)
            .align_items(iced::Alignment::Center)
        );
    }
    
    let d=container(Scrollable::new(a))
    .width(170)
    .height(610)
    .padding(Padding::from(20))
    .center_x()
    .center_x()
    .style(iced::theme::Container::Custom(Box::new(Containestyle::cargar)));

    container(d)
    //.width(220)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
    
}



enum  Containestyle {
    menu,
    cargar,
}


impl container::StyleSheet for Containestyle {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::from_rgb(0.0, 0.0, 0.0)),
            border: iced::Border::with_radius(20),
            background:Some(iced::Background::Color( match self{
                Self::menu=>color!(54, 68, 72),
                Self::cargar=>color!(215, 96, 62)
            })),
            shadow: Shadow {
                color: color!(215, 96, 62),
                offset: Vector::new(match self {
                    Self::menu=>0.0,
                    Self::cargar=>0.0,
                    
                },match self {
                    Self::menu=>0.2,
                    Self::cargar=>0.0,}
            )
                ,
                blur_radius: match self {
                    Self::menu=>40.0,
                    Self::cargar=>0.0,},
            },
        }
    }
}

struct Text_inputstyle;

impl text_input::StyleSheet for Text_inputstyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(color!(244, 246, 244)),
            border: iced::Border::with_radius(25),
            icon_color: color!(0, 0, 0),
        }
    }
    fn disabled_color(&self, style: &Self::Style) -> Color {
        color!(244, 246, 244)
    }
    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(color!(255, 0, 0)),
            border: iced::Border::with_radius(25),
            icon_color: color!(0, 0, 0),
        }
    }
    fn placeholder_color(&self, style: &Self::Style) -> Color {
        //color del texto de fondo
        color!(0, 0, 0)
    }
    fn value_color(&self, style: &Self::Style) -> Color {
        //color del texto que se inscribe
        color!(0, 0, 0)
    }
    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: iced::Background::Color(color!(164, 164, 146)),
            border: iced::Border::with_radius(25),
            icon_color: color!(0, 0, 0),
        }
    }
    fn selection_color(&self, style: &Self::Style) -> Color {
        color!(255, 0, 0)
    }
}

enum   Buttonstyless{


    menu,
    eliminar,

}

impl button::StyleSheet for Buttonstyless {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: color!(244, 246, 244),
            border: iced::Border::with_radius(25),
            background: Some(iced::Background::Color(match self{
               Self::menu=>color!(215, 96, 62),
               Self::eliminar=>color!(54, 68, 72),

            })),
            shadow: Shadow {
                color: match self{
                    Self::menu=>iced::Color::BLACK,
                    Self::eliminar=>iced::Color::TRANSPARENT,
                 },
                blur_radius: 40.0,
                offset: Vector::new(match self {
                    Self::menu=>0.0,
                    Self::eliminar=>0.0,
                    
                },match self {
                    Self::menu=>0.2,
                    Self::eliminar=>0.0,}
            ),
            },
            shadow_offset: Vector::new(match self {
                Self::menu=>0.0,
                Self::eliminar=>0.0,
                
            },match self {
                Self::menu=>0.2,
                Self::eliminar=>0.0,}
        ),
        }
    }
}
