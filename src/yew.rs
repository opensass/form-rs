// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../YEW.md")]

use crate::common::{
    Color, EncType, LabelPlacement, Margin, Method, Size, Target, ValidationBehavior,
    ValidationState, Variant, base_form_control_style, base_form_group_row_style,
    base_form_group_style, base_form_style, base_helper_text_style, base_input_field_style,
    base_label_style, field_disabled_style, field_error_style, field_valid_style,
    helper_error_style, helper_valid_style, label_error_style, required_asterisk_style,
};
pub use input_rs::yew::Input;
use yew::prelude::*;

/// Shared context propagated by [`Form`] to all descendant components.
///
/// Consumed by [`Control`], [`FormLabel`], and [`Helper`] to
/// inherit validation behavior and submission state automatically.
#[derive(Clone, Debug, PartialEq)]
pub struct FormContext {
    /// Active validation strategy for the entire form.
    pub validation_behavior: ValidationBehavior,
    /// `true` when the form is actively being submitted.
    pub is_submitting: bool,
    /// Whether the form has `novalidate` set.
    pub novalidate: bool,
}

/// Shared context provided by [`Control`] to its children.
///
/// Consumed by [`FormLabel`] and [`Helper`] so they can reflect the
/// parent control's state without prop drilling.
#[derive(Clone, Debug, PartialEq)]
pub struct ControlContext {
    /// Whether the field is in an error state.
    pub error: bool,
    /// Whether the field is disabled.
    pub disabled: bool,
    /// Whether the field is focused.
    pub focused: bool,
    /// Whether the field is required.
    pub required: bool,
    /// Whether the field has a non-empty value.
    pub filled: bool,
    /// Visual variant of the field.
    pub variant: Variant,
    /// Color theme of the field.
    pub color: Color,
    /// Size of the field.
    pub size: Size,
    /// ID of the underlying `<input>` element, used to link `<label>`.
    pub input_id: String,
}

/// Props for the [`Form`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct FormProps {
    /// Form content: fields, buttons, and other interactive controls.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the `<form>` element.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the `<form>` element.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the `<form>` element.
    #[prop_or_default]
    pub id: &'static str,

    /// URL that processes the form submission.
    #[prop_or_default]
    pub action: &'static str,

    /// HTTP method for form submission.
    #[prop_or_default]
    pub method: Method,

    /// MIME encoding type for submitted data.
    #[prop_or_default]
    pub enc_type: EncType,

    /// Where to display the form response.
    #[prop_or_default]
    pub target: Target,

    /// When `true`, disables native browser validation on submit.
    /// Has no effect when `validation_behavior` is `Aria`.
    #[prop_or_default]
    pub novalidate: bool,

    /// Browser autocomplete hint for the form.
    #[prop_or("on")]
    pub autocomplete: &'static str,

    /// Name of the form; must be unique in the `forms` collection.
    #[prop_or_default]
    pub name: &'static str,

    /// Controls whether validation is native (blocks submit) or ARIA (realtime).
    #[prop_or_default]
    pub validation_behavior: ValidationBehavior,

    /// Handler called when the form is submitted.
    #[prop_or_default]
    pub on_submit: Callback<SubmitEvent>,

    /// Handler called when the form is reset.
    #[prop_or_default]
    pub on_reset: Callback<Event>,

    /// Handler called when any field fails native validation.
    #[prop_or_default]
    pub on_invalid: Callback<Event>,

    /// Accessible label for screen readers (creates a form landmark).
    #[prop_or_default]
    pub aria_label: &'static str,

    /// ID of an element that labels this form.
    #[prop_or_default]
    pub aria_labelledby: &'static str,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// A semantic `<form>` wrapper that provides validation context to descendant fields.
///
/// Propagates [`FormContext`] via Yew context so children such as [`Control`],
/// [`FormLabel`], and [`Helper`] can reflect the form's validation mode
/// and submission state without additional prop drilling.
///
/// # Accessibility
///
/// - Renders as a native `<form>` element, which implies a `"form"` ARIA role.
/// - Supply `aria_label` **or** `aria_labelledby` to create a named form landmark.
/// - Native constraint validation errors are auto-announced by assistive technology.
///
/// # Examples
///
/// ```rust
/// use form_rs::yew::{Form, Control, FormLabel};
/// use yew::prelude::*;
///
/// #[function_component(LoginForm)]
/// pub fn login_form() -> Html {
///     let on_submit = Callback::from(|e: SubmitEvent| {
///         e.prevent_default();
///     });
///     html! {
///         <Form on_submit={on_submit} aria_label="Login">
///             <Control id="email">
///                 <FormLabel html_for="email">{"Email"}</FormLabel>
///             </Control>
///         </Form>
///     }
/// }
/// ```
#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let is_submitting = use_state(|| false);

    let ctx = FormContext {
        validation_behavior: props.validation_behavior,
        is_submitting: *is_submitting,
        novalidate: props.novalidate || props.validation_behavior == ValidationBehavior::Aria,
    };

    let on_submit = {
        let user_cb = props.on_submit.clone();
        let is_submitting = is_submitting.clone();
        Callback::from(move |e: SubmitEvent| {
            is_submitting.set(true);
            user_cb.emit(e);
            is_submitting.set(false);
        })
    };

    let on_reset = props.on_reset.clone();
    let on_invalid = props.on_invalid.clone();

    let novalidate = props.novalidate || props.validation_behavior == ValidationBehavior::Aria;
    let full_style = format!("{} {}", base_form_style(), props.style);

    html! {
        <ContextProvider<FormContext> context={ctx}>
            <form
                id={props.id}
                class={format!("form {}", props.class)}
                style={full_style}
                action={props.action}
                method={props.method.as_str()}
                enctype={props.enc_type.as_str()}
                target={props.target.as_str()}
                novalidate={novalidate}
                autocomplete={props.autocomplete}
                name={props.name}
                onsubmit={on_submit}
                onreset={Callback::from(move |e: Event| on_reset.emit(e))}
                oninvalid={Callback::from(move |e: Event| on_invalid.emit(e))}
                aria-label={props.aria_label}
                aria-labelledby={props.aria_labelledby}
                data-testid={props.data_testid}
            >
                { for props.children.iter() }
            </form>
        </ContextProvider<FormContext>>
    }
}

/// Props for the [`Control`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct ControlProps {
    /// Field components: [`FormLabel`], input, [`Helper`].
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the wrapper `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the wrapper `<div>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the wrapper `<div>`.
    #[prop_or_default]
    pub id: &'static str,

    /// The `id` of the inner `<input>` element; links `<label>` via `for`.
    #[prop_or_default]
    pub input_id: &'static str,

    /// When `true`, renders the label, input, and helper text in a disabled state.
    #[prop_or_default]
    pub disabled: bool,

    /// When `true`, renders the label and helper text in the error color.
    #[prop_or_default]
    pub error: bool,

    /// When `true`, applies the focused style. Use for controlled focus.
    #[prop_or_default]
    pub focused: bool,

    /// When `true`, the component takes up the full width of its container.
    #[prop_or_default]
    pub full_width: bool,

    /// When `true`, the label is hidden (useful with `aria-label` on the input).
    #[prop_or_default]
    pub hidden_label: bool,

    /// Vertical spacing adjustment.
    #[prop_or_default]
    pub margin: Margin,

    /// Whether the field is required.
    #[prop_or_default]
    pub required: bool,

    /// Size of the component.
    #[prop_or_default]
    pub size: Size,

    /// Visual style variant.
    #[prop_or_default]
    pub variant: Variant,

    /// Color theme applied to the label and focused/active field.
    #[prop_or_default]
    pub color: Color,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// A context provider that wraps a single form field with its label and helper text.
///
/// Provides [`ControlContext`] so that [`FormLabel`] and [`Helper`]
/// descendants can automatically reflect the `error`, `disabled`, `required`,
/// `variant`, and `color` states without additional prop drilling.
///
/// # Accessibility
///
/// - The wrapper `<div>` uses `role="group"` when `hidden_label` is `true`.
/// - Cascades `aria-disabled` semantics to all children via CSS opacity.
///
/// # Examples
///
/// ```rust
/// use form_rs::yew::{Control, FormLabel, Helper};
/// use form_rs::Variant;
/// use yew::prelude::*;
///
/// #[function_component(EmailField)]
/// pub fn email_field() -> Html {
///     html! {
///         <Control input_id="email" required=true variant={Variant::Outlined}>
///             <FormLabel html_for="email">{"Email"}</FormLabel>
///             <Helper>{"We'll never share your email."}</Helper>
///         </Control>
///     }
/// }
/// ```
#[function_component(Control)]
pub fn form_control(props: &ControlProps) -> Html {
    let ctx = ControlContext {
        error: props.error,
        disabled: props.disabled,
        focused: props.focused,
        required: props.required,
        filled: false,
        variant: props.variant,
        color: props.color,
        size: props.size,
        input_id: props.input_id.to_string(),
    };

    let mut class_parts = vec!["form-control".to_string()];
    class_parts.push(props.variant.to_class().to_string());
    class_parts.push(props.size.to_class().to_string());
    if !props.margin.to_class().is_empty() {
        class_parts.push(props.margin.to_class().to_string());
    }
    if props.full_width {
        class_parts.push("form-control--full-width".to_string());
    }
    if props.error {
        class_parts.push("form-control--error".to_string());
    }
    if props.disabled {
        class_parts.push("form-control--disabled".to_string());
    }
    if props.focused {
        class_parts.push("form-control--focused".to_string());
    }
    if props.hidden_label {
        class_parts.push("form-control--hidden-label".to_string());
    }
    class_parts.push(props.class.to_string());

    let mut style_parts = vec![base_form_control_style().to_string()];
    style_parts.push(props.margin.to_style().to_string());
    if props.full_width {
        style_parts.push("width: 100%;".to_string());
    }
    if props.disabled {
        style_parts.push(field_disabled_style().to_string());
    }
    style_parts.push(props.style.to_string());

    html! {
        <ContextProvider<ControlContext> context={ctx}>
            <div
                id={props.id}
                class={class_parts.join(" ")}
                style={style_parts.join(" ")}
                data-testid={props.data_testid}
            >
                { for props.children.iter() }
            </div>
        </ContextProvider<ControlContext>>
    }
}

/// Props for the [`FormLabel`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct FormLabelProps {
    /// Label text or any content.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the `<label>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the `<label>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the `<label>`.
    #[prop_or_default]
    pub id: &'static str,

    /// The `id` of the labeled `<input>`. Overrides the value from [`ControlContext`].
    #[prop_or_default]
    pub html_for: &'static str,

    /// Color theme override. Defaults to the [`ControlContext`] color.
    #[prop_or_default]
    pub color: Option<Color>,

    /// When `true`, renders in a disabled style.
    #[prop_or_default]
    pub disabled: bool,

    /// When `true`, renders in the error color.
    #[prop_or_default]
    pub error: bool,

    /// When `true`, applies the filled modifier class.
    #[prop_or_default]
    pub filled: bool,

    /// When `true`, applies the focused modifier class.
    #[prop_or_default]
    pub focused: bool,

    /// When `true`, shows a required asterisk after the label text.
    #[prop_or_default]
    pub required: bool,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// Renders an accessible `<label>` linked to a form field.
///
/// When nested inside [`Control`], it automatically reads `error`,
/// `disabled`, `focused`, `required`, and `color` from [`ControlContext`].
///
/// # Accessibility
///
/// - Always links to its input via `for` / [`FormLabelProps::html_for`].
/// - Shows a visual required indicator (`*`) when `required` is `true`.
/// - Reflects error, focused, and disabled states via class names and color.
///
/// # Examples
///
/// ```rust
/// use form_rs::yew::FormLabel;
/// use yew::prelude::*;
///
/// #[function_component(MyLabel)]
/// pub fn my_label() -> Html {
///     html! { <FormLabel html_for="username">{"Username"}</FormLabel> }
/// }
/// ```
#[function_component(FormLabel)]
pub fn form_label(props: &FormLabelProps) -> Html {
    let ctx = use_context::<ControlContext>();

    let error = props.error || ctx.as_ref().is_some_and(|c| c.error);
    let disabled = props.disabled || ctx.as_ref().is_some_and(|c| c.disabled);
    let focused = props.focused || ctx.as_ref().is_some_and(|c| c.focused);
    let required = props.required || ctx.as_ref().is_some_and(|c| c.required);
    let color = props
        .color
        .or_else(|| ctx.as_ref().map(|c| c.color))
        .unwrap_or_default();
    let linked_id = if !props.html_for.is_empty() {
        props.html_for.to_string()
    } else {
        ctx.as_ref().map(|c| c.input_id.clone()).unwrap_or_default()
    };

    let color_style = if error {
        label_error_style().to_string()
    } else if focused {
        color.to_label_color()
    } else {
        String::new()
    };

    let mut class_parts = vec!["form-label".to_string()];
    if error {
        class_parts.push("form-label--error".to_string());
    }
    if disabled {
        class_parts.push("form-label--disabled".to_string());
    }
    if focused {
        class_parts.push("form-label--focused".to_string());
    }
    if props.filled {
        class_parts.push("form-label--filled".to_string());
    }
    class_parts.push(props.class.to_string());

    let full_style = format!("{} {} {}", base_label_style(), color_style, props.style);

    html! {
        <label
            id={props.id}
            class={class_parts.join(" ")}
            style={full_style}
            for={linked_id}
            aria-disabled={if disabled { "true" } else { "false" }}
            data-testid={props.data_testid}
        >
            { for props.children.iter() }
            if required {
                <span style={required_asterisk_style()} aria-hidden="true">{ "*" }</span>
            }
        </label>
    }
}

/// Props for the [`Helper`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct HelperProps {
    /// Helper text content or any views.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the `<p>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the `<p>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the `<p>`. Use this as `aria-describedby` on the input.
    #[prop_or_default]
    pub id: &'static str,

    /// When `true`, renders in a disabled style.
    #[prop_or_default]
    pub disabled: bool,

    /// When `true`, renders in the error color.
    #[prop_or_default]
    pub error: bool,

    /// When `true`, renders in a valid/success color.
    #[prop_or_default]
    pub valid: bool,

    /// When `true`, applies the filled modifier class.
    #[prop_or_default]
    pub filled: bool,

    /// When `true`, applies the focused modifier class.
    #[prop_or_default]
    pub focused: bool,

    /// Vertical spacing adjustment.
    #[prop_or_default]
    pub margin: Margin,

    /// Whether the parent field is required (affects ARIA live region).
    #[prop_or_default]
    pub required: bool,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// Renders accessible helper text beneath a form field.
///
/// When nested inside [`Control`], it inherits `error`, `disabled`,
/// `focused`, and `filled` states from [`ControlContext`]. Error text is
/// announced to screen readers via `role="alert"`.
///
/// # Accessibility
///
/// - `role="alert"` is added automatically when `error` is `true`.
/// - Assign its `id` to the input's `aria-describedby` attribute.
///
/// # Examples
///
/// ```rust
/// use form_rs::yew::Helper;
/// use yew::prelude::*;
///
/// #[function_component(PasswordHint)]
/// pub fn password_hint() -> Html {
///     html! {
///         <Helper id="pw-hint">
///             {"Must be at least 8 characters."}
///         </Helper>
///     }
/// }
/// ```
#[function_component(Helper)]
pub fn form_helper_text(props: &HelperProps) -> Html {
    let ctx = use_context::<ControlContext>();

    let error = props.error || ctx.as_ref().is_some_and(|c| c.error);
    let disabled = props.disabled || ctx.as_ref().is_some_and(|c| c.disabled);

    let color_style = if error {
        helper_error_style()
    } else if props.valid {
        helper_valid_style()
    } else {
        ""
    };

    let mut class_parts = vec!["form-helper-text".to_string()];
    if error {
        class_parts.push("form-helper-text--error".to_string());
    }
    if props.valid {
        class_parts.push("form-helper-text--valid".to_string());
    }
    if disabled {
        class_parts.push("form-helper-text--disabled".to_string());
    }
    if props.focused {
        class_parts.push("form-helper-text--focused".to_string());
    }
    if props.filled {
        class_parts.push("form-helper-text--filled".to_string());
    }
    if !props.margin.to_class().is_empty() {
        class_parts.push(props.margin.to_class().to_string());
    }
    class_parts.push(props.class.to_string());

    let full_style = format!(
        "{} {} {}",
        base_helper_text_style(),
        color_style,
        props.style
    );

    html! {
        <p
            id={props.id}
            class={class_parts.join(" ")}
            style={full_style}
            role={if error { "alert" } else { "" }}
            aria-live={if error { "polite" } else { "" }}
            aria-disabled={if disabled { "true" } else { "false" }}
            data-testid={props.data_testid}
        >
            { for props.children.iter() }
        </p>
    }
}

/// Props for the [`Group`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct GroupProps {
    /// Controls such as checkboxes or switches.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class names on the wrapper `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the wrapper `<div>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the `<div>`.
    #[prop_or_default]
    pub id: &'static str,

    /// When `true`, renders children horizontally in a row.
    #[prop_or_default]
    pub row: bool,

    /// When `true`, applies the error modifier class.
    #[prop_or_default]
    pub error: bool,

    /// Accessible label for the group (`aria-label`).
    #[prop_or_default]
    pub aria_label: &'static str,

    /// ID of the element that labels this group (`aria-labelledby`).
    #[prop_or_default]
    pub aria_labelledby: &'static str,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// Wraps groups of controls such as checkboxes and switches.
///
/// Provides a compact column layout by default; set `row=true` for a
/// horizontal row. For radio groups, prefer `<RadioGroup>` instead.
///
/// # Accessibility
///
/// - Renders as `<div role="group">` with support for `aria_label` and
///   `aria_labelledby` to name the group for screen readers.
///
/// # Examples
///
/// ```rust
/// use form_rs::yew::Group;
/// use yew::prelude::*;
///
/// #[function_component(OptionsGroup)]
/// pub fn options_group() -> Html {
///     html! {
///         <Group aria_label="Notifications" row=true>
///             <span>{"Email"}</span>
///             <span>{"SMS"}</span>
///         </Group>
///     }
/// }
/// ```
#[function_component(Group)]
pub fn form_group(props: &GroupProps) -> Html {
    let base_style = if props.row {
        base_form_group_row_style()
    } else {
        base_form_group_style()
    };

    let mut class_parts = vec!["form-group".to_string()];
    if props.row {
        class_parts.push("form-group--row".to_string());
    }
    if props.error {
        class_parts.push("form-group--error".to_string());
    }
    class_parts.push(props.class.to_string());

    let full_style = format!("{} {}", base_style, props.style);

    html! {
        <div
            id={props.id}
            class={class_parts.join(" ")}
            style={full_style}
            role="group"
            aria-label={props.aria_label}
            aria-labelledby={props.aria_labelledby}
            data-testid={props.data_testid}
        >
            { for props.children.iter() }
        </div>
    }
}

/// Props for the [`ControlLabel`] Yew component.
#[derive(Properties, PartialEq, Clone)]
pub struct ControlLabelProps {
    /// The control element, e.g. a checkbox or switch `<input>`.
    pub control: Html,

    /// The label text or content shown next to the control.
    #[prop_or_default]
    pub label: Html,

    /// Additional CSS class names on the wrapper `<label>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the wrapper `<label>`.
    #[prop_or_default]
    pub style: &'static str,

    /// `id` attribute on the wrapper `<label>`.
    #[prop_or_default]
    pub id: &'static str,

    /// Whether the control appears checked (for display only; wire your own state).
    #[prop_or_default]
    pub checked: bool,

    /// When `true`, all interactive elements within are visually disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Position of the label text relative to the control element.
    #[prop_or_default]
    pub label_placement: LabelPlacement,

    /// Whether the label indicates a required field.
    #[prop_or_default]
    pub required: bool,

    /// The `value` associated with this labeled control.
    #[prop_or_default]
    pub value: &'static str,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// A drop-in label wrapper for Radio, Switch, and Checkbox controls.
///
/// Renders a `<label>` element that wraps a control (`<input type="checkbox">`,
/// etc.) alongside descriptive label text. The `label_placement` prop controls
/// the visual order of the control and its label.
///
/// # Accessibility
///
/// - The wrapping `<label>` element natively associates its text with the
///   contained control, so no explicit `for` attribute is needed.
/// - `aria-disabled` is set when `disabled` is `true`.
/// - `aria-required` is set when `required` is `true`.
///
/// # Examples
///
/// ```rust
/// use form_rs::yew::ControlLabel;
/// use form_rs::LabelPlacement;
/// use yew::prelude::*;
///
/// #[function_component(TermsCheckbox)]
/// pub fn terms_checkbox() -> Html {
///     html! {
///         <ControlLabel
///             control={html! { <input type="checkbox" /> }}
///             label={html! { <span>{"I agree to the terms"}</span> }}
///             label_placement={LabelPlacement::End}
///         />
///     }
/// }
/// ```
#[function_component(ControlLabel)]
pub fn form_control_label(props: &ControlLabelProps) -> Html {
    let flex_style = props.label_placement.to_flex_direction();
    let placement_class = props.label_placement.to_class();

    let mut class_parts = vec!["form-control-label".to_string()];
    class_parts.push(placement_class.to_string());
    if props.disabled {
        class_parts.push("form-control-label--disabled".to_string());
    }
    class_parts.push(props.class.to_string());

    let base_style = format!(
        "display: inline-flex; {} gap: 8px; align-items: center; cursor: {}; user-select: none;",
        flex_style,
        if props.disabled {
            "not-allowed"
        } else {
            "pointer"
        }
    );
    let full_style = format!("{} {}", base_style, props.style);

    html! {
        <label
            id={props.id}
            class={class_parts.join(" ")}
            style={full_style}
            aria-disabled={if props.disabled { "true" } else { "false" }}
            aria-required={if props.required { "true" } else { "false" }}
            data-testid={props.data_testid}
        >
            { props.control.clone() }
            <span
                class="form-control-label__label"
                style="font-size: 14px; color: #d4d4d8; line-height: 1.4;"
            >
                { props.label.clone() }
                if props.required {
                    <span style={required_asterisk_style()} aria-hidden="true">{ "*" }</span>
                }
            </span>
        </label>
    }
}

/// Props for the [`Field`] Yew component, a convenience wrapper for a
/// labeled, validated input built on top of `input-rs`.
#[derive(Properties, PartialEq, Clone)]
pub struct FieldProps {
    /// The unique `id` for the `<input>` element.
    pub id: &'static str,

    /// The `name` attribute for the `<input>`.
    #[prop_or_default]
    pub name: &'static str,

    /// The input type, e.g. `"text"`, `"email"`, `"password"`.
    #[prop_or("text")]
    pub r#type: &'static str,

    /// Label text displayed above the input.
    #[prop_or_default]
    pub label: &'static str,

    /// Placeholder text shown inside the empty input.
    #[prop_or_default]
    pub placeholder: &'static str,

    /// Helper text displayed below the input.
    #[prop_or_default]
    pub helper_text: &'static str,

    /// Validation state: `None`, `Valid`, or `Invalid(message)`.
    #[prop_or_default]
    pub validation_state: ValidationState,

    /// Whether the field is required.
    #[prop_or_default]
    pub required: bool,

    /// Whether the field is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the field takes the full container width.
    #[prop_or(true)]
    pub full_width: bool,

    /// Visual variant of the field border.
    #[prop_or_default]
    pub variant: Variant,

    /// Color theme for focus/label highlights.
    #[prop_or_default]
    pub color: Color,

    /// Size of the field.
    #[prop_or_default]
    pub size: Size,

    /// HTML5 `pattern` attribute for native regex validation.
    #[prop_or(".*")]
    pub pattern: &'static str,

    /// Maximum number of characters allowed.
    #[prop_or_default]
    pub maxlength: Option<usize>,

    /// Minimum number of characters required.
    #[prop_or_default]
    pub minlength: Option<usize>,

    /// Additional CSS class on the outer `<div>`.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline CSS on the outer `<div>`.
    #[prop_or_default]
    pub style: &'static str,

    /// Controlled value signal.
    pub handle: UseStateHandle<String>,

    /// Validity signal.
    pub valid_handle: UseStateHandle<bool>,

    /// Validation function called with the current value.
    pub validate_function: Callback<String, bool>,

    /// `data-testid` for automated testing.
    #[prop_or_default]
    pub data_testid: &'static str,
}

/// A convenience composition of [`Control`], [`FormLabel`], [`Input`], and
/// [`Helper`] into a single, fully validated form field.
///
/// This component removes boilerplate for the common pattern of a labeled input
/// with validation state and helper text. It wraps `input-rs` for the actual
/// `<input>` element, inheriting all its HTML5 validation attributes.
///
/// # Accessibility
///
/// - The `<label>` is explicitly linked to the `<input>` via `for`/`id`.
/// - Helper text is linked via `aria-describedby`.
/// - Error messages have `role="alert"` for immediate screen-reader announcement.
///
/// # Examples
///
/// ```rust
/// use form_rs::yew::Field;
/// use yew::prelude::*;
///
/// #[function_component(EmailInput)]
/// pub fn email_input() -> Html {
///     let handle = use_state(String::new);
///     let valid = use_state(|| true);
///     html! {
///         <Field
///             id="email"
///             name="email"
///             r#type="email"
///             label="Email address"
///             placeholder="ferris@opensass.org"
///             helper_text="We'll never share your email."
///             required=true
///             handle={handle}
///             valid_handle={valid}
///             validate_function={|v: String| !v.is_empty() && v.contains('@')}
///         />
///     }
/// }
/// ```
#[function_component(Field)]
pub fn form_field(props: &FieldProps) -> Html {
    let is_error = props.validation_state.is_invalid()
        || (!(*props.valid_handle) && !(*props.handle).is_empty());
    let is_valid = matches!(props.validation_state, ValidationState::Valid)
        || ((*props.valid_handle) && !(*props.handle).is_empty());

    let error_msg = props
        .validation_state
        .error_message()
        .map(|s| s.to_string());

    let helper_id: &'static str = Box::leak(format!("{}-helper", props.id).into_boxed_str());
    let focused = use_state(|| false);

    let on_focus = {
        let focused = focused.clone();
        Callback::from(move |_| focused.set(true))
    };
    let on_blur = {
        let focused = focused.clone();
        Callback::from(move |_| focused.set(false))
    };

    let field_border = props.variant.to_field_style();
    let input_size = props.size.to_input_style();
    let focus_ring = if *focused && !is_error {
        props.color.to_focus_ring()
    } else {
        String::new()
    };
    let error_ring = if is_error {
        field_error_style().to_string()
    } else {
        String::new()
    };
    let valid_ring = if is_valid && !is_error {
        field_valid_style().to_string()
    } else {
        String::new()
    };

    let input_style = format!(
        "{} {} {} {} {} {} transition: all 0.2s ease;",
        base_input_field_style(),
        field_border,
        input_size,
        focus_ring,
        error_ring,
        valid_ring
    );

    let input_style_static: &'static str = Box::leak(input_style.into_boxed_str());

    let helper_content = if let Some(ref msg) = error_msg {
        msg.clone()
    } else if is_error {
        "Invalid value.".to_string()
    } else if !props.helper_text.is_empty() {
        props.helper_text.to_string()
    } else {
        String::new()
    };

    let node_ref = use_node_ref();
    let validate_cb = props.validate_function.clone();

    html! {
        <Control
            input_id={props.id}
            error={is_error}
            disabled={props.disabled}
            focused={*focused}
            full_width={props.full_width}
            required={props.required}
            variant={props.variant}
            color={props.color}
            size={props.size}
            class={props.class}
            style={props.style}
            data_testid={props.data_testid}
        >
            if !props.label.is_empty() {
                <FormLabel
                    html_for={props.id}
                    error={is_error}
                    focused={*focused}
                    required={props.required}
                    disabled={props.disabled}
                >
                    { html!{props.label} }
                </FormLabel>
            }
            <Input
                r#type={props.r#type}
                id={props.id}
                name={props.name}
                placeholder={props.placeholder}
                r#ref={node_ref}
                handle={props.handle.clone()}
                valid_handle={props.valid_handle.clone()}
                validate_function={validate_cb}
                required={props.required}
                disabled={props.disabled}
                pattern={props.pattern}
                maxlength={props.maxlength}
                minlength={props.minlength}
                input_style={input_style_static}
                aria_describedby={helper_id}
                aria_required={if props.required { "true" } else { "false" }}
                aria_invalid={if is_error { "true" } else { "false" }}
                otp_mode=true
                on_focus={on_focus}
                on_blur={on_blur}
            />
            if !helper_content.is_empty() {
                <Helper id={helper_id} error={is_error} valid={is_valid && !is_error}>
                    { html!{helper_content} }
                </Helper>
            }
        </Control>
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
