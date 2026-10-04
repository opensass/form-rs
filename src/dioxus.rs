// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc = include_str!("../DIOXUS.md")]

use crate::common::{
    Color, EncType, LabelPlacement, Margin, Method, Size, Target, ValidationBehavior,
    ValidationState, Variant, base_form_control_style, base_form_group_row_style,
    base_form_group_style, base_form_style, base_helper_text_style, base_input_field_style,
    base_label_style, field_disabled_style, field_error_style, field_valid_style,
    helper_error_style, helper_valid_style, label_error_style, required_asterisk_style,
};
use dioxus::prelude::*;
pub use input_rs::dioxus::Input;

/// Shared context propagated by [`Form`] to all descendant components.
#[derive(Clone, Debug, PartialEq)]
pub struct FormContext {
    /// Active validation strategy for the entire form.
    pub validation_behavior: ValidationBehavior,
    /// `true` when the form is actively being submitted.
    pub is_submitting: bool,
    /// Whether native browser validation is suppressed.
    pub novalidate: bool,
}

/// Shared context provided by [`Control`] to its children.
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
    pub input_id: &'static str,
}

/// Props for the [`Form`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct FormProps {
    #[props(default)]
    pub children: Element,
    #[props(default)]
    pub class: &'static str,
    #[props(default)]
    pub style: &'static str,
    #[props(default)]
    pub id: &'static str,
    #[props(default)]
    pub action: &'static str,
    #[props(default)]
    pub method: Method,
    #[props(default)]
    pub enc_type: EncType,
    #[props(default)]
    pub target: Target,
    #[props(default)]
    pub novalidate: bool,
    #[props(default = "on")]
    pub autocomplete: &'static str,
    #[props(default)]
    pub name: &'static str,
    #[props(default)]
    pub validation_behavior: ValidationBehavior,
    #[props(default)]
    pub on_submit: Option<Callback<FormEvent>>,
    #[props(default)]
    pub on_reset: Option<Callback<FormEvent>>,
    #[props(default)]
    pub aria_label: &'static str,
    #[props(default)]
    pub aria_labelledby: &'static str,
    #[props(default)]
    pub data_testid: &'static str,
}

/// A semantic `<form>` wrapper that provides validation context to descendant fields.
///
/// Propagates [`FormContext`] via Dioxus context so that [`Control`],
/// [`FormLabel`], and [`Helper`] can read validation mode and submission
/// state without prop drilling.
///
/// # Accessibility
///
/// - Renders as a native `<form>` element (implicit `"form"` ARIA role).
/// - Supply `aria_label` or `aria_labelledby` to create a named landmark.
#[component]
pub fn Form(props: FormProps) -> Element {
    let mut is_submitting = use_signal(|| false);

    let novalidate = props.novalidate || props.validation_behavior == ValidationBehavior::Aria;

    let ctx = FormContext {
        validation_behavior: props.validation_behavior,
        is_submitting: is_submitting(),
        novalidate,
    };

    use_context_provider(|| ctx);

    let on_submit = move |e: FormEvent| {
        is_submitting.set(true);
        if let Some(cb) = &props.on_submit {
            cb.call(e);
        }
        is_submitting.set(false);
    };

    let on_reset = move |e: FormEvent| {
        if let Some(cb) = &props.on_reset {
            cb.call(e);
        }
    };

    let full_style = format!("{} {}", base_form_style(), props.style);

    rsx! {
        form {
            id: props.id,
            class: "form {props.class}",
            style: "{full_style}",
            action: props.action,
            method: props.method.as_str(),
            enctype: props.enc_type.as_str(),
            target: props.target.as_str(),
            novalidate: novalidate,
            autocomplete: props.autocomplete,
            name: props.name,
            onsubmit: on_submit,
            onreset: on_reset,
            aria_label: props.aria_label,
            aria_labelledby: props.aria_labelledby,
            "data-testid": props.data_testid,
            {props.children}
        }
    }
}

/// Props for the [`Control`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct ControlProps {
    #[props(default)]
    pub children: Element,
    #[props(default)]
    pub class: &'static str,
    #[props(default)]
    pub style: &'static str,
    #[props(default)]
    pub id: &'static str,
    #[props(default)]
    pub input_id: &'static str,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub error: bool,
    #[props(default)]
    pub focused: bool,
    #[props(default)]
    pub full_width: bool,
    #[props(default)]
    pub hidden_label: bool,
    #[props(default)]
    pub margin: Margin,
    #[props(default)]
    pub required: bool,
    #[props(default)]
    pub size: Size,
    #[props(default)]
    pub variant: Variant,
    #[props(default)]
    pub color: Color,
    #[props(default)]
    pub data_testid: &'static str,
}

/// A context provider that wraps a single form field with its label and helper text.
///
/// Provides [`ControlContext`] to [`FormLabel`] and [`Helper`]
/// descendants so they reflect error, disabled, required, variant, and color
/// states automatically.
#[component]
pub fn Control(props: ControlProps) -> Element {
    let ctx = ControlContext {
        error: props.error,
        disabled: props.disabled,
        focused: props.focused,
        required: props.required,
        filled: false,
        variant: props.variant,
        color: props.color,
        size: props.size,
        input_id: props.input_id,
    };

    use_context_provider(|| ctx);

    let mut cls = format!(
        "form-control {} {} {}",
        props.variant.to_class(),
        props.size.to_class(),
        props.margin.to_class()
    );
    if props.full_width {
        cls.push_str(" form-control--full-width");
    }
    if props.error {
        cls.push_str(" form-control--error");
    }
    if props.disabled {
        cls.push_str(" form-control--disabled");
    }
    if props.focused {
        cls.push_str(" form-control--focused");
    }
    if props.hidden_label {
        cls.push_str(" form-control--hidden-label");
    }
    cls.push(' ');
    cls.push_str(props.class);

    let mut sty = format!("{} {} ", base_form_control_style(), props.margin.to_style());
    if props.full_width {
        sty.push_str("width: 100%; ");
    }
    if props.disabled {
        sty.push_str(field_disabled_style());
    }
    sty.push_str(props.style);

    rsx! {
        div {
            id: props.id,
            class: "{cls}",
            style: "{sty}",
            "data-testid": props.data_testid,
            {props.children}
        }
    }
}

/// Props for the [`FormLabel`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct FormLabelProps {
    #[props(default)]
    pub children: Element,
    #[props(default)]
    pub class: &'static str,
    #[props(default)]
    pub style: &'static str,
    #[props(default)]
    pub id: &'static str,
    #[props(default)]
    pub html_for: &'static str,
    #[props(default = None)]
    pub color: Option<Color>,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub error: bool,
    #[props(default)]
    pub filled: bool,
    #[props(default)]
    pub focused: bool,
    #[props(default)]
    pub required: bool,
    #[props(default)]
    pub data_testid: &'static str,
}

/// Renders an accessible `<label>` linked to a form field.
///
/// When nested inside [`Control`], it automatically reads `error`,
/// `disabled`, `focused`, `required`, and `color` from [`ControlContext`].
#[component]
pub fn FormLabel(props: FormLabelProps) -> Element {
    let ctx = try_consume_context::<ControlContext>();

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
        ctx.as_ref()
            .map(|c| c.input_id.to_string())
            .unwrap_or_default()
    };

    let color_style = if error {
        label_error_style().to_string()
    } else if focused {
        color.to_label_color()
    } else {
        String::new()
    };

    let mut cls = "form-label".to_string();
    if error {
        cls.push_str(" form-label--error");
    }
    if disabled {
        cls.push_str(" form-label--disabled");
    }
    if focused {
        cls.push_str(" form-label--focused");
    }
    if props.filled {
        cls.push_str(" form-label--filled");
    }
    cls.push(' ');
    cls.push_str(props.class);

    let full_style = format!("{} {} {}", base_label_style(), color_style, props.style);

    rsx! {
        label {
            id: props.id,
            class: "{cls}",
            style: "{full_style}",
            r#for: "{linked_id}",
            aria_disabled: if disabled { "true" } else { "false" },
            "data-testid": props.data_testid,
            {props.children}
            if required {
                span {
                    style: required_asterisk_style(),
                    aria_hidden: "true",
                    "*"
                }
            }
        }
    }
}

/// Props for the [`Helper`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct HelperProps {
    #[props(default)]
    pub children: Element,
    #[props(default)]
    pub class: &'static str,
    #[props(default)]
    pub style: &'static str,
    #[props(default)]
    pub id: &'static str,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub error: bool,
    #[props(default)]
    pub valid: bool,
    #[props(default)]
    pub filled: bool,
    #[props(default)]
    pub focused: bool,
    #[props(default)]
    pub margin: Margin,
    #[props(default)]
    pub data_testid: &'static str,
}

/// Renders accessible helper text beneath a form field.
///
/// When nested inside [`Control`], it inherits `error` and `disabled`
/// states from [`ControlContext`]. Error text is announced via `role="alert"`.
#[component]
pub fn Helper(props: HelperProps) -> Element {
    let ctx = try_consume_context::<ControlContext>();

    let error = props.error || ctx.as_ref().is_some_and(|c| c.error);
    let disabled = props.disabled || ctx.as_ref().is_some_and(|c| c.disabled);

    let color_style = if error {
        helper_error_style()
    } else if props.valid {
        helper_valid_style()
    } else {
        ""
    };

    let mut cls = "form-helper-text".to_string();
    if error {
        cls.push_str(" form-helper-text--error");
    }
    if props.valid {
        cls.push_str(" form-helper-text--valid");
    }
    if disabled {
        cls.push_str(" form-helper-text--disabled");
    }
    if props.focused {
        cls.push_str(" form-helper-text--focused");
    }
    if props.filled {
        cls.push_str(" form-helper-text--filled");
    }
    if !props.margin.to_class().is_empty() {
        cls.push_str(&format!(" {}", props.margin.to_class()));
    }
    cls.push(' ');
    cls.push_str(props.class);

    let full_style = format!(
        "{} {} {}",
        base_helper_text_style(),
        color_style,
        props.style
    );

    rsx! {
        p {
            id: props.id,
            class: "{cls}",
            style: "{full_style}",
            role: if error { "alert" } else { "" },
            aria_live: if error { "polite" } else { "" },
            aria_disabled: if disabled { "true" } else { "false" },
            "data-testid": props.data_testid,
            {props.children}
        }
    }
}

/// Props for the [`Group`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct GroupProps {
    #[props(default)]
    pub children: Element,
    #[props(default)]
    pub class: &'static str,
    #[props(default)]
    pub style: &'static str,
    #[props(default)]
    pub id: &'static str,
    #[props(default)]
    pub row: bool,
    #[props(default)]
    pub error: bool,
    #[props(default = "")]
    pub aria_label: &'static str,
    #[props(default)]
    pub aria_labelledby: &'static str,
    #[props(default)]
    pub data_testid: &'static str,
}

/// Groups checkboxes or switch controls with optional horizontal layout.
///
/// Renders as `<div role="group">` with `aria_label` or `aria_labelledby`
/// to name the group for screen readers.
#[component]
pub fn Group(props: GroupProps) -> Element {
    let base_style = if props.row {
        base_form_group_row_style()
    } else {
        base_form_group_style()
    };

    let mut cls = "form-group".to_string();
    if props.row {
        cls.push_str(" form-group--row");
    }
    if props.error {
        cls.push_str(" form-group--error");
    }
    cls.push(' ');
    cls.push_str(props.class);

    let full_style = format!("{} {}", base_style, props.style);

    rsx! {
        div {
            id: props.id,
            class: "{cls}",
            style: "{full_style}",
            role: "group",
            aria_label: props.aria_label,
            aria_labelledby: props.aria_labelledby,
            "data-testid": props.data_testid,
            {props.children}
        }
    }
}

/// Props for the [`ControlLabel`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
pub struct ControlLabelProps {
    /// The control element, e.g. an `<input type="checkbox">`.
    pub control: Element,
    /// The label text or content.
    pub label: Option<Element>,
    #[props(default)]
    pub class: &'static str,
    #[props(default)]
    pub style: &'static str,
    #[props(default)]
    pub id: &'static str,
    #[props(default)]
    pub checked: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub label_placement: LabelPlacement,
    #[props(default)]
    pub required: bool,
    #[props(default)]
    pub value: &'static str,
    #[props(default)]
    pub data_testid: &'static str,
}

/// A label wrapper that pairs a control with its descriptive text.
///
/// Renders a `<label>` element containing both the control and its text,
/// with configurable label placement (start, end, top, bottom).
#[component]
pub fn ControlLabel(props: ControlLabelProps) -> Element {
    let flex_style = props.label_placement.to_flex_direction();
    let placement_class = props.label_placement.to_class();

    let mut cls = format!("form-control-label {} ", placement_class);
    if props.disabled {
        cls.push_str("form-control-label--disabled ");
    }
    cls.push_str(props.class);

    let base_style = format!(
        "display: inline-flex; {} gap: 8px; align-items: center; cursor: {}; user-select: none; {} {}",
        flex_style,
        if props.disabled {
            "not-allowed"
        } else {
            "pointer"
        },
        props.style,
        ""
    );

    rsx! {
        label {
            id: props.id,
            class: "{cls}",
            style: "{base_style}",
            aria_disabled: if props.disabled { "true" } else { "false" },
            aria_required: if props.required { "true" } else { "false" },
            "data-testid": props.data_testid,
            {props.control}
            span {
                class: "form-control-label__label",
                style: "font-size: 14px; color: #d4d4d8; line-height: 1.4;",
                {props.label}
                if props.required {
                    span {
                        style: required_asterisk_style(),
                        aria_hidden: "true",
                        "*"
                    }
                }
            }
        }
    }
}

/// Props for the [`Field`] Dioxus component.
#[derive(Props, Clone, PartialEq)]
#[allow(unpredictable_function_pointer_comparisons)]
pub struct FieldProps {
    pub id: &'static str,
    #[props(default)]
    pub name: &'static str,
    #[props(default = "text")]
    pub r#type: &'static str,
    #[props(default)]
    pub label: &'static str,
    #[props(default)]
    pub placeholder: &'static str,
    #[props(default)]
    pub helper_text: &'static str,
    #[props(default)]
    pub validation_state: ValidationState,
    #[props(default)]
    pub required: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(default = true)]
    pub full_width: bool,
    #[props(default)]
    pub variant: Variant,
    #[props(default)]
    pub color: Color,
    #[props(default)]
    pub size: Size,
    #[props(default = ".*")]
    pub pattern: &'static str,
    #[props(default = None)]
    pub maxlength: Option<usize>,
    #[props(default = None)]
    pub minlength: Option<usize>,
    #[props(default)]
    pub class: &'static str,
    #[props(default)]
    pub style: &'static str,
    pub handle: Signal<String>,
    pub valid_handle: Signal<bool>,
    pub validate_function: fn(String) -> bool,
    #[props(default)]
    pub data_testid: &'static str,
}

/// A convenience composition of [`Control`], [`FormLabel`], [`Input`], and
/// [`Helper`] into a single validated form field for Dioxus.
///
/// Uses `input-rs` for the underlying `<input>` element with HTML5 validation.
#[component]
pub fn Field(props: FieldProps) -> Element {
    let mut focused = use_signal(|| false);

    let is_error = props.validation_state.is_invalid()
        || (!(props.valid_handle)() && !(props.handle)().is_empty());
    let is_valid = matches!(props.validation_state, ValidationState::Valid)
        || ((props.valid_handle)() && !(props.handle)().is_empty());

    let error_msg = props
        .validation_state
        .error_message()
        .map(|s| s.to_string());

    let helper_id = format!("{}-helper", props.id);

    let focus_ring = if focused() && !is_error {
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
        props.variant.to_field_style(),
        props.size.to_input_style(),
        focus_ring,
        error_ring,
        valid_ring
    );

    let input_style_static: &'static str = Box::leak(input_style.into_boxed_str());
    let helper_id_str: &'static str = Box::leak(helper_id.clone().into_boxed_str());
    let helper_id_clone_str: &'static str = helper_id_str;

    let content_str: &'static str = {
        let s = if let Some(ref msg) = error_msg {
            msg.clone()
        } else if is_error {
            "Invalid value.".to_string()
        } else if !props.helper_text.is_empty() {
            props.helper_text.to_string()
        } else {
            String::new()
        };
        Box::leak(s.into_boxed_str())
    };
    let has_helper = !content_str.is_empty();
    let label_str: &'static str = Box::leak(props.label.to_string().into_boxed_str());
    let has_label: bool = !props.label.is_empty();
    let is_focused: bool = focused();
    let aria_required_str: &'static str = if props.required { "true" } else { "false" };
    let aria_invalid_str: &'static str = if is_error { "true" } else { "false" };

    rsx! {
        Control {
            id: "",
            input_id: props.id,
            error: is_error,
            disabled: props.disabled,
            focused: is_focused,
            full_width: props.full_width,
            required: props.required,
            variant: props.variant,
            color: props.color,
            size: props.size,
            class: props.class,
            style: props.style,
            data_testid: props.data_testid,
            if has_label {
                FormLabel {
                    html_for: props.id,
                    error: is_error,
                    focused: is_focused,
                    required: props.required,
                    disabled: props.disabled,
                    "{label_str}"
                }
            }
            Input {
                r#type: props.r#type,
                id: props.id,
                name: props.name,
                placeholder: props.placeholder,
                handle: props.handle,
                valid_handle: props.valid_handle,
                validate_function: props.validate_function,
                required: props.required,
                disabled: props.disabled,
                pattern: props.pattern,
                maxlength: props.maxlength,
                minlength: props.minlength,
                input_style: input_style_static,
                aria_describedby: helper_id_clone_str,
                aria_required: aria_required_str,
                aria_invalid: aria_invalid_str,
                otp_mode: true,
                on_focus: move |_| focused.set(true),
                on_blur: move |_| focused.set(false),
            }
            if has_helper {
                Helper {
                    id: helper_id_str,
                    error: is_error,
                    valid: is_valid && !is_error,
                    "{content_str}"
                }
            }
        }
    }
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
