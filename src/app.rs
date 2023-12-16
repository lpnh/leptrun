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
                    <h1 class="pb-6 md:text-3xl text-xl text-red-400 font-bold">"Leptos + Trunk + Tailwind"</h1>
                    <p class="md:text-lg">"This is a CSR Web Application Template"</p>
                    <p class="md:text-lg">"Made with Leptos, Trunk and Tailwind"</p>
                </div>
                <div>
                    <a class="md:text-2xl text-lg text-pink-500 hover:text-pink-400 active:text-pink-400 transition duration-200" href="https://github.com/lpnh/leptrun">"Check the Source Code"</a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <div class="tracking-widest mt-4 flex sm:text-lg text-orange-300 justify-center items-center inset-x-0 top-0 bottom-auto">"Hello, friend!"</div>
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