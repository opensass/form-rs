// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use dioxus::prelude::*;
use form_rs::dioxus::{
    Form, Control, ControlLabel, Field, Group, Helper, FormLabel, Input
};
use form_rs::{Color, LabelPlacement, Method, Size, ValidationBehavior, ValidationState, Variant};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        LandingPage {}
    }
}

fn validate_email(v: String) -> bool {
    !v.is_empty() && v.contains('@') && v.contains('.')
}

fn validate_password(v: String) -> bool {
    v.len() >= 8
}

fn validate_required(v: String) -> bool {
    !v.trim().is_empty()
}

fn validate_url(v: String) -> bool {
    v.starts_with("https://") || v.starts_with("http://")
}

fn validate_any(_v: String) -> bool {
    true
}

#[component]
fn ExampleBasicLogin() -> Element {
    let email = use_signal(String::new);
    let email_valid = use_signal(|| true);
    let password = use_signal(String::new);
    let password_valid = use_signal(|| true);
    let mut submitted = use_signal(|| false);

    rsx! {
        Form {
            method: Method::Post,
            on_submit: Some(Callback::new(move |e: FormEvent| {
                e.prevent_default();
                submitted.set(true);
            })),
            aria_label: "Basic login form",
            Field {
                id: "dx-login-email",
                name: "email",
                r#type: "email",
                label: "Email address",
                placeholder: "ferris@opensass.org",
                helper_text: "Enter your registered email.",
                required: true,
                full_width: true,
                handle: email,
                valid_handle: email_valid,
                validate_function: validate_email,
            }
            Field {
                id: "dx-login-password",
                name: "password",
                r#type: "password",
                label: "Password",
                placeholder: "*******",
                required: true,
                full_width: true,
                handle: password,
                valid_handle: password_valid,
                validate_function: validate_password,
            }
            button {
                r#type: "submit",
                style: "width:100%;padding:10px 0;border-radius:8px;\
                        background:#7c3aed;color:white;border:none;cursor:pointer;\
                        font-size:15px;font-weight:600;transition:opacity 0.2s;",
                "Sign in"
            }
            if submitted() {
                p { style: "color:#22c55e;font-size:12px;text-align:center;margin:0;",
                    "✓ Form submitted!"
                }
            }
        }
    }
}

#[component]
fn ExampleRegistration() -> Element {
    let username = use_signal(String::new);
    let username_valid = use_signal(|| true);
    let email = use_signal(String::new);
    let email_valid = use_signal(|| true);
    let password = use_signal(String::new);
    let password_valid = use_signal(|| true);
    let mut done = use_signal(|| false);

    rsx! {
        Form {
            method: Method::Post,
            on_submit: Some(Callback::new(move |e: FormEvent| {
                e.prevent_default();
                done.set(true);
            })),
            aria_label: "Registration form",
            Field {
                id: "dx-reg-username",
                name: "username",
                label: "Username",
                placeholder: "opensass_org",
                required: true,
                full_width: true,
                pattern: "[a-zA-Z0-9_]{{3,20}}",
                handle: username,
                valid_handle: username_valid,
                validate_function: validate_required,
            }
            Field {
                id: "dx-reg-email",
                name: "email",
                r#type: "email",
                label: "Email",
                placeholder: "ferris@opensass.org",
                required: true,
                full_width: true,
                handle: email,
                valid_handle: email_valid,
                validate_function: validate_email,
            }
            Field {
                id: "dx-reg-password",
                name: "password",
                r#type: "password",
                label: "Password",
                placeholder: "At least 8 characters",
                helper_text: "Must be 8+ characters.",
                required: true,
                full_width: true,
                minlength: Some(8),
                handle: password,
                valid_handle: password_valid,
                validate_function: validate_password,
            }
            button {
                r#type: "submit",
                style: "width:100%;padding:10px 0;border-radius:8px;\
                        background:#7c3aed;color:white;border:none;cursor:pointer;\
                        font-size:15px;font-weight:600;",
                "Create account"
            }
            if done() {
                p { style: "color:#22c55e;font-size:12px;text-align:center;margin:0;",
                    "✓ Account created!"
                }
            }
        }
    }
}

#[component]
fn ExampleValidationStates() -> Element {
    let v = use_signal(String::new);
    let vv = use_signal(|| true);
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:12px;width:100%;",
            Control { input_id: "dx-vs-valid", error: false,
                FormLabel { html_for: "dx-vs-valid", "Valid field" }
                div { style: "border:1.5px solid #16a34a;border-radius:8px;padding:10px 14px;font-size:15px;",
                    "ferris@opensass.org"
                }
                Helper { valid: true, "Looks great!" }
            }
            Control { input_id: "dx-vs-error", error: true,
                FormLabel { html_for: "dx-vs-error", error: true, "Error field" }
                div { style: "border:1.5px solid #dc2626;border-radius:8px;padding:10px 14px;\
                              font-size:15px;box-shadow:0 0 0 3px rgba(220,38,38,0.18);",
                    "bad-email"
                }
                Helper { error: true, "Not a valid email address." }
            }
            Field {
                id: "dx-vs-input",
                label: "Try me",
                placeholder: "Type something...",
                full_width: true,
                handle: v,
                valid_handle: vv,
                validate_function: validate_any,
            }
        }
    }
}

#[component]
fn ExampleAriaValidation() -> Element {
    let email = use_signal(String::new);
    let email_valid = use_signal(|| true);
    let is_invalid = !email().is_empty() && !validate_email(email());
    let is_valid = validate_email(email());
    rsx! {
        Form {
            validation_behavior: ValidationBehavior::Aria,
            aria_label: "Aria-mode validation form",
            Field {
                id: "dx-aria-email",
                name: "email",
                r#type: "email",
                label: "Email (ARIA mode)",
                placeholder: "ferris@opensass.org",
                helper_text: "Errors appear as you type, submit is never blocked.",
                required: true,
                full_width: true,
                validation_state: if is_invalid {
                    ValidationState::Invalid("Must be a valid email.".to_string())
                } else if is_valid {
                    ValidationState::Valid
                } else {
                    ValidationState::None
                },
                handle: email,
                valid_handle: email_valid,
                validate_function: validate_email,
            }
        }
    }
}

#[component]
fn ExampleDisabledForm() -> Element {
    let v = use_signal(|| "ferris@opensass.org".to_string());
    let vv = use_signal(|| true);
    let pv = use_signal(|| "secretpassword".to_string());
    let pvv = use_signal(|| true);
    rsx! {
        Form { aria_label: "Disabled form example",
            Field {
                id: "dx-dis-email",
                name: "email",
                r#type: "email",
                label: "Email",
                disabled: true,
                full_width: true,
                handle: v,
                valid_handle: vv,
                validate_function: validate_email,
            }
            Field {
                id: "dx-dis-password",
                name: "password",
                r#type: "password",
                label: "Password",
                disabled: true,
                full_width: true,
                handle: pv,
                valid_handle: pvv,
                validate_function: validate_password,
            }
            button {
                disabled: true,
                style: "width:100%;padding:10px 0;border-radius:8px;\
                        background:#7c3aed;color:white;border:none;\
                        font-size:15px;font-weight:600;opacity:0.4;cursor:not-allowed;",
                "Disabled Submit"
            }
        }
    }
}

#[component]
fn ExampleFilledVariant() -> Element {
    let email = use_signal(String::new);
    let email_valid = use_signal(|| true);
    let msg = use_signal(String::new);
    let msg_valid = use_signal(|| true);
    rsx! {
        Form { aria_label: "Filled variant form",
            Field {
                id: "dx-fill-email",
                name: "email",
                r#type: "email",
                label: "Email address",
                placeholder: "ferris@opensass.org",
                variant: Variant::Filled,
                color: Color::Secondary,
                full_width: true,
                handle: email,
                valid_handle: email_valid,
                validate_function: validate_email,
            }
            Field {
                id: "dx-fill-msg",
                name: "message",
                label: "Message",
                placeholder: "Your message...",
                variant: Variant::Filled,
                color: Color::Secondary,
                full_width: true,
                handle: msg,
                valid_handle: msg_valid,
                validate_function: validate_required,
            }
        }
    }
}

#[component]
fn ExampleContactForm() -> Element {
    let name = use_signal(String::new);
    let name_valid = use_signal(|| true);
    let email = use_signal(String::new);
    let email_valid = use_signal(|| true);
    let subject = use_signal(String::new);
    let subject_valid = use_signal(|| true);
    let mut sent = use_signal(|| false);

    rsx! {
        Form {
            method: Method::Post,
            on_submit: Some(Callback::new(move |e: FormEvent| {
                e.prevent_default();
                sent.set(true);
            })),
            aria_label: "Contact form",
            Field {
                id: "dx-ct-name",
                name: "name",
                label: "Full name",
                placeholder: "Ferris Prophet",
                required: true,
                full_width: true,
                handle: name,
                valid_handle: name_valid,
                validate_function: validate_required,
            }
            Field {
                id: "dx-ct-email",
                name: "email",
                r#type: "email",
                label: "Email",
                placeholder: "ferris@opensass.org",
                required: true,
                full_width: true,
                handle: email,
                valid_handle: email_valid,
                validate_function: validate_email,
            }
            Field {
                id: "dx-ct-subject",
                name: "subject",
                label: "Subject",
                placeholder: "How can we help?",
                required: true,
                full_width: true,
                handle: subject,
                valid_handle: subject_valid,
                validate_function: validate_required,
            }
            button {
                r#type: "submit",
                style: "width:100%;padding:10px 0;border-radius:8px;\
                        background:#7c3aed;color:white;border:none;cursor:pointer;\
                        font-size:15px;font-weight:600;",
                "Send message"
            }
            if sent() {
                p { style: "color:#22c55e;font-size:12px;text-align:center;margin:0;",
                    "✓ Message sent! We'll get back to you shortly."
                }
            }
        }
    }
}

#[component]
fn ExampleGroup() -> Element {
    let email_notify = use_signal(String::new);
    let email_valid = use_signal(|| true);
    let sms_notify = use_signal(String::new);
    let sms_valid = use_signal(|| true);
    let push_notify = use_signal(String::new);
    let push_valid = use_signal(|| true);

    rsx! {
        div { style: "width:100%;",
            p { style: "font-size:13px;color:#a1a1aa;margin:0 0 12px;",
                "Choose your notification preferences:"
            }
            Group { aria_label: "Notification preferences",
                ControlLabel {
                    control: rsx! {
                        Input {
                            r#type: "checkbox",
                            id: "dx-notif-email",
                            otp_mode: true,
                            input_style: "width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;",
                            handle: email_notify,
                            valid_handle: email_valid,
                            validate_function: |_: String| true,
                        }
                    },
                    label: Some(rsx! { span { "Email notifications" } }),
                    label_placement: LabelPlacement::End,
                }
                ControlLabel {
                    control: rsx! {
                        Input {
                            r#type: "checkbox",
                            id: "dx-notif-sms",
                            otp_mode: true,
                            input_style: "width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;",
                            handle: sms_notify,
                            valid_handle: sms_valid,
                            validate_function: |_: String| true,
                        }
                    },
                    label: Some(rsx! { span { "SMS notifications" } }),
                    label_placement: LabelPlacement::End,
                }
                ControlLabel {
                    control: rsx! {
                        Input {
                            r#type: "checkbox",
                            id: "dx-notif-push",
                            otp_mode: true,
                            input_style: "width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;",
                            handle: push_notify,
                            valid_handle: push_valid,
                            validate_function: |_: String| true,
                        }
                    },
                    label: Some(rsx! { span { "Push notifications" } }),
                    label_placement: LabelPlacement::End,
                }
            }
        }
    }
}

#[component]
fn ExampleGroupRow() -> Element {
    let opts = ["monthly", "quarterly", "yearly"];
    let monthly_h = use_signal(String::new);
    let monthly_v = use_signal(|| true);
    let quarterly_h = use_signal(String::new);
    let quarterly_v = use_signal(|| true);
    let yearly_h = use_signal(String::new);
    let yearly_v = use_signal(|| true);

    let km = "monthly";
    let kq = "quarterly";
    let ky = "yearly";

    rsx! {
        div { style: "width:100%;",
            p { style: "font-size:13px;color:#a1a1aa;margin:0 0 12px;", "Billing cycle:" }
            Group { aria_label: "Billing cycle options", row: true,
                ControlLabel {
                    key: "{km}",
                    control: rsx! {
                        Input {
                            r#type: "radio",
                            id: "dx-billing-monthly",
                            name: "dx-billing",
                            otp_mode: true,
                            input_style: "width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;",
                            handle: monthly_h,
                            valid_handle: monthly_v,
                            validate_function: |_: String| true,
                        }
                    },
                    label: Some(rsx! { span { "Monthly" } }),
                }
                ControlLabel {
                    key: "{kq}",
                    control: rsx! {
                        Input {
                            r#type: "radio",
                            id: "dx-billing-quarterly",
                            name: "dx-billing",
                            otp_mode: true,
                            input_style: "width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;",
                            handle: quarterly_h,
                            valid_handle: quarterly_v,
                            validate_function: |_: String| true,
                        }
                    },
                    label: Some(rsx! { span { "Quarterly" } }),
                }
                ControlLabel {
                    key: "{ky}",
                    control: rsx! {
                        Input {
                            r#type: "radio",
                            id: "dx-billing-yearly",
                            name: "dx-billing",
                            otp_mode: true,
                            input_style: "width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;",
                            handle: yearly_h,
                            valid_handle: yearly_v,
                            validate_function: |_: String| true,
                        }
                    },
                    label: Some(rsx! { span { "Yearly" } }),
                }
            }
        }
    }
}

#[component]
fn ExampleCustomColors() -> Element {
    let v = use_signal(String::new);
    let vv = use_signal(|| true);
    let v2 = use_signal(String::new);
    let vv2 = use_signal(|| true);
    rsx! {
        Form { aria_label: "Custom color form",
            Field {
                id: "dx-cc-success",
                name: "website",
                r#type: "url",
                label: "Website (green focus)",
                placeholder: "https://opensass.org",
                color: Color::Success,
                full_width: true,
                handle: v,
                valid_handle: vv,
                validate_function: validate_url,
            }
            Field {
                id: "dx-cc-info",
                name: "twitter",
                label: "Twitter handle (cyan focus)",
                placeholder: "@opensass",
                color: Color::Info,
                full_width: true,
                handle: v2,
                valid_handle: vv2,
                validate_function: validate_any,
            }
        }
    }
}

#[component]
fn ExampleServerError() -> Element {
    let email = use_signal(String::new);
    let email_valid = use_signal(|| true);
    rsx! {
        Form { aria_label: "Server-side error example",
            Field {
                id: "dx-srv-email",
                name: "email",
                r#type: "email",
                label: "Email",
                placeholder: "ferris@opensass.org",
                full_width: true,
                validation_state: ValidationState::Invalid("This email is already registered.".to_string()),
                handle: email,
                valid_handle: email_valid,
                validate_function: validate_email,
            }
            button {
                disabled: true,
                style: "width:100%;padding:10px 0;border-radius:8px;\
                        background:#dc2626;color:white;border:none;\
                        font-size:15px;font-weight:600;opacity:0.7;cursor:not-allowed;",
                "Continue (blocked by server error)"
            }
        }
    }
}

#[component]
fn ExampleSmallSize() -> Element {
    let v = use_signal(String::new);
    let vv = use_signal(|| true);
    let v2 = use_signal(String::new);
    let vv2 = use_signal(|| true);
    rsx! {
        Form { aria_label: "Small-size fields",
            Field {
                id: "dx-sm-first",
                name: "first_name",
                label: "First name",
                placeholder: "Ferris",
                size: Size::Small,
                full_width: true,
                handle: v,
                valid_handle: vv,
                validate_function: validate_required,
            }
            Field {
                id: "dx-sm-last",
                name: "last_name",
                label: "Last name",
                placeholder: "Prophet",
                size: Size::Small,
                full_width: true,
                handle: v2,
                valid_handle: vv2,
                validate_function: validate_required,
            }
        }
    }
}

#[component]
fn ExampleFormLabels() -> Element {
    rsx! {
        div { style: "display:flex;flex-direction:column;gap:16px;width:100%;",
            Control { input_id: "dx-lbl-normal",
                FormLabel { html_for: "dx-lbl-normal", "Normal label" }
                div { style: "border:1.5px solid #3f3f46;border-radius:8px;padding:10px 14px;color:#71717a;font-size:15px;", "Input placeholder" }
            }
            Control { input_id: "dx-lbl-required", required: true,
                FormLabel { html_for: "dx-lbl-required", required: true, "Required label" }
                div { style: "border:1.5px solid #3f3f46;border-radius:8px;padding:10px 14px;color:#71717a;font-size:15px;", "Input placeholder" }
            }
            Control { input_id: "dx-lbl-error", error: true,
                FormLabel { html_for: "dx-lbl-error", error: true, "Error label" }
                div { style: "border:1.5px solid #dc2626;border-radius:8px;padding:10px 14px;color:#71717a;font-size:15px;box-shadow:0 0 0 3px rgba(220,38,38,0.18);", "Invalid input" }
                Helper { error: true, "This field has an error." }
            }
            Control { input_id: "dx-lbl-disabled", disabled: true,
                FormLabel { html_for: "dx-lbl-disabled", disabled: true, "Disabled label" }
                div { style: "border:1.5px solid #3f3f46;border-radius:8px;padding:10px 14px;color:#71717a;font-size:15px;opacity:0.5;", "Disabled input" }
            }
        }
    }
}

#[component]
fn ExampleStandardVariant() -> Element {
    let v = use_signal(String::new);
    let vv = use_signal(|| true);
    rsx! {
        Form { aria_label: "Standard variant form",
            Field {
                id: "dx-std-name",
                name: "name",
                label: "Full name",
                placeholder: "Ferris Prophet",
                variant: Variant::Standard,
                full_width: true,
                handle: v,
                valid_handle: vv,
                validate_function: validate_required,
            }
        }
    }
}

#[component]
fn ExampleCustomStyle() -> Element {
    let v = use_signal(String::new);
    let vv = use_signal(|| true);
    rsx! {
        Form {
            Control {
                input_id: "dx-cs-name",
                full_width: true,
                FormLabel {
                    html_for: "dx-cs-name",
                    style: "text-transform:uppercase;color:#7c3aed;",
                    "Name"
                }
                Field {
                    id: "dx-cs-name",
                    variant: Variant::Standard,
                    handle: v,
                    valid_handle: vv,
                    validate_function: validate_required,
                }
                Helper {
                    style: "text-align:right;",
                    "Required"
                }
            }
        }
    }
}

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col items-center justify-center",
                    style: "color: #5e5c7f; background-color: #303030; font-family: 'Rubik', sans-serif; overflow-x: hidden;",

            h1 { class: "text-3xl font-bold mb-8 text-white", "Form RS Dioxus Examples" }

            section { aria_labelledby: "form-heading", class: "w-full max-w-6xl mb-12",
                div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Basic Login Form" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::dioxus::{{Form, Field}};
use form_rs::Method;
use dioxus::prelude::*;

#[component]
fn LoginForm() -> Element {{
    let email = use_signal(String::new);
    let ev = use_signal(|| true);
    let on_submit = Callback::new(|e: FormEvent| {{
        e.prevent_default();
    }});
    rsx! {{
        Form {{ method: Method::Post,
            on_submit: Some(on_submit),
            Field {{
                id: "email", r#type: "email",
                label: "Email", required: true,
                handle: email, valid_handle: ev,
                validate_function: |v: String|
                    v.contains('@'),
            }}
            button {{ r#type: "submit", "Sign in" }}
        }}
    }}
}}"# }
                        ExampleBasicLogin {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Registration Form" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::dioxus::{{Form, Field}};
use dioxus::prelude::*;

#[component]
fn RegForm() -> Element {{
    let email = use_signal(String::new);
    let ev = use_signal(|| true);
    let pass = use_signal(String::new);
    let pv = use_signal(|| true);
    rsx! {{
        Form {{
            Field {{
                id: "email", r#type: "email",
                label: "Email", required: true,
                handle: email, valid_handle: ev,
                validate_function: |v: String|
                    v.contains('@'),
            }}
            Field {{
                id: "password", r#type: "password",
                label: "Password", minlength: Some(8),
                handle: pass, valid_handle: pv,
                validate_function: |v: String|
                    v.len() >= 8,
            }}
            button {{ r#type: "submit", "Register" }}
        }}
    }}
}}"# }
                        ExampleRegistration {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Validation States" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::dioxus::{{
    Control, FormLabel, Helper,
}};
use dioxus::prelude::*;

// Valid state
#[component]
fn ValidField() -> Element {{
    rsx! {{
        Control {{ input_id: "f", error: false,
            FormLabel {{ html_for: "f", "Valid" }}
            Helper {{ valid: true,
                "Looks great!"
            }}
        }}
    }}
}}

// Error state
#[component]
fn ErrorField() -> Element {{
    rsx! {{
        Control {{ input_id: "f", error: true,
            FormLabel {{ html_for: "f", error: true,
                "Error"
            }}
            Helper {{ error: true,
                "Not a valid email."
            }}
        }}
    }}
}}"# }
                        ExampleValidationStates {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "ARIA Realtime Validation" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::{{ValidationBehavior, ValidationState}};
use form_rs::dioxus::{{Form, Field}};
use dioxus::prelude::*;

#[component]
fn AriaForm() -> Element {{
    let email = use_signal(String::new);
    let ev = use_signal(|| true);
    let is_bad = !email().is_empty()
        && !email().contains('@');
    rsx! {{
        Form {{ validation_behavior:
            ValidationBehavior::Aria,
            Field {{
                id: "e", r#type: "email",
                label: "Email (ARIA mode)",
                validation_state: if is_bad {{
                    ValidationState::Invalid(
                        "Invalid email".to_string())
                }} else {{ ValidationState::None }},
                handle: email, valid_handle: ev,
                validate_function: |v: String|
                    v.contains('@'),
            }}
        }}
    }}
}}"# }
                        ExampleAriaValidation {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Disabled Form" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::dioxus::{{Form, Field}};
use dioxus::prelude::*;

#[component]
fn DisabledForm() -> Element {{
    let v = use_signal(||
        "ferris@opensass.org".to_string()
    );
    let vv = use_signal(|| true);
    rsx! {{
        Form {{
            Field {{
                id: "dis-email", r#type: "email",
                label: "Email", disabled: true,
                handle: v, valid_handle: vv,
                validate_function: |v: String|
                    v.contains('@'),
            }}
            button {{ disabled: true, "Disabled" }}
        }}
    }}
}}"# }
                        ExampleDisabledForm {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Filled Variant" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::{{Color, Variant}};
use form_rs::dioxus::{{Form, Field}};
use dioxus::prelude::*;

#[component]
fn FilledForm() -> Element {{
    let v = use_signal(String::new);
    let vv = use_signal(|| true);
    rsx! {{
        Form {{
            Field {{
                id: "filled-email", r#type: "email",
                label: "Email address",
                variant: Variant::Filled,
                color: Color::Secondary,
                full_width: true,
                handle: v, valid_handle: vv,
                validate_function: |v: String|
                    v.contains('@'),
            }}
        }}
    }}
}}"# }
                        ExampleFilledVariant {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Standard Variant" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::Variant;
use form_rs::dioxus::{{Form, Field}};
use dioxus::prelude::*;

#[component]
fn StandardForm() -> Element {{
    let v = use_signal(String::new);
    let vv = use_signal(|| true);
    rsx! {{
        Form {{
            Field {{
                id: "std-name",
                label: "Full name",
                variant: Variant::Standard,
                full_width: true,
                handle: v, valid_handle: vv,
                validate_function: |v: String|
                    !v.is_empty(),
            }}
        }}
    }}
}}"# }
                        ExampleStandardVariant {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Contact Form" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::dioxus::{{Form, Field}};
use form_rs::Method;
use dioxus::prelude::*;

#[component]
fn ContactForm() -> Element {{
    let name = use_signal(String::new);
    let nv = use_signal(|| true);
    let email = use_signal(String::new);
    let ev = use_signal(|| true);
    let mut sent = use_signal(|| false);
    rsx! {{
        Form {{
            method: Method::Post,
            on_submit: Some(Callback::new(
                move |e: FormEvent| {{
                    e.prevent_default();
                    sent.set(true);
                }}
            )),
            Field {{
                id: "name", label: "Name",
                handle: name, valid_handle: nv,
                validate_function: |v| !v.is_empty(),
            }}
            Field {{
                id: "email", r#type: "email",
                label: "Email",
                handle: email, valid_handle: ev,
                validate_function: |v| v.contains('@'),
            }}
            button {{ r#type: "submit", "Send" }}
        }}
    }}
}}"# }
                        ExampleContactForm {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Group Checkboxes" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::dioxus::{{Group, ControlLabel, Input}};
use dioxus::prelude::*;

#[component]
fn NotifPrefs() -> Element {{
    let mut email = use_signal(String::new);
    let mut ev = use_signal(|| true);
    rsx! {{
        Group {{ aria_label: "Notifications",
            ControlLabel {{
                control: rsx! {{
                    Input {{
                        r#type: "checkbox",
                        id: "notif-email",
                        otp_mode: true,
                        handle: email,
                        valid_handle: ev,
                        validate_function:
                            |_: String| true,
                    }}
                }},
                label: Some(rsx! {{
                    span {{ "Email" }}
                }}),
            }}
        }}
    }}
}}"# }
                        ExampleGroup {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Group Radio Row" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::dioxus::{{Group, ControlLabel, Input}};
use dioxus::prelude::*;

#[component]
fn BillingCycle() -> Element {{
    let monthly_h = use_signal(String::new);
    let monthly_v = use_signal(|| true);
    let km = "monthly";
    rsx! {{
        Group {{ aria_label: "Billing", row: true,
            ControlLabel {{
                key: "{{km}}",
                control: rsx! {{
                    Input {{
                        r#type: "radio",
                        id: "billing-monthly",
                        name: "billing",
                        otp_mode: true,
                        handle: monthly_h,
                        valid_handle: monthly_v,
                        validate_function:
                            |_: String| true,
                    }}
                }},
                label: Some(rsx! {{
                    span {{ "Monthly" }}
                }}),
            }}
        }}
    }}
}}"# }
                        ExampleGroupRow {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Custom Color Themes" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::Color;
use form_rs::dioxus::{{Form, Field}};
use dioxus::prelude::*;

// Color variants: Primary, Secondary,
// Success, Info, Warning, Error
#[component]
fn ColoredForm() -> Element {{
    let v = use_signal(String::new);
    let vv = use_signal(|| true);
    rsx! {{
        Form {{
            Field {{
                id: "url", r#type: "url",
                label: "Website (Success)",
                color: Color::Success,
                handle: v, valid_handle: vv,
                validate_function: |v: String|
                    v.starts_with("https://"),
            }}
        }}
    }}
}}"# }
                        ExampleCustomColors {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Server-Side Error" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::ValidationState;
use form_rs::dioxus::{{Form, Field}};
use dioxus::prelude::*;

#[component]
fn ServerErrorForm() -> Element {{
    let v = use_signal(String::new);
    let vv = use_signal(|| true);
    rsx! {{
        Form {{
            Field {{
                id: "srv-email", r#type: "email",
                label: "Email",
                validation_state:
                    ValidationState::Invalid(
                        "Email already registered."
                            .to_string()
                    ),
                handle: v, valid_handle: vv,
                validate_function: |v: String|
                    v.contains('@'),
            }}
        }}
    }}
}}"# }
                        ExampleServerError {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "FormLabel Showcase" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::dioxus::{{
    Control, FormLabel, Helper
}};
use dioxus::prelude::*;

#[component]
fn LabelShowcase() -> Element {{
    rsx! {{
        Control {{ input_id: "l1",
            FormLabel {{ html_for: "l1", "Normal" }}
        }}
        Control {{ input_id: "l2", required: true,
            FormLabel {{ html_for: "l2",
                required: true, "Required"
            }}
        }}
        Control {{ input_id: "l3", error: true,
            FormLabel {{ html_for: "l3",
                error: true, "Error label"
            }}
            Helper {{ error: true,
                "Error message here"
            }}
        }}
    }}
}}"# }
                        ExampleFormLabels {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Small Size Fields" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::Size;
use form_rs::dioxus::{{Form, Field}};
use dioxus::prelude::*;

#[component]
fn SmallForm() -> Element {{
    let v = use_signal(String::new);
    let vv = use_signal(|| true);
    rsx! {{
        Form {{
            Field {{
                id: "first",
                label: "First name",
                size: Size::Small,
                handle: v, valid_handle: vv,
                validate_function: |v: String|
                    !v.is_empty(),
            }}
        }}
    }}
}}"# }
                        ExampleSmallSize {}
                    }

                    article { class: "flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black",
                        h3 { class: "text-xl font-bold mb-2", "Custom Styled (Headless)" }
                        pre { class: "font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre",
r#"use form_rs::Variant;
use form_rs::dioxus::{{
    Form, Control, FormLabel,
    Helper, Field,
}};
use dioxus::prelude::*;

// Pass style to any component for full control
#[component]
fn HeadlessForm() -> Element {{
    let v = use_signal(String::new);
    let vv = use_signal(|| true);
    rsx! {{
        Form {{
            Control {{
                input_id: "cs-name",
                full_width: true,
                FormLabel {{
                    html_for: "cs-name",
                    style: "text-transform:uppercase;color:#7c3aed;",
                    "Name"
                }}
                Field {{
                    id: "cs-name",
                    variant: Variant::Standard,
                    handle: v, valid_handle: vv,
                    validate_function: |v: String|
                        !v.is_empty(),
                }}
                Helper {{
                    style: "text-align:right;",
                    "Required"
                }}
            }}
        }}
    }}
}}"# }
                        ExampleCustomStyle {}
                    }

                }
            }
        }
    }
}
