use clap::builder::Str;
use yew::prelude::*;
use web_sys::{HtmlInputElement};
use gloo::console::log;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;


pub struct ComponentUpload{
    content: String,
    file_input_ref: NodeRef,
}

pub enum Msg {
    Submit,
    PromiseResult(String)
}

#[derive(Properties,PartialEq)]
pub struct Props{
    pub on_change:Callback<String>,
}

impl Component for ComponentUpload{
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            content: "".to_owned(),
            file_input_ref: NodeRef::default(),
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
                _ctx.props().on_change.emit(result);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <>
                <div class="input-group mb-3">
                  <input type="file" class="form-control" id="inputGroupFile04" aria-describedby="inputGroupFileAddon04" aria-label="Upload" ref={self.file_input_ref.clone()}/>
                  <button class="btn btn-outline-secondary" type="button" id="inputGroupFileAddon04" onclick={_ctx.link().callback(|_| Msg::Submit)}>{"提交"}</button>
                </div>
            </>
        }
    }
}