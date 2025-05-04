use dioxus::prelude::*;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use navbar::dioxus::{DropdownItem, MegaMenuItem, Menu, Navbar};

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        document::Stylesheet { href: asset!("assets/main.css") }
        LandingPage {}
    }
}

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        div {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
            h1 { class: "text-3xl font-bold mb-8 text-white", "Navbar Dioxus Examples" }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Example 1" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use navbar::dioxus::{{Menu, Navbar}};

#[component]
fn Example1() -> Element {{
    rsx! {{
        Navbar {{
            menus: vec![
                Menu {{ id: 1, link: "/", name: "Home", icon_start: None, icon_end: None }},
                Menu {{ id: 2, link: "/about", name: "About", icon_start: None, icon_end: None }},
            ],
        }}
    }}
}}"##
                    }
                    Example1 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Example 2" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use navbar::dioxus::{{Menu, Navbar}};

#[component]
fn Example2() -> Element {{
    rsx! {{
        Navbar {{
            show_search: true,
            menus: vec![
                Menu {{ id: 1, link: "/", name: "Dashboard", icon_start: None, icon_end: None }},
                Menu {{ id: 2, link: "/reports", name: "Reports", icon_start: None, icon_end: None }},
            ],
        }}
    }}
}}"##
                    }
                    Example2 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Example 3" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use navbar::dioxus::{{Menu, DropdownItem, Navbar}};

#[component]
fn Example3() -> Element {{
    rsx! {{
        Navbar {{
            show_profile_menu: true,
            dropdown_items: vec![
                DropdownItem {{ id: 1, link: "/profile", label: "Profile", icon: None }},
                DropdownItem {{ id: 2, link: "/settings", label: "Settings", icon: None }},
                DropdownItem {{ id: 3, link: "/logout", label: "Logout", icon: None }},
            ],
            menus: vec![
                Menu {{ id: 1, link: "/", name: "Services", icon_start: None, icon_end: None }},
            ],
        }}
    }}
}}"##
                    }
                    Example3 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Example 4" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use navbar::dioxus::{{Menu, MegaMenuItem, Navbar}};

#[component]
fn Example4() -> Element {{
    rsx! {{
        Navbar {{
            show_mega_menu: true,
            mega_menu_items: vec![
                MegaMenuItem {{ title: "Hosting", description: "Fast cloud hosting", link: "/hosting" }},
                MegaMenuItem {{ title: "Storage", description: "Secure storage plans", link: "/storage" }},
            ],
            menus: vec![
                Menu {{ id: 1, link: "/products", name: "Products", icon_start: None, icon_end: None }},
            ],
        }}
    }}
}}"##
                    }
                    Example4 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Example 5" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use navbar::dioxus::{{Menu, DropdownItem, Navbar}};

#[component]
fn Example5() -> Element {{
    rsx! {{
        Navbar {{
            button_text: "Upgrade",
            button_href: "/upgrade",
            show_profile_menu: true,
            dropdown_items: vec![
                DropdownItem {{ id: 1, link: "/account", label: "Account", icon: None }},
                DropdownItem {{ id: 2, link: "/logout", label: "Logout", icon: None }},
            ],
            menus: vec![
                Menu {{ id: 1, link: "/", name: "Overview", icon_start: None, icon_end: None }},
                Menu {{ id: 2, link: "/billing", name: "Billing", icon_start: None, icon_end: None }},
            ],
        }}
    }}
}}"##
                    }
                    Example5 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Example 6" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use navbar::dioxus::{{Menu, Navbar}};

#[component]
fn Example6() -> Element {{
    rsx! {{
        Navbar {{
            show_search: true,
            search_placeholder: "Search products...",
            menus: vec![
                Menu {{ id: 1, link: "/shop", name: "Shop", icon_start: None, icon_end: None }},
                Menu {{ id: 2, link: "/categories", name: "Categories", icon_start: None, icon_end: None }},
            ],
        }}
    }}
}}"##
                    }
                    Example6 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Example 7" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use navbar::dioxus::{{Menu, Navbar}};

#[component]
fn Example7() -> Element {{
    rsx! {{
        Navbar {{
            button_text: "Start Free Trial",
            button_href: "/signup",
            button_target: "_self",
            menus: vec![
                Menu {{ id: 1, link: "/solutions", name: "Solutions", icon_start: None, icon_end: None }},
                Menu {{ id: 2, link: "/pricing", name: "Pricing", icon_start: None, icon_end: None }},
            ],
        }}
    }}
}}"##
                    }
                    Example7 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Example 8" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use navbar::dioxus::{{Menu, DropdownItem, Navbar}};

#[component]
fn Example8() -> Element {{
    rsx! {{
        Navbar {{
            show_profile_menu: true,
            profile_button_text: "🌐 languages",
            dropdown_items: vec![
                DropdownItem {{ id: 1, link: "/lang/en", label: "English", icon: None }},
                DropdownItem {{ id: 2, link: "/lang/es", label: "Español", icon: None }},
                DropdownItem {{ id: 3, link: "/lang/fr", label: "Français", icon: None }},
            ],
            menus: vec![
                Menu {{ id: 1, link: "/", name: "Home", icon_start: None, icon_end: None }},
                Menu {{ id: 2, link: "/about", name: "About", icon_start: None, icon_end: None }},
            ],
        }}
    }}
}}"##
                    }
                    Example8 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Example 9" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use navbar::dioxus::{{Menu, MegaMenuItem, Navbar}};

#[component]
fn Example9() -> Element {{
    rsx! {{
        Navbar {{
            show_mega_menu: true,
            mega_menu_items: vec![
                MegaMenuItem {{ title: "CRM", description: "Customer management", link: "/crm" }},
                MegaMenuItem {{ title: "Marketing", description: "Automation tools", link: "/marketing" }},
                MegaMenuItem {{ title: "Analytics", description: "Business insights", link: "/analytics" }},
            ],
            menus: vec![
                Menu {{ id: 1, link: "/platform", name: "Platform", icon_start: None, icon_end: None }},
            ],
        }}
    }}
}}"##
                    }
                    Example9 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Example 10" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use navbar::dioxus::{{Menu, Navbar}};

#[component]
fn Example10() -> Element {{
    rsx! {{
        Navbar {{
            menus: vec![
                Menu {{ id: 1, link: "/docs", name: "Docs", icon_start: None, icon_end: None }},
                Menu {{ id: 2, link: "/api", name: "API", icon_start: None, icon_end: None }},
                Menu {{ id: 3, link: "/guides", name: "Guides", icon_start: None, icon_end: None }},
            ],
        }}
    }}
}}"##
                    }
                    Example10 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Example 11" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use navbar::dioxus::{{Menu, DropdownItem, Navbar}};

#[component]
fn Example11() -> Element {{
    rsx! {{
        Navbar {{
            menus: vec![
                Menu {{ id: 1, link: "/courses", name: "Courses", icon_start: None, icon_end: None }},
                Menu {{ id: 2, link: "/tutorials", name: "Tutorials", icon_start: None, icon_end: None }},
                Menu {{ id: 3, link: "/certifications", name: "Certifications", icon_start: None, icon_end: None }},
            ],
            show_profile_menu: true,
            dropdown_items: vec![
                DropdownItem {{ id: 1, link: "/dashboard", label: "Dashboard", icon: None }},
                DropdownItem {{ id: 2, link: "/logout", label: "Logout", icon: None }},
            ],
        }}
    }}
}}"##
                    }
                    Example11 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Example 12" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"use dioxus::prelude::*;
use navbar::dioxus::{{Menu, DropdownItem, MegaMenuItem, Navbar}};

#[component]
fn Example12() -> Element {{
    rsx! {{
        Navbar {{
            show_search: true,
            show_mega_menu: true,
            show_profile_menu: true,
            search_placeholder: "Search courses, docs...",
            mega_menu_items: vec![
                MegaMenuItem {{ title: "Docs", description: "Official docs", link: "/docs" }},
                MegaMenuItem {{ title: "Tutorials", description: "Step-by-step guides", link: "/tutorials" }},
                MegaMenuItem {{ title: "API", description: "Full API Reference", link: "/api" }},
            ],
            dropdown_items: vec![
                DropdownItem {{ id: 1, link: "/account", label: "Account", icon: None }},
                DropdownItem {{ id: 2, link: "/notifications", label: "Notifications", icon: None }},
                DropdownItem {{ id: 3, link: "/logout", label: "Logout", icon: None }},
            ],
            menus: vec![
                Menu {{ id: 1, link: "/", name: "Home", icon_start: None, icon_end: None }},
                Menu {{ id: 2, link: "/explore", name: "Explore", icon_start: None, icon_end: None }},
                Menu {{ id: 3, link: "/pricing", name: "Pricing", icon_start: None, icon_end: None }},
            ],
        }}
    }}
}}"##
                    }
                    Example12 {}
                }
            }
        }
    }
}

#[component]
fn Example1() -> Element {
    rsx! {
        Navbar {
            menus: vec![
                Menu { id: 1, link: "/", name: "Home", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/about", name: "About", icon_start: None, icon_end: None },
            ],
        }
    }
}
#[component]
fn Example2() -> Element {
    rsx! {
        Navbar {
            show_search: true,
            menus: vec![
                Menu { id: 1, link: "/", name: "Dashboard", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/reports", name: "Reports", icon_start: None, icon_end: None },
            ],
        }
    }
}
#[component]
fn Example3() -> Element {
    rsx! {
        Navbar {
            show_profile_menu: true,
            dropdown_items: vec![
                DropdownItem { id: 1, link: "/profile", label: "Profile", icon: None },
                DropdownItem { id: 2, link: "/settings", label: "Settings", icon: None },
                DropdownItem { id: 3, link: "/logout", label: "Logout", icon: None },
            ],
            menus: vec![
                Menu { id: 1, link: "/", name: "Services", icon_start: None, icon_end: None },
            ],
        }
    }
}
#[component]
fn Example4() -> Element {
    rsx! {
        Navbar {
            show_mega_menu: true,
            mega_menu_items: vec![
                MegaMenuItem { title: "Hosting", description: "Fast cloud hosting", link: "/hosting" },
                MegaMenuItem { title: "Storage", description: "Secure storage plans", link: "/storage" },
            ],
            menus: vec![
                Menu { id: 1, link: "/products", name: "Products", icon_start: None, icon_end: None },
            ],
        }
    }
}
#[component]
fn Example5() -> Element {
    rsx! {
        Navbar {
            button_text: "Upgrade",
            button_href: "/upgrade",
            show_profile_menu: true,
            dropdown_items: vec![
                DropdownItem { id: 1, link: "/account", label: "Account", icon: None },
                DropdownItem { id: 2, link: "/logout", label: "Logout", icon: None },
            ],
            menus: vec![
                Menu { id: 1, link: "/", name: "Overview", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/billing", name: "Billing", icon_start: None, icon_end: None },
            ],
        }
    }
}
#[component]
fn Example6() -> Element {
    rsx! {
        Navbar {
            show_search: true,
            search_placeholder: "Search products...",
            menus: vec![
                Menu { id: 1, link: "/shop", name: "Shop", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/categories", name: "Categories", icon_start: None, icon_end: None },
            ],
        }
    }
}
#[component]
fn Example7() -> Element {
    rsx! {
        Navbar {
            button_text: "Start Free Trial",
            button_href: "/signup",
            button_target: "_self",
            menus: vec![
                Menu { id: 1, link: "/solutions", name: "Solutions", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/pricing", name: "Pricing", icon_start: None, icon_end: None },
            ],
        }
    }
}
#[component]
fn Example8() -> Element {
    rsx! {
        Navbar {
            show_profile_menu: true,
            profile_button_text: "🌐 languages",
            dropdown_items: vec![
                DropdownItem { id: 1, link: "/lang/en", label: "English", icon: None },
                DropdownItem { id: 2, link: "/lang/es", label: "Español", icon: None },
                DropdownItem { id: 3, link: "/lang/fr", label: "Français", icon: None },
            ],
            menus: vec![
                Menu { id: 1, link: "/", name: "Home", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/about", name: "About", icon_start: None, icon_end: None },
            ],
        }
    }
}
#[component]
fn Example9() -> Element {
    rsx! {
        Navbar {
            show_mega_menu: true,
            mega_menu_items: vec![
                MegaMenuItem { title: "CRM", description: "Customer management", link: "/crm" },
                MegaMenuItem { title: "Marketing", description: "Automation tools", link: "/marketing" },
                MegaMenuItem { title: "Analytics", description: "Business insights", link: "/analytics" },
            ],
            menus: vec![
                Menu { id: 1, link: "/platform", name: "Platform", icon_start: None, icon_end: None },
            ],
        }
    }
}
#[component]
fn Example10() -> Element {
    rsx! {
        Navbar {
            menus: vec![
                Menu { id: 1, link: "/docs", name: "Docs", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/api", name: "API", icon_start: None, icon_end: None },
                Menu { id: 3, link: "/guides", name: "Guides", icon_start: None, icon_end: None },
            ],
        }
    }
}
#[component]
fn Example11() -> Element {
    rsx! {
        Navbar {
            menus: vec![
                Menu { id: 1, link: "/courses", name: "Courses", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/tutorials", name: "Tutorials", icon_start: None, icon_end: None },
                Menu { id: 3, link: "/certifications", name: "Certifications", icon_start: None, icon_end: None },
            ],
            show_profile_menu: true,
            dropdown_items: vec![
                DropdownItem { id: 1, link: "/dashboard", label: "Dashboard", icon: None },
                DropdownItem { id: 2, link: "/logout", label: "Logout", icon: None },
            ],
        }
    }
}
#[component]
fn Example12() -> Element {
    rsx! {
        Navbar {
            show_search: true,
            show_mega_menu: true,
            show_profile_menu: true,
            search_placeholder: "Search courses, docs...",
            mega_menu_items: vec![
                MegaMenuItem { title: "Docs", description: "Official docs", link: "/docs" },
                MegaMenuItem { title: "Tutorials", description: "Step-by-step guides", link: "/tutorials" },
                MegaMenuItem { title: "API", description: "Full API Reference", link: "/api" },
            ],
            dropdown_items: vec![
                DropdownItem { id: 1, link: "/account", label: "Account", icon: None },
                DropdownItem { id: 2, link: "/notifications", label: "Notifications", icon: None },
                DropdownItem { id: 3, link: "/logout", label: "Logout", icon: None },
            ],
            menus: vec![
                Menu { id: 1, link: "/", name: "Home", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/explore", name: "Explore", icon_start: None, icon_end: None },
                Menu { id: 3, link: "/pricing", name: "Pricing", icon_start: None, icon_end: None },
            ],
        }
    }
}
