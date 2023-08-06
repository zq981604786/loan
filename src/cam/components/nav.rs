use std::rc::Rc;
use gloo::console::__macro::JsValue;
use yew::prelude::*;
use gloo::console::log;
use web_sys::{Element, HtmlBaseElement};
use js_sys::{Array};

pub struct ComponentNav{
    ul_ref:NodeRef,
    current_ref:Option<Element>,
    current_page:String,
}

#[derive(Properties,PartialEq)]
pub struct Props{
    pub records:Vec<String>,

    pub on_change:Callback<String>,
}

pub enum Msg{
    OnClick(String,NodeRef)
}

impl Component for ComponentNav{
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            current_page:"".to_string(),
            ul_ref:NodeRef::default(),
            current_ref:Option::None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnClick(event,node_ref)=>{
                let class_arr = Array::new();
                class_arr.push(&JsValue::from("active".to_string()));


                let old_nav_element = Some(self.current_ref.clone()).unwrap().unwrap();
                let old_nav_classes = old_nav_element.class_list();
                old_nav_classes.remove(&class_arr);

                let new_nav_element =  node_ref.cast::<Element>().unwrap();
                let new_nav_classes = new_nav_element.class_list();
                new_nav_classes.add(&class_arr);
                self.current_ref = Some(new_nav_element);
                // file_input.set_attribute("aria-current", "page");
                ctx.props().on_change.emit(event.clone());

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render{
            let nav_ref =  self.ul_ref.clone().cast::<HtmlBaseElement>().unwrap();
            // let first_nav = nav_ref.first_child().unwrap();
            let first_nav:Element = nav_ref.first_element_child().unwrap();
            let first_nav_classes = first_nav.clone().class_list();
            let class_arr = Array::new();
            class_arr.push(&JsValue::from("active".to_string()));
            first_nav_classes.add(&class_arr).unwrap();

            self.current_ref = Some(first_nav);
        }
    }
}