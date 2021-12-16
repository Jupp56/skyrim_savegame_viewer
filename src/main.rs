use std::collections::HashMap;

use controller::FetchState;
use controller::form_id::fetch_form_ids;
use yew::prelude::*;

use gloo_console as console;
use gloo_file::callbacks::FileReader;
use gloo_file::File;
use web_sys::{Event, HtmlInputElement};
use yew::{html, Component, Html};


mod components;

use components::form_id_resolver::FormIdResolver;

mod controller;

use crate::components::save_game::{
    render_change_forms, render_form_ids, render_global_data_table, render_header,
    render_light_plugins, render_meta_data, render_plugins, render_screenshot,
    render_visited_world_space,
};

extern crate skyrim_savegame;

#[derive(Copy, Clone, Debug)]
pub enum SaveFileParts {
    Plugins,
    LightPlugins,
    GlobalData1,
    GlobalData2,
    GlobalData3,
    ChangeForms,
    FormIds,
    VisitedWorldSpace,
    Unknown3,
}

pub enum Msg {
    FileLoaded(String, Vec<u8>),
    InputFileContentChanged(Vec<File>),
    Response(FormIdResolver),
    SelectedPart(SaveFileParts),
    FetchStateChanged(FetchState<String>),
    GetFormIds,
}

pub struct Upload {
    readers: HashMap<String, FileReader>,
    files: Vec<String>,
    save_game: Option<skyrim_savegame::SaveFile>,
    form_id_list: Option<FormIdResolver>,
    selected_part: SaveFileParts,
}

impl Component for Upload {
    type Message = Msg;
    type Properties = ();

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::GetFormIds);
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: vec![],
            save_game: None,
            form_id_list: None,
            selected_part: SaveFileParts::Plugins,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetFormIds => {
                ctx.link().send_future(async {
                    match fetch_form_ids().await {
                        Ok(md) => Msg::FetchStateChanged(FetchState::Success(md)),
                        Err(err) => Msg::FetchStateChanged(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::FetchStateChanged(FetchState::Fetching));
                false
            }
            Msg::FetchStateChanged(result) => {
                match result {
                    FetchState::NotFetching => {}
                    FetchState::Fetching => {}
                    FetchState::Success(x) => {
                        self.form_id_list = Some(serde_json::from_str(&x).unwrap());
                    }
                    FetchState::Failed(_) => todo!(),
                }
                true
            }
            Msg::FileLoaded(file_name, data) => {
                let savegame = skyrim_savegame::parse_save_file(data);
                self.files.push(format!("{:?}", file_name));
                self.readers.remove(&file_name);
                self.save_game = Some(savegame);
                true
            }
            Msg::InputFileContentChanged(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let link = ctx.link().clone();

                    let task = {
                        let file_name = file_name.clone();
                        gloo_file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::FileLoaded(
                                file_name.clone(),
                                res.expect("failed to read file!"),
                            ));
                        })
                    };

                    self.readers.insert(file_name.clone(), task);
                }
                true
            }
            Msg::SelectedPart(part) => {
                self.selected_part = part;
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <h1> { "Savegame Loader" }</h1>
                <div>
                    <p>{ "Choose a SE save file to upload to see content" }</p>
                    <input type="file" multiple=false accept=".ess" onchange={ctx.link().callback(move |e: Event| {

                            let mut result = Vec::new();
                            let input: HtmlInputElement = e.target_unchecked_into();

                            if let Some(files) = input.files() {
                                let files = js_sys::try_iter(&files)
                                    .unwrap()
                                    .unwrap()
                                    .map(|v| web_sys::File::from(v.unwrap()))
                                    .map(File::from);
                                result.extend(files);
                            }
                            Msg::InputFileContentChanged(result)
                        })}
                    />
                </div>

                    { match &self.save_game {
                        Some(game) => self.view_game(game.clone(), &self.form_id_list.as_ref().unwrap(), &self.selected_part, ctx),
                        None => {Self::nothing()}
                    }
                    }
            </div>
        }
    }
}

impl Upload {
    fn view_game(
        &self,
        save_game: skyrim_savegame::SaveFile,
        form_id_list: &FormIdResolver,
        selected_part: &SaveFileParts,
        ctx: &Context<Self>,
    ) -> Html {
        let file_section_onclick = move |selected_part: SaveFileParts| {
            ctx.link().callback(move |_: MouseEvent| Msg::SelectedPart(selected_part))
        };

        let menu = html! {
            <ul class="nav flex-column">
                <li class="nav-item">
                    <a class="nav-link active" onclick={file_section_onclick(SaveFileParts::Plugins)}>{"Plugins ("} { save_game.plugin_info.len() } { ")" }</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" onclick={file_section_onclick(SaveFileParts::LightPlugins)}>{"Light Plugins ("} { save_game.light_plugin_info.len() } { ")" }</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" onclick={file_section_onclick(SaveFileParts::GlobalData1)}>{"Global Data 1"}</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" onclick={file_section_onclick(SaveFileParts::GlobalData2)}>{"Global Data 2"}</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" onclick={file_section_onclick(SaveFileParts::GlobalData3)}>{"Global Data 3"}</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" onclick={file_section_onclick(SaveFileParts::ChangeForms)}>{"Change Forms ("} { save_game.change_forms.len() } { ")" }</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" onclick={file_section_onclick(SaveFileParts::FormIds)}>{ "Form IDs (" } { save_game.form_id_array.len() } { ")" }</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" onclick={file_section_onclick(SaveFileParts::VisitedWorldSpace)}>{ "Visited Worldspace (" } { save_game.visited_worldspace_array.len() } { ")" }</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" onclick={file_section_onclick(SaveFileParts::Unknown3)}>{ "Unknown 3 (" } { save_game.unknown_3_table.len() } { ")" }</a>
                </li>
            </ul>
        };

        let content = match selected_part {
            SaveFileParts::Plugins => render_plugins(&save_game.plugin_info),
            SaveFileParts::LightPlugins => render_light_plugins(&save_game.light_plugin_info),
            SaveFileParts::GlobalData1 => render_global_data_table(
                &save_game.global_data_table_1,
                &save_game.form_id_array,
                form_id_list,
            ),
            SaveFileParts::GlobalData2 => render_global_data_table(
                &save_game.global_data_table_2,
                &save_game.form_id_array,
                form_id_list,
            ),
            SaveFileParts::GlobalData3 => render_global_data_table(
                &save_game.global_data_table_3,
                &save_game.form_id_array,
                form_id_list,
            ),
            SaveFileParts::ChangeForms => render_change_forms(
                &save_game.change_forms,
                &save_game.form_id_array,
                form_id_list,
            ),
            SaveFileParts::FormIds => render_form_ids(&save_game.form_id_array),
            SaveFileParts::VisitedWorldSpace => {
                render_visited_world_space(&save_game.visited_worldspace_array, form_id_list)
            }
            SaveFileParts::Unknown3 => {
                html! {"Unknown 3"}
            }
        };

        html! {
            <>
            <div class="row">
            <div class="col">
            { render_meta_data(&save_game) }
            </div>
            <div class="col">
            { render_header(&save_game.header) }
            </div>
            <div class="col">
            { render_screenshot(save_game.screenshot_data.clone()) }
            </div>
            </div>
            <div class="row">
            <div class="col-2">
            { menu }
            </div>
            <div class="col-10">
            { content }
            </div>
            </div>
            </>
        }

        //use crate::components::save_game::*;
        //render_save_game(save_game, form_id_list)
    }

    fn nothing() -> Html {
        html! {
        <>
        { "nothing selected. Yet..." }
        </>
        }
    }
}



fn main() {
    yew::start_app::<Upload>();
}
