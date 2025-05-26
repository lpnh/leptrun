use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <div class="flex min-h-screen flex-col bg-stone-950 text-center font-fira-mono text-base text-gray-300">
            <div class="flex flex-1 flex-col items-center">
                <Header />
                <Home />
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="mx-4 flex flex-1 items-center pb-20">
            <div class="mx-auto max-w-3xl space-y-14 text-center md:space-y-20">
                <div>
                    <h1 class="pb-6 text-lg font-bold text-rose-400 md:text-2xl">"Rust on the Client Side?!"</h1>
                    <p class="m-auto w-3/4 md:text-xl">
                        "This is a client-side rendering website template made with "
                        <a class="text-rose-300 transition duration-200 hover:text-rose-200 active:text-rose-200"
                            href="https://leptos.dev/" target="_blank">"Leptos"
                        </a>
                        ", "
                        <a class="text-rose-300 transition duration-200 hover:text-rose-200 active:text-rose-200"
                            href="https://trunkrs.dev/" target="_blank">"Trunk"</a>
                        " and "
                        <a class="text-rose-300 transition duration-200 hover:text-rose-200 active:text-rose-200"
                            href="https://tailwindcss.com/" target="_blank">"Tailwind"</a>
                    </p>
                </div>
                <div class="text-sm md:text-base">
                    <p>"You can check the source code "
                        <a class="border-b border-b-rose-300 text-rose-300 transition duration-200 hover:border-b-rose-200 hover:text-rose-200 active:border-b-rose-200 active:text-rose-200"
                            href="https://github.com/lpnh/leptrun" target="_blank">"here"</a>
                    </p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <div
            class="inset-x-0 bottom-auto top-0 mt-4 flex items-center justify-center tracking-widest text-amber-200 sm:text-lg">
            "Hello, friend!"
        </div>
    }
}
