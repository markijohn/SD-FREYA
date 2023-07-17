#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

mod component;

use component::split::{SplitDirection, Split};
use component::auto_complete::{AutoComplete, AutoCompleteItem};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    use_init_focus(cx);
    let mut value = use_state(cx, String::new);
    let mut times = use_state(cx, || 1);

    let values = cx.use_hook(|| vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    let selected_dropdown = use_state(cx, || "A".to_string());
    
    let (node_ref, node) = use_node(cx);
    let exclamations = "!".repeat(*times.get());

    let values = cx.use_hook(|| vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    let selected_dropdown = use_state(cx, || "A".to_string());

    render!(
        rect {
            width: "100%",
            height: "100%",
            background: "rgb(0, 109, 119)",
            direction: "vertical",
            // display: "center",
            onclick: move |_| times += 1,
            reference : node_ref,
            rect {
                width : "100%",
                height : "200",
                Split {
                    direction : SplitDirection::Horizontal,
                    first_child : render!( 
                        label { "Split1" }  
                    ),
                    second_child : render!( rect {
                        label { "Split2" } 
                    })
                }
            }
            AutoComplete {
                value: selected_dropdown.get().clone(),
                values.iter().map(|ch| {
                    rsx!(
                        AutoCompleteItem {
                            value: ch.to_string(),
                            onclick: move |_| selected_dropdown.set(ch.to_string()),
                            label { "{ch}" }
                        }
                    )
                })
            }
            rect {
                width: "100%",
                direction: "horizontal",
                for i in 0..10 {
                    Button { label { "IterButton {i}" } }
                }
            },
            Dropdown {
                value: selected_dropdown.get().clone(),
                values.iter().map(|ch| {
                    rsx!(
                        DropdownItem {
                            value: ch.to_string(),
                            onclick: move |_| selected_dropdown.set(ch.to_string()),
                            label { "{ch}" }
                        }
                    )
                })
            },
            
            Input { value : value.get().clone(), onchange : |e| { value.set(e) } },
            for i in 0 .. *times.get() {
                label {
                    width: "100%",
                    font_size: "50",
                    align: "center",
                    color: "white",
                    "Hello, World {i} {exclamations}"
                }
            }
            
        }
    )
}
