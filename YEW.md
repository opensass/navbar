# 🍔 Navbar Yew Usage

Adding the `Navbar` component to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Navbar component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add navbar --features=yew
   ```

1. Import the `Navbar` component into your Yew component and start using it in your app.

## 🛠️ Usage

Integrating the `Navbar` into your Yew application is simple. Here's how:

```rust
use yew::prelude::*;
use navbar::yew::{Navbar, Menu};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Navbar
            logo_src="/assets/logo.svg"
            logo_alt="My App"
            menus={vec![
                Menu { id: 1, link: "/", name: "Dashboard", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/reports", name: "Reports", icon_start: None, icon_end: None }
            ]}
            button_text="Sign Up"
            button_href="/signup"
            show_search=true
            show_mega_menu=true
            show_profile_menu=true
        />
    }
}
```

## 🧩 Props

### `Navbar` Component Props

#### Main Props

| Property              | Type                     | Description                                    | Default     |
| --------------------- | ------------------------ | ---------------------------------------------- | ----------- |
| `logo_src`            | `&'static str`           | Path to the logo image.                        | `""`        |
| `logo_alt`            | `&'static str`           | Alt text for the logo.                         | `"Logo"`    |
| `logo_link`           | `&'static str`           | Optional link for the logo.                    | `"/"`       |
| `menus`               | `Vec<MenuItem>`          | List of top-level menu items.                  | `[]`        |
| `show_search`         | `bool`                   | Displays the search input if `true`.           | `false`     |
| `search_state`        | `UseStateHandle<String>` | Optional shared state for the search input.    | `None`      |
| `search_placeholder`  | `&'static str`           | Placeholder for the search input.              | `"Search"`  |
| `button_text`         | `&'static str`           | Text for the CTA button.                       | `""`        |
| `button_href`         | `&'static str`           | Link for the CTA button.                       | `"#"`       |
| `button_target`       | `&'static str`           | Target attribute for CTA link (e.g. `_blank`). | `"_self"`   |
| `show_mega_menu`      | `bool`                   | Enables the mega menu when `true`.             | `false`     |
| `mega_menu_items`     | `Vec<MegaMenuItem>`      | Items to show in the mega menu.                | `[]`        |
| `show_profile_menu`   | `bool`                   | Shows the profile dropdown menu.               | `false`     |
| `dropdown_items`      | `Vec<DropdownItem>`      | Items for the profile dropdown.                | `[]`        |
| `profile_image_url`   | `&'static str`           | URL for profile image.                         | `""`        |
| `profile_button_text` | `&'static str`           | Text label for profile menu toggle.            | `"Profile"` |

#### Styling Props

```sh
+--------------------------------------------------------------------------+
|                                [Navbar]                                  |  <-- `navbar_class` & `navbar_style`
|                                                                          |
|   +--------------------------------------------------------------+       |  <-- `container_class` & `container_style`
|   | [Logo] [Menu Items] [Search] [CTA Button] [Profile Menu]     |       |
|   +--------------------------------------------------------------+       |
|                                                                          |
+--------------------------------------------------------------------------+
```

| Property               | Type           | Description                            | Default Style                                                |
| ---------------------- | -------------- | -------------------------------------- | ------------------------------------------------------------ |
| `navbar_class`         | `&'static str` | Class for outer navbar element.        | `""`                                                         |
| `navbar_style`         | `&'static str` | Style for outer navbar element.        | `display: flex; justify-content: space-between; ...`         |
| `container_class`      | `&'static str` | Class for max-width inner container.   | `""`                                                         |
| `container_style`      | `&'static str` | Style for inner container.             | `max-width: 1200px; margin: auto; ...`                       |
| `inner_class`          | `&'static str` | Class for the content wrapper.         | `""`                                                         |
| `inner_style`          | `&'static str` | Style for the content wrapper.         | `display: flex; align-items: center; ...`                    |
| `logo_class`           | `&'static str` | Class for the logo.                    | `""`                                                         |
| `logo_style`           | `&'static str` | Style for the logo.                    | `height: 40px;`                                              |
| `menu_item_class`      | `&'static str` | Class for menu items.                  | `""`                                                         |
| `menu_item_style`      | `&'static str` | Style for each menu item.              | `padding: 0.5rem 1rem; color: black;`                        |
| `dropdown_class`       | `&'static str` | Class for dropdown menu.               | `""`                                                         |
| `dropdown_style`       | `&'static str` | Style for dropdown menu.               | `position: absolute; box-shadow: 0 4px 8px rgba(0,0,0,0.1);` |
| `dropdown_item_class`  | `&'static str` | Class for dropdown items.              | `""`                                                         |
| `dropdown_item_style`  | `&'static str` | Style for dropdown items.              | `padding: 0.5rem 1rem;`                                      |
| `search_input_class`   | `&'static str` | Class for search input.                | `""`                                                         |
| `search_input_style`   | `&'static str` | Style for search input.                | `padding: 0.5rem; font-size: 1rem;`                          |
| `button_class`         | `&'static str` | Class for CTA button wrapper.          | `""`                                                         |
| `button_style`         | `&'static str` | Style for CTA button wrapper.          | `margin-left: 1rem;`                                         |
| `button_link_class`    | `&'static str` | Class for CTA anchor inside button.    | `""`                                                         |
| `button_link_style`    | `&'static str` | Style for CTA anchor inside button.    | `background: #007bff; color: white; ...`                     |
| `more_button_class`    | `&'static str` | Class for the mega menu "More" button. | `""`                                                         |
| `more_button_style`    | `&'static str` | Style for the mega menu "More" button. | `font-weight: bold; border: none;`                           |
| `mega_menu_class`      | `&'static str` | Class for mega menu wrapper.           | `""`                                                         |
| `mega_menu_style`      | `&'static str` | Style for mega menu wrapper.           | `position: absolute; padding: 0;`                            |
| `mega_menu_card_class` | `&'static str` | Class for each mega menu card.         | `""`                                                         |
| `mega_menu_card_style` | `&'static str` | Style for each mega menu card.         | `background: white; display: flex; ...`                      |
| `menu_toggle_class`    | `&'static str` | Class for mobile hamburger icon.       | `""`                                                         |
| `menu_toggle_style`    | `&'static str` | Style for mobile hamburger icon.       | `flex-direction: column; gap: 5px;`                          |
| `line_class`           | `&'static str` | Class for hamburger icon lines.        | `""`                                                         |
| `line_style`           | `&'static str` | Style for hamburger icon lines.        | `width: 25px; height: 2px; background: black;`               |

## 💡 Notes

- The navbar is **responsive** by default and adapts to screen size.
- Hamburger toggle appears when the window width is <= 768px.
- Click outside to auto-close mobile and dropdown menus via event listeners.
- You can fully customize the layout using `style` and `class` props for each section.
- Mega menu, search, CTA button, and profile menu are **optional** features that can be toggled via props.
- All callback-based interactions like search input or menu toggling are handled with `use_state`, `Callback`, and `use_effect`.
- Supports accessibility with custom labels, alt tags, and interactive behaviors.
