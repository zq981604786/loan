use yew::prelude::*;
use gloo::console::log;
use web_sys::Element;
use js_sys::{Array};
use gloo::console::__macro::JsValue;

use crate::cam::components::components_loan::utils::{get_repay_type_nav,get_repay_type_nav_index};
use crate::cam::model::base::LoanType;


pub struct ComponentOneTimeRepayLoan {
    father_ref:NodeRef,
}

#[derive(Properties,PartialEq)]
pub struct Props{
    pub current_nav:String
}

impl Component for ComponentOneTimeRepayLoan{
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            father_ref:NodeRef::default(),
        }
    }
    // "最新质押物信息".to_string(),
    // "利息变动记录".to_string(),
    // "质押信息变动记录".to_string(),
    // "本金变动记录".to_string(),
    // "期限调整记录".to_string(),
    // "管理费记录".to_string(),
    // "利息计提记录".to_string(),
    // "资金变动记录".to_string(),
    // "业务日志".to_string(),
    // "附件".to_string(),
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div ref={self.father_ref.clone()}>
                // <h1 class={"hidden-table"}>{"3"}</h1>
                <h1>{"3"}</h1>
            </div>
        }
    }
    // let div = self.div.cast::<HtmlDivElement>().unwrap();
    // div.style().set_property("color", "red").unwrap();
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // let current_nav = ctx.props().current_nav.clone();
        // let father_ref = self.father_ref.clone().cast::<Element>().unwrap();
        // let children = father_ref.children();
        //
        // let nav_list = get_repay_type_nav(LoanType::OneTimeRepay.to_string());
        //
        // let index = get_repay_type_nav_index(nav_list,current_nav.clone());
        //
        // let class_arr = Array::new();
        // let class_hidden = Array::new();
        // class_hidden.push(&JsValue::from("hidden-table".to_string()));
        // // class_arr.push(&JsValue::from("active".to_string()));
        //
        // // let new_nav_element =  node_ref.cast::<Element>().unwrap();
        //
        //
        // for i in 0..children.length(){
        //     let child = children.item(i).unwrap();
        //     let new_nav_classes = child.class_list();
        //     if i == index{
        //         new_nav_classes.add(&class_arr);
        //     }else{
        //         new_nav_classes.add(&class_hidden);
        //     }
        // }
        // // let child = children.item(index).unwrap();
        //
        //
        //
        // if current_nav == "最新质押物信息"{
        //
        // }
        // if current_nav == "利息变动记录"{
        //
        // }
        todo!()
    }
}