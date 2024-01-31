use crate::components::api::ipfs_request::ipfs_call;
use crate::components::api::select_ipfs_provider::DEFAULT_IPFS_PROVIDER;
use gloo::console::log;
use stylist::{yew::styled_component, Style};
use web_sys::{File, FileList, HtmlInputElement};
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_onchange_cid: Callback<String>,
}

const STYLE_FILE: &str = include_str!("upload_video.css");
#[styled_component(UploadVideo)]
pub fn file_upload(props: &Props) -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    let handle_onchange_cid = props.handle_onchange_cid.clone();
    let handle_onchange_cid_onchange = props.handle_onchange_cid.clone();

    let spinner_state = use_state(|| false);
    let spinner_sate_ondrop = spinner_state.clone();
    let spinner_sate_onchange = spinner_state.clone();

    // let filelist: UseStateHandle<Vec<File>> = use_state(|| vec![]);
    // let clone_filelist = filelist.clone();
    let ondrop = {
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            let spinner_state_clone = spinner_sate_ondrop.clone();
            spinner_state_clone.set(true);
            let handle_onchange_cid_clone = handle_onchange_cid.clone();
            let file = event.data_transfer().unwrap().files().unwrap().get(0);
            // log!(file.clone().unwrap());
            let file_type = file.clone().unwrap().type_();
            let file_name = file.clone().unwrap().name();
            // log!(file_type);
            // let mut filelistvalue: Vec<_> = clone_filelist.to_vec();
            // filelistvalue.push(file.clone().unwrap());
            // clone_filelist.set(filelistvalue);
            if file_type == "video/mp4" {
                wasm_bindgen_futures::spawn_local(async move {
                    let ipfs_cid =
                        ipfs_call(DEFAULT_IPFS_PROVIDER, file.unwrap(), file_name).await;
                    log!(ipfs_cid.clone());
                    handle_onchange_cid_clone.emit(ipfs_cid);
                    spinner_state_clone.set(false);
                });
            } else {
                panic!("Upload mp4 video file");
            }
        })
    };
    let ondragover = {
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
        })
    };
    let ondragenter = {
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
        })
    };

    let onchange = {
        Callback::from(move |event: Event| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let spinner_state_clone = spinner_sate_onchange.clone();
            spinner_state_clone.set(true);
            // log!(input.files());
            let file = input.files().unwrap().get(0);
            let handle_onchange_cid_clone = handle_onchange_cid_onchange.clone();

            let file_type = file.clone().unwrap().type_();
            let file_name = file.clone().unwrap().name();



            if file_type == "video/mp4" {
                wasm_bindgen_futures::spawn_local(async move {
                    let ipfs_cid =
                        ipfs_call(DEFAULT_IPFS_PROVIDER, file.unwrap(), file_name).await;
                    log!(ipfs_cid.clone());
                    handle_onchange_cid_clone.emit(ipfs_cid);
                    spinner_state_clone.set(false);
                });
            } else {
                panic!("Upload mp4 video file");
            }
        })
    };

    html! {
        <div class="container">
            <div class={stylesheet}>
                <div class="row">
                    <div class="col-md-6 offset-md-3
                    text-center">
                        // <p id="title">{"I am inside file upload"}</p>
                        <label for="file-upload">
                            <div
                                id="drop-container"
                                ondrop={ondrop}
                                ondragover={ondragover}
                                ondragenter={ondragenter}
                            >
                            <Icon icon_id={IconId::BootstrapCloudUpload} />
                            <p>{"Drop your mp4 video here or click to select"}</p>
                            </div>
                        </label>
                        <input
                            id="file-upload"
                            type="file"
                            accept="video/mp4"
                            multiple={false}
                            onchange={onchange}
                        />
                        </div>

                    // <div>{format!("{:?}", filelist)}</div>
                </div>
                <div class="row">
                    <div class="col-md-6 offset-md-3
                    text-center">
                    if *spinner_state {
                        <img src="img/rolling.gif" alt="loading" width="40"/>
                    }
                    </div>
                </div>

            </div>
        </div>

    }
}
