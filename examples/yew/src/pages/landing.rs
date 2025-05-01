use yew::prelude::*;
use navbar::yew::{Menu, DropdownItem, MegaMenuItem, Navbar};

#[function_component(Example1)]
pub fn example1() -> Html {
    html! {
        <Navbar
            menus={vec![
            Menu { id: 1, link: "/", name: "Home", icon_start: None, icon_end: None },
            Menu { id: 2, link: "/about", name: "About", icon_start: None, icon_end: None },
        ]}
        />
    }
}

#[function_component(Example2)]
pub fn example2() -> Html {
    html! {
        <Navbar
            show_search=true
            menus={vec![
                Menu { id: 1, link: "/", name: "Dashboard", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/reports", name: "Reports", icon_start: None, icon_end: None },
            ]}
        />
    }
}

#[function_component(Example3)]
pub fn example3() -> Html {
    html! {
        <Navbar
            show_profile_menu=true
            dropdown_items={vec![
                DropdownItem { id: 1, link: "/profile", label: "Profile", icon: None },
                DropdownItem { id: 2, link: "/settings", label: "Settings", icon: None },
                DropdownItem { id: 3, link: "/logout", label: "Logout", icon: None },
            ]}
            menus={vec![
                Menu { id: 1, link: "/", name: "Services", icon_start: None, icon_end: None },
            ]}
        />
    }
}

#[function_component(Example4)]
pub fn example4() -> Html {
    html! {
        <Navbar
            show_mega_menu=true
            mega_menu_items={vec![
                MegaMenuItem { title: "Hosting", description: "Fast cloud hosting", link: "/hosting" },
                MegaMenuItem { title: "Storage", description: "Secure storage plans", link: "/storage" },
            ]}
            menus={vec![
                Menu { id: 1, link: "/products", name: "Products", icon_start: None, icon_end: None },
            ]}
        />
    }
}

#[function_component(Example5)]
pub fn example5() -> Html {
    html! {
        <Navbar
            button_text="Upgrade"
            button_href="/upgrade"
            show_profile_menu=true
            dropdown_items={vec![
                DropdownItem { id: 1, link: "/account", label: "Account", icon: None },
                DropdownItem { id: 2, link: "/logout", label: "Logout", icon: None },
            ]}
            menus={vec![
                Menu { id: 1, link: "/", name: "Overview", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/billing", name: "Billing", icon_start: None, icon_end: None },
            ]}
        />
    }
}

#[function_component(Example6)]
pub fn example6() -> Html {
    html! {
        <Navbar
            show_search=true
            search_placeholder="Search products..."
            menus={vec![
                Menu { id: 1, link: "/shop", name: "Shop", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/categories", name: "Categories", icon_start: None, icon_end: None },
            ]}
        />
    }
}

#[function_component(Example7)]
pub fn example7() -> Html {
    html! {
        <Navbar
            button_text="Start Free Trial"
            button_href="/signup"
            button_target="_self"
            menus={vec![
                Menu { id: 1, link: "/solutions", name: "Solutions", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/pricing", name: "Pricing", icon_start: None, icon_end: None },
            ]}
        />
    }
}

#[function_component(Example8)]
pub fn example8() -> Html {
    html! {
        <Navbar
            show_profile_menu=true
            profile_button_text="🌐 languages"
            dropdown_items={vec![
                DropdownItem { id: 1, link: "/lang/en", label: "English", icon: None },
                DropdownItem { id: 2, link: "/lang/es", label: "Español", icon: None },
                DropdownItem { id: 3, link: "/lang/fr", label: "Français", icon: None },
            ]}
            menus={vec![
                Menu { id: 1, link: "/", name: "Home", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/about", name: "About", icon_start: None, icon_end: None },
            ]}
        />
    }
}

#[function_component(Example9)]
pub fn example9() -> Html {
    html! {
        <Navbar
            show_mega_menu=true
            mega_menu_items={vec![
                MegaMenuItem { title: "CRM", description: "Customer management", link: "/crm" },
                MegaMenuItem { title: "Marketing", description: "Automation tools", link: "/marketing" },
                MegaMenuItem { title: "Analytics", description: "Business insights", link: "/analytics" },
            ]}
            menus={vec![
                Menu { id: 1, link: "/platform", name: "Platform", icon_start: None, icon_end: None },
            ]}
        />
    }
}

#[function_component(Example10)]
pub fn example10() -> Html {
    html! {
        <Navbar
            menus={vec![
                Menu { id: 1, link: "/docs", name: "Docs", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/api", name: "API", icon_start: None, icon_end: None },
                Menu { id: 3, link: "/guides", name: "Guides", icon_start: None, icon_end: None },
            ]}
        />
    }
}

#[function_component(Example11)]
pub fn example11() -> Html {
    html! {
        <Navbar
            menus={vec![
                Menu { id: 1, link: "/courses", name: "Courses", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/tutorials", name: "Tutorials", icon_start: None, icon_end: None },
                Menu { id: 3, link: "/certifications", name: "Certifications", icon_start: None, icon_end: None },
            ]}
            show_profile_menu=true
            dropdown_items={vec![
                DropdownItem { id: 1, link: "/dashboard", label: "Dashboard", icon: None },
                DropdownItem { id: 2, link: "/logout", label: "Logout", icon: None },
            ]}
        />
    }
}

#[function_component(Example12)]
pub fn example12() -> Html {
    html! {
        <Navbar
            show_search=true
            show_mega_menu=true
            show_profile_menu=true
            search_placeholder="Search courses, docs..."
            mega_menu_items={vec![
                MegaMenuItem { title: "Docs", description: "Official docs", link: "/docs" },
                MegaMenuItem { title: "Tutorials", description: "Step-by-step guides", link: "/tutorials" },
                MegaMenuItem { title: "API", description: "Full API Reference", link: "/api" },
            ]}
            dropdown_items={vec![
                DropdownItem { id: 1, link: "/account", label: "Account", icon: None },
                DropdownItem { id: 2, link: "/notifications", label: "Notifications", icon: None },
                DropdownItem { id: 3, link: "/logout", label: "Logout", icon: None },
            ]}
            menus={vec![
                Menu { id: 1, link: "/", name: "Home", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/explore", name: "Explore", icon_start: None, icon_end: None },
                Menu { id: 3, link: "/pricing", name: "Pricing", icon_start: None, icon_end: None },
            ]}
        />
    }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Navbar Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Basic Navbar" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use navbar::yew::{Menu, Navbar};

#[function_component(Example1)]
pub fn example1() -> Html {
    html! {
        <Navbar menus={vec![
            Menu { id: 1, link: "/", name: "Home", icon_start: None, icon_end: None },
            Menu { id: 2, link: "/about", name: "About", icon_start: None, icon_end: None },
        ]}/>
    }
}"# }
                    </pre>
                    <Example1 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "With Search" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use navbar::yew::{Menu, Navbar};


#[function_component(Example2)]
pub fn example2() -> Html {
    html! {
        <Navbar
            show_search={true}
            menus={vec![
                Menu { id: 1, link: "/", name: "Dashboard", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/reports", name: "Reports", icon_start: None, icon_end: None },
            ]}
        />
    }
}"# }
                    </pre>
                    <Example2 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "With Profile Menu" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use navbar::yew::{Menu, DropdownItem, Navbar};


#[function_component(Example3)]
pub fn example3() -> Html {
    html! {
        <Navbar
            show_profile_menu={true}
            dropdown_items={vec![
                DropdownItem { id: 1, link: "/profile", label: "Profile", icon: None },
                DropdownItem { id: 2, link: "/settings", label: "Settings", icon: None },
                DropdownItem { id: 3, link: "/logout", label: "Logout", icon: None },
            ]}
            menus={vec![
                Menu { id: 1, link: "/", name: "Services", icon_start: None, icon_end: None },
            ]}
        />
    }
}"# }
                    </pre>
                    <Example3 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "With Mega Menu" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use navbar::yew::{Menu, MegaMenuItem, Navbar};


#[function_component(Example4)]
pub fn example4() -> Html {
    html! {
        <Navbar
            show_mega_menu=true
            mega_menu_items={vec![
                MegaMenuItem { title: "Hosting", description: "Fast cloud hosting", link: "/hosting" },
                MegaMenuItem { title: "Storage", description: "Secure storage plans", link: "/storage" },
            ]}
            menus={vec![
                Menu { id: 1, link: "/products", name: "Products", icon_start: None, icon_end: None },
            ]}
        />
    }
}"# }
                    </pre>
                    <Example4 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Upgrade + Profile" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use navbar::yew::{Menu, DropdownItem, Navbar};


#[function_component(Example5)]
pub fn example5() -> Html {
    html! {
        <Navbar
            button_text="Upgrade"
            button_href="/upgrade"
            show_profile_menu=true
            dropdown_items={vec![
                DropdownItem { id: 1, link: "/account", label: "Account", icon: None },
                DropdownItem { id: 2, link: "/logout", label: "Logout", icon: None },
            ]}
            menus={vec![
                Menu { id: 1, link: "/", name: "Overview", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/billing", name: "Billing", icon_start: None, icon_end: None },
            ]}
        />
    }
}"# }
                    </pre>
                    <Example5 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Product Search" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use navbar::yew::{Menu, Navbar};


#[function_component(Example6)]
pub fn example6() -> Html {
    html! {
        <Navbar
            show_search=true
            search_placeholder="Search products..."
            menus={vec![
                Menu { id: 1, link: "/shop", name: "Shop", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/categories", name: "Categories", icon_start: None, icon_end: None },
            ]}
        />
    }
}"# }
                    </pre>
                    <Example6 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "CTA Button" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use navbar::yew::{Menu, Navbar};


#[function_component(Example7)]
pub fn example7() -> Html {
    html! {
        <Navbar
            button_text="Start Free Trial"
            button_href="/signup"
            button_target="_self"
            menus={vec![
                Menu { id: 1, link: "/solutions", name: "Solutions", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/pricing", name: "Pricing", icon_start: None, icon_end: None },
            ]}
        />
    }
}"# }
                    </pre>
                    <Example7 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Navbar With Languages" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use navbar::yew::{Menu, Navbar};


#[function_component(Example8)]
pub fn example8() -> Html {
    html! {
        <Navbar
            show_profile_menu=true
            dropdown_items={vec![
                DropdownItem { id: 1, link: "/lang/en", label: "English", icon: None },
                DropdownItem { id: 2, link: "/lang/es", label: "Español", icon: None },
                DropdownItem { id: 3, link: "/lang/fr", label: "Français", icon: None },
            ]}
            menus={vec![
                Menu { id: 1, link: "/", name: "Home", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/about", name: "About", icon_start: None, icon_end: None },
            ]}
        />
    }
}"# }
                    </pre>
                    <Example8 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Mega Products" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use navbar::yew::{Menu, MegaMenuItem, Navbar};


#[function_component(Example9)]
pub fn example9() -> Html {
    html! {
        <Navbar
            show_mega_menu=true
            profile_button_text="🌐 languages"
            mega_menu_items={vec![
                MegaMenuItem { title: "CRM", description: "Customer management", link: "/crm" },
                MegaMenuItem { title: "Marketing", description: "Automation tools", link: "/marketing" },
                MegaMenuItem { title: "Analytics", description: "Business insights", link: "/analytics" },
            ]}
            menus={vec![
                Menu { id: 1, link: "/platform", name: "Platform", icon_start: None, icon_end: None },
            ]}
        />
    }
}"# }
                    </pre>
                    <Example9 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "API Docs Nav" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use navbar::yew::{Menu, Navbar};


#[function_component(Example10)]
pub fn example10() -> Html {
    html! {
        <Navbar
            menus={vec![
                Menu { id: 1, link: "/docs", name: "Docs", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/api", name: "API", icon_start: None, icon_end: None },
                Menu { id: 3, link: "/guides", name: "Guides", icon_start: None, icon_end: None },
            ]}
        />
    }
}"# }
                    </pre>
                    <Example10 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Learning Platform" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use navbar::yew::{Menu, DropdownItem, Navbar};


#[function_component(Example11)]
pub fn example11() -> Html {
    html! {
        <Navbar
            menus={vec![
                Menu { id: 1, link: "/courses", name: "Courses", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/tutorials", name: "Tutorials", icon_start: None, icon_end: None },
                Menu { id: 3, link: "/certifications", name: "Certifications", icon_start: None, icon_end: None },
            ]}
            show_profile_menu=true
            dropdown_items={vec![
                DropdownItem { id: 1, link: "/dashboard", label: "Dashboard", icon: None },
                DropdownItem { id: 2, link: "/logout", label: "Logout", icon: None },
            ]}
        />
    }
}"# }
                    </pre>
                    <Example11 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Full Featured Navbar" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use navbar::yew::{Menu, DropdownItem, MegaMenuItem, Navbar};


#[function_component(Example12)]
pub fn example12() -> Html {
    html! {
        <Navbar
            show_search=true
            show_mega_menu=true
            show_profile_menu=true
            search_placeholder="Search courses, docs..."
            mega_menu_items={vec![
                MegaMenuItem { title: "Docs", description: "Official docs", link: "/docs" },
                MegaMenuItem { title: "Tutorials", description: "Step-by-step guides", link: "/tutorials" },
                MegaMenuItem { title: "API", description: "Full API Reference", link: "/api" },
            ]}
            dropdown_items={vec![
                DropdownItem { id: 1, link: "/account", label: "Account", icon: None },
                DropdownItem { id: 2, link: "/notifications", label: "Notifications", icon: None },
                DropdownItem { id: 3, link: "/logout", label: "Logout", icon: None },
            ]}
            menus={vec![
                Menu { id: 1, link: "/", name: "Home", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/explore", name: "Explore", icon_start: None, icon_end: None },
                Menu { id: 3, link: "/pricing", name: "Pricing", icon_start: None, icon_end: None },
            ]}
        />
    }
}"# }
                    </pre>
                    <Example12 />
                </div>
            </div>
        </div>
    }
}
