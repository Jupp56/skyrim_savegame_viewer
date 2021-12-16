use yew::prelude::*;
use skyrim_savegame::global_data::{GlobalDataType, TES};
use skyrim_savegame::{FormIdType, TESUnknown0, ChangeForm};


extern crate image;

use crate::components::form_id_resolver::render_hex;

use self::image::{RgbaImage, DynamicImage};

use super::collapsible_table::{render_collapsible_table, RenderFunctionType};
use super::form_id_resolver::{FormIdResolver, FormIdEntry};


extern crate base64;

pub fn render_save_game(save_game: skyrim_savegame::SaveFile, form_id_resolver: &FormIdResolver) -> Html {
    let form_id_array = &save_game.form_id_array;

    html! {
        <>
        <h2>
        {"Header"}
        </h2>
        { render_header(&save_game.header) }

        <h2>
        {"Global Data Table 1"}
        </h2>
        {
            save_game.global_data_table_1.iter().map(|x| render_global_data(x, form_id_array, form_id_resolver)).collect::<Html>()
        }
        </>
    }
}

pub fn render_meta_data(save_game: &skyrim_savegame::SaveFile) -> Html {
    html! {
        <>
    <h2>
        {"Metadata"}
        </h2>
        <table class="table table-striped">
        <thead>
        <tr>
        <th> {"Field"} </th>
        <th> {"Value"} </th>
        </tr>
        </thead>
        <tbody>
        <tr>
        <td>
        { "Magic" }
        </td>
        <td>
        { &save_game.magic }
        </td>
        </tr>
        <tr>
        <td>
        { "FormVersion" }
        </td>
        <td>
        { &save_game.form_version }
        </td>
        </tr>
        <tr>
        <td>
        { "Plugins Installed" }
        </td>
        <td>
        { &save_game.plugin_info.len() }
        </td>
        </tr>
        <tr>
        <td>
        { "Light Plugins Installed" }
        </td>
        <td>
        { &save_game.light_plugin_info.len() }
        </td>
        </tr>
        <tr>
        <td>
        { "Body length (compressed)" }
        </td>
        <td>
        { &save_game.body_compressed_len } { " bytes" }
    </td>
        </tr>
        <tr>
        <td>
        { "Body length (uncompressed)" }
        </td>
        <td>
        { &save_game.body_uncompressed_len }  { " bytes" }
    </td>
        </tr>
        </tbody>
        </table>

    </>}
}

pub fn render_screenshot(i: skyrim_savegame::ScreenshotData) -> Html {
    let img = DynamicImage::ImageRgba8(RgbaImage::from_raw(i.width, i.height, i.data).expect("Screenshot data corrupted"));

    let mut png_bytes: Vec<u8> = Vec::new();

    img.write_to(&mut png_bytes, self::image::ImageOutputFormat::Png).unwrap();

    let encoded_image = self::base64::encode(png_bytes);

    let base64_string = format!("data:image/png;base64,{}", encoded_image);

    html! {
        <img src={base64_string} alt="savegame screenshot" class="mt-2 img-fluid"/>
    }
}

pub fn render_header(h: &skyrim_savegame::header::Header) -> Html {
    html! {
        <>
        <h3>
        { "Header" }
        </h3>
        <table class="table table-striped">
            <thead>
        <tr>
        <th> {"Field"} </th>
        <th> {"Value"} </th>
        </tr>
        </thead>
            <tbody>
                <tr>
                    <td>
                        { "Version" }
                    </td>
                    <td>
                        { h.version }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Save Number" }
                    </td>
                    <td>
                        { h.save_number }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Player Name" }
                    </td>
                    <td>
                        { &h.player_name }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Player Level" }
                    </td>
                    <td>
                        { h.player_level }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Player Location" }
                    </td>
                    <td>
                        { &h.player_location }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Game Date" }
                    </td>
                    <td>
                        { &h.game_date }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Race Editor Id" }
                    </td>
                    <td>
                        { &h.player_race_editor_id }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Player Sex" }
                    </td>
                    <td>
                        { h.player_sex }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Player current XP" }
                    </td>
                    <td>
                        { h.player_cur_exp }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Player Level Up XP" }
                    </td>
                    <td>
                        { h.player_lvl_up_exp }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Filetime" }
                    </td>
                    <td>
                        { format!("{:?}", h.filetime) }
                    </td>
                </tr>
            </tbody>
        </table>
        </>
    }
}

pub fn render_plugins(plugins: &Vec<String>) -> Html {
    html! {
        <>
        <h2>
        {"Plugins"}
        </h2>
        <table class="table table-striped table-sm" style="overflow-y: scroll; height: 100px;">
        <tbody>
        { plugins.iter().map(render_plugin).collect::<Html>() }
        </tbody>
        </table>
        </>
    }
}

pub fn render_light_plugins(light_plugins: &Vec<String>) -> Html {
    html! {
        <>
        <h2>
        {"Light Plugins"}
        </h2>
        <table class="table table-striped">
        <tbody>
        { light_plugins.iter().map(render_plugin).collect::<Html>() }
        </tbody>
        </table>
        </>
    }
}

pub fn render_plugin(plugin: &String) -> Html {
    html! {
        <tr>
        <td>
            { plugin }
        </td>
        </tr>
    }
}

pub fn render_global_data_table(d: &Vec<GlobalDataType>, form_id_array: &Vec<u32>, form_id_map: &FormIdResolver) -> Html {
    html! {
        { d.iter().map(|x| { render_global_data(x, form_id_array, form_id_map)}).collect::<Html>() }
    }
}

fn render_global_data(d: &GlobalDataType, form_id_array: &Vec<u32>, form_id_map: &FormIdResolver) -> Html {
    match d {
        GlobalDataType::TES(tes) => render_tes(tes, form_id_array, form_id_map),
        GlobalDataType::Weather(weather) => render_weather(weather, form_id_array, form_id_map),
        GlobalDataType::MiscStats(misc_stats) => render_misc_stats(misc_stats),
        _ => html! {<></>}
    }
}

fn render_weather(w: &skyrim_savegame::Weather, form_id_array: &Vec<u32>, form_id_map: &FormIdResolver) -> Html {
    html! {
        <>
        <div class="row">
            <div class="col">
                <h3>
                { "Weather Data" }
                </h3>
            </div>
            <div class="col text-right">
                <button class="btn btn-primary" type="button" data-toggle="collapse" data-target="#weather" aria-expanded="false" aria-controls="weather">{ "Expand" }</button>
            </div>
        </div>
        <div id="weather" class="collapse">
        <table class="table table-striped">
            <tbody>
                <tr>
                    <td>
                        { "Climate" }
                    </td>
                    <td>
                        { render_form_id_type(&w.climate, form_id_array, form_id_map) }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Weather" }
                    </td>
                    <td>
                        { render_form_id_type(&w.weather, form_id_array, form_id_map) }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Previous Weather" }
                    </td>
                    <td>
                        { render_form_id_type(&w.prev_weather, form_id_array, form_id_map) }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Unknown 1" }
                    </td>
                    <td>
                        { render_form_id_type(&w.unk_weather_1, form_id_array, form_id_map) }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Unknown 2" }
                    </td>
                    <td>
                        { render_form_id_type(&w.unk_weather_2, form_id_array, form_id_map) }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Regn Weather" }
                    </td>
                    <td>
                        { render_form_id_type(&w.regn_weather, form_id_array, form_id_map) }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Current Time" }
                    </td>
                    <td>
                        { w.cur_time }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "Beginning Time" }
                    </td>
                    <td>
                        { w.beg_time }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "u1" }
                    </td>
                    <td>
                        { w.u1 }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "u2" }
                    </td>
                    <td>
                        { w.u2 }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "u3" }
                    </td>
                    <td>
                        { w.u3 }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "u4" }
                    </td>
                    <td>
                        { w.u4 }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "u5" }
                    </td>
                    <td>
                        { w.u5 }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "u6" }
                    </td>
                    <td>
                        { w.u6 }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "u7" }
                    </td>
                    <td>
                        { w.u7 }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "u8" }
                    </td>
                    <td>
                        { w.u8 }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "flags" }
                    </td>
                    <td>
                        { w.flags }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "u9" }
                    </td>
                    <td>
                        {
                            match w.u9.clone() {
                                Some(x) => x,
                                None => {
                                    String::from("Value not set (flag dependent)")
                                }
                            }
                        }
                    </td>
                </tr>
                <tr>
                    <td>
                        { "u10" }
                    </td>
                    <td>
                        { match w.u10.clone() {
                            Some(x) => x,
                            None => {
                                    String::from("Value not set (flag dependent)")
                            }
                        }}
                    </td>
                </tr>
            </tbody>
        </table>
        </div>
        </>
    }
}

fn render_misc_stats(misc_stats: &Vec<skyrim_savegame::MiscStats>) -> Html {
    render_collapsible_table(
        "Misc Stats".to_string(),
        "miscStatTable".to_string(),
        vec!("Name".to_string(),
             "Category".to_string(),
             "Value".to_string()),
        misc_stats,
        RenderFunctionType::SingleArgument(render_misc_stat)
    )
}

fn render_misc_stat(misc_stat: &skyrim_savegame::MiscStats) -> Html {
    html! {
        <tr>
            <td> { &misc_stat.name } </td>
            <td> { &misc_stat.category } </td>
            <td> { &misc_stat.value } </td>
        </tr>
    }
}

fn render_tes(tes: &TES, form_id_array: &Vec<u32>, form_id_map: &FormIdResolver) -> Html {
    html! {
        <>
        <div class="row">
            <div class="col">
                <h2>
                { "TES" }
                </h2>
            </div>
            <div class="col text-right">
                <button class="btn btn-primary" type="button" data-toggle="collapse" data-target="#tes" aria-expanded="false" aria-controls="tes">{ "Expand" }</button>
            </div>
        </div>
        <div class="collapse" id="tes">
        { render_collapsible_table("TesUnknwown0s".to_string(), "tesunknown0table".to_string(), vec!("Form Id".to_string(), "Unknown Entry".to_string()), &tes.u1, RenderFunctionType::WithFormIds(render_tes_unknown_0, form_id_array, form_id_map)) }

        <h3>
        {
            "u2: "
        }
        </h3>
        {
            tes.u2.iter().map(|x| render_form_id_type(x, form_id_array, form_id_map)).collect::<Html>()
        }
        <br/>
        <h3>
        {
            "u3: "
        }
        </h3>
        {
            tes.u3.iter().map(|x| render_form_id_type(x, form_id_array, form_id_map) ).collect::<Html>()
        }
        </div>
        </>
    }
}

fn render_tes_unknown_0(t: &TESUnknown0, form_id_array: &Vec<u32>, form_id_map: &FormIdResolver) -> Html {
    html! {
            <tr>
                <td>
                    {
                        render_form_id_type(&t.form_id, form_id_array, form_id_map)
                    }
                </td>
                <td>
                    { t.unknown }
                </td>
            </tr>
    }
}

fn render_form_id_type(rid: &FormIdType, form_id_array: &Vec<u32>, form_id_map: &FormIdResolver) -> Html {
    match rid {
        FormIdType::Index(i) => { html!( <>{ "Form ID from FormIDIndex: " } { render_hex(*form_id_array.get(*i as usize).unwrap()) } </>) }
        FormIdType::Default(i) => {
            if *i == 0 {
                return html! {<>{ "Zero Id: " }{render_hex(0)}</>};
            }
            render_look_up_form_id(*i, form_id_map)
        }
        FormIdType::Created(i) => { html!( <>{ "Ref_ID (created): " } { render_hex(*i) }</>) }
        FormIdType::Unknown(i) => { html!( <>{ "Ref_ID (unknown): " } { render_hex(*i) }</>) }
    }
}

fn render_form_id_entry(entry: &FormIdEntry) -> String {
    format!("Name: {}, Category: {:?}", entry.name, entry.category)
}

pub fn render_change_forms(c: &Vec<ChangeForm>, _form_id_array: &Vec<u32>, _form_id_map: &FormIdResolver) -> Html {
    html! {
        <>
        {"Change forms! ("} { c.len() } {" of them to be exact)"}
        </>
    }
}

pub fn render_change_form(_c: &ChangeForm, _form_id_array: &Vec<u32>, _form_id_map: &FormIdResolver) -> Html {
    html! {
        <table class="table">
        <thead>
        <tr>
        <th>
        {"Field"}
        </th>
        <th>
        {"Value"}
        </th>
        </tr>
        </thead>
        <tbody>
        <tr>
        <th>

        </th>
        </tr>
        </tbody>
        </table>
    }
}

pub fn render_form_ids(_f: &Vec<u32>) -> Html {
    html! {
        "Form ids."
    }
}

pub fn render_visited_world_space(v: &Vec<u32>, form_id_map: &FormIdResolver) -> Html {
    html! {
        <>
        <h3>
        { "Locations visited:" }
        </h3>
        <table class="table table-striped">
        <tbody>
        { v.iter().map(|x| { render_worldspace_entry(*x, form_id_map)}).collect::<Html>() }
        </tbody>
        </table>
        </>
    }
}

fn render_worldspace_entry(id: u32, form_id_map: &FormIdResolver) -> Html {
    html! {
        <tr>
        <td>
        { render_look_up_form_id(id, form_id_map) }
        </td>
        </tr>
    }
}

fn render_look_up_form_id(id: u32, form_id_map: &FormIdResolver) -> Html {
    let entry: Option<&FormIdEntry> = form_id_map.get(&id);

    html! {
        <>
        { "Form ID from Skyrim original: " }
        { render_hex(id) }
        { " " }
        {
            match entry {
            Some(entry) => render_form_id_entry(entry),
            None => String::from("Could not resolve name.")
            }
        }
        </>
    }
}