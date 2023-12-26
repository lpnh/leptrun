use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="font-fira-mono flex flex-col bg-stone-950 min-h-screen text-base text-center text-gray-300">
            <div class="flex flex-1 flex-col items-center">
                <Header/>               
                <Home/>
                <Footer/>
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="pb-20 flex flex-1 items-center mx-4">
            <div class="mx-auto max-w-3xl text-center md:space-y-20 space-y-14">
                <div>
                    <h1 class="pb-6 md:text-2xl text-lg text-rose-400 font-bold">"A Website Written in Rust"</h1>
                    <p class="m-auto w-3/4 md:text-xl">
                        "This is a personal CSR Website Template made with "
                        <a class="text-rose-300 hover:text-rose-200 active:text-rose-200 transition duration-200"
                        href="https://leptos.dev/" target="_blank">"Leptos"
                        </a>
                        ", "
                        <a class="text-rose-300 hover:text-rose-200 active:text-rose-200 transition duration-200"
                        href="https://trunkrs.dev/" target="_blank">"Trunk"</a> 
                        " and "
                        <a class="text-rose-300 hover:text-rose-200 active:text-rose-200 transition duration-200"
                            href="https://tailwindcss.com/" target="_blank">"Tailwind"</a>
                    </p>
                </div>
                <div class="md:text-base text-sm">
                    <p>"You can check the source code "
                        <a class="border-b border-b-rose-300 text-rose-300 hover:text-rose-200 hover:border-b-rose-200 active:text-rose-200 active:border-b-rose-200 transition duration-200"
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
        <div class="tracking-widest mt-4 flex sm:text-lg text-amber-200 justify-center items-center inset-x-0 top-0 bottom-auto">"Hello, friend!"</div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <div class="pb-0 m-auto left-0 right-0">
            <div class="text-xs">"© 2023 lpnh"</div>
        </div>
    }
}