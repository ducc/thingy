use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let (expand_burger, set_expand_burger) = create_signal(false);

    view! {
        <nav class="bg-white border-gray-200 dark:bg-gray-900">
            <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                <a
                    href="https://flowbite.com/"
                    class="flex items-center space-x-3 rtl:space-x-reverse"
                >
                    <img
                        src="https://flowbite.com/docs/images/logo.svg"
                        class="h-8"
                        alt="Flowbite Logo"
                    />
                    <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">
                        "Thingy"
                    </span>
                </a>
                <button
                    data-collapse-toggle="navbar-default"
                    type="button"
                    class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
                    aria-controls="navbar-default"
                    aria-expanded="false"
                    on:click=move |_| {
                        set_expand_burger.set(!(expand_burger.get()));
                    }
                >

                    <span class="sr-only">Open main menu</span>
                    <svg
                        class="w-5 h-5"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 17 14"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M1 1h15M1 7h15M1 13h15"
                        ></path>
                    </svg>
                </button>
                <div class="hidden w-full md:block md:w-auto" id="navbar-default">
                    <ul class="font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
                        <DesktopTab name="Diary" active=true/>
                        <DesktopTab name="Shopping List"/>
                        <DesktopTab name="Analytics"/>
                        <DesktopTab name="Logout"/>
                    </ul>
                </div>
                <div
                    class=move || if expand_burger() { "w-full" } else { "hidden w-full" }
                    id="navbar-hamburger"
                >
                    <ul class="flex flex-col font-medium mt-4 rounded-lg bg-gray-50 dark:bg-gray-800 dark:border-gray-700">
                        <MobileTab name="Diary" active=true/>
                        <MobileTab name="Shopping List"/>
                        <MobileTab name="Analytics"/>
                        <MobileTab name="Logout"/>
                    </ul>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn MobileTab(name: &'static str, #[prop(optional)] active: bool) -> impl IntoView {
    if active {
        view! { <ActiveMobileTab name=name/> }
    } else {
        view! { <InactiveMobileTab name=name/> }
    }
}

#[component]
fn InactiveMobileTab(name: &'static str) -> impl IntoView {
    view! {
        <li>
            <a
                href="#"
                class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white"
            >
                {name}
            </a>
        </li>
    }
}

#[component]
fn ActiveMobileTab(name: &'static str) -> impl IntoView {
    view! {
        <li>
            <a
                href="#"
                class="block py-2 px-3 text-white bg-blue-700 rounded dark:bg-blue-600"
                aria-current="page"
            >
                {name}
            </a>
        </li>
    }
}

#[component]
fn DesktopTab(name: &'static str, #[prop(optional)] active: bool) -> impl IntoView {
    if active {
        view! { <ActiveDesktopTab name=name/> }
    } else {
        view! { <InactiveDesktopTab name=name/> }
    }
}

#[component]
fn ActiveDesktopTab(name: &'static str) -> impl IntoView {
    view! {
        <li>
            <a
                href="#"
                class="block py-2 px-3 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500"
                aria-current="page"
            >
                {name}
            </a>
        </li>
    }
}

#[component]
fn InactiveDesktopTab(name: &'static str) -> impl IntoView {
    view! {
        <li>
            <a
                href="#"
                class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
            >
                {name}
            </a>
        </li>
    }
}
