use yew::prelude::*;
use web_sys::{HtmlInputElement};
use gloo::console::log;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

use crate::cam::model::loan_record::LoanRecordVM;
use crate::cam::components::calculate_dcm::ComponentCalculateDcm;
use crate::cam::components::upload::ComponentUpload;
use crate::cam::components::tables::ComponentTable;
use crate::cam::components::nav::ComponentNav;

#[function_component]
pub fn Home() -> Html {
    html!{
        <>
            <UploadFile/>
        </>
    }
}

enum Msg {
    Submit,
    PromiseResult(String),
    ChangeNav(String),
}

struct UploadFile {
    content: String,
    file_input_ref: NodeRef,
    loan_record_vm: Option<LoanRecordVM>,
    current_nav:String,
}
impl Component for UploadFile{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            content: "".to_owned(),
            file_input_ref: NodeRef::default(),
            loan_record_vm: None,
            current_nav:"".to_string(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let loan = &self.loan_record_vm;
        html!{
            <div>
                <ComponentUpload on_change={_ctx.link().callback(Msg::PromiseResult)}/>
                <ComponentNav records={vec!["Active".to_string(),"Link1".to_string(),"Link2".to_string()]}
                on_change={_ctx.link().callback(Msg::ChangeNav)}/>
                // <p>{ &self.content }</p>
                {
                    match loan{
                    Some(_value) => html! {
                        <>
                            <p>{_value.ltv_rate}</p>
                            <p>{_value.stablecoin_decoupling_ltv_rate}</p>
                            <ComponentTable data={_value.interest_count_records.clone()} title={vec!["1".to_string()]}/>
                        </>
                    },
                    None => html! { <p>{"No Value"}</p> },
                     }
                }
                {
                    { if self.current_nav == "Link1" {
                        html! { <>
                            <p>{"zzzzz"}</p>
                        </> }
                    } else {
                        html! { <>
                            <p>{"Other content"}</p>
                        </> }
                    } }
                }
                <ComponentCalculateDcm/>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render{
        }
    }

     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit=>{
                let file_input =  &self.file_input_ref.clone().cast::<HtmlInputElement>().unwrap();
                if let Some(file) = file_input.files(){
                    let first_file = file.get(0);
                    let content = first_file.map(|f| f.slice().unwrap());
                    if let Some(content) = content{
                        let promise:Promise = content.text().into();
                        let future = JsFuture::from(promise);
                        let callback = _ctx.link().callback(|result| Msg::PromiseResult(result));
                        wasm_bindgen_futures::spawn_local(async move {
                            let result = future.await;
                            if let Ok(js_value) = result {
                                let text: String = js_value.as_string().unwrap();
                                callback.emit(text);
                            }
                        });
                    }
                }
                true
            },
            Msg::PromiseResult(result)=>{
                let loan:crate::cam::model::loan_record::LoanRecordVM = serde_json::from_str(result.as_str()).unwrap();
                let json = serde_json::to_string(&loan).unwrap();
                self.content = json;
                self.loan_record_vm = Some(loan);
                log!("loan");
                true
            },
            Msg::ChangeNav(nav)=>{
                self.current_nav = nav.clone();
                log!("nav",nav);
                true
            }
        }
    }

    // 来自父组件
    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        println!("change");
        return false
    }
}

fn get_table_heard() {

}