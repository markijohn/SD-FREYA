use std::ops::Range;

use freya::prelude::*;

//type BuilderFunction<'a> = dyn Fn( (usize, Scope<'a, VirtualScrollViewProps<'a,()>>) ) -> LazyNodes<'a, 'a>;

type BuilderFunction<'a> = fn(usize, Scope<'a,VirtualScrollViewProps<'a,()>>) -> Element<'a>;


#[derive(Props)]
pub struct SquareGridProps<'a> {
	h_gap : f32,
	v_gap : f32,
	item_width : f32,
	item_height : f32,
	item_length : usize,
	builder : BuilderFunction<'static>,
	children : Element<'a>
}

pub fn SquareGrid<'a>(cx:Scope<'a,SquareGridProps<'a>> ) -> Element<'a> {
	let (node_ref, size) = use_node(cx);
	let (width, height) = (size.inner.width, size.inner.height);
	
	const SCROLL_BAR_WIDTH:f32 = 15.;
	let x_item_num = (width-SCROLL_BAR_WIDTH) / (cx.props.item_width+cx.props.h_gap);
	let y_item_num = height / (cx.props.item_width+cx.props.v_gap);
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
				
				builder: Box::new( move |(key, index, pcx, _)| {
					println!("{}",key);
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
									// (cx.props.builder) ( i, pcx ) //lifetime error
									crate::make_item( i , pcx)
								}
							}
						}
					}
				})
			}
		}
	)
}