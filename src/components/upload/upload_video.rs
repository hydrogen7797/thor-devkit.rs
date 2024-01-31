use crate::components::api::ipfs_request::ipfs_call;
use crate::components::api::select_ipfs_provider::DEFAULT_IPFS_PROVIDER;

use icondata;
use leptos::ev::{Event, DragEvent};
use leptos::*;
use leptos_icons::*;
use web_sys::HtmlInputElement;

#[component]
pub fn FileUpload(// #[prop(into)] ipfs_cid: WriteSignal<String>
) -> impl IntoView {
    let file_handle = move |e: Event| {
        let input: HtmlInputElement = event_target(&e);
        let file_data = input.files().unwrap().get(0);
        gloo::console::log!(format! {"{:?}", file_data});

        // if let Some(file) = files.get(0) {
        //     // Uncomment and add your file handling logic here
        //     gloo::console::log!("File selected: {:?}", file);
        // }
    };
    let ondrop = move |e: DragEvent| {
        gloo::console::log!(format! {"inside drag"});


        e.prevent_default();
        let file_data = e.data_transfer().unwrap().files().unwrap().get(0);
        gloo::console::log!(format! {"{:?}", file_data});
    };

    let ondragover = move |e: DragEvent| {
        e.prevent_default();
    };

    let ondragenter = move |e: DragEvent| {
        e.prevent_default();
    };

    view! {
        <div class="md:container md:mx-auto pt-20">
            <div class="mb-6">
                <div
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full h-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 min-h-80 flex justify-center items-center"
                    id="drag-container"
                    on:drop=ondrop
                    on:dragover=ondragover
                    on:dragenter=ondragenter
                >
                    <div class="text-center">
                        <div class="mb-4">
                            <Icon
                                icon=icondata::BsCloudUpload
                                width="10em"
                                height="10em"
                                style="color: green"
                                class="inline-block"
                            />
                        </div>
                        <p class="text-lg text-gray-700">
                            {"Drop your mp4 video here or click to select"}
                        </p>
                        <br/>
                        <div>
                            <input
                                id="file-upload"
                                type="file"
                                accept="video/mp4"
                                multiple=false
                                on:input=file_handle
                            />

                        </div>
                    </div>

                </div>
            </div>
        </div>
    }
}
