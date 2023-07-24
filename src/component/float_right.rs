use freya::prelude::*;

#[derive(Props)]
pub struct FloatRightProps<'a> {
	children : Element<'a>
}

pub fn FloatRight<'a>(cx:Scope<'a, FloatRightProps<'a>>) -> Element {
	let (node_ref_parent, parent_size) = use_node(cx);
	let (item_ref, item_size) = use_node(cx);
	let gap = parent_size.area.width() - item_size.area.width();

	
	//println!("Called {} {:?}", gap, cx.props.children.as_ref().unwrap().dynamic_attrs.iter().find( |e| e.name == "width"));
	// println!("{} {} {}", gap, parent_size.area.width() , item_size.area.width());

	render!( rect {
		width : "100%",
		reference : node_ref_parent,
		direction : "horizontal",
		rect {
			width : "{gap}",
		}
		rect {
			// margin : "0 0 0 {gap}",
			reference : item_ref,
			&cx.props.children
		}
	} )
}