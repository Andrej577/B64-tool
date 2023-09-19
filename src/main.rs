#![windows_subsystem = "windows"]

use dioxus::{prelude::*, html::input_data::keyboard_types::Key};

use serde_json::json;

#[warn(deprecated)]
use base64::encode;

use clipboard_win::{formats, set_clipboard};

fn main() {
    dioxus_desktop::launch(app)
}

fn app(cx: Scope) -> Element
{
    //varijable
                                //Udruga nižih časnika za izgradnju glavnih operativnih elektroenergetskih stanica za parobrodsku plovidbu na Dunavu.
    let ime = use_state(cx, || "Matic".to_string());
                                    //Zakon o prijenosu zadataka za nadzor označavanja govedine.
    let prezime = use_state(cx, || "Mate".to_string());
    let json = use_state(cx, || "".to_string());
    let b64 = use_state(cx, || "".to_string());
    //renderiranje svega zivog
    cx.render(rsx!(    
        head 
        {
            link 
            {
                link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/bootstrap@5.1.1/dist/css/bootstrap.min.css" }
                link { rel: "script", href: "https://cdn.jsdelivr.net/npm/bootstrap@5.1.1/dist/js/bootstrap.bundle.min.js" }
            }
        }
        div
        {
            class: "container",
            div
            {
                class: "column",
                input
                {
                    class: "form-control otuline-primary mt-4",
                    placeholder: "Unesi ime",
                    value: "{ime}",
                    oninput: move |e| 
                    {
                        ime.set(e.value.clone());
                        json.set(string_to_json(e.value.clone().to_string(), prezime.clone().to_string()));
                        b64.set(json_to_b64(json.clone().to_string())) 
                    },
                    onkeydown: move |evt| { if evt.key() == Key::Enter {} }
                },
                input
                {
                    class: "form-control otuline-primary mt-4",
                    placeholder: "Unesi ime",
                    value: "{prezime}",
                    oninput: move |e| 
                    {
                        prezime.set(e.value.clone());
                        json.set(string_to_json(ime.clone().to_string(), e.value.clone().to_string()));
                        b64.set(json_to_b64(json.clone().to_string()));
                    },
                    onkeydown: move |evt| { if evt.key() == Key::Enter {} }
                },
                div
                {   
                    class: "row",
                    div
                    {
                        h2
                        {
                            class: "mt-4",
                            "JSON:"
                        }
                        div
                        {
                            class: "otuline-primary text-break m-4",
                            "{json}"
                        }
                    }
                    div
                    {   
                        class: "col",
                        h2
                        {
                            class: "mt-4",
                            "Base 64:"
                        }
                        div
                        {
                            class: "otuline-primary text-break m-4",
                            "{b64}"
                        }
                    }
                }
                button
                {
                    class: "btn btn-outline-primary mt-4",
                    style: "width: 100px",
                    onclick: move |_|
                    {
                        let _ = set_clipboard(formats::Unicode, b64.clone().to_string());
                    },
                    "Copy to clipboard",
                    
                }
            }
        }))

}

fn string_to_json(name: String, prezime: String) -> String {
    let json = json!({
        "name": name,
        "prezime": prezime });

    let json_string = format!("{:#}", json).replace("", "");
    dbg!(&json);
    dbg!(&json_string);
    json_string
    //json.to_string()
}

fn json_to_b64(json: String) -> String
{
    let a = encode(&json);
    a
}

