

/// [`AutoComplete`] component properties.
#[derive(Props)]

pub struct SquareGridProps {
	item_x_padding : f64,
	item_y_padding : f64,
	item_width : f64,
	item_height : f64,
}


pub fn SquareGrid(cx:Scope<SquareGridProps>) -> Element {


	render!(
		rect {
			width:"100%",
			height:"100%",
			reference:
		}
	)
}