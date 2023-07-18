use std::fmt::Display;

use freya::prelude::*;
use freya::hooks::{use_focus, use_get_theme};
use freya::prelude::keyboard::Key;

/// [`AutoCompleteItem`] component properties.
#[derive(Props)]
pub struct AutoCompleteItemProps<'a, T: 'static> {
    /// Selectable items, like [`DropdownItem`]
    children: Element<'a>,
    /// Selected value.
    value: T,
    /// Handler for the `onclick` event.
    #[props(optional)]
    onclick: Option<EventHandler<'a, ()>>,
}

/// Current status of the AutoCompleteItem.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum AutoCompleteItemState {
    /// Default state.
    #[default]
    Idle,
    /// Dropdown item is being hovered.
    Hovering,
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
    T: PartialEq + 'static,
{
    let selected = use_shared_state::<T>(cx).unwrap();
    let theme = use_get_theme(cx);
    let focus = use_focus(cx);
    let state = use_state(cx, AutoCompleteItemState::default);

    let is_selected = *selected.read() == cx.props.value;

    let background = match *state.get() {
        _ if is_selected => "rgb(50,50,50)",
        AutoCompleteItemState::Hovering => theme.dropdown_item.hover_background,
        AutoCompleteItemState::Idle => theme.dropdown_item.background,
    };
    let color = theme.dropdown_item.font_theme.color;

    let onclick = move |_: MouseEvent| {
        if let Some(onclick) = &cx.props.onclick {
            onclick.call(())
        }
    };

    let onmouseenter = move |_| {
        state.set(AutoCompleteItemState::Hovering);
    };

    let onmouseleave = move |_| {
        state.set(AutoCompleteItemState::default());
    };

    let onkeydown = move |ev: KeyboardEvent| {
        if ev.key == Key::Enter {
            if let Some(onclick) = &cx.props.onclick {
                onclick.call(())
            }
        }
    };

    render!(rect {
        color: color,
        width: "100%",
        height: "35",
        background: background,
        padding: "6",
        onmouseenter: onmouseenter,
        onmouseleave: onmouseleave,
        onclick: onclick,
        onkeydown: onkeydown,
        &cx.props.children
    })
}

/// [`Dropdown`] component properties.
#[derive(Props)]
pub struct AutoCompleteProps<'a, T: 'static> {
    /// Selectable items, like [`DropdownItem`]
    children: Element<'a>,
    /// Input value.
    value: T,
}

/// Current status of the Dropdown.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum AutoCompleteState {
    /// Default state.
    #[default]
    Idle,
    /// Dropdown is being hovered.
    Hovering,
}

/// `Dropdown` component.
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
///     let values = cx.use_hook(|| vec!["A".to_string(), "B".to_string(), "C".to_string()]);
///     let selected_dropdown = use_state(cx, || "A".to_string());
///     render!(
///         AutoComplete {
///             value: selected_dropdown.get().clone(),
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
pub fn AutoComplete<'a, T>(cx: Scope<'a, AutoCompleteProps<'a, T>>) -> Element<'a>
where
    T: PartialEq + Clone + Display + 'static,
{
    use_shared_state_provider(cx, || cx.props.value.clone());
    let selected = use_shared_state::<T>(cx).unwrap();
    let theme = use_get_theme(cx);
    let focus = use_focus(cx);
    let state = use_state(cx, DropdownState::default);
    let opened = use_state(cx, || false);

	let is_focused = focus.is_focused();
    let focus_id = focus.attribute(cx);
    let is_opened = *opened.get();

    let desplegable_background = theme.dropdown.desplegable_background;
    let button_background = match *state.get() {
        AutoCompleteProps::Hovering => theme.dropdown.hover_background,
        AutoCompleteProps::Idle => theme.dropdown.background_button,
    };
    let color = theme.dropdown.font_theme.color;

    // Update the provided value if the passed value changes
    use_effect(cx, &cx.props.value, move |value| {
        *selected.write() = value;
        async move {}
    });

    // Close the dropdown if clicked anywhere
    let onglobalclick = move |_: MouseEvent| {
        opened.set(false);
    };

    // let onclick = move |_| {
    //     focus.focus();
    //     opened.set(true)
    // };

    let onkeydown = move |e: KeyboardEvent| {
        match e.key {
            // Close when `Escape` key is pressed
            Key::Escape => {
                opened.set(false);
            }
            // Open the dropdown items when the `Enter` key is pressed
            Key::Enter if is_focused && !is_opened => {
                opened.set(true);
            }
            _ => {}
        }
    };

	let input_text = use_state(cx,  String::new);

	render!(
		rect {
			width: "70",
			height: "350",
			margin: "5",
			overflow: "clip",
			focus_id: focus_id,
			background: button_background,
			color: color,
			corner_radius: "3",
			onglobalclick: onglobalclick,
			onkeydown: onkeydown,
			Input {
				max_lines: "none",
				value: input_text.get().clone(),
				onchange: |e| {
					opened.set(true);
					input_text.set( e )
				}
			}
			rect {
				width : "100%",
				height : "auto",
				//overflow : "clip",
				layer : "-1",
				if *opened.get() {
					&cx.props.children
				}
			}
		}
	)

    // if *opened.get() {
    //     render!(
    //         rect {
    //             width: "70",
    //             height: "50",
    //             margin: "5",
    //             rect {
	// 				offset_y : 40,
    //                 overflow: "clip",
    //                 focus_id: focus_id,
    //                 layer: "-1",
    //                 corner_radius: "3",
    //                 onglobalclick: onglobalclick,
    //                 onkeydown: onkeydown,
    //                 width: "130",
    //                 height: "auto",
    //                 background: desplegable_background,
    //                 shadow: "0 0 20 0 rgb(0, 0, 0, 100)",
    //                 padding: "7",
    //                 &cx.props.children
    //             }
    //         }
    //     )
    // } else {
    //     render!(
    //         rect {
    //             margin: "5",
    //             overflow: "clip",
    //             focus_id: focus_id,
    //             background: button_background,
    //             color: color,
    //             corner_radius: "3",
    //             onclick: onclick,
    //             onkeydown: onkeydown,
    //             width: "70",
    //             height: "auto",
    //             padding: "7",
    //             Input {
	// 				max_lines: "none",
	// 				value: input_text.get().clone(),
	// 				onchange: |e| {
	// 					input_text.set( e )
	// 				}
	// 			}
    //         }
    //     )
    // }
}