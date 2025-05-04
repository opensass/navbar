#![doc = include_str!("../YEW.md")]

use gloo::events::EventListener;
use web_sys::HtmlInputElement;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;
use yew::prelude::*;
/// Represents a standard menu item in the navigation bar.
///
/// This struct is used for rendering main navigation links.
#[derive(Clone, PartialEq)]
pub struct Menu {
    /// Unique identifier for the menu item.
    pub id: usize,

    /// The destination URL when the menu item is clicked.
    pub link: &'static str,

    /// The display name of the menu item.
    pub name: &'static str,

    /// Optional icon displayed before the name.
    pub icon_start: Option<Html>,

    /// Optional icon displayed after the name.
    pub icon_end: Option<Html>,
}

/// Represents an item in a dropdown menu.
///
/// Used for profile or secondary dropdowns in the navbar.
#[derive(Clone, PartialEq)]
pub struct DropdownItem {
    /// Unique identifier for the dropdown item.
    pub id: usize,

    /// Optional icon displayed before the label.
    pub icon: Option<Html>,

    /// The label/text for the dropdown item.
    pub label: &'static str,

    /// The destination URL for the item.
    pub link: &'static str,
}

/// Represents an item in a mega menu.
///
/// Used for showcasing grouped or featured links in a grid.
#[derive(Clone, PartialEq)]
pub struct MegaMenuItem {
    /// The title or name of the item.
    pub title: &'static str,

    /// A short description explaining the item.
    pub description: &'static str,

    /// The link URL associated with the item.
    pub link: &'static str,
}

/// Props for the `Navbar` component.
///
/// These props control the display, content, and styling of a responsive navigation bar.
#[derive(Properties, Clone, PartialEq)]
pub struct NavbarProps {
    /// Top-level menu items for the main navigation bar.
    #[prop_or_default]
    pub menus: Vec<Menu>,

    /// Items for the profile dropdown menu.
    #[prop_or_default]
    pub dropdown_items: Vec<DropdownItem>,

    /// Items shown in a mega menu panel.
    #[prop_or_default]
    pub mega_menu_items: Vec<MegaMenuItem>,

    /// Whether to display the search input field.
    #[prop_or_default]
    pub show_search: bool,

    /// Whether to show the mega menu.
    #[prop_or_default]
    pub show_mega_menu: bool,

    /// Whether to show the profile menu (avatar or dropdown).
    #[prop_or_default]
    pub show_profile_menu: bool,

    /// Text for the main call-to-action button.
    #[prop_or("Sign up")]
    pub button_text: &'static str,

    /// Href for the main call-to-action button.
    #[prop_or("#")]
    pub button_href: &'static str,

    /// Source path for the logo image.
    #[prop_or("/assets/logo.webp")]
    pub logo_src: &'static str,

    /// Alternative text for the logo image.
    #[prop_or("logo")]
    pub logo_alt: &'static str,

    /// Link the logo should redirect to when clicked.
    #[prop_or("/")]
    pub logo_link: &'static str,

    /// Target attribute for the CTA button link.
    #[prop_or("_blank")]
    pub button_target: &'static str,

    /// Placeholder text for the search input.
    #[prop_or("Search...")]
    pub search_placeholder: &'static str,

    /// Text label for the profile menu button/avatar.
    #[prop_or("Profile")]
    pub profile_button_text: &'static str,

    // Styles
    /// Style for the main navbar container.
    #[prop_or(
        "display: flex; align-items: center; justify-content: space-between; padding: 1rem; background-color: #fff;"
    )]
    pub navbar_style: &'static str,

    /// Style for the inner container within the navbar.
    #[prop_or("display: flex; align-items: center; width: 100%;")]
    pub inner_style: &'static str,

    /// Style for the maximum width container wrapping all navbar elements.
    #[prop_or(
        "max-width: 1200px; margin: auto; display: flex; width: 100%; align-items: center; justify-content: space-between;"
    )]
    pub container_style: &'static str,

    /// Style applied to the logo image.
    #[prop_or("height: 40px;")]
    pub logo_style: &'static str,

    /// Style for the hamburger menu button.
    #[prop_or("display: flex; flex-direction: column; gap: 5px; cursor: pointer;")]
    pub menu_toggle_style: &'static str,

    /// Style for the lines in the hamburger icon.
    #[prop_or("width: 25px; height: 2px; background: black;")]
    pub line_style: &'static str,

    /// Style applied to the `<ul>` navigation container.
    #[prop_or("display: flex; gap: 1rem; list-style: none; margin: 0; padding: 0;")]
    pub nav_style: &'static str,

    /// Style for individual `<li>` items in the navbar.
    #[prop_or(
        "text-decoration: none; color: black; padding: 0.5rem 1rem; transition: background 0.3s ease;"
    )]
    pub menu_item_style: &'static str,

    /// Style for the dropdown menu panel.
    #[prop_or(
        "position: absolute; top: 100%; left: 0; background: white; box-shadow: 0 4px 8px rgba(0,0,0,0.1); z-index: 1000;"
    )]
    pub dropdown_style: &'static str,

    /// Style for each dropdown menu item.
    #[prop_or("padding: 0.5rem 1rem; white-space: nowrap;")]
    pub dropdown_item_style: &'static str,

    /// Style applied to the search input field.
    #[prop_or("padding: 0.5rem; font-size: 1rem; border: 1px solid #ccc;")]
    pub search_input_style: &'static str,

    /// Style for the CTA button wrapper.
    #[prop_or("margin-left: 1rem; white-space: nowrap;")]
    pub button_style: &'static str,

    /// Style for the `<a>` link inside the CTA button.
    #[prop_or(
        "text-decoration: none; color: white; background: #007bff; padding: 0.5rem 1rem; border-radius: 4px;"
    )]
    pub button_link_style: &'static str,

    /// Style for the mega menu dropdown.
    #[prop_or(
        "position: absolute; top: 100%; left: 0; background: white; padding: 0; margin-top: 0.5rem; z-index: 1000;"
    )]
    pub mega_menu_style: &'static str,

    /// Style for each card/item in the mega menu.
    #[prop_or(
        "background: white; display: flex; gap: 2rem; padding: 1rem; box-shadow: 0 2px 8px rgba(0,0,0,0.1);"
    )]
    pub mega_menu_card_style: &'static str,

    /// Text for the "More" button shown in the navbar.
    #[prop_or("More")]
    pub more_button_text: &'static str,

    /// Style applied to the "More" button element.
    #[prop_or("background: transparent; border: none; cursor: pointer; font-weight: bold;")]
    pub more_button_style: &'static str,

    // State and image props
    /// Optional external state for the search input value.
    #[prop_or_default]
    pub search_state: Option<UseStateHandle<String>>,

    /// Optional profile image URL for the avatar in the navbar.
    #[prop_or_default]
    pub profile_image_url: Option<String>,
    /// CSS class for the outer `<nav>` element of the navbar.
    ///
    /// Allows customization of the outermost wrapper of the navbar. Defaults to an empty string.
    #[prop_or_default]
    pub navbar_class: &'static str,

    /// CSS class for the inner wrapper of the navbar content.
    ///
    /// Used to apply styles to the inner layout container that holds all navbar elements.
    #[prop_or_default]
    pub inner_class: &'static str,

    /// CSS class for the container that constrains navbar width.
    ///
    /// Useful for centering and limiting the maximum width of navbar contents.
    #[prop_or_default]
    pub container_class: &'static str,

    /// CSS class for the logo image element.
    ///
    /// Use this to style the logo, such as sizing or spacing.
    #[prop_or_default]
    pub logo_class: &'static str,

    /// CSS class for the menu toggle button (commonly the hamburger icon).
    ///
    /// Used in responsive design for toggling the navigation menu on smaller screens.
    #[prop_or_default]
    pub menu_toggle_class: &'static str,

    /// CSS class for individual lines inside the hamburger toggle button.
    ///
    /// Typically used to style each bar in the toggle icon.
    #[prop_or_default]
    pub line_class: &'static str,

    /// CSS class for the navigation list element.
    ///
    /// Styles the `<ul>` or equivalent container that holds menu items.
    #[prop_or_default]
    pub nav_class: &'static str,

    /// CSS class for individual menu item links.
    ///
    /// Use this to style each navigation link (e.g., padding, hover effects).
    #[prop_or_default]
    pub menu_item_class: &'static str,

    /// CSS class for the dropdown menu container.
    ///
    /// This class applies to the container holding dropdown items below a menu.
    #[prop_or_default]
    pub dropdown_class: &'static str,

    /// CSS class for individual dropdown menu items.
    ///
    /// Used to style each item within a dropdown menu.
    #[prop_or_default]
    pub dropdown_item_class: &'static str,

    /// CSS class for the search input element.
    ///
    /// Applies styles to the optional search bar input field.
    #[prop_or_default]
    pub search_input_class: &'static str,

    /// CSS class for the container wrapping the button.
    ///
    /// Use this to style the wrapper that holds the sign-up/login button.
    #[prop_or_default]
    pub button_class: &'static str,

    /// CSS class for the sign-up/login button link.
    ///
    /// Applies styles directly to the clickable `<a>` or button element.
    #[prop_or_default]
    pub button_link_class: &'static str,

    /// CSS class for the mega menu container.
    ///
    /// Used when the mega menu is shown—applies to the entire dropdown card area.
    #[prop_or_default]
    pub mega_menu_class: &'static str,

    /// CSS class for the cards within the mega menu.
    ///
    /// Applies to each item in the mega menu layout (e.g., a flex card).
    #[prop_or_default]
    pub mega_menu_card_class: &'static str,

    /// CSS class for the "More" button in the navbar.
    ///
    /// This button typically toggles more navigation options or links.
    #[prop_or_default]
    pub more_button_class: &'static str,
}

/// Navbar Component
///
/// A responsive and interactive navigation bar component built using Yew. It adapts to screen size
/// changes and supports features such as a mobile toggle menu, dropdowns, mega menu, search input,
/// and profile menu. The component is highly configurable through styles and class properties.
///
/// # Features
///
/// - **Responsive Design**:
///   - Automatically switches to mobile view if `window.innerWidth() <= 768`.
///   - A `resize` event listener dynamically updates the layout state.
///
/// - **Mobile Menu Toggle**:
///   - A hamburger icon appears on smaller screens.
///   - Opens a collapsible menu containing navigation links, search bar, and profile dropdown (if enabled).
///   - Global click event listener closes the mobile menu when clicking outside.
///
/// - **Dropdown Menu**:
///   - Profile dropdown toggled on user click.
///   - Can be styled via dropdown props.
///
/// - **Mega Menu**:
///   - Shown on mouse hover when `show_mega_menu` is true.
///   - Items appear in a grid-style layout under the "More" button.
///
/// - **Search Input**:
///   - Optional text input for searching.
///   - Supports local state or controlled component mode via `search_state`.
///
/// - **Call-to-Action Button**:
///   - An optional button beside the menu that links to an external/internal page.
///
/// # Examples
///
/// ## Basic Navbar
/// ```rust
/// use yew::prelude::*;
/// use navbar::yew::{Menu, MegaMenuItem, Navbar};
///
/// #[function_component]
/// fn App() -> Html {
///     html! {
///         <Navbar menus={vec![
///             Menu { id: 1, link: "/home", name: "Home", icon_start: None, icon_end: None },
///         ]} />
///     }
/// }
/// ```
///
/// ## With Profile Menu and Search
/// ```rust
/// use yew::prelude::*;
/// use navbar::yew::{Menu, MegaMenuItem, Navbar, DropdownItem};
///
/// #[function_component]
/// fn App() -> Html {
///     html! {
///         <Navbar
///             show_search={true}
///             search_placeholder={"Search..."}
///             show_profile_menu={true}
///             profile_button_text={"Profile"}
///             dropdown_items={vec![
///                 DropdownItem { id: 1, link: "/settings", label: "Settings", icon: None },
///                 DropdownItem { id: 2, link: "/logout", label: "Logout", icon: None },
///             ]}
///             menus={vec![
///                 Menu { id: 1, link: "/dashboard", name: "Dashboard", icon_start: None, icon_end: None },
///             ]}
///         />
///     }
/// }
/// ```
///
/// ## With Mega Menu
/// ```rust
/// use yew::prelude::*;
/// use navbar::yew::{Menu, MegaMenuItem, Navbar};
///
/// #[function_component]
/// fn App() -> Html {
///     html! {
///         <Navbar
///             show_mega_menu={true}
///             more_button_text={"Explore"}
///             mega_menu_items={vec![
///                 MegaMenuItem { title: "Docs", description: "Learn more about our APIs", link: "/docs" },
///                 MegaMenuItem { title: "Blog", description: "See what we're up to", link: "/blog" },
///             ]}
///             menus={vec![
///                 Menu { id: 1, link: "/platform", name: "Platform", icon_start: None, icon_end: None },
///             ]}
///         />
///     }
/// }
/// ```
///
/// # Notes
/// - The component uses `use_state` for responsiveness and open/close behaviors.
/// - Subcomponents include `NavbarMenu`, `MegaMenu`, `ProfileMenu`, `MobileMenu`, and `NavbarToggle`.
/// - This component is customizable and works well with Tailwind CSS or other utility-first CSS frameworks.
///
/// # See Also
/// - [MDN `<nav>` Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/nav)
#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let is_mobile = use_state(|| {
        let width = web_sys::window()
            .unwrap()
            .inner_width()
            .unwrap()
            .as_f64()
            .unwrap();
        width <= 768.0
    });

    let is_mobile_menu_open = use_state(|| false);
    let is_dropdown_open = use_state(|| false);
    let is_mega_menu_open = use_state(|| false);

    {
        let is_mobile = is_mobile.clone();
        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move || {
                let width = web_sys::window()
                    .unwrap()
                    .inner_width()
                    .unwrap()
                    .as_f64()
                    .unwrap();
                is_mobile.set(width <= 768.0);
            }) as Box<dyn Fn()>);

            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();

            closure.forget();
            || ()
        });
    }

    {
        let is_mobile_menu_open = is_mobile_menu_open.clone();
        use_effect(move || {
            let document = web_sys::window().unwrap().document().unwrap();
            let listener = EventListener::new(&document, "click", move |_| {
                is_mobile_menu_open.set(false);
            });
            || drop(listener)
        });
    }

    let toggle_mobile_menu = {
        let state = is_mobile_menu_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            state.set(!*state)
        })
    };

    let toggle_dropdown = {
        let state = is_dropdown_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            state.set(!*state)
        })
    };

    let on_mouse_enter = {
        let state = is_mega_menu_open.clone();
        Callback::from(move |_| state.set(true))
    };

    let on_mouse_leave = {
        let state = is_mega_menu_open.clone();
        Callback::from(move |_| state.set(false))
    };
    let search_state = props
        .search_state
        .clone()
        .unwrap_or(use_state(|| "".to_string()));
    let search_val = (*search_state).clone();
    let on_input = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        search_state.set(input.value());
    });
    html! {
        <nav style={props.navbar_style} class={props.navbar_class}>
            <div style={props.container_style} class={props.container_class}>
                <NavbarLogo
                    logo_src={props.logo_src}
                    logo_alt={props.logo_alt}
                    logo_link={props.logo_link}
                    logo_style={props.logo_style}
                    logo_class={props.logo_class}
                />
                <div style={props.inner_style} class={props.inner_class}>
                    if !*is_mobile {
                        <NavbarMenu
                            menus={props.menus.clone()}
                            menu_item_style={props.menu_item_style}
                            menu_item_class={props.menu_item_class}
                        />
                        if props.show_mega_menu {
                            <div
                                onmouseenter={on_mouse_enter.clone()}
                                style="position: relative; display: inline-block;"
                            >
                                <button
                                    style={props.more_button_style}
                                    class={props.more_button_class}
                                >
                                    { props.more_button_text }
                                </button>
                                if *is_mega_menu_open {
                                    <div onmouseleave={on_mouse_leave.clone()}>
                                        <MegaMenu
                                            items={props.mega_menu_items.clone()}
                                            wrapper_style={props.mega_menu_style}
                                            wrapper_class={props.mega_menu_class}
                                            card_style={props.mega_menu_card_style}
                                            card_class={props.mega_menu_card_class}
                                        />
                                    </div>
                                }
                            </div>
                        }
                        if props.show_search {
                            if let Some(_search_state) = &props.search_state {
                                <input
                                    type="text"
                                    placeholder={props.search_placeholder}
                                    style={props.search_input_style}
                                    class={props.search_input_class}
                                    value={search_val}
                                    oninput={on_input.clone()}
                                />
                            } else {
                                <input
                                    type="text"
                                    placeholder={props.search_placeholder}
                                    style={props.search_input_style}
                                    class={props.search_input_class}
                                />
                            }
                        }
                        if !props.button_text.is_empty() {
                            <NavbarButton
                                href={props.button_href}
                                text={props.button_text}
                                button_style={props.button_style}
                                button_class={props.button_class}
                                link_style={props.button_link_style}
                                link_class={props.button_link_class}
                                target={props.button_target}
                            />
                        }
                        if props.show_profile_menu {
                            <ProfileMenu
                                profile_image_url={props.profile_image_url.clone()}
                                items={props.dropdown_items.clone()}
                                dropdown_style={props.dropdown_style}
                                dropdown_class={props.dropdown_class}
                                item_style={props.dropdown_item_style}
                                item_class={props.dropdown_item_class}
                                is_open={*is_dropdown_open}
                                toggle={toggle_dropdown.clone()}
                                profile_text={props.profile_button_text}
                            />
                        }
                    }
                </div>
            </div>
            <div style="position: relative; display: inline-block;">
                if *is_mobile {
                    <NavbarToggle
                        onclick={toggle_mobile_menu.clone()}
                        toggle_style={props.menu_toggle_style}
                        toggle_class={props.menu_toggle_class}
                        line_style={props.line_style}
                        line_class={props.line_class}
                    />
                }
                if *is_mobile && *is_mobile_menu_open {
                    <MobileMenu
                        menus={props.menus.clone()}
                        menu_item_style={props.menu_item_style}
                        menu_item_class={props.menu_item_class}
                        dropdown_items={props.dropdown_items.clone()}
                        dropdown_style={props.dropdown_style}
                        dropdown_class={props.dropdown_class}
                        dropdown_item_style={props.dropdown_item_style}
                        dropdown_item_class={props.dropdown_item_class}
                        show_profile_menu={props.show_profile_menu}
                        show_search={props.show_search}
                        search_input_style={props.search_input_style}
                        search_input_class={props.search_input_class}
                        search_placeholder={props.search_placeholder}
                    />
                }
            </div>
        </nav>
    }
}

/// Properties for rendering a logo inside a navigation bar.
#[derive(Properties, PartialEq)]
pub struct LogoProps {
    /// Path to the logo image.
    #[prop_or("/logo.png")]
    pub logo_src: &'static str,
    /// Alternative text for the logo image.
    #[prop_or("Logo")]
    pub logo_alt: &'static str,
    /// URL to redirect when the logo is clicked.
    #[prop_or("/")]
    pub logo_link: &'static str,
    /// Inline CSS style for the logo image.
    #[prop_or("height: 40px;")]
    pub logo_style: &'static str,
    /// Inline CSS style for the link wrapping the logo.
    #[prop_or("display: inline-block;")]
    pub link_style: &'static str,
    /// Optional class name for the logo image.
    #[prop_or_default]
    pub logo_class: String,
    /// Optional class name for the anchor tag wrapping the logo.
    #[prop_or_default]
    pub link_class: String,
}

#[function_component(NavbarLogo)]
fn navbar_logo(props: &LogoProps) -> Html {
    html! {
        <a href={props.logo_link} style={props.link_style} class={props.link_class.clone()}>
            <img
                src={props.logo_src}
                alt={props.logo_alt}
                style={props.logo_style}
                class={props.logo_class.clone()}
            />
        </a>
    }
}

/// Properties for rendering a hamburger toggle icon (used in responsive menus).
#[derive(Properties, PartialEq)]
pub struct ToggleProps {
    /// Callback triggered when the toggle is clicked.
    pub onclick: Callback<MouseEvent>,
    /// Inline style for the toggle container.
    #[prop_or("display: flex; flex-direction: column; gap: 5px; cursor: pointer;")]
    pub toggle_style: &'static str,
    /// Inline style for each line/bar in the hamburger icon.
    #[prop_or("width: 25px; height: 3px; background: black;")]
    pub line_style: &'static str,
    /// Optional class for the toggle container.
    #[prop_or_default]
    pub toggle_class: String,
    /// Optional class for each line in the icon.
    #[prop_or_default]
    pub line_class: String,
}

#[function_component(NavbarToggle)]
fn navbar_toggle(props: &ToggleProps) -> Html {
    html! {
        <div
            style={props.toggle_style}
            class={props.toggle_class.clone()}
            onclick={props.onclick.clone()}
            tabindex="0"
            role="button"
            aria-label="Toggle Menu"
        >
            <div style={props.line_style} class={props.line_class.clone()} />
            <div style={props.line_style} class={props.line_class.clone()} />
            <div style={props.line_style} class={props.line_class.clone()} />
        </div>
    }
}

/// Properties for rendering a horizontal navigation menu.
#[derive(Properties, PartialEq)]
pub struct MenuProps {
    /// List of menu items to display.
    pub menus: Vec<Menu>,
    /// Inline style for each menu item anchor tag.
    #[prop_or("padding: 0.5rem 1rem; text-decoration: none; color: black;")]
    pub menu_item_style: &'static str,
    /// Inline style for the entire list container.
    #[prop_or("display: flex; gap: 1rem; list-style: none; margin: 0; padding: 0;")]
    pub list_style: &'static str,
    /// Optional class for each menu item.
    #[prop_or_default]
    pub menu_item_class: String,
    /// Optional class for the list container.
    #[prop_or_default]
    pub list_class: String,
}

#[function_component(NavbarMenu)]
fn navbar_menu(props: &MenuProps) -> Html {
    html! {
        <ul style={props.list_style} class={props.list_class.clone()}>
            { for props.menus.iter().map(|m| html! {
                <li key={m.id}>
                    <a href={m.link} style={props.menu_item_style} class={props.menu_item_class.clone()}>
                        { m.icon_start.clone().unwrap_or_default() }
                        { m.name }
                        { m.icon_end.clone().unwrap_or_default() }
                    </a>
                </li>
            }) }
        </ul>
    }
}

/// Properties for rendering a styled button with a link.
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    /// The URL the button should navigate to.
    #[prop_or("#")]
    pub href: &'static str,
    /// Text displayed inside the button.
    #[prop_or("Click Me")]
    pub text: &'static str,
    /// Style for the outer div wrapping the button.
    #[prop_or("padding: 0.5rem 1rem; background-color: #007bff; border-radius: 4px;")]
    pub button_style: &'static str,
    /// Style applied to the anchor inside the button.
    #[prop_or("color: white; text-decoration: none;")]
    pub link_style: &'static str,
    /// Target attribute for the anchor link (e.g. `_blank`, `_self`).
    #[prop_or("_self")]
    pub target: &'static str,
    /// Optional class for the outer button div.
    #[prop_or_default]
    pub button_class: String,
    /// Optional class for the inner anchor tag.
    #[prop_or_default]
    pub link_class: String,
}

#[function_component(NavbarButton)]
fn navbar_button(props: &ButtonProps) -> Html {
    html! {
        <div style={props.button_style} class={props.button_class.clone()}>
            <a
                href={props.href}
                target={props.target}
                style={props.link_style}
                class={props.link_class.clone()}
            >
                { props.text }
            </a>
        </div>
    }
}

/// Properties for rendering a user profile menu dropdown.
#[derive(Properties, PartialEq)]
pub struct ProfileMenuProps {
    /// List of dropdown items in the profile menu.
    pub items: Vec<DropdownItem>,
    /// Style for the dropdown container.
    #[prop_or(
        "position: absolute; top: 100%; left: 0; background: white; box-shadow: 0 4px 8px rgba(0,0,0,0.1); z-index: 1000;"
    )]
    pub dropdown_style: &'static str,
    /// Style for each item in the dropdown.
    #[prop_or("padding: 0.5rem 1rem; white-space: nowrap;")]
    pub item_style: &'static str,
    /// Style for the wrapper around the profile button.
    #[prop_or("position: relative; display: inline-block; margin: 0 0 0 1rem;")]
    pub wrapper_style: &'static str,
    /// Style for the clickable profile button.
    #[prop_or("background: none; border: none; cursor: pointer;")]
    pub button_style: &'static str,
    /// Style for the avatar image.
    #[prop_or("width: 40px; height: 40px; border-radius: 50%; object-fit: cover;")]
    pub avatar_img_style: &'static str,
    /// Style for fallback avatar (e.g., initials).
    #[prop_or(
        "width: 40px; height: 40px; border-radius: 50%; background-color: #ccc; color: #fff; display: flex; align-items: center; justify-content: center; font-weight: bold;"
    )]
    pub avatar_fallback_style: &'static str,
    /// Displayed if avatar image is not available.
    #[prop_or("Profile")]
    pub profile_text: &'static str,
    /// Optional URL for the avatar image.
    #[prop_or_default]
    pub profile_image_url: Option<String>,
    /// Indicates if the dropdown is currently open.
    pub is_open: bool,
    /// Callback to toggle the dropdown menu.
    pub toggle: Callback<MouseEvent>,
    /// Class name for the dropdown container.
    #[prop_or_default]
    pub dropdown_class: String,
    /// Class name for dropdown items.
    #[prop_or_default]
    pub item_class: String,
    /// Class name for wrapper element.
    #[prop_or_default]
    pub wrapper_class: String,
    /// Class name for avatar image/fallback.
    #[prop_or_default]
    pub avatar_class: String,
    /// Class name for the button.
    #[prop_or_default]
    pub button_class: String,
}

#[function_component(ProfileMenu)]
fn profile_menu(props: &ProfileMenuProps) -> Html {
    html! {
        <div style={props.wrapper_style} class={props.wrapper_class.clone()}>
            <button
                onclick={props.toggle.clone()}
                aria-haspopup="true"
                aria-expanded={props.is_open.to_string()}
                style={props.button_style}
                class={props.button_class.clone()}
            >
                { if let Some(url) = &props.profile_image_url {
                        html! {
                            <img
                                src={url.clone()}
                                alt="Profile"
                                style={props.avatar_img_style}
                                class={props.avatar_class.clone()}
                            />
                        }
                    } else {
                        let first_letter = props.profile_text.chars().next().unwrap_or('P');
                        html! {
                            <div
                                style={props.avatar_fallback_style}
                                class={props.avatar_class.clone()}
                            >
                                { first_letter }
                            </div>
                        }
                    } }
            </button>
            { if props.is_open {
                    html! {
                        <ul style={props.dropdown_style} class={props.dropdown_class.clone()}>
                            { for props.items.iter().map(|item| html! {
                                <li key={item.id} style={props.item_style} class={props.item_class.clone()}>
                                    <a href={item.link}>
                                        { item.icon.clone().unwrap_or_default() }
                                        { item.label }
                                    </a>
                                </li>
                            }) }
                        </ul>
                    }
                } else {
                    html! {}
                } }
        </div>
    }
}

/// Properties for rendering a complex mega menu layout.
#[derive(Properties, PartialEq)]
pub struct MegaMenuProps {
    /// Items displayed in the mega menu.
    pub items: Vec<MegaMenuItem>,
    /// Style for the mega menu wrapper.
    #[prop_or("position: absolute; top: 60px; left: 0; z-index: 999;")]
    pub wrapper_style: &'static str,
    /// Style for the inner card/container element.
    #[prop_or(
        "background: white; display: flex; gap: 2rem; padding: 1rem; box-shadow: 0 2px 8px rgba(0,0,0,0.1);"
    )]
    pub card_style: &'static str,
    /// Style for each link in the mega menu.
    #[prop_or("text-decoration: none; color: black;")]
    pub item_link_style: &'static str,
    /// Optional class for the wrapper.
    #[prop_or_default]
    pub wrapper_class: String,
    /// Optional class for the card.
    #[prop_or_default]
    pub card_class: String,
    /// Optional class for each menu item.
    #[prop_or_default]
    pub item_class: String,
}

#[function_component(MegaMenu)]
fn mega_menu(props: &MegaMenuProps) -> Html {
    html! {
        <div style={props.wrapper_style} class={props.wrapper_class.clone()}>
            <div style={props.card_style} class={props.card_class.clone()}>
                { for props.items.iter().map(|item| html! {
                    <a href={item.link} style={props.item_link_style} class={props.item_class.clone()}>
                        <div>
                            <p><strong>{ item.title }</strong></p>
                            <p>{ item.description }</p>
                        </div>
                    </a>
                }) }
            </div>
        </div>
    }
}

/// Properties for rendering a responsive mobile navigation menu.
#[derive(Properties, PartialEq)]
pub struct MobileMenuProps {
    /// Menu items to render.
    pub menus: Vec<Menu>,
    /// Style for individual menu items.
    #[prop_or("padding: 0.5rem 1rem;")]
    pub menu_item_style: &'static str,
    /// Dropdown items (usually profile-related).
    pub dropdown_items: Vec<DropdownItem>,
    /// Style for the dropdown container.
    #[prop_or("background: #f9f9f9; padding: 1rem; border-radius: 4px;")]
    pub dropdown_style: &'static str,
    /// Style for each dropdown item.
    #[prop_or("padding: 0.5rem;")]
    pub dropdown_item_style: &'static str,
    /// Whether to show profile menu.
    #[prop_or(false)]
    pub show_profile_menu: bool,
    /// Whether to show search input.
    #[prop_or(false)]
    pub show_search: bool,
    /// Style for the search input field.
    #[prop_or("padding: 0.5rem; width: 100%; border: 1px solid #ccc;")]
    pub search_input_style: &'static str,
    /// Placeholder text for the search input.
    #[prop_or("Search...")]
    pub search_placeholder: &'static str,
    /// Style for the mobile menu container.
    #[prop_or(
        "display: flex; flex-direction: column; gap: 1rem; padding: 1rem; position: absolute; top: 100%; left: 0; background: white; z-index: 999; width: max-content;"
    )]
    pub container_style: &'static str,
    /// Optional class for container.
    #[prop_or_default]
    pub container_class: String,
    /// Optional class for menu items.
    #[prop_or_default]
    pub menu_item_class: String,
    /// Optional class for dropdown.
    #[prop_or_default]
    pub dropdown_class: String,
    /// Optional class for dropdown items.
    #[prop_or_default]
    pub dropdown_item_class: String,
    /// Optional class for search input.
    #[prop_or_default]
    pub search_input_class: String,
}

#[function_component(MobileMenu)]
fn mobile_menu(props: &MobileMenuProps) -> Html {
    html! {
        <div style={props.container_style} class={props.container_class.clone()}>
            <NavbarMenu
                menus={props.menus.clone()}
                menu_item_style={props.menu_item_style}
                menu_item_class={props.menu_item_class.clone()}
                list_style=""
                list_class=""
            />
            if props.show_search {
                <input
                    type="text"
                    style={props.search_input_style}
                    placeholder={props.search_placeholder}
                    class={props.search_input_class.clone()}
                />
            }
            if props.show_profile_menu {
                <ul style={props.dropdown_style} class={props.dropdown_class.clone()}>
                    { for props.dropdown_items.iter().map(|item| html! {
                        <li key={item.id} style={props.dropdown_item_style} class={props.dropdown_item_class.clone()}>
                            <a href={item.link}>{ item.label }</a>
                        </li>
                    }) }
                </ul>
            }
        </div>
    }
}
