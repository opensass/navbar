#![doc = include_str!("../DIOXUS.md")]

use dioxus::prelude::*;
use gloo::events::EventListener;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::window;

/// Properties for rendering a logo inside a navigation bar.
#[derive(Props, PartialEq, Clone)]
pub struct LogoProps {
    /// Path to the logo image.
    #[props(default = "/logo.png")]
    pub logo_src: &'static str,
    /// Alternative text for the logo image.
    #[props(default = "Logo")]
    pub logo_alt: &'static str,
    /// URL to redirect when the logo is clicked.
    #[props(default = "/")]
    pub logo_link: &'static str,
    /// Inline CSS style for the logo image.
    #[props(default = "height: 40px;")]
    pub logo_style: &'static str,
    /// Inline CSS style for the link wrapping the logo.
    #[props(default = "display: inline-block;")]
    pub link_style: &'static str,
    /// Optional class name for the logo image.
    #[props(default)]
    pub logo_class: String,
    /// Optional class name for the anchor tag wrapping the logo.
    #[props(default)]
    pub link_class: String,
}

#[component]
pub fn NavbarLogo(props: LogoProps) -> Element {
    rsx! {
        a {
            href: props.logo_link,
            style: props.link_style,
            class: "{props.link_class}",
            img {
                src: props.logo_src,
                alt: props.logo_alt,
                style: props.logo_style,
                class: "{props.logo_class}"
            }
        }
    }
}

/// Properties for rendering a hamburger toggle icon (used in responsive menus).
#[derive(Props, PartialEq, Clone)]
pub struct ToggleProps {
    /// Callback triggered when the toggle is clicked.
    pub onclick: EventHandler<MouseEvent>,
    /// Inline style for the toggle container.
    #[props(default = "display: flex; flex-direction: column; gap: 5px; cursor: pointer;")]
    pub toggle_style: &'static str,
    /// Inline style for each line/bar in the hamburger icon.
    #[props(default = "width: 25px; height: 3px; background: black;")]
    pub line_style: &'static str,
    /// Optional class for the toggle container.
    #[props(default = String::new())]
    pub toggle_class: String,
    /// Optional class for each line in the icon.
    #[props(default = String::new())]
    pub line_class: String,
}

#[component]
pub fn NavbarToggle(props: ToggleProps) -> Element {
    rsx! {
        div {
            tabindex: "0",
            role: "button",
            aria_label: "Toggle Menu",
            style: props.toggle_style,
            class: "{props.toggle_class}",
            onclick: move |e| props.onclick.call(e),
            div { style: props.line_style, class: "{props.line_class}" }
            div { style: props.line_style, class: "{props.line_class}" }
            div { style: props.line_style, class: "{props.line_class}" }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Menu {
    pub id: usize,
    pub name: &'static str,
    pub link: &'static str,
    pub icon_start: Option<Element>,
    pub icon_end: Option<Element>,
}

/// Properties for rendering a horizontal navigation menu.
#[derive(Props, PartialEq, Clone)]
pub struct MenuProps {
    /// List of menu items to display.
    pub menus: Vec<Menu>,
    /// Inline style for each menu item anchor tag.
    #[props(default = "padding: 0.5rem 1rem; text-decoration: none; color: black;")]
    pub menu_item_style: &'static str,
    /// Inline style for the entire list container.
    #[props(default = "display: flex; gap: 1rem; list-style: none; margin: 0; padding: 0;")]
    pub list_style: &'static str,
    /// Optional class for each menu item.
    #[props(default = String::new())]
    pub menu_item_class: String,
    /// Optional class for the list container.
    #[props(default = String::new())]
    pub list_class: String,
}

#[component]
pub fn NavbarMenu(props: MenuProps) -> Element {
    rsx! {
        ul {
            style: props.list_style,
            class: "{props.list_class}",
            for menu in props.menus.iter() {
                li {
                    key: "{menu.id}",
                    a {
                        href: menu.link,
                        style: props.menu_item_style,
                        class: "{props.menu_item_class}",
                        { menu.icon_start.clone().unwrap_or(rsx!("")) }
                        "{menu.name}"
                        { menu.icon_end.clone().unwrap_or(rsx!("")) }
                    }
                }
            }
        }
    }
}

/// Properties for rendering a styled button with a link.
#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    /// The URL the button should navigate to.
    #[props(default = "#")]
    pub href: &'static str,
    /// Text displayed inside the button.
    #[props(default = "Click Me")]
    pub text: &'static str,
    /// Style for the outer div wrapping the button.
    #[props(default = "padding: 0.5rem 1rem; background-color: #007bff; border-radius: 4px;")]
    pub button_style: &'static str,
    /// Style applied to the anchor inside the button.
    #[props(default = "color: white; text-decoration: none;")]
    pub link_style: &'static str,
    /// Target attribute for the anchor link (e.g. `_blank`, `_self`).
    #[props(default = "_self")]
    pub target: &'static str,
    /// Optional class for the outer button div.
    #[props(default = String::new())]
    pub button_class: String,
    /// Optional class for the inner anchor tag.
    #[props(default = String::new())]
    pub link_class: String,
}

#[component]
pub fn NavbarButton(props: ButtonProps) -> Element {
    rsx! {
        div {
            style: props.button_style,
            class: "{props.button_class}",
            a {
                href: props.href,
                target: props.target,
                style: props.link_style,
                class: "{props.link_class}",
                "{props.text}"
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DropdownItem {
    pub id: usize,
    pub label: &'static str,
    pub link: &'static str,
    pub icon: Option<Element>,
}

#[derive(Props, PartialEq, Clone)]
pub struct ProfileMenuProps {
    pub items: Vec<DropdownItem>,
    #[props(
        default = "position: absolute; top: 100%; left: 0; background: white; box-shadow: 0 4px 8px rgba(0,0,0,0.1); z-index: 1000;"
    )]
    pub dropdown_style: &'static str,
    #[props(default = "padding: 0.5rem 1rem; white-space: nowrap;")]
    pub item_style: &'static str,
    #[props(default = "position: relative; display: inline-block; margin: 0 0 0 1rem;")]
    pub wrapper_style: &'static str,
    #[props(default = "background: none; border: none; cursor: pointer;")]
    pub button_style: &'static str,
    #[props(default = "width: 40px; height: 40px; border-radius: 50%; object-fit: cover;")]
    pub avatar_img_style: &'static str,
    #[props(
        default = "width: 40px; height: 40px; border-radius: 50%; background-color: #ccc; color: #fff; display: flex; align-items: center; justify-content: center; font-weight: bold;"
    )]
    pub avatar_fallback_style: &'static str,
    #[props(default = "Profile")]
    pub profile_text: &'static str,
    #[props(default = None)]
    pub profile_image_url: Option<String>,
    pub is_open: bool,
    pub toggle: EventHandler<MouseEvent>,
    #[props(default = String::new())]
    pub dropdown_class: String,
    #[props(default = String::new())]
    pub item_class: String,
    #[props(default = String::new())]
    pub wrapper_class: String,
    #[props(default = String::new())]
    pub avatar_class: String,
    #[props(default = String::new())]
    pub button_class: String,
}

#[component]
pub fn ProfileMenu(props: ProfileMenuProps) -> Element {
    let fallback = props.profile_text.chars().next().unwrap_or('P');
    rsx! {
        div {
            style: props.wrapper_style,
            class: "{props.wrapper_class}",
            button {
                onclick: move |e| props.toggle.call(e),
                aria_haspopup: "true",
                aria_expanded: props.is_open.to_string(),
                style: props.button_style,
                class: "{props.button_class}",
                match &props.profile_image_url {
                    Some(url) => rsx!(
                        img {
                            src: "{url}",
                            alt: "Profile",
                            style: props.avatar_img_style,
                            class: "{props.avatar_class}"
                        }
                    ),
                    None => rsx!(
                        div {
                            style: props.avatar_fallback_style,
                            class: "{props.avatar_class}",
                            "{fallback}"
                        }
                    )
                }
            }
            if props.is_open {
                ul {
                    style: props.dropdown_style,
                    class: "{props.dropdown_class}",
                    for item in props.items.iter() {
                        li {
                            key: "{item.id}",
                            style: props.item_style,
                            class: "{props.item_class}",
                            a {
                                href: item.link,
                                { item.icon.clone().unwrap_or(rsx!("")) }
                                "{item.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct MegaMenuItem {
    pub link: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

#[derive(Props, PartialEq, Clone)]
pub struct MegaMenuProps {
    pub items: Vec<MegaMenuItem>,
    #[props(default = "position: absolute; top: 60px; left: 0; z-index: 999;")]
    pub wrapper_style: &'static str,
    #[props(
        default = "background: white; display: flex; gap: 2rem; padding: 1rem; box-shadow: 0 2px 8px rgba(0,0,0,0.1);"
    )]
    pub card_style: &'static str,
    #[props(default = "text-decoration: none; color: black;")]
    pub item_link_style: &'static str,
    #[props(default = String::new())]
    pub wrapper_class: String,
    #[props(default = String::new())]
    pub card_class: String,
    #[props(default = String::new())]
    pub item_class: String,
}

#[component]
pub fn MegaMenu(props: MegaMenuProps) -> Element {
    rsx! {
        div {
            style: props.wrapper_style,
            class: "{props.wrapper_class}",
            div {
                style: props.card_style,
                class: "{props.card_class}",
                for item in props.items.iter() {
                    a {
                        href: item.link,
                        style: props.item_link_style,
                        class: "{props.item_class}",
                        div {
                            p { strong { "{item.title}" } }
                            p { "{item.description}" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct MobileMenuProps {
    pub menus: Vec<Menu>,
    #[props(default = "padding: 0.5rem 1rem;")]
    pub menu_item_style: &'static str,
    pub dropdown_items: Vec<DropdownItem>,
    #[props(default = "background: #f9f9f9; padding: 1rem; border-radius: 4px;")]
    pub dropdown_style: &'static str,
    #[props(default = "padding: 0.5rem;")]
    pub dropdown_item_style: &'static str,
    #[props(default = false)]
    pub show_profile_menu: bool,
    #[props(default = false)]
    pub show_search: bool,
    #[props(default = "padding: 0.5rem; width: 100%; border: 1px solid #ccc;")]
    pub search_input_style: &'static str,
    #[props(default = "Search...")]
    pub search_placeholder: &'static str,
    #[props(
        default = "display: flex; flex-direction: column; gap: 1rem; padding: 1rem; position: absolute; top: 100%; left: 0; background: white; z-index: 999; width: max-content;"
    )]
    pub container_style: &'static str,
    #[props(default = String::new())]
    pub container_class: String,
    #[props(default = String::new())]
    pub menu_item_class: String,
    #[props(default = String::new())]
    pub dropdown_class: String,
    #[props(default = String::new())]
    pub dropdown_item_class: String,
    #[props(default = String::new())]
    pub search_input_class: String,
}

#[component]
pub fn MobileMenu(props: MobileMenuProps) -> Element {
    rsx! {
        div {
            style: props.container_style,
            class: "{props.container_class}",
            NavbarMenu {
                menus: props.menus.clone(),
                menu_item_style: props.menu_item_style,
                menu_item_class: props.menu_item_class.clone(),
                list_style: "",
                list_class: "",
            }
            if props.show_search {
                input {
                    r#type: "text",
                    style: props.search_input_style,
                    placeholder: props.search_placeholder,
                    class: "{props.search_input_class}",
                }
            }
            if props.show_profile_menu {
                ul {
                    style: props.dropdown_style,
                    class: "{props.dropdown_class}",
                    for item in props.dropdown_items.iter() {
                        li {
                            key: "{item.id}",
                            style: props.dropdown_item_style,
                            class: "{props.dropdown_item_class}",
                            a { href: item.link, "{item.label}" }
                        }
                    }
                }
            }
        }
    }
}

/// Props for the `Navbar` component.
///
/// These props control the display, content, and styling of a responsive navigation bar.
#[derive(Props, Clone, PartialEq)]
pub struct NavbarProps {
    /// Top-level menu items for the main navigation bar.
    #[props(default)]
    pub menus: Vec<Menu>,

    /// Items for the profile dropdown menu.
    #[props(default)]
    pub dropdown_items: Vec<DropdownItem>,

    /// Items shown in a mega menu panel.
    #[props(default)]
    pub mega_menu_items: Vec<MegaMenuItem>,

    /// Whether to display the search input field.
    #[props(default)]
    pub show_search: bool,

    /// Whether to show the mega menu.
    #[props(default)]
    pub show_mega_menu: bool,

    /// Whether to show the profile menu (avatar or dropdown).
    #[props(default)]
    pub show_profile_menu: bool,

    /// Text for the main call-to-action button.
    #[props(default = "Sign up")]
    pub button_text: &'static str,

    /// Href for the main call-to-action button.
    #[props(default = "#")]
    pub button_href: &'static str,

    /// Source path for the logo image.
    #[props(default = "/assets/logo.webp")]
    pub logo_src: &'static str,

    /// Alternative text for the logo image.
    #[props(default = "logo")]
    pub logo_alt: &'static str,

    /// Link the logo should redirect to when clicked.
    #[props(default = "/")]
    pub logo_link: &'static str,

    /// Target attribute for the CTA button link.
    #[props(default = "_blank")]
    pub button_target: &'static str,

    /// Placeholder text for the search input.
    #[props(default = "Search...")]
    pub search_placeholder: &'static str,

    /// Text label for the profile menu button/avatar.
    #[props(default = "Profile")]
    pub profile_button_text: &'static str,

    // Styles
    /// Style for the main navbar container.
    #[props(
        default = "display: flex; align-items: center; justify-content: space-between; padding: 1rem; background-color: #fff;"
    )]
    pub navbar_style: &'static str,

    /// Style for the inner container within the navbar.
    #[props(default = "display: flex; align-items: center; width: 100%;")]
    pub inner_style: &'static str,

    /// Style for the maximum width container wrapping all navbar elements.
    #[props(
        default = "max-width: 1200px; margin: auto; display: flex; width: 100%; align-items: center; justify-content: space-between;"
    )]
    pub container_style: &'static str,

    /// Style applied to the logo image.
    #[props(default = "height: 40px;")]
    pub logo_style: &'static str,

    /// Style for the hamburger menu button.
    #[props(default = "display: flex; flex-direction: column; gap: 5px; cursor: pointer;")]
    pub menu_toggle_style: &'static str,

    /// Style for the lines in the hamburger icon.
    #[props(default = "width: 25px; height: 2px; background: black;")]
    pub line_style: &'static str,

    /// Style applied to the `<ul>` navigation container.
    #[props(default = "display: flex; gap: 1rem; list-style: none; margin: 0; padding: 0;")]
    pub nav_style: &'static str,

    /// Style for individual `<li>` items in the navbar.
    #[props(
        default = "text-decoration: none; color: black; padding: 0.5rem 1rem; transition: background 0.3s ease;"
    )]
    pub menu_item_style: &'static str,

    /// Style for the dropdown menu panel.
    #[props(
        default = "position: absolute; top: 100%; left: 0; background: white; box-shadow: 0 4px 8px rgba(0,0,0,0.1); z-index: 1000;"
    )]
    pub dropdown_style: &'static str,

    /// Style for each dropdown menu item.
    #[props(default = "padding: 0.5rem 1rem; white-space: nowrap;")]
    pub dropdown_item_style: &'static str,

    /// Style applied to the search input field.
    #[props(default = "padding: 0.5rem; font-size: 1rem; border: 1px solid #ccc;")]
    pub search_input_style: &'static str,

    /// Style for the CTA button wrapper.
    #[props(default = "margin-left: 1rem; white-space: nowrap;")]
    pub button_style: &'static str,

    /// Style for the `<a>` link inside the CTA button.
    #[props(
        default = "text-decoration: none; color: white; background: #007bff; padding: 0.5rem 1rem; border-radius: 4px;"
    )]
    pub button_link_style: &'static str,

    /// Style for the mega menu dropdown.
    #[props(
        default = "position: absolute; top: 100%; left: 0; background: white; padding: 0; margin-top: 0.5rem; z-index: 1000;"
    )]
    pub mega_menu_style: &'static str,

    /// Style for each card/item in the mega menu.
    #[props(
        default = "background: white; display: flex; gap: 2rem; padding: 1rem; box-shadow: 0 2px 8px rgba(0,0,0,0.1);"
    )]
    pub mega_menu_card_style: &'static str,

    /// Text for the "More" button shown in the navbar.
    #[props(default = "More")]
    pub more_button_text: &'static str,

    /// Style applied to the "More" button element.
    #[props(
        default = "background: transparent; border: none; cursor: pointer; font-weight: bold;"
    )]
    pub more_button_style: &'static str,

    // State and image props
    /// Optional external state for the search input value.
    #[props(default)]
    pub search_state: Option<Signal<String>>,

    /// Optional profile image URL for the avatar in the navbar.
    #[props(default)]
    pub profile_image_url: Option<String>,
    /// CSS class for the outer `<nav>` element of the navbar.
    ///
    /// Allows customization of the outermost wrapper of the navbar. Defaults to an empty string.
    #[props(default)]
    pub navbar_class: &'static str,

    /// CSS class for the inner wrapper of the navbar content.
    ///
    /// Used to apply styles to the inner layout container that holds all navbar elements.
    #[props(default)]
    pub inner_class: &'static str,

    /// CSS class for the container that constrains navbar width.
    ///
    /// Useful for centering and limiting the maximum width of navbar contents.
    #[props(default)]
    pub container_class: &'static str,

    /// CSS class for the logo image element.
    ///
    /// Use this to style the logo, such as sizing or spacing.
    #[props(default)]
    pub logo_class: &'static str,

    /// CSS class for the menu toggle button (commonly the hamburger icon).
    ///
    /// Used in responsive design for toggling the navigation menu on smaller screens.
    #[props(default)]
    pub menu_toggle_class: &'static str,

    /// CSS class for individual lines inside the hamburger toggle button.
    ///
    /// Typically used to style each bar in the toggle icon.
    #[props(default)]
    pub line_class: &'static str,

    /// CSS class for the navigation list element.
    ///
    /// Styles the `<ul>` or equivalent container that holds menu items.
    #[props(default)]
    pub nav_class: &'static str,

    /// CSS class for individual menu item links.
    ///
    /// Use this to style each navigation link (e.g., padding, hover effects).
    #[props(default)]
    pub menu_item_class: &'static str,

    /// CSS class for the dropdown menu container.
    ///
    /// This class applies to the container holding dropdown items below a menu.
    #[props(default)]
    pub dropdown_class: &'static str,

    /// CSS class for individual dropdown menu items.
    ///
    /// Used to style each item within a dropdown menu.
    #[props(default)]
    pub dropdown_item_class: &'static str,

    /// CSS class for the search input element.
    ///
    /// Applies styles to the optional search bar input field.
    #[props(default)]
    pub search_input_class: &'static str,

    /// CSS class for the container wrapping the button.
    ///
    /// Use this to style the wrapper that holds the sign-up/login button.
    #[props(default)]
    pub button_class: &'static str,

    /// CSS class for the sign-up/login button link.
    ///
    /// Applies styles directly to the clickable `<a>` or button element.
    #[props(default)]
    pub button_link_class: &'static str,

    /// CSS class for the mega menu container.
    ///
    /// Used when the mega menu is shown—applies to the entire dropdown card area.
    #[props(default)]
    pub mega_menu_class: &'static str,

    /// CSS class for the cards within the mega menu.
    ///
    /// Applies to each item in the mega menu layout (e.g., a flex card).
    #[props(default)]
    pub mega_menu_card_class: &'static str,

    /// CSS class for the "More" button in the navbar.
    ///
    /// This button typically toggles more navigation options or links.
    #[props(default)]
    pub more_button_class: &'static str,
}

/// Navbar Component
///
/// A responsive and dynamic navigation bar component built with Dioxus, supporting features like
/// dropdowns, mega menus, mobile toggles, profile menus, and search inputs. The `Navbar` adapts
/// automatically to different screen sizes and includes interactive elements.
///
/// # Features
/// - **Responsive Behavior**:
///   Adapts based on screen width using a signal driven by `window.innerWidth()` with a resize listener.
///   - Mobile breakpoint: `<= 768px`
///
/// - **Dropdown & Mega Menu**:
///   - Profile dropdown toggled on click
///   - Mega menu shown on hover with mouse enter/leave detection
///
/// - **Search Support**:
///   A customizable search input field with shared state handling
///
/// - **Mobile Toggle Menu**:
///   Displays a slide-in or dropdown menu when the hamburger is clicked on smaller screens
///
/// - **CTA Button**:
///   Optional call-to-action button next to the menu
///
/// - **Event Management**:
///   Global document click listener closes open mobile menu automatically
///
/// # Examples
///
/// ## Basic Navbar
/// ```rust
/// use dioxus::prelude::*;
/// use navbar::dioxus::{Menu, Navbar};
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Navbar {
///             menus: vec![
///                 Menu { id: 1, link: "/", name: "Home", icon_start: None, icon_end: None },
///             ],
///         }
///     }
/// }
/// ```
///
/// ## Navbar with Search and Profile
/// ```rust
/// use dioxus::prelude::*;
/// use navbar::dioxus::{Menu, DropdownItem, Navbar};
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Navbar {
///             show_search: true,
///             search_placeholder: "Search here...",
///             show_profile_menu: true,
///             profile_button_text: "User",
///             dropdown_items: vec![
///                 DropdownItem { id: 1, link: "/account", label: "Account", icon: None },
///                 DropdownItem { id: 2, link: "/logout", label: "Logout", icon: None },
///             ],
///             menus: vec![
///                 Menu { id: 1, link: "/", name: "Dashboard", icon_start: None, icon_end: None },
///             ],
///         }
///     }
/// }
/// ```
///
/// ## Mega Menu Navbar
/// ```rust
/// use dioxus::prelude::*;
/// use navbar::dioxus::{Menu, MegaMenuItem, Navbar};
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         Navbar {
///             show_mega_menu: true,
///             more_button_text: "Explore",
///             mega_menu_items: vec![
///                 MegaMenuItem { title: "Docs", description: "Technical docs", link: "/docs" },
///                 MegaMenuItem { title: "Blog", description: "Latest updates", link: "/blog" },
///             ],
///             menus: vec![
///                 Menu { id: 1, link: "/", name: "Platform", icon_start: None, icon_end: None },
///             ],
///         }
///     }
/// }
/// ```
///
/// # Notes
/// - Search uses `use_signal` for reactive state, or accepts an external state via `search_state`.
/// - The component is composed of subcomponents: `NavbarMenu`, `MegaMenu`, `ProfileMenu`, etc.
/// - All sub-styles and class props allow fine-grained CSS control.
///
/// # See Also
/// - [MDN `<nav>` Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/nav)
#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let mut is_mobile = use_signal(|| {
        window()
            .and_then(|w| w.inner_width().ok())
            .and_then(|w| w.as_f64())
            .map(|w| w <= 768.0)
            .unwrap_or(false)
    });

    let mut is_mobile_menu_open = use_signal(|| false);
    let mut is_dropdown_open = use_signal(|| false);
    let mut is_mega_menu_open = use_signal(|| false);

    use_effect(move || {
        let closure = Closure::<dyn FnMut()>::wrap(Box::new(move || {
            let width = window()
                .and_then(|w| w.inner_width().ok())
                .and_then(|w| w.as_f64())
                .unwrap_or(1024.0);
            is_mobile.set(width <= 768.0);
        }));

        if let Some(w) = window() {
            w.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
        }

        closure.forget();
    });

    use_effect(move || {
        let document = window().unwrap().document().unwrap();
        let listener = EventListener::new(&document, "click", move |_| {
            is_mobile_menu_open.set(false);
        });
        drop(listener)
    });

    let toggle_mobile_menu = {
        move |e: Event<MouseData>| {
            e.stop_propagation();
            is_mobile_menu_open.set(!is_mobile_menu_open());
        }
    };

    let toggle_dropdown = {
        move |e: Event<MouseData>| {
            e.stop_propagation();
            is_dropdown_open.set(!is_dropdown_open());
        }
    };

    let on_mouse_enter = { move |_| is_mega_menu_open.set(true) };

    let on_mouse_leave = { move |_| is_mega_menu_open.set(false) };

    let mut search_state = props.search_state.unwrap_or(use_signal(|| "".to_string()));
    let on_input = move |e: Event<FormData>| {
        search_state.set(e.value());
    };

    rsx! {
        nav {
            style: "{props.navbar_style}",
            class: "{props.navbar_class}",
            div {
                style: "{props.container_style}",
                class: "{props.container_class}",
                NavbarLogo {
                    logo_src: props.logo_src,
                    logo_alt: props.logo_alt,
                    logo_link: props.logo_link,
                    logo_style: props.logo_style,
                    logo_class: props.logo_class,
                }
                div {
                    style: "{props.inner_style}",
                    class: "{props.inner_class}",
                    if !is_mobile() {
                        NavbarMenu {
                            menus: props.menus.clone(),
                            menu_item_style: props.menu_item_style,
                            menu_item_class: props.menu_item_class,
                        }
                        if props.show_mega_menu {
                            div {
                                onmouseenter: on_mouse_enter,
                                style: "position: relative; display: inline-block;",
                                button {
                                    style: "{props.more_button_style}",
                                    class: "{props.more_button_class}",
                                    "{props.more_button_text}"
                                }
                                if is_mega_menu_open() {
                                    div {
                                        onmouseleave: on_mouse_leave,
                                        MegaMenu {
                                            items: props.mega_menu_items.clone(),
                                            wrapper_style: props.mega_menu_style,
                                            wrapper_class: props.mega_menu_class,
                                            card_style: props.mega_menu_card_style,
                                            card_class: props.mega_menu_card_class,
                                        }
                                    }
                                }
                            }
                        }
                        if props.show_search {
                            input {
                                r#type: "text",
                                placeholder: "{props.search_placeholder}",
                                style: "{props.search_input_style}",
                                class: "{props.search_input_class}",
                                value: "{search_state()}",
                                oninput: on_input,
                            }
                        }
                        if !props.button_text.is_empty() {
                            NavbarButton {
                                href: props.button_href,
                                text: props.button_text,
                                button_style: props.button_style,
                                button_class: props.button_class,
                                link_style: props.button_link_style,
                                link_class: props.button_link_class,
                                target: props.button_target,
                            }
                        }
                        if props.show_profile_menu {
                            ProfileMenu {
                                profile_image_url: props.profile_image_url.clone(),
                                items: props.dropdown_items.clone(),
                                dropdown_style: props.dropdown_style,
                                dropdown_class: props.dropdown_class,
                                item_style: props.dropdown_item_style,
                                item_class: props.dropdown_item_class,
                                is_open: is_dropdown_open(),
                                toggle: toggle_dropdown,
                                profile_text: props.profile_button_text,
                            }
                        }
                    }
                }
            }
            div {
                style: "position: relative; display: inline-block;",
                if is_mobile() {
                    NavbarToggle {
                        onclick: toggle_mobile_menu,
                        toggle_style: props.menu_toggle_style,
                        toggle_class: props.menu_toggle_class,
                        line_style: props.line_style,
                        line_class: props.line_class,
                    }
                }
                if is_mobile() && is_mobile_menu_open() {
                    MobileMenu {
                        menus: props.menus.clone(),
                        menu_item_style: props.menu_item_style,
                        menu_item_class: props.menu_item_class,
                        dropdown_items: props.dropdown_items.clone(),
                        dropdown_style: props.dropdown_style,
                        dropdown_class: props.dropdown_class,
                        dropdown_item_style: props.dropdown_item_style,
                        dropdown_item_class: props.dropdown_item_class,
                        show_profile_menu: props.show_profile_menu,
                        show_search: props.show_search,
                        search_input_style: props.search_input_style,
                        search_input_class: props.search_input_class,
                        search_placeholder: props.search_placeholder,
                    }
                }
            }
        }
    }
}
