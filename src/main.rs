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
enum Message {

     proceso(String),

     traza(String),
    incrementar,

    decrementar,

    guardar,

    eliminar(String),

    pala(String),
}

struct Calcular {
    value: i32,
    ve: Vec<String>,
    proceso: String,
    traza: String,
    cant: usize,
}

pub fn main() -> iced::Result {
    Calcular::run(Settings::default())
}

impl Application for Calcular {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(fla: ()) -> (Calcular, Command<Message>) {
        (
            Calcular {
                value: 0,
                ve: vec![],
                proceso: "".to_string(),
                traza: "".to_string(),
                cant: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Calculadora")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {

            Message::traza(tra)=>{

                self.traza=tra;
            }

            Message::proceso(proce)=>{

                self.proceso=proce;
            }

            Message::incrementar => {
                self.value += 1;
            }
            Message::decrementar => {
                self.value -= 1;
            }
            Message::pala(palabra) => {}

            Message::guardar => {

                self.ve.push(self.traza.clone());
                self.traza="".to_string();
            }
            Message::eliminar(eliminar) => {
                self.ve = self
                    .ve
                    .clone()
                    .into_iter()
                    .filter(|x| *x != eliminar)
                    .collect();
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
       //login().into()
        cargar(self.traza.clone(),self.proceso.clone(), self.ve.clone()).into()
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
        .on_press(Message::incrementar),
        Button::new(
            text("Cargar Procesos")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(15)
        )
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
        .on_press(Message::incrementar),
        Button::new(
            text("Atender procesos")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(15)
        )
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
        .on_press(Message::incrementar),
        Button::new(
            text("Terminar proceso")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(15)
        )
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
        .on_press(Message::incrementar),
        Button::new(
            text("Cancelar proceso")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(15)
        )
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
        .on_press(Message::incrementar),
        Button::new(
            text("Salir")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(15)
        )
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
        .on_press(Message::incrementar),
    )
    .width(Length::Fill)
    .spacing(20)
    .align_items(iced::Alignment::Center);

    let c = column!(imagen.width(150).height(150), a)
        .spacing(30)
        .align_items(iced::Alignment::Center);

    let b = container(c)
        .width(300)
        .padding(Padding::from(40))
        .center_x()
        .center_y()
        .style(iced::theme::Container::Custom(Box::new(Containestyle::cargar)));

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

            Button::new(
                text("Agregar proceso")
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .vertical_alignment(iced::alignment::Vertical::Center)
                    .size(15)
            )
            .width(Length::Fixed(500.0))
            .height(Length::Fixed(45.0))
            .style(iced::theme::Button::Custom(Box::new(Buttonstyless::menu)))
            .on_press(Message::incrementar)  

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