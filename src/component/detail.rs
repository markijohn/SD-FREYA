use std::borrow::Cow;

/// Copy of `freya::component::Accordion`
/// Just diffrent style

use freya::prelude::*;

/// [`DetailProps`] component properties.
#[derive(Props)]
pub struct DetailProps<'a> {
	#[props(optional)]
	icon_opened : Option<Cow<'static, str>>,

	#[props(optional)]
	icon_closed : Option<Cow<'static, str>>,

    #[props(default=0)]
    depth : usize,

    /// Inner children for the Detail.
    children: Element<'a>,
    /// Summary element.
    summary: Cow<'static, str>
}

pub fn Detail<'a>(cx:Scope<'a,DetailProps<'a>>) -> Element<'a> {
    const ARROW_RIGHT_PAD:f32 = 3.;
	static ARROW_RIGHT: &str = "⯈";
	static ARROW_DOWN:&str = "⯆";
    let open = use_state(cx, || false);
    let (node_ref_root, size_root) = use_node(cx);
	let (node_ref, size) = use_node(cx);

    let onclick = move |e: MouseEvent| {
        open.set(!*open.get());
    };

	let (arrow,height) = if *open.get() {
		(ARROW_DOWN, "auto")
	} else {
		(ARROW_RIGHT, "0")
	};
    let arrow_width = size.inner.width;
    println!("...");

    render!(
        rect {
            overflow: "clip",
            width: "100%",
            height: "auto",
            reference:node_ref_root,
            margin : 0,
            padding : 0,
			rect { 
				direction:"horizontal", 
                onclick: onclick,
				rect { reference:node_ref, padding: "0 {ARROW_RIGHT_PAD} 0 0", label { "{arrow} " } },
				label { "{&cx.props.summary}" }
			}
            rect {
                overflow: "clip",
                width: "100%",
                height: "{height}",
                direction : "horizontal",

                //line
                rect { width:"1", height:"*", margin: "0 {arrow_width/2. + ARROW_RIGHT_PAD-1.} 0 {arrow_width/2.}", background : "rgb(80,80,80)" }
				
                //no line
                //margin : "0 0 0 {size.area.width()}", //no line

                &cx.props.children
            }
        }
    )
}