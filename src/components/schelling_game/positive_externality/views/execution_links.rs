use leptos::prelude::*;

#[component]
pub fn ExecutionLinks(user_to_calculate: String) -> impl IntoView {
    view! {
        <div class="flex justify-center p-4 bg-gray-100 dark:bg-gray-900">
            <div class="space-y-4">

                <div class="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 max-w-sm text-center">
                    <a
                        href={format!(
                            "/positive-externality/add_incentives_count/{}",
                            user_to_calculate,
                        )}
                        class="text-gray-800 dark:text-gray-200 font-medium hover:text-blue-600 dark:hover:text-blue-400 transition duration-300 ease-in-out block"
                    >
                        Add incentives counts
                    </a>
                </div>

                <div class="bg-white dark:bg-gray-800 p-4 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 max-w-sm text-center">
                    <a
                        href={format!(
                            "/positive-externality/view_incentives_count/{}",
                            user_to_calculate,
                        )}
                        class="text-gray-800 dark:text-gray-200 font-medium hover:text-blue-600 dark:hover:text-blue-400 transition duration-300 ease-in-out block"
                    >
                        View incentives counts
                    </a>
                </div>
            </div>
        </div>
    }
}
