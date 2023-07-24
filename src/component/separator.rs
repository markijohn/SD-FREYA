use freya::prelude::*;

pub fn HR(cx:Scope) -> Element {
	render!( rect {
		width : "calc( 100% - 10 )",
		height : "1",
		background : "rgb(200,200,200)",
		margin : "8 0 8 5"
	} )
}

pub fn VR(cx:Scope) -> Element {
	render!( rect {
		height : "100% - 6",
		background : "rgb(200,200,200)",
		margin : "3 8 0 8"
	} )
}