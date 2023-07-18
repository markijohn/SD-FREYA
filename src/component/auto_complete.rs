use std::fmt::Display;

use freya::prelude::*;
use freya::hooks::{use_focus, use_get_theme};
use freya::prelude::keyboard::Key;

/// [`AutoCompleteItem`] component properties.
#[derive(Props)]
pub struct AutoCompleteItemProps<'a, T: 'static> {
    /// Presentation of value
    children: Element<'a>,

    /// popup list index
    item_idx : i32,

    /// value
    value: T,

    /// Handler for the `onclick` event.
    #[props(optional)]
    onclick: Option<EventHandler<'a, &'a T >>,
}

/// `AutoCompleteItem` component.
///
/// # Props
/// See [`AutoCompleteItemProps`].
///
/// # Styling
/// Inherits the [`DropdownItemTheme`](freya_hooks::DropdownItemTheme) theme.
#[allow(non_snake_case)]
pub fn AutoCompleteItem<'a, T>(cx: Scope<'a, AutoCompleteItemProps<'a, T>>) -> Element<'a>
where
    T: PartialEq + 'static + Clone,
{
    let theme = use_get_theme(cx);
    let state = use_shared_state::<AutoCompleteState>(cx).unwrap();
    let is_focused = state.read().idx() == cx.props.item_idx;

    let background = if is_focused {
        theme.dropdown_item.hover_background
    } else {
        theme.dropdown_item.background
    };
    let color = theme.dropdown_item.font_theme.color;

    let onclick = move |_: MouseEvent| {
        if state.read().is_focused() {
            *state.write() = AutoCompleteState::Selected( cx.props.item_idx );
        }
    };

    let onmouseenter = move |_| {
        *state.write() = AutoCompleteState::Focused( cx.props.item_idx );
    };

    render!(rect {
        color: color,
        width: "100%",
        height: "35",
        background: background,
        padding: "6",
        onmouseenter: onmouseenter,
        onclick: onclick,
        &cx.props.children
    })
}

/// [`AutoComplete`] component properties.
#[derive(Props)]
pub struct AutoCompleteProps<'a, T: 'static> {
    /// Input value
    value: T,

    // #[props(optional, default=None)]
    // updated : Option<bool>,
    item_count : i32,

    #[props(optional)]
    onchange : Option< EventHandler<'a, String> >,

    #[props(optional)]
    onselected : Option< EventHandler<'a, String> >,

    children: Element<'a>,
}

/// Current status of the AutoComplete.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum AutoCompleteState {
    /// Default state.
    #[default]
    None,

    /// Focus item
    Focused(i32),

    /// Selected item (click item, key enter)
    Selected(i32)
}

impl AutoCompleteState {
    pub fn idx(&self) -> i32 {
        match self {
            Self::None => -1,
            Self::Focused(v) => *v,
            Self::Selected(v) => *v
        }
    }

    pub fn is_focused(&self) -> bool {
        matches!( self, Self::Focused(_) )
    }

    pub fn is_selected(&self) -> bool {
        matches!( self, Self::Selected(_) )
    }
}

/// `AutoComplete` component.
///
/// # Props
/// See [`AutoCompleteProps`].
///
/// # Styling
/// Inherits the [`DropdownTheme`](freya_hooks::DropdownTheme) theme.
///
/// # Example
/// ```no_run
/// # use freya::prelude::*;
///
/// fn app(cx: Scope) -> Element {
///     let hints = ["alice", "bob", "carol"]
///     let input_value = use_state(cx, || "input here...".to_string());
///     render!(
///         AutoComplete {
///             value: input_value.get().clone(),
///             values.iter().map(|ch| {
///                 rsx!(
///                     AutoCompleteItem {
///                         value: ch.to_string(),
///                         onclick: move |_| selected_dropdown.set(ch.to_string()),
///                         label { "! {ch}" }
///                     }
///                 )
///             })
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
fn AutoComplete<'a, T>(cx: Scope<'a, AutoCompleteProps<'a, T>>) -> Element<'a>
where
    T: PartialEq + Clone + Display + 'static,
{
    let theme = use_get_theme(cx);
    let state = use_shared_state::<AutoCompleteState>(cx).unwrap();
    let opened = use_shared_state::<bool>(cx).unwrap();

    println!("is_opened : {}" , *opened.read() && cx.props.children.is_some() );

    let color = theme.dropdown.font_theme.color;

    // Update the provided value if the passed value changes
    // use_effect(cx, &cx.props.value, move |value| {
    //     *selected.write() = value;
    //     async move {}
    // });

    // Close the dropdown if clicked anywhere
    let onglobalclick = move |_: MouseEvent| {
        *state.write() = AutoCompleteState::None;
        *opened.write() = false;
    };

    let onkeyup = move |e:KeyboardEvent| {
        let is_opened = *opened.read() && cx.props.children.is_some();
        let idx = state.read().idx();
        match e.key {
            // Close when `Escape` key is pressed
            Key::Escape => {
                *opened.write() = false;
                *state.write() = AutoCompleteState::None;
            }
            // Open the dropdown items when the `Enter` key is pressed
            Key::Enter if is_opened => {
                if cx.props.item_count > 0 {
                    *opened.write() = false;
                    *state.write() = AutoCompleteState::Selected( idx );
                }
            }
            Key::ArrowUp => {
                if is_opened && cx.props.item_count > 0 {
                    if idx - 1 < 0 {
                        *state.write() = AutoCompleteState::Focused( cx.props.item_count-1 );
                    } else {
                        *state.write() = AutoCompleteState::Focused( idx - 1 );
                    }
                }
            }
            Key::ArrowDown => {
                if is_opened && cx.props.item_count > 0 {
                    if idx + 1 >= cx.props.item_count {
                        *state.write() = AutoCompleteState::Focused( 0 );
                    } else {
                        *state.write() = AutoCompleteState::Focused( idx + 1 );
                    }
                }
            }
            _ => {}
        }
    };

	let input_text = use_state(cx, String::new);

    render!(
        rect {
            width: "auto",
            height: "auto",
            margin: "5",
            rect {
                offset_y : 40,
                overflow: "clip",
                layer: "-1",
                corner_radius: "3",
                onglobalclick: onglobalclick,
                onkeyup: onkeyup,
                width: "130",
                height: "auto",
                shadow: "0 0 20 0 rgb(0, 0, 0, 100)",
                Input {
                    max_lines: "none",
                    value: input_text.get().clone(),
                    onchange: |e:String| {
                        input_text.set( e.clone() );
                        if let Some(caller) = &cx.props.onchange {
                            caller.call( e );
                        }
                    }
                }
                &cx.props.children
            }
        }
    )
}


#[inline_props]
pub fn SimpleWordComplete(cx: Scope, get_word_hints:fn(&str) -> Vec<String>) -> Element {
    // state
    use_shared_state_provider(cx, AutoCompleteState::default);

    // open handle
    use_shared_state_provider(cx, ||false);

    let opened = use_shared_state::<bool>(cx).unwrap();
    let state = use_shared_state::<AutoCompleteState>(cx).unwrap();
    let input_value = use_state(cx, String::new);
    let auto_hints:&UseState<Vec<String>> = use_state(cx, Vec::new);

    let onchange = |e:String| {
        if input_value.get() != &e {
            let last = &e[ e.rfind( char::is_whitespace ).map( |e| e+1 ).unwrap_or( e.len() ) .. ];
            
            let new_hints = 
            if last.len() > 0 {
                get_word_hints(last)
            } else {
                vec![]
            };

            if auto_hints.get() != &new_hints {
                *state.write_silent() = AutoCompleteState::None;
                if new_hints.len() > 0 {
                    *opened.write() = true;
                } else {
                    *opened.write() = false;
                }
                auto_hints.set( new_hints );
            }

            input_value.set(e.clone() );
        }
    };

    println!("Painting sw");

    render!(
        AutoComplete {
            value : input_value.get().clone(),
            item_count : auto_hints.get().len() as _,
            onchange : onchange,
            for (i,h) in auto_hints.get().iter().enumerate() {
                AutoCompleteItem {
                    value : h.to_owned(),
                    item_idx : i as _,
                    onclick : |_| {},
                    label { "{h}" }
                }
            }        
        }
    )
}
