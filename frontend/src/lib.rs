// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};
use tracing::{instrument, debug, info};
use logic::AddOperation;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
#[instrument]
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        a: 0,
        input_element_a: ElRef::new(),
        b: 0,
        input_element_b: ElRef::new(),
        total: 0,
        method: CalculationMethod::Frontend
    }
}

// ------ ------
//     Model
// ------ ------

// probably don't need all of these...
#[derive(Debug, Eq, PartialEq, EnumIter, EnumString, Display, IntoStaticStr)]
#[strum(serialize_all = "title_case")]
enum CalculationMethod {
    Frontend,
    Backend,
    AsyncBackend
}

impl CalculationMethod {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Frontend => "Calculate in web assembly",
            Self::Backend => "Calculate serverside",
            Self::AsyncBackend => "Calculate serverside asyncrously with websockets",
        }
    }
}

// `Model` describes our app state.
struct Model {
    a: i32,
    input_element_a: ElRef<web_sys::HtmlInputElement>,
    b: i32,
    input_element_b: ElRef<web_sys::HtmlInputElement>,
    total: i32,
    method: CalculationMethod,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Debug, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    InputAChanged(i32),
    InputBChanged(i32),
    Equals,
    TotalCalculated(i32),
    CalculationMethodChanged(String),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::InputAChanged(number) => {
            debug!(number);
            model.a = number;
        }
        Msg::InputBChanged(number) => {
            debug!(number);
            model.b = number;
        }
        Msg::Equals => {
            log!("calculate");
            orders.skip();  // don't need to rerender now

            let operation = AddOperation::new(&[model.a, model.b]);
            match model.method {
                CalculationMethod::Frontend => {
                    debug!("add up numbers in the front end");
                    let total = operation.to_output().get_result();
                    orders.send_msg(Msg::TotalCalculated(total));
                },
                CalculationMethod::Backend => {
                    debug!("add up numbers in a lamdba at the backend");
                    // could use url create here as well
                    let url = format!("{}/add-numbers", constants::API_BASE_URL);
                    let request = Request::new(url)
                        .method(Method::Post)
                        .json(&operation)
                        .expect("serialisation go pop");

                    orders.perform_cmd(async {
                        let response = fetch(request).await.expect("HTTP request go pop");

                        let total = response
                            .check_status()
                            .expect("Status go pop")
                            .json::<logic::OperationResult>()
                            .await
                            .expect("deserialisation go pop")
                            .get_result();

                            debug!("received total {}", total);

                        Msg::TotalCalculated(total)
                    });
                },
                CalculationMethod::AsyncBackend => {
                    debug!("a websockety type thingy")
                },
            }
        },
        Msg::TotalCalculated(total) => {
            debug!("total: {}", total);
            model.total = total
        }
        Msg::CalculationMethodChanged(method) => {
            model.method = CalculationMethod::from_str(&method).unwrap_or(CalculationMethod::Frontend);
        }
    }
}

// ------ ------
//     View
// ------ ------

mod constants;

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        "Enterprise ready addition: ",
        C!["counter"],
        input![
            el_ref(&model.input_element_a),
            attrs! {
                At::Type => "number"
            },
            ev(Ev::Input, |event| {
                let element = event
                    .target()
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlInputElement>();
                IF!(element.report_validity() => Msg::InputAChanged(element.value().parse().unwrap_or_default()))
            }),
        ],
        " + ",
        input![
            el_ref(&model.input_element_b),
            attrs! {
                At::Type => "number"
            },
            ev(Ev::Input, |event| {
                let element = event
                    .target()
                    .unwrap()
                    .unchecked_into::<web_sys::HtmlInputElement>();
                IF!(element.report_validity() => Msg::InputBChanged(element.value().parse().unwrap_or_default()))
            }),
        ],
        button![" = ", ev(Ev::Click, |_| Msg::Equals)],
        input![attrs! {
            At::Type => "number",
            At::ReadOnly => true,
            At::Value => model.total.to_string()
        }],
        div![
            br![],
            "Calculation method: ",
            select![
                attrs! {
                    At::Name => "calc_method"
                },
                CalculationMethod::iter().map(|method| option![attrs! {At::Value => method}, method.label()]),
                input_ev(Ev::Change, Msg::CalculationMethodChanged),
            ]
        ],
        div![
            br![],
            "API base url: ",
            constants::API_BASE_URL
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::TRACE)
    //     .with_ansi(false)
    //     .without_time()
    //     .init();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_ansi(true)
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("starting app");

    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
