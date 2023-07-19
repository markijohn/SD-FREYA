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
	let first_size = use_state(cx, || (50f64) );
	let bar_size = use_state(cx, || 3);
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
				SplitDirection::Horizontal => first_size.set( e.get_element_coordinates().x ),
				SplitDirection::Vertical => first_size.set( e.get_element_coordinates().y ),
			}
		}
		
		// if *status.get() == SplitStatus::Dragging {
		// 	e.data.trigger_button

			// let size = size.read();
			// let coord = e.get_screen_coordinates();
			// pos.set(
			// 	(
			// 		coord.x - size.area.min_x() as f64,
			// 		coord.y - size.area.min_y() as f64,
			// 	)
			// 		.into(),
			// );
			// dragging.set(true);
			// *drags.unwrap().write() = Some(cx.props.data.clone());
		// }
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
						&cx.props.first_child,
					},
					rect {
						background : "yellow", 
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
						width : "auto",
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
						&cx.props.first_child
					},
					rect {
						display : "center",
						background : "yellow", 
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
						height : "auto",
						&cx.props.second_child
					}
				}
			)
		}
	}
}