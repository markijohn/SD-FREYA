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
	let (node_ref, size) = use_node(cx);

    let onclick = move |_: MouseEvent| {
        open.set(!*open.get());
    };

	let (arrow,height) = if *open.get() {
		(ARROW_DOWN, "auto")
	} else {
		(ARROW_RIGHT, "0")
	};
    let arrow_width = size.inner.width;

    render!(
        rect {
            overflow: "clip",
            width: "100%",
            height: "auto",
            onclick: onclick,
			rect { 
				direction:"horizontal", 
				rect { reference:node_ref, padding: "0 {ARROW_RIGHT_PAD} 0 0", label { "{arrow} " } },
				label { "{&cx.props.summary}" }
			}
            rect {
                overflow: "clip",
                width: "100%",
                height: "{height}",
                direction : "horizontal",
				
                //margin : "0 0 0 {size.area.width()}", //no line
                rect { width:"1", height:"15", margin: "0 {arrow_width/2. + ARROW_RIGHT_PAD-1.} 0 {arrow_width/2.}", background : "rgb(80,80,80)" }

                &cx.props.children
            }
        }
    )
}