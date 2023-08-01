#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::borrow::Cow;

use freya::prelude::*;

mod component;

use crate::component::prelude::*;

fn main() {
    launch(app);
}

pub fn make_item<'a,T>( index:usize, cx:Scope<'a, T>) -> Element<'a> {
    render!( rect {
        label { "{index}" }
    } )
}

fn app(cx: Scope) -> Element {
    use_init_focus(cx);
    let mut value = use_state(cx, String::new);
    let mut times = use_state(cx, || 1);
    let mut item_size = use_state(cx, || 50f32);

    let (node_ref, node) = use_node(cx);

    let values = cx.use_hook(|| vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    let selected_dropdown = use_state(cx, || "A".to_string());

    render!(
        rect {
            width: "100%",
            height: "100%",
            background: "rgb(0, 109, 119)",
            direction: "vertical",
            reference : node_ref,
            rect {
                width : "100%",
                height : "100%",
                
                Split {
                    initial_size : 200,
                    direction : SplitDirection::Horizontal,
                    first_child : render!( rect {
                        width:"100%",
                        height:"100%",
                        Detail {
                            summary : Cow::Borrowed("Local Device Info"),
                            // label { "⯈  My Computer" }
                            Detail {
                                summary : Cow::Borrowed("Summer Time!"),
                            }
                        }
                        HR {}
                    }),
                    second_child : render!( 
                        Split {
                            direction : SplitDirection::Vertical,
                            first_child : render!( 
                                rect {
                                    width : "100%",
                                    direction : "horizontal",
                                    SimpleWordComplete {
                                        get_word_hints : |last| {
                                            const hints:[&'static str;79] = [
                                                "Alice", "Bob", "Car","Dog","Elephant","Fish","Giraffe","Horse","Ice cream","Jaguar","Kangaroo","Lion","Monkey","Nectarine","Octopus","Penguin","Queen","Rabbit","Snake","Tiger","Umbrella","Vase","Whale","Xylophone","Yak","Zebra",
                                                "Aardvark","Bison","Cheetah","Dolphin","Elephant","Falcon","Gorilla","Hippopotamus","Ibis","Jaguar","Kangaroo","Lion","Moose","Nightingale","Ostrich","Penguin","Quokka","Raccoon","Squirrel","Tiger","Umbrella","Vulture","Walrus","X-ray tetra","Yak","Zebra",
                                                "Alligator","Bear","Cat","Dog","Elephant","Fox","Giraffe","Horse","Iguana","Jaguar","Kangaroo","Lion","Monkey","Nightingale","Octopus","Penguin","Quokka","Rabbit","Snake","Tiger","Umbrella","Vulture","Whale","X-ray fish","Yak","Zoo",
                                                "Alpine"];
                                            hints.iter().filter( |h| h.len() != last.len() && h.starts_with(last) )
                                                .map( |e| e.to_string() ).collect::<Vec<String>>()
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
                                }
                            )
                            second_child : render!( rect {
                                width:"100%",
                                height:"100%",
                                rect {
                                    width:"100%",
                                    FloatRight {
                                        Slider {
                                            width: 100.0,
                                            value: *item_size.get() as f64,
                                            onmoved: |p| {
                                                item_size.set( p as f32 );
                                            }
                                        }
                                    }
                                }
                                
                                SquareGrid {
                                    h_gap : 5.,
                                    v_gap : 5.,
                                    item_width : *item_size.get(),
                                    item_height : *item_size.get(),
                                    item_length : 30,
                                    builder : make_item
                                    // builder : Box::new( |i,cx:Scope<'_,VirtualScrollViewProps<'_,()>>| {
                                    //     render!( rect {
                                    //         label { "{i}" }
                                    //     } )
                                    // })
                                }
                            })
                        }
                    )
                }
            }
        }
    )
}