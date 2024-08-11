mod weather;

use iced::{Alignment, Background, Border, Element, Length, Padding, Sandbox, Settings, Shadow, Vector};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, container, TextInput, text, Button, Container, Column, Row};
use iced::theme::Theme;
use iced::widget::text::Shaping;
use crate::weather::{display_weather_info, get_weather_info};

fn main() -> iced::Result {
    WeatherUI::run(Settings::default())
}

struct WeatherUI {
    location_info: LocationInfo,
    page: Page,
    theme: Theme,
    api_info: ApiInfo,
    weather_results: WeatherResults,
}

struct LocationInfo {
    city_name: String,
    country_name: String,
}
struct ApiInfo {
    api_key: String,
}

struct WeatherResults {
    weather: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Page {
    ApiPage,
    LocationPage,
    WeatherResultPage,
}
#[derive(Clone, Debug)]
enum Message {
    ButtonSubmit,
    ThemeSelect,
    ApiInfoSubmit(String),
    Router(String),
    LocationInfoChanged(String, String),
}

impl Sandbox for WeatherUI {
    type Message = Message;

    fn new() -> Self {
        Self {
            location_info: LocationInfo {
                city_name: String::new(),
                country_name: String::new(),
            },
            theme: Theme::Dark,
            page: Page::ApiPage,
            api_info: ApiInfo {
                api_key: String::new(),
            },
            weather_results: WeatherResults {
                weather: String::new(),
            },

        }
    }
    fn title(&self) -> String {
        String::from(" Rusty Weather Station")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonSubmit => {
                return self.weather_results.weather = match get_weather_info(&self.location_info.city_name, &self.location_info.country_name, &self.api_info.api_key) {
                    Ok(response) => {
                        self.page = Page::WeatherResultPage;
                        display_weather_info(&response) // Displaying weather information
                    }
                    Err(err) => {
                        format!("{}", err).to_string() // Printing error message in case of failure
                    }
                }
            }
            Message::ThemeSelect => {
                self.theme = if self.theme == Theme::Light {
                    Theme::Dark
                } else {
                    Theme::Light
                }
            }
            Message::ApiInfoSubmit(api_key) => { self.api_info.api_key = api_key; }
            Message::LocationInfoChanged(city_name, country_name) => {
                self.location_info.city_name = city_name;
                self.location_info.country_name = country_name;
            }
            Message::Router(route) => {
                if route == "api" {
                    self.page = Page::ApiPage
                } else if route == "location" {
                    self.page = Page::LocationPage
                } else if route == "weather" {
                    self.page = Page::WeatherResultPage
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self.page {
            Page::LocationPage => location_page(&self.location_info),
            Page::ApiPage => api_page(&self.api_info),
            Page::WeatherResultPage => weather_page(&self.weather_results),
        };

        // wrapper for page content
        let wrapper = Column::new()
            .spacing(50)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(content)
            .push(
                match self.page {
                    Page::ApiPage => footer_page(
                        button("Next Page")
                            .on_press(Message::Router("location".to_string()))
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton)))
                    ),
                    Page::LocationPage => footer_page(
                        button("Next Page")
                            .on_press(Message::Router("weather".to_string()))
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton)))
                    ),
                    Page::WeatherResultPage => footer_page(
                        button("Start Page")
                            .on_press(Message::Router("api".to_string()))
                            .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton)))
                    ),
                }
            );

        container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from(20))
            .center_y()
            .center_x()
            .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
// input field
fn input_field(_placeholder: &str, _value: &str) -> TextInput<'static, Message> {
    TextInput::new(_placeholder, _value)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(10))
        .line_height(text::LineHeight::Relative(1.75))
}

fn api_page(api_info: &ApiInfo) -> Container<Message> {
    let column = Column::new()
        .push(text("OpenWeather API Key").size(24))
        .align_items(Alignment::Center)
        .padding(Padding::from([20, 20]))
        .spacing(50)
        .push(
            input_field("Provide Api key ...", &api_info.api_key)
                .on_input(
                    |api_key| {
                        Message::ApiInfoSubmit(api_key.clone())
                    }
                )
        );

    container(column)
        .padding(Padding::from(20))
        .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
}
fn location_page(location_info: &LocationInfo) -> Container<Message> {
    let column = Column::new()
        .push(text("Check weather by City name and Country code")
            .size(20))
        .push(
            input_field("City name...", &location_info.city_name)
                .on_input(
                    |city_name| {
                        Message::LocationInfoChanged(city_name, location_info.country_name.clone())
                    }
                )
        )
        .push(
            input_field("Country code...", &location_info.country_name)
                .on_input(
                    |country_name| {
                        Message::LocationInfoChanged(location_info.city_name.clone(), country_name)
                    }
                )
        )
        .push(submit_button("Check Weather!", Message::ButtonSubmit))
        .padding(Padding::from([20, 20]))
        .align_items(Alignment::Center)
        .spacing(40);

    container(column)
        .padding(Padding::from(20))
        .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
}

fn weather_page(weather_results: &WeatherResults) -> Container<Message> {
    let column = Column::new()
        .push(text(format!("{}", weather_results.weather))
            .size(20)
            .shaping(Shaping::Advanced)
        )
        .width(Length::Fixed(500.0))
        .padding(Padding::from([20, 20]))
        .align_items(Alignment::Center)
        .spacing(40);

    container(column)
        .padding(Padding::from(20))
        .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
}

fn footer_page(butt: Button<Message>) -> Container<Message> {
    let footer = Row::new()
        .push(
            button("Toggle Theme")
                .on_press(Message::ThemeSelect)
                .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton)))
        )
        .push(butt)
        .align_items(Alignment::Center)
        .spacing(10);

    container(footer).center_x().center_y()
}

// submit button
fn submit_button(name: &str, event: Message) -> Button<Message> {
    Button::new(
        text(name)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(21)
    )
        .on_press(event)
        .width(Length::Fixed(300.0))
        .height(Length::Fixed(45.0))
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Standard)))
}


// button styling defined
enum ButtonStyle { Standard, ThemeButton }

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, theme: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(match self {
                Self::Standard => iced::Color::from_rgb(0.06, 0.46, 0.7),
                Self::ThemeButton => iced::Color::default(),
            })),
            border: match self {
                Self::Standard => Border::with_radius(5),
                Self::ThemeButton => Border::default(),
            },
            shadow: match self {
                Self::Standard => Shadow {
                    color: iced::Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
                Self::ThemeButton => Shadow::default(),
            },
            text_color: {
                if theme == &Theme::Light {
                    match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::BLACK,
                    }
                } else {
                    match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::WHITE,
                    }
                }
            },
            ..Default::default()
        }
    }
}

struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _theme: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Default::default(),
            border: Border::with_radius(5),
            background: None,
            shadow: Shadow {
                color: iced::Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 20.0,
            },
        }
    }
}