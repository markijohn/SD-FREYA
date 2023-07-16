
//Implement like `FancyTree`
pub struct TreeProps {
	//pub active_node

	//pub selected_nodes

	//pub can_multiple_selection : bool,

	//pub can_select_same_level : bool,

	//some item selected
	//pub onselect : EventHandler,

	//some iten name changed
	//pub onrename : EventHandler,

	//childrens : Vec<Element>
}

/// Controlled `Tree` component.
///
/// # Props
/// See [`TreeProps`].
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
pub fn Tree(cs:Scope) -> Element {
	todo!()
}