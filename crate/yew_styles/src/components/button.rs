use crate::styles::{get_pallete, get_style, Palette, Style};
use yew::prelude::*;

/// The standard sizes for button
#[derive(Clone)]
pub enum Size {
    Small,
    Medium,
    Big,
}

/// # Button component
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew::services::ConsoleService;
/// use yew_styles::{
///     button::{Button, Size},
///     styles::{Palette, Style},
/// };
///
/// pub struct App {
///   link: ComponentLink<Self>,
/// }
///
/// pub enum Msg {
///   Clicked(String),
/// }
/// #[derive(Clone, Properties)]
/// pub struct Props {}
///
/// impl Component for App {
///     type Message = Msg;
///     type Properties = Props;
///
///     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
///         App {
///             link
///         }
///     }
///
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         match msg {
///             Msg::Clicked(greeting) => {
///                 let mut console = ConsoleService::new();
///                 console.log(format!("{}", greeting))
///             }
///         }
///         false
///     }
///
///     fn view(&self) -> Html {
///        html! {
///          <Button
///             onsignal=link.callback(move |_| Msg::Clicked("Hello world"))
///             class_name="hello-world"
///             button_type=Pallete::Standard
///             button_style=Style::Light
///             size=Size::Medium
///          >{"Greeting"}</Button>
///        }
///     }
/// }
/// ```
pub struct Button {
    link: ComponentLink<Self>,
    props: ButtonProps,
}

struct ButtonProps {
    button_type: String,
    size: String,
    button_style: String,
    class_name: String,
    onsignal: Callback<()>,
    children: Children,
}

impl From<Props> for ButtonProps {
    fn from(props: Props) -> Self {
        ButtonProps {
            button_type: get_pallete(props.button_type),
            size: get_size(props.size),
            button_style: get_style(props.button_style),
            class_name: props.class_name,
            onsignal: props.onsignal,
            children: props.children,
        }
    }
}

#[derive(Clone, Properties)]
pub struct Props {
    /// Type botton style. Options included in `Pallete`
    #[prop_or(Palette::Standard)]
    pub button_type: Palette,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// Three diffent button standard sizes. Options included in `Size`
    #[prop_or(Size::Medium)]
    pub size: Size,
    /// Button styles. Options included in `Style`
    #[prop_or(Style::Regular)]
    pub button_style: Style,
    /// Click event for button
    pub onsignal: Callback<()>,
    pub children: Children,
}

pub enum Msg {
    Clicked,
}

pub fn get_size(size: Size) -> String {
    match size {
        Size::Small => String::from("small"),
        Size::Medium => String::from("medium"),
        Size::Big => String::from("big"),
    }
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            link,
            props: ButtonProps::from(props),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onsignal.emit(());
            }
        };

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = ButtonProps::from(props);
        true
    }

    fn view(&self) -> Html {
        html! {
            <button
                onclick=self.link.callback(|_| Msg::Clicked)
                class=format!("button {} {} {} {}",
                    self.props.button_type.clone(),
                    self.props.size.clone(),
                    self.props.button_style.clone(),
                    self.props.class_name.clone())
            > { self.props.children.render() }
            </button>
        }
    }
}