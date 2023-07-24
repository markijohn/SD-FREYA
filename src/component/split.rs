use dioxus::prelude::onclick;
use freya::prelude::{*, pointer::MouseButton};

use winit::window::CursorIcon;

/// Split direction
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SplitDirection {
	#[default]
	Horizontal,
	Vertical
}

/// Identifies the current status of the Split.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SplitStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is dragging the split bar
    Dragging,
}

/// [`Switch`] component properties.
#[derive(Props)]
pub struct SplitProps<'a> {
	pub direction : SplitDirection,

	#[props(optional)]
	pub initial_size : Option<u32>,

	#[props(optional)]
	pub bar_size : Option<u32>,

	pub first_child : Element<'a>,
	pub second_child : Element<'a>
}

/// Controlled `Split` component.
///
/// # Props
/// See [`SplitProps`].
///
/// # Styling
/// Inherits the [`SplitTheme`](theme::SplitTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     render!(
/// 		Split {
/// 			direction : "horizontal",
/// 			rect {
/// 				background : "red"
/// 			},
/// 			rect {
/// 				background : "blue"
/// 			}
/// 		}
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn Split<'a>(cx:Scope<'a,SplitProps<'a>>) -> Element {
	let first_size = use_state(cx, || cx.props.initial_size.unwrap_or(50) );
	let bar_size = use_state(cx, || cx.props.bar_size.unwrap_or(3) );
    let dragging = use_state(cx, || false);
	let status = use_state(cx, SplitStatus::default);

	let (node_ref,size) = use_node(cx);

	let onmousedown = move |e: MouseEvent| {
		if let Some(MouseButton::Left) = e.data.get_trigger_button() {
			status.set( SplitStatus::Dragging );
		}
    };

	let onmouseup = move |e:MouseEvent| {
		if let Some(MouseButton::Left) = e.data.get_trigger_button() {
			status.set( SplitStatus::Idle );
		}
	};
	
	let onmouseover = move |e:MouseEvent| {
		//Can't get `MouseButton` status. It's always 'None'
		if let SplitStatus::Dragging = status.get() {
			match cx.props.direction {
				SplitDirection::Horizontal => first_size.set( e.get_element_coordinates().x as u32 ),
				SplitDirection::Vertical => first_size.set( e.get_element_coordinates().y as u32 ),
			}
		}
	};

	

	match cx.props.direction {
		SplitDirection::Horizontal => {
			render!(
				rect {
					width:"100%",
					height:"100%",
					direction:"horizontal",
					onmouseover:onmouseover,
					onclick:onmouseup,
					rect {
						width: "{first_size}",
						height:"100%",
						overflow : "clip",
						&cx.props.first_child,
					},
					rect {
						background : "rgb(50,50,50)", 
						width:"{bar_size}", 
						height:"100%",
						onmousedown:onmousedown,
						CursorArea {
							rect { width:"100%",height:"100%",}
							icon: CursorIcon::EwResize,
						}
					}
					rect {
						reference : node_ref,
						// width : "auto",
						width : "calc(100% - {first_size} - {bar_size})",
						height : "100%",
						&cx.props.second_child
					}
				}
			)
		},
		SplitDirection::Vertical => {
			render!(
				rect {
					width:"100%",
					height:"100%",
					direction:"vertical",
					onmouseover:onmouseover,
					onclick:onmouseup,
					rect {
						width : "100%",
						height: "{first_size}",
						overflow : "clip",
						&cx.props.first_child
					},
					rect {
						background : "rgb(50,50,50)", 
						width:"100%", 
						height:"{bar_size}",
						onmousedown:onmousedown,
						CursorArea {
							rect { width:"100%",height:"100%" }
							icon: CursorIcon::NsResize,
						}
					},
					rect {
						width : "100%",
						// height : "auto",
						height : "calc(100% - {first_size} - {bar_size})",
						&cx.props.second_child
					}
				}
			)
		}
	}
}