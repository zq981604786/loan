use yew::prelude::*;
use web_sys::{HtmlInputElement};
use gloo::console::log;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

use crate::cam::model::loan_record::LoanRecordVM;

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
    PromiseResult(String)
}

struct UploadFile {
    content: String,
    file_input_ref: NodeRef,
    loan_record_vm: Option<LoanRecordVM>,
}
impl Component for UploadFile{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            content: "".to_owned(),
            file_input_ref: NodeRef::default(),
            loan_record_vm: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div>
                <input ref={self.file_input_ref.clone()} type="file" />
                <button onclick={_ctx.link().callback(|_| Msg::Submit)}>{ "Upload" }</button>
                <p>{ &self.content }</p>
                {
                    match self.loan_record_vm{
                    Some(_value) => html! { <p>{_value.ltv_rate}</p> },
                    None => html! { <p>{"No Value"}</p> },
                }
            }
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render{
            log!("first rendered")
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
                // self.content = result.clone();
                let loan:crate::cam::model::loan_record::LoanRecordVM = serde_json::from_str(result.as_str()).unwrap();
                let json = serde_json::to_string(&loan).unwrap();
                self.content = json;
                self.loan_record_vm = Some(loan);
                log!("loan");
                true
            },
        }
    }

    // 来自父组件
    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        println!("change");
        return false
    }
}
