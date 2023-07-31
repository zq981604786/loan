// use yew::prelude::*;

// pub enum Msg {
//     AddOne,
// }

// pub struct App {
//     value: i64,
// }

// impl Component for App {
//     type Message = Msg;
//     type Properties = ();

//     fn create(ctx: &Context<Self>) -> Self {
//         Self { value: 0 }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::AddOne => {
//                 self.value += 1;
//                 true
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {
//             <div>
//                 <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "+1" }</button>
//                 <p>{ self.value }</p>
//             </div>
//         }
//     }
// }

// #[function_component]
// pub fn App() -> Html {
//     let counter = use_state(|| 0);
//     let onclick = {
//         let counter = counter.clone();
//         move |_| {
//             let value = *counter + 1;
//             counter.set(value);
//         }
//     };

//     html! {
//         <div>
//             <button {onclick}>{ "+1" }</button>
//             <p>{ *counter }</p>
//         </div>
//     }
// }

// #[function_component]
// fn Header() -> Html {
//   html! {
//     <header>
//       <h1>{ "My Website" }</h1>
//       // 其他 header 内容
//     </header>
//   }
// }

// #[function_component]
// fn Home() -> Html {
//   html! {
//     <Header />

//     <main>
//       <p>{ "Home page" }</p>
//     </main>
//   }
// }

// #[function_component]
// fn About() -> Html {
//   html! {
//     <Header />

//     <main>
//       <p>{ "About page" }</p>
//     </main>
//   }
// }

// html! {
//   <Home />
//   <About />
// }


// let counter = use_state(|| 0);
// let onclick = {
//     let counter = counter.clone();
//     move |_| {
//         let value = *counter + 1;
//         counter.set(value);
//     }
// };

// html! {
//     <div>
//         <button {onclick}>{ "+1" }</button>
//         <p>{ *counter }</p>
//     </div>
// }