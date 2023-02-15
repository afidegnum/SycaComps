use serde::{Deserialize, Serialize};
use serde_json::Value;

use sycamore::prelude::*;

use super::checkbox::CheckBoxInput;
use super::color::ColorInput;
use super::datalist::DataList;
use super::date::DateInput;
use super::datetime::DateTimeInput;
use super::email::EmailInput;
use super::fileinput::FileInput;
use super::integer::IntegerInput;
use super::password::PassWordInput;
use super::radio::RadioInput;
use super::range::RangeInput;
use super::search::SearchInput;
use super::tel::TelelphoneInput;
use super::textarea::TextArea;
use super::textinput::TextInput;
use super::url::UrlInput;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Item {
    name: String,
    key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Array {
    items: Vec<Item>,
}

#[component]
pub fn InputForm<G: Html>(cx: Scope, s: (String, Value)) -> View<G> {
    let field_type =
        s.1.clone()
            .get("type")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();

    let y = s.clone();

    let input_type = match field_type.as_ref() {
        "string" => TextInput(cx, y),
        "textarea" => TextArea(cx, y),
        "password" => PassWordInput(cx, y),
        "integer" => IntegerInput(cx, y),
        "range" => RangeInput(cx, y),
        "color" => ColorInput(cx, s),
        "email" => EmailInput(cx, y),
        "tel" => TelelphoneInput(cx, y),
        "date" => DateInput(cx, y),
        "datetime" => DateTimeInput(cx, y),
        "file" => FileInput(cx, y),
        "url" => UrlInput(cx, y),
        "radio" => RadioInput(cx, y),
        "checkbox" => CheckBoxInput(cx, y),
        "search" => SearchInput(cx, y),
        "datalist" => DataList(cx, y),
        _ => view! {cx, div{}},
    };
    input_type
}
