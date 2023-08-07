use rust_decimal::Decimal;
use serde_json::Value;
use std::{any::Any, collections::HashMap, fmt};

pub type Pnl = HashMap<String, f64>;

pub type Json = HashMap<String, Value>;

pub type DcmPnl = HashMap<String, Decimal>;

pub type StrPnl = HashMap<String, String>;
// pub struct StrPnl(HashMap<String, String>);
//
// impl fmt::Display for StrPnl{
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for (key, val) in self {
//             write!(f, "{}: {}\n", key, val)?;
//         }
//         Ok(())
//     }
// }
//
// impl Clone for StrPnl{
//     fn clone(&self) -> Self {
//         StrPnl(self.0.clone())
//     }
// }
// 另一种方法是在 StrPnl 定义时直接带上 Clone trait bound,这样编译器会自动为我们实现 Clone:
// pub struct StrPnl(HashMap<String, String>) where HashMap<String, String>: Clone;