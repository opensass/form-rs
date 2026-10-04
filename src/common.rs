// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Controls whether validation errors are enforced natively by the browser or
/// communicated via ARIA attributes for realtime display.
///
/// # Default
///
/// [`ValidationBehavior::Native`] is the default, which blocks form submission
/// when fields are invalid using the browser's built-in constraint validation.
///
/// # Examples
///
/// ```rust
/// use form_rs::ValidationBehavior;
///
/// let b = ValidationBehavior::Aria;
/// assert_eq!(b.as_str(), "aria");
/// ```
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum ValidationBehavior {
    /// Uses native HTML5 constraint validation. Blocks submission on errors.
    #[default]
    Native,

    /// Uses ARIA attributes for validation display. Errors shown in realtime
    /// as the user types; does not block form submission.
    Aria,
}

impl ValidationBehavior {
    /// Returns the string representation used in internal logic.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Aria => "aria",
        }
    }

    /// Returns `true` when native browser validation is active.
    pub fn is_native(self) -> bool {
        self == Self::Native
    }
}

/// The MIME encoding type for form data on submission.
///
/// # Default
///
/// [`EncType::UrlEncoded`] is the default, matching the HTML `<form>` default.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum EncType {
    /// `application/x-www-form-urlencoded`, the default.
    #[default]
    UrlEncoded,

    /// `multipart/form-data`, required when the form contains file inputs.
    MultipartFormData,

    /// `text/plain`, useful for debugging; not recommended for production.
    TextPlain,
}

impl EncType {
    /// Returns the MIME type string for the `enctype` HTML attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UrlEncoded => "application/x-www-form-urlencoded",
            Self::MultipartFormData => "multipart/form-data",
            Self::TextPlain => "text/plain",
        }
    }
}

/// The HTTP method used when submitting a form.
///
/// # Default
///
/// [`Method::Get`] is the default, matching the HTML `<form>` default.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Method {
    /// Appends form data to the URL as query parameters. No side effects.
    #[default]
    Get,

    /// Sends form data in the request body. Use for side-effecting operations.
    Post,

    /// Closes the enclosing `<dialog>` on submission without sending data.
    Dialog,
}

impl Method {
    /// Returns the lowercase string for the HTML `method` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Post => "post",
            Self::Dialog => "dialog",
        }
    }
}

/// Where the browser displays the response after form submission.
///
/// # Default
///
/// [`Target::Self_`] is the default, loading the response in the same frame.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Target {
    /// Load into the same browsing context. Default.
    #[default]
    Self_,

    /// Load into a new unnamed browsing context (new tab/window).
    Blank,

    /// Load into the parent browsing context.
    Parent,

    /// Load into the top-level browsing context.
    Top,

    /// Arbitrary `target` name for named frames.
    Custom(&'static str),
}

impl Target {
    /// Returns the string for the HTML `target` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Self_ => "_self",
            Self::Blank => "_blank",
            Self::Parent => "_parent",
            Self::Top => "_top",
            Self::Custom(s) => s,
        }
    }
}

/// Lifecycle state of a [`Form`] submission.
///
/// Tracks the current submission cycle from idle through completion or error.
///
/// # Default
///
/// [`FormStatus::Idle`] is the default variant.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum FormStatus {
    /// No submission is in progress.
    #[default]
    Idle,

    /// A submission is currently in progress.
    Submitting,

    /// The last submission completed successfully.
    Submitted,

    /// The last submission ended with an error.
    Error,
}

impl FormStatus {
    /// Returns `true` when a submission is actively running.
    pub fn is_submitting(self) -> bool {
        self == Self::Submitting
    }
}

/// Visual style variant for [`Control`]-wrapped input fields.
///
/// # Default
///
/// [`Variant::Outlined`] is the default, rendering a bordered box style.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Variant {
    /// Outlined field with a visible border. Default.
    #[default]
    Outlined,

    /// Filled field with a background tint, no bottom border by default.
    Filled,

    /// Minimal underline-only style.
    Standard,
}

impl Variant {
    /// Returns the BEM modifier class for this variant.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::Outlined => "form-control--outlined",
            Self::Filled => "form-control--filled",
            Self::Standard => "form-control--standard",
        }
    }

    /// Returns the inline border CSS for a field in this variant.
    pub fn to_field_style(self) -> &'static str {
        match self {
            Self::Outlined => {
                "border: 1.5px solid #3f3f46; border-radius: 8px; background: transparent;"
            }
            Self::Filled => {
                "border: none; border-bottom: 1.5px solid #3f3f46; border-radius: 8px 8px 0 0; background: rgba(255,255,255,0.05);"
            }
            Self::Standard => {
                "border: none; border-bottom: 1.5px solid #3f3f46; border-radius: 0; background: transparent;"
            }
        }
    }
}

/// Color theme applied to a [`Control`] and its label/helper text.
///
/// # Default
///
/// [`Color::Primary`] is the default.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Color {
    /// Primary purple accent: `#7c3aed`.
    #[default]
    Primary,

    /// Secondary blue accent: `#3b82f6`.
    Secondary,

    /// Error red: `#dc2626`.
    Error,

    /// Info cyan: `#06b6d4`.
    Info,

    /// Success green: `#16a34a`.
    Success,

    /// Warning amber: `#d97706`.
    Warning,

    /// Arbitrary inline CSS color value, e.g. `"#ff6b6b"`.
    Custom(&'static str),
}

impl Color {
    /// Returns the hex color string for this variant.
    pub fn to_hex(self) -> &'static str {
        match self {
            Self::Primary => "#7c3aed",
            Self::Secondary => "#3b82f6",
            Self::Error => "#dc2626",
            Self::Info => "#06b6d4",
            Self::Success => "#16a34a",
            Self::Warning => "#d97706",
            Self::Custom(s) => s,
        }
    }

    /// Returns the focus ring box-shadow CSS for this color.
    pub fn to_focus_ring(self) -> String {
        format!(
            "border-color: {}; box-shadow: 0 0 0 3px {}40;",
            self.to_hex(),
            self.to_hex()
        )
    }

    /// Returns the label color CSS when the field is focused.
    pub fn to_label_color(self) -> String {
        format!("color: {};", self.to_hex())
    }
}

/// Size of a [`Control`] and its contained input field.
///
/// # Default
///
/// [`Size::Medium`] is the default.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Size {
    /// Compact size: smaller padding and font.
    Small,

    /// Standard size. Default.
    #[default]
    Medium,
}

impl Size {
    /// Returns the inline padding CSS for an input field at this size.
    pub fn to_input_style(self) -> &'static str {
        match self {
            Self::Small => "padding: 6px 10px; font-size: 13px;",
            Self::Medium => "padding: 10px 14px; font-size: 15px;",
        }
    }

    /// Returns the BEM modifier class for this size.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::Small => "form-control--small",
            Self::Medium => "form-control--medium",
        }
    }
}

/// Vertical spacing adjustment for a [`Control`].
///
/// # Default
///
/// [`Margin::None`] is the default.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum Margin {
    /// No additional vertical margin.
    #[default]
    None,

    /// Reduced vertical margin for denser layouts.
    Dense,

    /// Standard vertical margin matching the form baseline rhythm.
    Normal,
}

impl Margin {
    /// Returns the inline margin CSS for this spacing level.
    pub fn to_style(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Dense => "margin-top: 4px; margin-bottom: 4px;",
            Self::Normal => "margin-top: 8px; margin-bottom: 4px;",
        }
    }

    /// Returns the BEM modifier class for this margin.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Dense => "form-control--margin-dense",
            Self::Normal => "form-control--margin-normal",
        }
    }
}

/// Position of a label relative to its control in [`ControlLabel`].
///
/// # Default
///
/// [`LabelPlacement::End`] is the default (label to the right of the control).
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum LabelPlacement {
    /// Label placed at the end (right) of the control. Default.
    #[default]
    End,

    /// Label placed at the start (left) of the control.
    Start,

    /// Label placed above the control.
    Top,

    /// Label placed below the control.
    Bottom,
}

impl LabelPlacement {
    /// Returns the flex-direction CSS for the label placement.
    pub fn to_flex_direction(self) -> &'static str {
        match self {
            Self::End => "flex-direction: row; align-items: center;",
            Self::Start => "flex-direction: row-reverse; align-items: center;",
            Self::Top => "flex-direction: column-reverse; align-items: flex-start;",
            Self::Bottom => "flex-direction: column; align-items: flex-start;",
        }
    }

    /// Returns the BEM modifier class for this label placement.
    pub fn to_class(self) -> &'static str {
        match self {
            Self::End => "form-control-label--end",
            Self::Start => "form-control-label--start",
            Self::Top => "form-control-label--top",
            Self::Bottom => "form-control-label--bottom",
        }
    }
}

/// Validation state for a field, carrying an optional error message.
///
/// # Default
///
/// [`ValidationState::None`] is the default, no active validation display.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ValidationState {
    /// No validation state applied.
    #[default]
    None,

    /// The field value is valid.
    Valid,

    /// The field value is invalid; the message is shown as helper text.
    Invalid(String),
}

impl ValidationState {
    /// Returns `true` when the field is in an error state.
    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }

    /// Returns the error message if the state is [`ValidationState::Invalid`].
    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Invalid(msg) => Some(msg.as_str()),
            _ => None,
        }
    }
}

/// Returns the base inline CSS for the `<form>` element.
pub fn base_form_style() -> &'static str {
    "display: flex; flex-direction: column; gap: 16px;"
}

/// Returns the base inline CSS for a [`Control`] container `<div>`.
pub fn base_form_control_style() -> &'static str {
    "display: flex; flex-direction: column; gap: 4px; position: relative; width: 100%;"
}

/// Returns the base inline CSS for a [`FormLabel`] element.
pub fn base_label_style() -> &'static str {
    "font-size: 13px; font-weight: 500; color: #a1a1aa; line-height: 1.4; \
transition: color 0.15s ease; letter-spacing: 0.01em;"
}

/// Returns the base inline CSS for a [`Helper`] element.
pub fn base_helper_text_style() -> &'static str {
    "font-size: 11.5px; color: #71717a; margin: 0; line-height: 1.5; \
transition: color 0.15s ease;"
}

/// Returns the base inline CSS for a [`Group`] container.
pub fn base_form_group_style() -> &'static str {
    "display: flex; flex-direction: column; gap: 8px;"
}

/// Returns the base inline CSS for a row-layout [`Group`].
pub fn base_form_group_row_style() -> &'static str {
    "display: flex; flex-direction: row; flex-wrap: wrap; gap: 16px; align-items: center;"
}

/// Returns the base inline CSS for an input field within a [`Control`].
pub fn base_input_field_style() -> &'static str {
    "width: 100%; background: transparent; outline: none; \
font-family: 'Inter', ui-sans-serif, system-ui, sans-serif; \
transition: border-color 0.2s ease, box-shadow 0.2s ease, background-color 0.2s ease; \
box-sizing: border-box;"
}

/// Returns the inline CSS applied to an input field in its error state.
pub fn field_error_style() -> &'static str {
    "border-color: #dc2626 !important; box-shadow: 0 0 0 3px rgba(220,38,38,0.18);"
}

/// Returns the inline CSS applied to an input field in its valid state.
pub fn field_valid_style() -> &'static str {
    "border-color: #16a34a; box-shadow: 0 0 0 3px rgba(22,163,74,0.15);"
}

/// Returns the inline CSS applied to a disabled input field.
pub fn field_disabled_style() -> &'static str {
    "opacity: 0.5; cursor: not-allowed; pointer-events: none;"
}

/// Returns the error label color CSS string.
pub fn label_error_style() -> &'static str {
    "color: #dc2626;"
}

/// Returns the valid label color CSS string.
pub fn label_valid_style() -> &'static str {
    "color: #16a34a;"
}

/// Returns the helper text error CSS string.
pub fn helper_error_style() -> &'static str {
    "color: #dc2626; font-weight: 500;"
}

/// Returns the helper text valid CSS string.
pub fn helper_valid_style() -> &'static str {
    "color: #16a34a;"
}

/// Returns the inline CSS for the required asterisk `*` suffix on labels.
pub fn required_asterisk_style() -> &'static str {
    "color: #dc2626; margin-inline-start: 2px;"
}

// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
