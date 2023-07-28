use std::ops::Range;

use freya::prelude::*;

//The `Scope` is required for the parent element to render the item
type BuilderFunction<'a> = dyn Fn(
        (Scope<'a, VirtualScrollViewProps<'a, ()>>,usize)
) -> LazyNodes<'a, 'a>;

#[derive(Props)]
pub struct SquareGridProps<'a> {
	h_gap : f32,
	v_gap : f32,
	item_width : f32,
	item_height : f32,
	item_length : usize,
	builder : Option<Box<BuilderFunction<'a>>>
}

pub fn SquareGrid<'a>(cx:Scope<'a,SquareGridProps<'a>> ) -> Element<'a> {
	let (node_ref, size) = use_node(cx);
	
	const SCROLL_BAR_WIDTH:f32 = 15.;
	let x_item_num = (size.inner.width-SCROLL_BAR_WIDTH) / (cx.props.item_width+cx.props.h_gap);
	let y_item_num = size.inner.height / (cx.props.item_width+cx.props.v_gap);
	let x_item_num = (x_item_num as usize).max(1);
	let y_item_num = (y_item_num as usize).max(1);
	let calc_row_num = cx.props.item_length / x_item_num + 1;

	let (xi, w,h, hgap, vgap, len) = ( 
		x_item_num 
	, (cx.props.item_width) as f64
	, (cx.props.item_height) as f64 
	, cx.props.h_gap
	, cx.props.v_gap
	, (cx.props.item_length)
	);

	// cx.props.

	// std::mem::replace(cx.props.builder.as_mut(), None);
	// let take_fn = std::mem::take( cx.props.builder );

	//let builder:BuilderFunction = cx.props.builder.into();
	let builder = std::mem::take( &mut cx.props.builder ).unwrap();

	// let vs_builder = Box::new( move |(key, index, cx, _)| {
	// 	rsx! {
	// 		rect {
	// 			key: "{key}",
	// 			direction : "horizontal",
	// 			item_builder( index )
	// 			// for i in (index*xi) .. (index*xi+xi).min( len ) {
	// 			// 	rect {
	// 			// 		background : "rgb(128,128,128)",
	// 			// 		margin : "{vgap} 0 0 {hgap}",
	// 			// 		display : "center",
	// 			// 		width : "{w}",
	// 			// 		height : "{h}",
	// 			// 		label { width : "100%", align:"center", "{i}" }
	// 			// 		// (pcx.props.builder) ( (pcx, i) )
	// 			// 	}
	// 			// }
	// 		}
	// 	}
	// });

	let pcx = cx;

	render!(
		rect {
			width : "100%",
			// height : "100%",
			reference : node_ref,
			VirtualScrollView {
				width: "100%",
				height: "100%",
				length: calc_row_num,
				item_size: cx.props.item_height+cx.props.v_gap,
				direction:"vertical",
				builder_values : (),
				
				builder: Box::new( move |(key, index, cx, _)| {
					rsx! {
						rect {
							key: "{key}",
							direction : "horizontal",
							for i in (index*xi) .. (index*xi+xi).min( len ) {
								rect {
									background : "rgb(128,128,128)",
									margin : "{vgap} 0 0 {hgap}",
									display : "center",
									width : "{w}",
									height : "{h}",
									//label { width : "100%", align:"center", "{i}" }
									builder ( (cx,i) )
									// (pcx.props.builder) ( (i,) )
								}
							}
						}
					}
				})
			}
		}
	)
}