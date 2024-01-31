use crate::components::markdown::markdown_to_html::parse_text_to_html;
use leptos::ev::{Event, KeyboardEvent};
use leptos::*;

#[component]
pub fn MarkdownField(
    class: String,
    name: String,
    #[prop(default = true)] required: bool,
    #[prop(into)] markdown: WriteSignal<String>,
) -> impl IntoView {
    let (html_data, set_html_data) = create_signal(String::from(""));
    let markdown_changed = move |e: KeyboardEvent| {
        let html = parse_text_to_html(&event_target_value(&e));
        // gloo::console::log!(format!("{}", html));
        set_html_data(html);
    };

    let handle_onchange = move |e: Event| {
        markdown(event_target_value(&e));
    };

    view! {
        <>
            <div class="md:container md:mx-auto pt-20">
                <div>
                    <form>
                        <div class="mb-6">
                            <textarea
                                rows="10"
                                cols="50"
                                name=name
                                required=required
                                class=format!(
                                    "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 {}",
                                    class,
                                )
                                on:keyup=markdown_changed
                                on:input=handle_onchange
                            ></textarea>
                        </div>
                    </form>
                </div>
                <div inner_html=html_data></div>
            </div>
        </>
    }
}
