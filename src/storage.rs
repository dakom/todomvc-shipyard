use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::{
    actions::append_todo,
    systems::workloads::run_update
};
use shipyard::*;
use web_sys::Storage;
use wasm_bindgen_futures::spawn_local;

const STORAGE_NAME:&'static str = "todomvc-shipyard";

#[derive(Serialize)]
struct SavedData <'a>{
    pub items: Vec<(&'a str, bool)>
}
#[derive(Deserialize)]
pub struct LoadedData {
    pub items: Vec<(String, bool)> 
}

//We could make this easier by just requiring that SavedData is also String
//But saving happens much more frequently than loading, so we should optimize a bit
impl <'a, I> From<I> for SavedData<'a> where
 I: IntoIterator<Item=(&'a str, bool)>
{
    fn from(items:I) -> SavedData<'a> {
        let items:Vec<(&str, bool)> = items.into_iter().collect();

        SavedData {
            items
        }
    }
}

//These aren't actually used but it serves as a sanity check
//Since they _should_ be able to be converted into eachother
impl From<SavedData<'_>> for LoadedData {
    fn from(data:SavedData) -> LoadedData {
        LoadedData {
            items: data
                .items
                .iter()
                .map(|(s, b)| {
                    (s.to_string(), *b)
                })
                .collect()
        }
    }
}
impl <'a> From<&'a LoadedData> for SavedData<'a> {
    fn from(data:&'a LoadedData) -> SavedData<'a> {
        SavedData {
            items: data
                .items
                .iter()
                .map(|(s, b)| {
                    (s.as_ref(), *b)
                })
                .collect()
        }
    }
}



pub fn load_data() -> Option<LoadedData> {
    get_local_storage()
        .get(STORAGE_NAME)
        .unwrap_throw()
        .map(|json_str| {
            serde_json::from_str(&json_str).unwrap_throw()
        })
}

pub fn save_data<'a, I>(items:I) where
 I: IntoIterator<Item=(&'a str, bool)>
{
    let local_storage = get_local_storage();
    let data:SavedData = items.into();
    let json_str = serde_json::to_string(&data).unwrap_throw();

    local_storage.set(STORAGE_NAME, &json_str).unwrap_throw();
}

pub fn get_local_storage() -> Storage { 
    web_sys::window()
        .unwrap_throw()
        .local_storage()
        .unwrap_throw()
        .unwrap_throw()
}
