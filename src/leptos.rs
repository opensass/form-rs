// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../LEPTOS.md")]

use crate::common::{
    Color, EncType, LabelPlacement, Margin, Method, Size, Target, ValidationBehavior,
    ValidationState, Variant, base_form_control_style, base_form_group_row_style,
    base_form_group_style, base_form_style, base_helper_text_style, base_input_field_style,
    base_label_style, field_disabled_style, field_error_style, field_valid_style,
    helper_error_style, helper_valid_style, label_error_style, required_asterisk_style,
};
pub use input_rs::leptos::Input;
use leptos::ev::{Event, FocusEvent, SubmitEvent};
use leptos::prelude::*;

/// Shared context propagated by [`Form`] to all descendant components.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct FormContext {
    /// Active validation strategy for the entire form.
    pub validation_behavior: ValidationBehavior,
    /// `true` when the form is actively submitting.
    pub is_submitting: bool,
    /// Whether native browser validation is suppressed.
    pub novalidate: bool,
}

/// Shared context provided by [`Control`] to its children.
#[derive(Clone, Debug, PartialEq, Copy)]
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
}

/// A semantic `<form>` wrapper that provides validation context to descendant fields.
///
/// Propagates [`FormContext`] via Leptos context so that [`Control`],
/// [`FormLabel`], and [`Helper`] can read validation mode and submission
/// state without prop drilling.
///
/// # Accessibility
///
/// - Renders as a native `<form>` element (implicit `"form"` ARIA role).
/// - Supply `aria_label` or `aria_labelledby` to create a named form landmark.
///
/// # Examples
///
/// ```rust
/// use form_rs::leptos::{Form, Control, FormLabel};
/// use leptos::prelude::*;
/// use leptos::ev::SubmitEvent;
///
/// #[component]
/// pub fn LoginForm() -> impl IntoView {
///     view! {
///         <Form aria_label="Login" on_submit=Callback::new(|e: SubmitEvent| e.prevent_default())>
///             <Control input_id="email">
///                 <FormLabel html_for="email">"Email"</FormLabel>
///             </Control>
///         </Form>
///     }
/// }
/// ```
#[allow(non_snake_case)]
#[component]
pub fn Form(
    /// Form content: fields, buttons, and other interactive controls.
    children: Children,
    /// Additional CSS class names on the `<form>` element.
    #[prop(default = "")]
    class: &'static str,
    /// Inline CSS on the `<form>` element.
    #[prop(default = "")]
    style: &'static str,
    /// `id` attribute on the `<form>` element.
    #[prop(default = "")]
    id: &'static str,
    /// URL that processes the form submission.
    #[prop(default = "")]
    action: &'static str,
    /// HTTP method for form submission.
    #[prop(default = Method::Get)]
    method: Method,
    /// MIME encoding type for submitted data.
    #[prop(default = EncType::UrlEncoded)]
    enc_type: EncType,
    /// Where to display the form response.
    #[prop(default = Target::Self_)]
    target: Target,
    /// When `true`, disables native browser validation.
    #[prop(default = false)]
    novalidate: bool,
    /// Browser autocomplete hint.
    #[prop(default = "on")]
    autocomplete: &'static str,
    /// Name of the form.
    #[prop(default = "")]
    name: &'static str,
    /// Controls whether validation is native or ARIA-based.
    #[prop(default = ValidationBehavior::Native)]
    validation_behavior: ValidationBehavior,
    /// Handler called when the form is submitted.
    #[prop(optional)]
    on_submit: Option<Callback<SubmitEvent>>,
    /// Handler called when the form is reset.
    #[prop(optional)]
    on_reset: Option<Callback<Event>>,
    /// Accessible label for screen readers.
    #[prop(default = "")]
    aria_label: &'static str,
    /// ID of element that labels this form.
    #[prop(default = "")]
    aria_labelledby: &'static str,
    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let is_submitting = RwSignal::new(false);
    let should_novalidate = novalidate || validation_behavior == ValidationBehavior::Aria;

    let ctx = FormContext {
        validation_behavior,
        is_submitting: is_submitting.get(),
        novalidate: should_novalidate,
    };
    provide_context(ctx);

    let full_style = format!("{} {}", base_form_style(), style);
    let full_class = format!("form {}", class);

    view! {
        <form
            id=id
            class=full_class
            style=full_style
            action=action
            method=method.as_str()
            enctype=enc_type.as_str()
            target=target.as_str()
            novalidate=should_novalidate
            autocomplete=autocomplete
            name=name
            on:submit=move |e| {
                is_submitting.set(true);
                if let Some(cb) = on_submit { cb.run(e); }
                is_submitting.set(false);
            }
            on:reset=move |e: Event| {
                if let Some(cb) = on_reset { cb.run(e); }
            }
            aria-label=aria_label
            aria-labelledby=aria_labelledby
            data-testid=data_testid
        >
            {children()}
        </form>
    }
}

/// A context provider that wraps a single form field with its label and helper text.
///
/// Provides [`ControlContext`] to [`FormLabel`] and [`Helper`]
/// descendants so they reflect error, disabled, required, variant, and color
/// states automatically.
///
/// # Examples
///
/// ```rust
/// use form_rs::leptos::{Control, FormLabel, Helper};
/// use form_rs::Variant;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn EmailField() -> impl IntoView {
///     view! {
///         <Control input_id="email" required=true variant=Variant::Outlined>
///             <FormLabel html_for="email">"Email"</FormLabel>
///             <Helper>"We'll never share your email."</Helper>
///         </Control>
///     }
/// }
/// ```
#[allow(non_snake_case)]
#[component]
pub fn Control(
    /// Field components: [`FormLabel`], input, [`Helper`].
    children: Children,
    /// Additional CSS class names on the wrapper `<div>`.
    #[prop(default = "")]
    class: &'static str,
    /// Inline CSS on the wrapper `<div>`.
    #[prop(default = "")]
    style: &'static str,
    /// `id` attribute on the wrapper `<div>`.
    #[prop(default = "")]
    id: &'static str,
    /// The `id` of the inner `<input>` element.
    #[allow(unused)]
    #[prop(default = "")]
    input_id: &'static str,
    /// When `true`, renders all children in a disabled state.
    #[prop(default = false)]
    disabled: bool,
    /// When `true`, renders the label and helper text in the error color.
    #[prop(default = false)]
    error: bool,
    /// When `true`, applies the focused style.
    #[prop(default = false)]
    focused: bool,
    /// When `true`, the component takes up full container width.
    #[prop(default = false)]
    full_width: bool,
    /// When `true`, the label is hidden.
    #[prop(default = false)]
    hidden_label: bool,
    /// Vertical spacing adjustment.
    #[prop(default = Margin::None)]
    margin: Margin,
    /// Whether the field is required.
    #[prop(default = false)]
    required: bool,
    /// Size of the component.
    #[prop(default = Size::Medium)]
    size: Size,
    /// Visual style variant.
    #[prop(default = Variant::Outlined)]
    variant: Variant,
    /// Color theme.
    #[prop(default = Color::Primary)]
    color: Color,
    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let ctx = ControlContext {
        error,
        disabled,
        focused,
        required,
        filled: false,
        variant,
        color,
        size,
    };
    provide_context(ctx);

    let mut cls_parts = vec![
        "form-control".to_string(),
        variant.to_class().to_string(),
        size.to_class().to_string(),
    ];
    if !margin.to_class().is_empty() {
        cls_parts.push(margin.to_class().to_string());
    }
    if full_width {
        cls_parts.push("form-control--full-width".to_string());
    }
    if error {
        cls_parts.push("form-control--error".to_string());
    }
    if disabled {
        cls_parts.push("form-control--disabled".to_string());
    }
    if focused {
        cls_parts.push("form-control--focused".to_string());
    }
    if hidden_label {
        cls_parts.push("form-control--hidden-label".to_string());
    }
    cls_parts.push(class.to_string());

    let mut sty_parts = vec![
        base_form_control_style().to_string(),
        margin.to_style().to_string(),
    ];
    if full_width {
        sty_parts.push("width: 100%;".to_string());
    }
    if disabled {
        sty_parts.push(field_disabled_style().to_string());
    }
    sty_parts.push(style.to_string());

    let full_class = cls_parts.join(" ");
    let full_style = sty_parts.join(" ");

    view! {
        <div
            id=id
            class=full_class
            style=full_style
            data-testid=data_testid
        >
            {children()}
        </div>
    }
}

/// Renders an accessible `<label>` linked to a form field.
///
/// When nested inside [`Control`], it automatically reads `error`,
/// `disabled`, `focused`, `required`, and `color` from [`ControlContext`].
///
/// # Examples
///
/// ```rust
/// use form_rs::leptos::FormLabel;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn MyLabel() -> impl IntoView {
///     view! { <FormLabel html_for="username">"Username"</FormLabel> }
/// }
/// ```
#[allow(non_snake_case)]
#[component]
pub fn FormLabel(
    /// Label text or content.
    children: Children,
    /// Additional CSS class names on the `<label>`.
    #[prop(default = "")]
    class: &'static str,
    /// Inline CSS on the `<label>`.
    #[prop(default = "")]
    style: &'static str,
    /// `id` attribute on the `<label>`.
    #[prop(default = "")]
    id: &'static str,
    /// The `id` of the labeled `<input>`.
    #[prop(default = "")]
    html_for: &'static str,
    /// Color theme override.
    #[prop(optional)]
    color: Option<Color>,
    /// When `true`, renders in disabled style.
    #[prop(default = false)]
    disabled: bool,
    /// When `true`, renders in error color.
    #[prop(default = false)]
    error: bool,
    /// When `true`, applies the filled modifier.
    #[prop(default = false)]
    filled: bool,
    /// When `true`, applies the focused modifier.
    #[prop(default = false)]
    focused: bool,
    /// When `true`, shows a required asterisk.
    #[prop(default = false)]
    required: bool,
    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let ctx = use_context::<ControlContext>();

    let is_error = error || ctx.is_some_and(|c| c.error);
    let is_disabled = disabled || ctx.is_some_and(|c| c.disabled);
    let is_focused = focused || ctx.is_some_and(|c| c.focused);
    let is_required = required || ctx.is_some_and(|c| c.required);
    let eff_color = color.or_else(|| ctx.map(|c| c.color)).unwrap_or_default();
    let for_attr = if !html_for.is_empty() {
        html_for.to_string()
    } else {
        String::new()
    };

    let color_style = if is_error {
        label_error_style().to_string()
    } else if is_focused {
        eff_color.to_label_color()
    } else {
        String::new()
    };

    let mut cls_parts = vec!["form-label".to_string()];
    if is_error {
        cls_parts.push("form-label--error".to_string());
    }
    if is_disabled {
        cls_parts.push("form-label--disabled".to_string());
    }
    if is_focused {
        cls_parts.push("form-label--focused".to_string());
    }
    if filled {
        cls_parts.push("form-label--filled".to_string());
    }
    cls_parts.push(class.to_string());

    let full_style = format!("{} {} {}", base_label_style(), color_style, style);
    let full_class = cls_parts.join(" ");

    view! {
        <label
            id=id
            class=full_class
            style=full_style
            for=for_attr
            aria-disabled=if is_disabled { "true" } else { "false" }
            data-testid=data_testid
        >
            {children()}
            <Show when=move || is_required>
                <span style=required_asterisk_style() aria-hidden="true">{"*"}</span>
            </Show>
        </label>
    }
}

/// Renders accessible helper text beneath a form field.
///
/// When nested inside [`Control`], it inherits `error` and `disabled`
/// from [`ControlContext`]. Error text uses `role="alert"` for immediate
/// screen-reader announcement.
///
/// # Examples
///
/// ```rust
/// use form_rs::leptos::Helper;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn PasswordHint() -> impl IntoView {
///     view! {
///         <Helper id="pw-hint">"Must be at least 8 characters."</Helper>
///     }
/// }
/// ```
#[allow(non_snake_case)]
#[component]
pub fn Helper(
    /// Helper text content.
    children: Children,
    /// Additional CSS class names on the `<p>`.
    #[prop(default = "")]
    class: &'static str,
    /// Inline CSS on the `<p>`.
    #[prop(default = "")]
    style: &'static str,
    /// `id` attribute, use as `aria-describedby` on the input.
    #[prop(default = "")]
    id: &'static str,
    /// When `true`, renders in disabled style.
    #[prop(default = false)]
    disabled: bool,
    /// When `true`, renders in error color with `role="alert"`.
    #[prop(default = false)]
    error: bool,
    /// When `true`, renders in success color.
    #[prop(default = false)]
    valid: bool,
    /// When `true`, applies the filled modifier.
    #[prop(default = false)]
    filled: bool,
    /// When `true`, applies the focused modifier.
    #[prop(default = false)]
    focused: bool,
    /// Vertical spacing adjustment.
    #[prop(default = Margin::None)]
    margin: Margin,
    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let ctx = use_context::<ControlContext>();

    let is_error = error || ctx.is_some_and(|c| c.error);
    let is_disabled = disabled || ctx.is_some_and(|c| c.disabled);

    let color_style = if is_error {
        helper_error_style()
    } else if valid {
        helper_valid_style()
    } else {
        ""
    };

    let mut cls_parts = vec!["form-helper-text".to_string()];
    if is_error {
        cls_parts.push("form-helper-text--error".to_string());
    }
    if valid {
        cls_parts.push("form-helper-text--valid".to_string());
    }
    if is_disabled {
        cls_parts.push("form-helper-text--disabled".to_string());
    }
    if focused {
        cls_parts.push("form-helper-text--focused".to_string());
    }
    if filled {
        cls_parts.push("form-helper-text--filled".to_string());
    }
    if !margin.to_class().is_empty() {
        cls_parts.push(margin.to_class().to_string());
    }
    cls_parts.push(class.to_string());

    let full_style = format!("{} {} {}", base_helper_text_style(), color_style, style);
    let full_class = cls_parts.join(" ");

    view! {
        <p
            id=id
            class=full_class
            style=full_style
            role=if is_error { "alert" } else { "" }
            aria-live=if is_error { "polite" } else { "" }
            aria-disabled=if is_disabled { "true" } else { "false" }
            data-testid=data_testid
        >
            {children()}
        </p>
    }
}

/// Groups checkboxes or switch controls with optional horizontal layout.
///
/// Renders as `<div role="group">` with `aria_label` or `aria_labelledby`
/// to name the group for screen readers.
///
/// # Examples
///
/// ```rust
/// use form_rs::leptos::Group;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn OptionsGroup() -> impl IntoView {
///     view! {
///         <Group aria_label="Notifications" row=true>
///             <span>"Email"</span>
///             <span>"SMS"</span>
///         </Group>
///     }
/// }
/// ```
#[allow(non_snake_case)]
#[component]
pub fn Group(
    /// Controls such as checkboxes or switches.
    children: Children,
    /// Additional CSS class names.
    #[prop(default = "")]
    class: &'static str,
    /// Inline CSS.
    #[prop(default = "")]
    style: &'static str,
    /// `id` attribute.
    #[prop(default = "")]
    id: &'static str,
    /// When `true`, renders children horizontally.
    #[prop(default = false)]
    row: bool,
    /// When `true`, applies the error modifier class.
    #[prop(default = false)]
    error: bool,
    /// Accessible label for the group.
    #[prop(default = "")]
    aria_label: &'static str,
    /// ID of element that labels the group.
    #[prop(default = "")]
    aria_labelledby: &'static str,
    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let base_style = if row {
        base_form_group_row_style()
    } else {
        base_form_group_style()
    };

    let mut cls_parts = vec!["form-group".to_string()];
    if row {
        cls_parts.push("form-group--row".to_string());
    }
    if error {
        cls_parts.push("form-group--error".to_string());
    }
    cls_parts.push(class.to_string());

    let full_style = format!("{} {}", base_style, style);
    let full_class = cls_parts.join(" ");

    view! {
        <div
            id=id
            class=full_class
            style=full_style
            role="group"
            aria-label=aria_label
            aria-labelledby=aria_labelledby
            data-testid=data_testid
        >
            {children()}
        </div>
    }
}

/// A label wrapper that pairs a control with its descriptive text.
///
/// Renders a `<label>` element containing both the control and its text,
/// with configurable label placement (start, end, top, bottom).
///
/// # Examples
///
/// ```rust
/// use form_rs::leptos::ControlLabel;
/// use form_rs::LabelPlacement;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn TermsCheckbox() -> impl IntoView {
///     view! {
///         <ControlLabel
///             control=view! { <input type="checkbox" /> }.into_any()
///             label=view! { <span>"I agree to the terms"</span> }.into_any()
///         />
///     }
/// }
/// ```
#[allow(non_snake_case)]
#[component]
pub fn ControlLabel(
    /// The control element.
    control: AnyView,
    /// The label text or content.
    #[prop(optional)]
    label: Option<AnyView>,
    /// Additional CSS class names.
    #[prop(default = "")]
    class: &'static str,
    /// Inline CSS.
    #[prop(default = "")]
    style: &'static str,
    /// `id` attribute.
    #[prop(default = "")]
    id: &'static str,
    /// Whether the control appears checked.
    #[prop(default = false)]
    checked: bool,
    /// When `true`, all elements within are visually disabled.
    #[prop(default = false)]
    disabled: bool,
    /// Position of the label text relative to the control.
    #[prop(default = LabelPlacement::End)]
    label_placement: LabelPlacement,
    /// Whether the label indicates a required field.
    #[prop(default = false)]
    required: bool,
    /// The value associated with this labeled control.
    #[prop(default = "")]
    value: &'static str,
    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let flex_style = label_placement.to_flex_direction();
    let placement_class = label_placement.to_class();

    let mut cls_parts = vec![
        "form-control-label".to_string(),
        placement_class.to_string(),
    ];
    if disabled {
        cls_parts.push("form-control-label--disabled".to_string());
    }
    cls_parts.push(class.to_string());

    let base_style = format!(
        "display: inline-flex; {} gap: 8px; align-items: center; cursor: {}; user-select: none; {}",
        flex_style,
        if disabled { "not-allowed" } else { "pointer" },
        style
    );

    let full_class = cls_parts.join(" ");

    let _ = checked;
    let _ = value;

    view! {
        <label
            id=id
            class=full_class
            style=base_style
            aria-disabled=if disabled { "true" } else { "false" }
            aria-required=if required { "true" } else { "false" }
            data-testid=data_testid
        >
            {control}
            <span
                class="form-control-label__label"
                style="font-size: 14px; color: #d4d4d8; line-height: 1.4;"
            >
                {label}
                <Show when=move || required>
                    <span style=required_asterisk_style() aria-hidden="true">{"*"}</span>
                </Show>
            </span>
        </label>
    }
}

/// A convenience composition of [`Control`], [`FormLabel`], `Input`, and
/// [`Helper`] into a single validated form field for Leptos.
///
/// Uses `input-rs` for the underlying `<input>` element with HTML5 validation.
///
/// # Examples
///
/// ```rust
/// use form_rs::leptos::Field;
/// use leptos::prelude::*;
///
/// #[component]
/// pub fn EmailInput() -> impl IntoView {
///     let handle = RwSignal::new(String::new());
///     let valid = RwSignal::new(true);
///     view! {
///         <Field
///             id="email"
///             name="email"
///             r#type="email"
///             label="Email address"
///             placeholder="ferris@opensass.org"
///             required=true
///             handle=handle
///             valid_handle=valid
///             validate_function=|v: String| !v.is_empty() && v.contains('@')
///         />
///     }
/// }
/// ```
#[allow(non_snake_case)]
#[component]
pub fn Field(
    /// The unique `id` for the `<input>` element.
    id: &'static str,
    /// The `name` attribute for the `<input>`.
    #[prop(default = "")]
    name: &'static str,
    /// The input type.
    #[prop(default = "text")]
    r#type: &'static str,
    /// Label text displayed above the input.
    #[prop(default = "")]
    label: &'static str,
    /// Placeholder text.
    #[prop(default = "")]
    placeholder: &'static str,
    /// Helper text below the input.
    #[prop(default = "")]
    helper_text: &'static str,
    /// Validation state.
    #[prop(default = ValidationState::None)]
    validation_state: ValidationState,
    /// Whether the field is required.
    #[prop(default = false)]
    required: bool,
    /// Whether the field is disabled.
    #[prop(default = false)]
    disabled: bool,
    /// Whether the field takes the full container width.
    #[prop(default = true)]
    full_width: bool,
    /// Visual variant.
    #[prop(default = Variant::Outlined)]
    variant: Variant,
    /// Color theme.
    #[prop(default = Color::Primary)]
    color: Color,
    /// Size.
    #[prop(default = Size::Medium)]
    size: Size,
    /// HTML5 `pattern` for native regex validation.
    #[prop(default = ".*")]
    pattern: &'static str,
    /// Max character count.
    #[prop(optional)]
    maxlength: Option<usize>,
    /// Min character count.
    #[prop(optional)]
    minlength: Option<usize>,
    /// Outer `<div>` CSS class.
    #[prop(default = "")]
    class: &'static str,
    /// Outer `<div>` inline CSS.
    #[prop(default = "")]
    style: &'static str,
    /// Controlled value signal.
    handle: RwSignal<String>,
    /// Validity signal.
    valid_handle: RwSignal<bool>,
    /// Validation function.
    validate_function: fn(String) -> bool,
    /// `data-testid` for automated testing.
    #[prop(default = "")]
    data_testid: &'static str,
) -> impl IntoView {
    let focused = RwSignal::new(false);

    let vs1 = validation_state.clone();
    let is_error =
        Memo::new(move |_| vs1.is_invalid() || (!valid_handle.get() && !handle.get().is_empty()));

    let vs2 = validation_state.clone();
    let is_valid = Memo::new(move |_| {
        matches!(vs2, ValidationState::Valid) || (valid_handle.get() && !handle.get().is_empty())
    });

    let error_msg = validation_state.error_message().map(|s| s.to_string());
    let helper_id = format!("{}-helper", id);

    let input_style = move || {
        let focus_ring = if focused.get() && !is_error.get() {
            color.to_focus_ring()
        } else {
            String::new()
        };
        let error_ring = if is_error.get() {
            field_error_style().to_string()
        } else {
            String::new()
        };
        let valid_ring = if is_valid.get() && !is_error.get() {
            field_valid_style().to_string()
        } else {
            String::new()
        };
        format!(
            "{} {} {} {} {} {} transition: all 0.2s ease;",
            base_input_field_style(),
            variant.to_field_style(),
            size.to_input_style(),
            focus_ring,
            error_ring,
            valid_ring
        )
    };

    let has_helper = !helper_text.is_empty() || error_msg.is_some();
    let input_style_static: &'static str = Box::leak(input_style().into_boxed_str());
    let helper_id_static: &'static str = Box::leak(helper_id.clone().into_boxed_str());
    let (handle_read, handle_write) = handle.split();
    let (valid_read, valid_write) = valid_handle.split();

    view! {
        <Control
            id=""
            input_id=id
            error=is_error.get()
            disabled=disabled
            focused=focused.get()
            full_width=full_width
            required=required
            variant=variant
            color=color
            size=size
            class=class
            style=style
            data_testid=data_testid
        >
            <Show when=move || !label.is_empty()>
                <FormLabel
                    html_for=id
                    error=is_error.get()
                    focused=focused.get()
                    required=required
                    disabled=disabled
                >
                    {label}
                </FormLabel>
            </Show>
            <Input
                r#type=r#type
                id=id
                name=name
                placeholder=placeholder
                handle=(handle_read, handle_write)
                valid_handle=(valid_read, valid_write)
                validate_function=validate_function
                required=required
                disabled=disabled
                pattern=pattern
                maxlength=maxlength
                minlength=minlength
                input_style=input_style_static
                aria_describedby=helper_id_static
                aria_required=if required { "true" } else { "false" }
                aria_invalid=if is_error.get() { "true" } else { "false" }
                otp_mode=true
                on_focus=Callback::new(move |_: FocusEvent| focused.set(true))
                on_blur=Callback::new(move |_: FocusEvent| focused.set(false))
            />
            <Show when=move || has_helper>
                {
                    let content = if let Some(ref msg) = error_msg {
                        msg.clone()
                    } else if is_error.get() {
                        "Invalid value.".to_string()
                    } else {
                        helper_text.to_string()
                    };
                    view! {
                        <Helper
                            id=helper_id_static
                            error=is_error.get()
                            valid=is_valid.get() && !is_error.get()
                        >
                            {content}
                        </Helper>
                    }
                }
            </Show>
        </Control>
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
