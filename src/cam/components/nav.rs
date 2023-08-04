use std::rc::Rc;
use gloo::console::__macro::JsValue;
use yew::prelude::*;
use gloo::console::log;
use web_sys::{HtmlBaseElement};
use js_sys::{Array};

pub struct ComponentNav{
    refs:Vec<NodeRef>,
    current_page:String,
    ul_ref:NodeRef,
}

#[derive(Properties,PartialEq)]
pub struct Props{
    pub records:Vec<String>,
}

pub enum Msg{
    OnClick(String,NodeRef)
}

impl Component for ComponentNav{
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            refs:vec![],
            current_page:"".to_string(),
            ul_ref:NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnClick(event,node_ref)=>{
                let nav_ref =  node_ref.cast::<HtmlBaseElement>().unwrap();
                // file_input.set_attribute("aria-current", "page");
                let first_nav_classes = nav_ref.class_list();
                let class_arr = Array::new();
                class_arr.push(&JsValue::from("active".to_string()));
                first_nav_classes.add(&class_arr).unwrap();
                log!("event",event);
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render{
            let nav_ref =  self.ul_ref.clone().cast::<HtmlBaseElement>().unwrap();
            // let first_nav = nav_ref.first_child().unwrap();
            let first_nav = nav_ref.first_element_child().unwrap();
            let first_nav_classes = first_nav.class_list();
            let class_arr = Array::new();
            class_arr.push(&JsValue::from("active".to_string()));
            first_nav_classes.add(&class_arr).unwrap();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!{"nav first render111"};
        html!{
            <>
                <nav class="nav nav-pills nav-fill" ref={self.ul_ref.clone()}>
                {for ctx.props().records.clone().iter().map(|item| {
                    let item_clone = Rc::new(item.clone());
                    let a_ref = NodeRef::default();
                   html!{
                        <a class="nav-link" href="#"  onclick={ctx.link().callback(move |_|
                            Msg::OnClick(item_clone.clone().to_string(),a_ref.clone())
                        )} ref={a_ref.clone()}>{item.clone().to_string()}</a>
                    }
                })}
                </nav>

                <h1>{"test1"}</h1>
                <h1>{"test2"}</h1>
                <h1>{"test3"}</h1>
            </>
        }
    }
}