# 🌱 Form RS Leptos Usage

Adding Form RS to your project is simple:

1. Make sure your project is set up with **Leptos**. Follow their [Getting Started Guide](https://book.leptos.dev/getting_started/index.html) for setup instructions.

1. Add the Form RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add form-rs --features=lep
   ```

1. Import the form components into your Leptos component and start using them in your app.

## 🛠️ Usage

### Basic Login Form

```rust
use form_rs::leptos::{Form, Field};
use form_rs::{Method, ValidationState};
use leptos::prelude::*;

fn validate_email(v: String) -> bool {
    !v.is_empty() && v.contains('@') && v.contains('.')
}

#[component]
pub fn LoginForm() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let email_valid = RwSignal::new(true);

    view! {
        <Form method=Method::Post>
            <Field
                id="email"
                label="Email"
                r#type="email"
                placeholder="ferris@opensass.org"
                helper_text="Enter a valid email."
                required=true
                full_width=true
                handle=email
                valid_handle=email_valid
                validate_function=validate_email
            />
        </Form>
    }
}
```

### ARIA Validation with Helper Text

```rust
use form_rs::leptos::{Form, Field};
use form_rs::ValidationState;
use leptos::prelude::*;

fn validate_username(v: String) -> bool { v.len() >= 3 }

#[component]
pub fn AriaForm() -> impl IntoView {
    let val = RwSignal::new(String::new());
    let valid = RwSignal::new(true);

    view! {
        <Form>
            <Field
                id="username"
                label="Username"
                r#type="text"
                helper_text="3-20 characters."
                validation_state=ValidationState::Invalid("Too short".into())
                handle=val
                valid_handle=valid
                validate_function=validate_username
            />
        </Form>
    }
}
```

### Group with Checkboxes

```rust
use form_rs::leptos::{Form, Group, ControlLabel};
use form_rs::LabelPlacement;
use leptos::prelude::*;

#[component]
pub fn CheckboxGroup() -> impl IntoView {
    view! {
        <Form>
            <Group>
                <ControlLabel
                    control=view! { <input type="checkbox" /> }.into_any()
                    label=view! { <span>"Email notifications"</span> }.into_any()
                    label_placement=LabelPlacement::End
                />
                <ControlLabel
                    control=view! { <input type="checkbox" /> }.into_any()
                    label=view! { <span>"SMS notifications"</span> }.into_any()
                    label_placement=LabelPlacement::End
                />
            </Group>
        </Form>
    }
}
```

## 🔧 Props

### `Form`

| Property       | Type                            | Description                       | Default               |
| -------------- | ------------------------------- | --------------------------------- | --------------------- |
| `method`       | `Method`                        | HTTP submission method.           | `Method::Get`         |
| `action`       | `&'static str`                  | Form submission URL.              | `""`                  |
| `enc_type`     | `EncType`                       | Encoding type for submission.     | `EncType::UrlEncoded` |
| `novalidate`   | `bool`                          | Bypass browser native validation. | `false`               |
| `autocomplete` | `&'static str`                  | `autocomplete` attribute.         | `"off"`               |
| `on_submit`    | `Option<Callback<SubmitEvent>>` | Submit handler.                   | `None`                |
| `class`        | `&'static str`                  | Extra CSS classes.                | `""`                  |
| `style`        | `&'static str`                  | Extra inline CSS.                 | `""`                  |
| `id`           | `&'static str`                  | `id` attribute.                   | `""`                  |
| `aria_label`   | `&'static str`                  | Accessible label.                 | `""`                  |

### `Control`

| Property     | Type           | Description                                      | Default    |
| ------------ | -------------- | ------------------------------------------------ | ---------- |
| `disabled`   | `bool`         | Disables the control.                            | `false`    |
| `error`      | `bool`         | Shows error state.                               | `false`    |
| `focused`    | `bool`         | Shows focused state.                             | `false`    |
| `full_width` | `bool`         | Expands to full width.                           | `false`    |
| `required`   | `bool`         | Marks as required.                               | `false`    |
| `variant`    | `Variant`      | Visual style (`Outlined`, `Filled`, `Standard`). | `Outlined` |
| `color`      | `Color`        | Accent color.                                    | `Primary`  |
| `size`       | `Size`         | Input size (`Small`, `Medium`).                  | `Medium`   |
| `class`      | `&'static str` | Extra CSS classes.                               | `""`       |
| `style`      | `&'static str` | Extra inline CSS.                                | `""`       |

### `Field`

| Property            | Type                 | Description                                  | Default    |
| ------------------- | -------------------- | -------------------------------------------- | ---------- |
| `id`                | `&'static str`       | Input `id` and label `for`. **Required.**    | -          |
| `label`             | `&'static str`       | Label text.                                  | `""`       |
| `r#type`            | `&'static str`       | Input type (`text`, `email`, `password`...). | `"text"`   |
| `placeholder`       | `&'static str`       | Placeholder text.                            | `""`       |
| `helper_text`       | `&'static str`       | Helper message below the input.              | `""`       |
| `validation_state`  | `ValidationState`    | External validation result.                  | `None`     |
| `required`          | `bool`               | HTML required attribute.                     | `false`    |
| `disabled`          | `bool`               | Disables the input.                          | `false`    |
| `full_width`        | `bool`               | Expands to full width.                       | `true`     |
| `variant`           | `Variant`            | Visual style.                                | `Outlined` |
| `color`             | `Color`              | Accent color.                                | `Primary`  |
| `size`              | `Size`               | Input size.                                  | `Medium`   |
| `pattern`           | `&'static str`       | Regex pattern for native validation.         | `".*"`     |
| `maxlength`         | `usize`              | Maximum character count (`0` = no limit).    | `0`        |
| `minlength`         | `usize`              | Minimum character count.                     | `0`        |
| `handle`            | `RwSignal<String>`   | Controlled value signal. **Required.**       | -          |
| `valid_handle`      | `RwSignal<bool>`     | Validity state signal. **Required.**         | -          |
| `validate_function` | `fn(String) -> bool` | Custom validator. **Required.**              | -          |

### `FormLabel`

| Property   | Type           | Description                            | Default |
| ---------- | -------------- | -------------------------------------- | ------- |
| `html_for` | `&'static str` | `for` attribute linking to input `id`. | `""`    |
| `error`    | `bool`         | Shows error color.                     | `false` |
| `focused`  | `bool`         | Shows focused color.                   | `false` |
| `required` | `bool`         | Appends required asterisk.             | `false` |
| `disabled` | `bool`         | Renders in disabled style.             | `false` |
| `class`    | `&'static str` | Extra CSS classes.                     | `""`    |
| `style`    | `&'static str` | Extra inline CSS.                      | `""`    |

### `Helper`

| Property   | Type           | Description                | Default |
| ---------- | -------------- | -------------------------- | ------- |
| `error`    | `bool`         | Renders in error color.    | `false` |
| `valid`    | `bool`         | Renders in success color.  | `false` |
| `disabled` | `bool`         | Renders in disabled style. | `false` |
| `class`    | `&'static str` | Extra CSS classes.         | `""`    |
| `style`    | `&'static str` | Extra inline CSS.          | `""`    |

### `Group`

| Property | Type           | Description                 | Default |
| -------- | -------------- | --------------------------- | ------- |
| `row`    | `bool`         | Lays children horizontally. | `false` |
| `class`  | `&'static str` | Extra CSS classes.          | `""`    |
| `style`  | `&'static str` | Extra inline CSS.           | `""`    |

### `ControlLabel`

| Property          | Type             | Description                                          | Default |
| ----------------- | ---------------- | ---------------------------------------------------- | ------- |
| `control`         | `AnyView`        | The form control (checkbox, radio...). **Required.** | -       |
| `label`           | `AnyView`        | Label element or text. **Required.**                 | -       |
| `label_placement` | `LabelPlacement` | `End`, `Start`, `Top`, or `Bottom`.                  | `End`   |
| `disabled`        | `bool`           | Renders in disabled style.                           | `false` |
| `required`        | `bool`           | Appends required asterisk.                           | `false` |
| `class`           | `&'static str`   | Extra CSS classes.                                   | `""`    |
| `style`           | `&'static str`   | Extra inline CSS.                                    | `""`    |

## 💡 Notes

- `Field` is a convenience composition of `Control + FormLabel + Input + Helper`. Use it for the common case.
- Use `Control` + `FormLabel` + `Helper` manually when you need full layout control.
- Set `validation_state=ValidationState::Invalid("message".into())` to show server-side errors.
- All components are WCAG 2.2 AA compliant, `aria-describedby`, `aria-required`, and `aria-invalid` are wired automatically.

## 🔗 See Also

- [Input RS](https://crates.io/crates/input-rs): The unstyled `<input>` abstraction layer powering `Field`.
- [MDN `<form>` element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form): Full reference for HTML form attributes and behavior.
- [MDN `<input>` element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input): Full reference for input types, validation attributes, and events.
- [WCAG 2.2: Form Labels](https://www.w3.org/WAI/WCAG22/Understanding/labels-or-instructions): Accessibility guidance for labeling form controls.
- [WCAG 2.2: Error Identification](https://www.w3.org/WAI/WCAG22/Understanding/error-identification): Accessibility guidance for surfacing validation errors.
