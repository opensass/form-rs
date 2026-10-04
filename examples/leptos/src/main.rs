// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use form_rs::leptos::{
    Form, Control, ControlLabel, Field, Group, Helper, FormLabel, Input
};
use form_rs::{Color, LabelPlacement, Method, Size, ValidationBehavior, ValidationState, Variant};
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! { <LandingPage /> }
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
pub fn ExampleBasicLogin() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let email_valid = RwSignal::new(true);
    let password = RwSignal::new(String::new());
    let password_valid = RwSignal::new(true);
    let submitted = RwSignal::new(false);

    view! {
        <Form
            method=Method::Post
            on_submit=Callback::new(move |e: SubmitEvent| {
                e.prevent_default();
                submitted.set(true);
            })
            aria_label="Basic login form"
        >
            <Field
                id="lp-login-email"
                name="email"
                r#type="email"
                label="Email address"
                placeholder="ferris@opensass.org"
                helper_text="Enter your registered email."
                required=true
                full_width=true
                handle=email
                valid_handle=email_valid
                validate_function=validate_email
            />
            <Field
                id="lp-login-password"
                name="password"
                r#type="password"
                label="Password"
                placeholder="*******"
                required=true
                full_width=true
                handle=password
                valid_handle=password_valid
                validate_function=validate_password
            />
            <button
                type="submit"
                style="width:100%;padding:10px 0;border-radius:8px;\
                       background:#7c3aed;color:white;border:none;cursor:pointer;\
                       font-size:15px;font-weight:600;transition:opacity 0.2s;"
            >
                "Sign in"
            </button>
            <Show when=move || submitted.get()>
                <p style="color:#22c55e;font-size:12px;text-align:center;margin:0;">
                    "✓ Form submitted!"
                </p>
            </Show>
        </Form>
    }
}

#[component]
pub fn ExampleRegistration() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let username_valid = RwSignal::new(true);
    let email = RwSignal::new(String::new());
    let email_valid = RwSignal::new(true);
    let password = RwSignal::new(String::new());
    let password_valid = RwSignal::new(true);
    let done = RwSignal::new(false);

    view! {
        <Form
            method=Method::Post
            on_submit=Callback::new(move |e: SubmitEvent| {
                e.prevent_default();
                done.set(true);
            })
            aria_label="Registration form"
        >
            <Field
                id="lp-reg-username"
                name="username"
                label="Username"
                placeholder="opensass_org"
                required=true
                full_width=true
                pattern="[a-zA-Z0-9_]{3,20}"
                handle=username
                valid_handle=username_valid
                validate_function=validate_required
            />
            <Field
                id="lp-reg-email"
                name="email"
                r#type="email"
                label="Email"
                placeholder="ferris@opensass.org"
                required=true
                full_width=true
                handle=email
                valid_handle=email_valid
                validate_function=validate_email
            />
            <Field
                id="lp-reg-password"
                name="password"
                r#type="password"
                label="Password"
                placeholder="At least 8 characters"
                helper_text="Must be 8+ characters."
                required=true
                full_width=true
                minlength=8
                handle=password
                valid_handle=password_valid
                validate_function=validate_password
            />
            <button
                type="submit"
                style="width:100%;padding:10px 0;border-radius:8px;\
                       background:#7c3aed;color:white;border:none;cursor:pointer;\
                       font-size:15px;font-weight:600;"
            >
                "Create account"
            </button>
            <Show when=move || done.get()>
                <p style="color:#22c55e;font-size:12px;text-align:center;margin:0;">
                    "✓ Account created!"
                </p>
            </Show>
        </Form>
    }
}

#[component]
pub fn ExampleValidationStates() -> impl IntoView {
    let v = RwSignal::new(String::new());
    let vv = RwSignal::new(true);
    view! {
        <div style="display:flex;flex-direction:column;gap:12px;width:100%;">
            <Control input_id="lp-vs-valid" error=false>
                <FormLabel html_for="lp-vs-valid" focused=false>"Valid field"</FormLabel>
                <div style="border:1.5px solid #16a34a;border-radius:8px;padding:10px 14px;font-size:15px;">
                    "ferris@opensass.org"
                </div>
                <Helper valid=true>"Looks great!"</Helper>
            </Control>
            <Control input_id="lp-vs-error" error=true>
                <FormLabel html_for="lp-vs-error" error=true>"Error field"</FormLabel>
                <div style="border:1.5px solid #dc2626;border-radius:8px;padding:10px 14px;\
                            font-size:15px;box-shadow:0 0 0 3px rgba(220,38,38,0.18);">
                    "bad-email"
                </div>
                <Helper error=true>"Not a valid email address."</Helper>
            </Control>
            <Field
                id="lp-vs-input"
                label="Try me"
                placeholder="Type something..."
                full_width=true
                handle=v
                valid_handle=vv
                validate_function=validate_any
            />
        </div>
    }
}

#[component]
pub fn ExampleAriaValidation() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let email_valid = RwSignal::new(true);

    let is_invalid = move || !email.get().is_empty() && !validate_email(email.get());
    let is_valid = move || validate_email(email.get());

    view! {
        <Form
            validation_behavior=ValidationBehavior::Aria
            aria_label="Aria-mode validation form"
        >
            <Field
                id="lp-aria-email"
                name="email"
                r#type="email"
                label="Email (ARIA mode)"
                placeholder="ferris@opensass.org"
                helper_text="Errors appear as you type, submit is never blocked."
                required=true
                full_width=true
                validation_state=if is_invalid() {
                    ValidationState::Invalid("Must be a valid email.".to_string())
                } else if is_valid() {
                    ValidationState::Valid
                } else {
                    ValidationState::None
                }
                handle=email
                valid_handle=email_valid
                validate_function=validate_email
            />
        </Form>
    }
}

#[component]
pub fn ExampleDisabledForm() -> impl IntoView {
    let v = RwSignal::new("ferris@opensass.org".to_string());
    let vv = RwSignal::new(true);
    let pv = RwSignal::new("secretpassword".to_string());
    let pvv = RwSignal::new(true);
    view! {
        <Form aria_label="Disabled form example">
            <Field
                id="lp-dis-email"
                name="email"
                r#type="email"
                label="Email"
                disabled=true
                full_width=true
                handle=v
                valid_handle=vv
                validate_function=validate_email
            />
            <Field
                id="lp-dis-password"
                name="password"
                r#type="password"
                label="Password"
                disabled=true
                full_width=true
                handle=pv
                valid_handle=pvv
                validate_function=validate_password
            />
            <button disabled=true
                style="width:100%;padding:10px 0;border-radius:8px;\
                       background:#7c3aed;color:white;border:none;\
                       font-size:15px;font-weight:600;opacity:0.4;cursor:not-allowed;">
                "Disabled Submit"
            </button>
        </Form>
    }
}

#[component]
pub fn ExampleFilledVariant() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let email_valid = RwSignal::new(true);
    let msg = RwSignal::new(String::new());
    let msg_valid = RwSignal::new(true);
    view! {
        <Form aria_label="Filled variant form">
            <Field
                id="lp-fill-email"
                name="email"
                r#type="email"
                label="Email address"
                placeholder="ferris@opensass.org"
                variant=Variant::Filled
                color=Color::Secondary
                full_width=true
                handle=email
                valid_handle=email_valid
                validate_function=validate_email
            />
            <Field
                id="lp-fill-msg"
                name="message"
                label="Message"
                placeholder="Your message..."
                variant=Variant::Filled
                color=Color::Secondary
                full_width=true
                handle=msg
                valid_handle=msg_valid
                validate_function=validate_required
            />
        </Form>
    }
}

#[component]
pub fn ExampleContactForm() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let name_valid = RwSignal::new(true);
    let email = RwSignal::new(String::new());
    let email_valid = RwSignal::new(true);
    let subject = RwSignal::new(String::new());
    let subject_valid = RwSignal::new(true);
    let sent = RwSignal::new(false);

    view! {
        <Form
            method=Method::Post
            on_submit=Callback::new(move |e: SubmitEvent| {
                e.prevent_default();
                sent.set(true);
            })
            aria_label="Contact form"
        >
            <Field
                id="lp-ct-name"
                name="name"
                label="Full name"
                placeholder="Ferris Prophet"
                required=true
                full_width=true
                handle=name
                valid_handle=name_valid
                validate_function=validate_required
            />
            <Field
                id="lp-ct-email"
                name="email"
                r#type="email"
                label="Email"
                placeholder="ferris@opensass.org"
                required=true
                full_width=true
                handle=email
                valid_handle=email_valid
                validate_function=validate_email
            />
            <Field
                id="lp-ct-subject"
                name="subject"
                label="Subject"
                placeholder="How can we help?"
                required=true
                full_width=true
                handle=subject
                valid_handle=subject_valid
                validate_function=validate_required
            />
            <button
                type="submit"
                style="width:100%;padding:10px 0;border-radius:8px;\
                       background:#7c3aed;color:white;border:none;cursor:pointer;\
                       font-size:15px;font-weight:600;"
            >
                "Send message"
            </button>
            <Show when=move || sent.get()>
                <p style="color:#22c55e;font-size:12px;text-align:center;margin:0;">
                    "✓ Message sent! We'll get back to you shortly."
                </p>
            </Show>
        </Form>
    }
}

#[component]
pub fn ExampleGroup() -> impl IntoView {
    let email_notify = RwSignal::new(String::new());
    let email_valid = RwSignal::new(true);
    let sms_notify = RwSignal::new(String::new());
    let sms_valid = RwSignal::new(true);
    let push_notify = RwSignal::new(String::new());
    let push_valid = RwSignal::new(true);

    view! {
        <div style="width:100%;">
            <p style="font-size:13px;color:#a1a1aa;margin:0 0 12px;">
                "Choose your notification preferences:"
            </p>
            <Group aria_label="Notification preferences">
                <ControlLabel
                    control=view! {
                        <Input
                            r#type="checkbox"
                            id="lp-notif-email"
                            otp_mode=true
                            input_style="width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;"
                            handle=email_notify.split()
                            valid_handle=email_valid.split()
                            validate_function=|_| true
                        />
                    }.into_any()
                    label=view! { <span>"Email notifications"</span> }.into_any()
                    label_placement=LabelPlacement::End
                />
                <ControlLabel
                    control=view! {
                        <Input
                            r#type="checkbox"
                            id="lp-notif-sms"
                            otp_mode=true
                            input_style="width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;"
                            handle=sms_notify.split()
                            valid_handle=sms_valid.split()
                            validate_function=|_| true
                        />
                    }.into_any()
                    label=view! { <span>"SMS notifications"</span> }.into_any()
                    label_placement=LabelPlacement::End
                />
                <ControlLabel
                    control=view! {
                        <Input
                            r#type="checkbox"
                            id="lp-notif-push"
                            otp_mode=true
                            input_style="width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;"
                            handle=push_notify.split()
                            valid_handle=push_valid.split()
                            validate_function=|_| true
                        />
                    }.into_any()
                    label=view! { <span>"Push notifications"</span> }.into_any()
                    label_placement=LabelPlacement::End
                />
            </Group>
            <p style="font-size:11px;color:#71717a;margin:12px 0 0;">
                {move || format!("Selected: email={}, sms={}, push={}", email_notify.get(), sms_notify.get(), push_notify.get())}
            </p>
        </div>
    }
}

#[component]
pub fn ExampleGroupRow() -> impl IntoView {
    let choice = RwSignal::new("monthly");
    let monthly_h = RwSignal::new(String::new());
    let monthly_v = RwSignal::new(true);
    let quarterly_h = RwSignal::new(String::new());
    let quarterly_v = RwSignal::new(true);
    let yearly_h = RwSignal::new(String::new());
    let yearly_v = RwSignal::new(true);

    view! {
        <div style="width:100%;">
            <p style="font-size:13px;color:#a1a1aa;margin:0 0 12px;">"Billing cycle:"</p>
            <Group aria_label="Billing cycle options" row=true>
                <ControlLabel
                    control=view! {
                        <Input
                            r#type="radio"
                            id="lp-billing-monthly"
                            name="lp-billing"
                            otp_mode=true
                            input_style="width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;"
                            handle=monthly_h.split()
                            valid_handle=monthly_v.split()
                            validate_function=|_| true
                        />
                    }.into_any()
                    label=view! { <span>"Monthly"</span> }.into_any()
                />
                <ControlLabel
                    control=view! {
                        <Input
                            r#type="radio"
                            id="lp-billing-quarterly"
                            name="lp-billing"
                            otp_mode=true
                            input_style="width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;"
                            handle=quarterly_h.split()
                            valid_handle=quarterly_v.split()
                            validate_function=|_| true
                        />
                    }.into_any()
                    label=view! { <span>"Quarterly"</span> }.into_any()
                />
                <ControlLabel
                    control=view! {
                        <Input
                            r#type="radio"
                            id="lp-billing-yearly"
                            name="lp-billing"
                            otp_mode=true
                            input_style="width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;"
                            handle=yearly_h.split()
                            valid_handle=yearly_v.split()
                            validate_function=|_| true
                        />
                    }.into_any()
                    label=view! { <span>"Yearly"</span> }.into_any()
                />
            </Group>
            <p style="font-size:11px;color:#71717a;margin:12px 0 0;">
                {move || format!("Selected: {}", choice.get())}
            </p>
        </div>
    }
}

#[component]
pub fn ExampleCustomColors() -> impl IntoView {
    let v = RwSignal::new(String::new());
    let vv = RwSignal::new(true);
    let v2 = RwSignal::new(String::new());
    let vv2 = RwSignal::new(true);
    view! {
        <Form aria_label="Custom color form">
            <Field
                id="lp-cc-success"
                name="website"
                r#type="url"
                label="Website (green focus)"
                placeholder="https://opensass.org"
                color=Color::Success
                full_width=true
                handle=v
                valid_handle=vv
                validate_function=validate_url
            />
            <Field
                id="lp-cc-info"
                name="twitter"
                label="Twitter handle (cyan focus)"
                placeholder="@opensass"
                color=Color::Info
                full_width=true
                handle=v2
                valid_handle=vv2
                validate_function=validate_any
            />
        </Form>
    }
}

#[component]
pub fn ExampleServerError() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let email_valid = RwSignal::new(true);
    view! {
        <Form aria_label="Server-side error example">
            <Field
                id="lp-srv-email"
                name="email"
                r#type="email"
                label="Email"
                placeholder="ferris@opensass.org"
                full_width=true
                validation_state=ValidationState::Invalid("This email is already registered.".to_string())
                handle=email
                valid_handle=email_valid
                validate_function=validate_email
            />
            <button disabled=true
                style="width:100%;padding:10px 0;border-radius:8px;\
                       background:#dc2626;color:white;border:none;\
                       font-size:15px;font-weight:600;opacity:0.7;cursor:not-allowed;">
                "Continue (blocked by server error)"
            </button>
        </Form>
    }
}

#[component]
pub fn ExampleSmallSize() -> impl IntoView {
    let v = RwSignal::new(String::new());
    let vv = RwSignal::new(true);
    let v2 = RwSignal::new(String::new());
    let vv2 = RwSignal::new(true);
    view! {
        <Form aria_label="Small-size fields">
            <Field
                id="lp-sm-first"
                name="first_name"
                label="First name"
                placeholder="Ferris"
                size=Size::Small
                full_width=true
                handle=v
                valid_handle=vv
                validate_function=validate_required
            />
            <Field
                id="lp-sm-last"
                name="last_name"
                label="Last name"
                placeholder="Prophet"
                size=Size::Small
                full_width=true
                handle=v2
                valid_handle=vv2
                validate_function=validate_required
            />
        </Form>
    }
}

#[component]
pub fn ExampleFormLabels() -> impl IntoView {
    view! {
        <div style="display:flex;flex-direction:column;gap:16px;width:100%;">
            <Control input_id="lp-lbl-normal">
                <FormLabel html_for="lp-lbl-normal">"Normal label"</FormLabel>
                <div style="border:1.5px solid #3f3f46;border-radius:8px;padding:10px 14px;color:#71717a;font-size:15px;">
                    "Input placeholder"
                </div>
            </Control>
            <Control input_id="lp-lbl-required" required=true>
                <FormLabel html_for="lp-lbl-required" required=true>"Required label"</FormLabel>
                <div style="border:1.5px solid #3f3f46;border-radius:8px;padding:10px 14px;color:#71717a;font-size:15px;">
                    "Input placeholder"
                </div>
            </Control>
            <Control input_id="lp-lbl-error" error=true>
                <FormLabel html_for="lp-lbl-error" error=true>"Error label"</FormLabel>
                <div style="border:1.5px solid #dc2626;border-radius:8px;padding:10px 14px;color:#71717a;font-size:15px;\
                            box-shadow:0 0 0 3px rgba(220,38,38,0.18);">
                    "Invalid input"
                </div>
                <Helper error=true>"This field has an error."</Helper>
            </Control>
            <Control input_id="lp-lbl-disabled" disabled=true>
                <FormLabel html_for="lp-lbl-disabled" disabled=true>"Disabled label"</FormLabel>
                <div style="border:1.5px solid #3f3f46;border-radius:8px;padding:10px 14px;color:#71717a;font-size:15px;opacity:0.5;">
                    "Disabled input"
                </div>
            </Control>
        </div>
    }
}

#[component]
pub fn ExampleStandardVariant() -> impl IntoView {
    let v = RwSignal::new(String::new());
    let vv = RwSignal::new(true);
    view! {
        <Form aria_label="Standard variant form">
            <Field
                id="lp-std-name"
                name="name"
                label="Full name"
                placeholder="Ferris Prophet"
                variant=Variant::Standard
                full_width=true
                handle=v
                valid_handle=vv
                validate_function=validate_required
            />
        </Form>
    }
}

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">"Form RS Leptos Examples"</h1>

            <section aria-labelledby="form-heading" class="w-full max-w-6xl mb-12">
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Basic Login Form"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::leptos::{Form, Field};
use form_rs::Method;
use leptos::prelude::*;

#[component]
pub fn LoginForm() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let ev = RwSignal::new(true);
    view! {
        <Form method=Method::Post
            on_submit=Callback::new(|e: SubmitEvent| {
                e.prevent_default();
            })
        >
            <Field id="email" r#type="email"
                label="Email" required=true
                handle=email valid_handle=ev
                validate_function=|v: String|
                    v.contains('@')
            />
            <button type="submit">"Sign in"</button>
        </Form>
    }
}"#}</pre>
                        <ExampleBasicLogin />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Registration Form"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::leptos::{Form, Field};
use leptos::prelude::*;

#[component]
pub fn RegForm() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let ev = RwSignal::new(true);
    let pass = RwSignal::new(String::new());
    let pv = RwSignal::new(true);
    view! {
        <Form>
            <Field id="email" r#type="email"
                label="Email" required=true
                handle=email valid_handle=ev
                validate_function=|v: String|
                    v.contains('@')
            />
            <Field id="password" r#type="password"
                label="Password" minlength=Some(8)
                handle=pass valid_handle=pv
                validate_function=|v: String|
                    v.len() >= 8
            />
            <button type="submit">"Register"</button>
        </Form>
    }
}"#}</pre>
                        <ExampleRegistration />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Validation States"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::leptos::{
    Control, FormLabel, Helper,
};
use leptos::prelude::*;

// Valid state
#[component]
pub fn ValidField() -> impl IntoView {
    view! {
        <Control input_id="f" error=false>
            <FormLabel html_for="f">"Valid"</FormLabel>
            <Helper valid=true>
                "Looks great!"
            </Helper>
        </Control>
    }
}
// Error state
#[component]
pub fn ErrorField() -> impl IntoView {
    view! {
        <Control input_id="f" error=true>
            <FormLabel html_for="f" error=true>
                "Error"
            </FormLabel>
            <Helper error=true>
                "Not a valid email."
            </Helper>
        </Control>
    }
}"#}</pre>
                        <ExampleValidationStates />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"ARIA Realtime Validation"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::{ValidationBehavior, ValidationState};
use form_rs::leptos::{Form, Field};
use leptos::prelude::*;

#[component]
pub fn AriaForm() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let ev = RwSignal::new(true);
    let is_bad = move || !email.get().is_empty()
        && !email.get().contains('@');
    view! {
        <Form validation_behavior=
            ValidationBehavior::Aria
        >
            <Field id="e" r#type="email"
                label="Email (ARIA mode)"
                validation_state=if is_bad() {
                    ValidationState::Invalid(
                        "Invalid email".to_string())
                } else { ValidationState::None }
                handle=email valid_handle=ev
                validate_function=|v: String|
                    v.contains('@')
            />
        </Form>
    }
}"#}</pre>
                        <ExampleAriaValidation />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Disabled Form"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::leptos::{Form, Field};
use leptos::prelude::*;

#[component]
pub fn DisabledForm() -> impl IntoView {
    let v = RwSignal::new(
        "ferris@opensass.org".to_string()
    );
    let vv = RwSignal::new(true);
    view! {
        <Form aria_label="Disabled form">
            <Field id="dis-email"
                r#type="email"
                label="Email" disabled=true
                handle=v valid_handle=vv
                validate_function=|v: String|
                    v.contains('@')
            />
            <button disabled=true>"Disabled"</button>
        </Form>
    }
}"#}</pre>
                        <ExampleDisabledForm />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Filled Variant"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::{Color, Variant};
use form_rs::leptos::{Form, Field};
use leptos::prelude::*;

#[component]
pub fn FilledForm() -> impl IntoView {
    let v = RwSignal::new(String::new());
    let vv = RwSignal::new(true);
    view! {
        <Form>
            <Field id="filled-email"
                r#type="email"
                label="Email address"
                variant=Variant::Filled
                color=Color::Secondary
                full_width=true
                handle=v valid_handle=vv
                validate_function=|v: String|
                    v.contains('@')
            />
        </Form>
    }
}"#}</pre>
                        <ExampleFilledVariant />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Standard Variant"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::Variant;
use form_rs::leptos::{Form, Field};
use leptos::prelude::*;

#[component]
pub fn StandardForm() -> impl IntoView {
    let v = RwSignal::new(String::new());
    let vv = RwSignal::new(true);
    view! {
        <Form>
            <Field id="std-name"
                label="Full name"
                variant=Variant::Standard
                full_width=true
                handle=v valid_handle=vv
                validate_function=|v: String|
                    !v.is_empty()
            />
        </Form>
    }
}"#}</pre>
                        <ExampleStandardVariant />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Contact Form"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::leptos::{Form, Field};
use form_rs::Method;
use leptos::prelude::*;

#[component]
pub fn ContactForm() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let nv = RwSignal::new(true);
    let email = RwSignal::new(String::new());
    let ev = RwSignal::new(true);
    let sent = RwSignal::new(false);
    view! {
        <Form method=Method::Post
            on_submit=Callback::new(move |e: SubmitEvent| {
                e.prevent_default();
                sent.set(true);
            })
        >
            <Field id="name" label="Name"
                handle=name valid_handle=nv
                validate_function=|v| !v.is_empty()
            />
            <Field id="email" r#type="email"
                label="Email"
                handle=email valid_handle=ev
                validate_function=|v| v.contains('@')
            />
            <button type="submit">"Send"</button>
        </Form>
    }
}"#}</pre>
                        <ExampleContactForm />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Group Checkboxes"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::leptos::{Group, ControlLabel, Input};
use leptos::prelude::*;

#[component]
pub fn NotifPrefs() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let ev = RwSignal::new(true);
    view! {
        <Group aria_label="Notifications">
            <ControlLabel
                control=view! {
                    <Input
                        r#type="checkbox"
                        id="notif-email"
                        otp_mode=true
                        handle=email.split()
                        valid_handle=ev.split()
                        validate_function=|_| true
                    />
                }.into_any()
                label=Some(view! {
                    <span>"Email"</span>
                }.into_any())
            />
        </Group>
    }
}"#}</pre>
                        <ExampleGroup />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Group Radio Row"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::leptos::{Group, ControlLabel, Input};
use leptos::prelude::*;

#[component]
pub fn BillingCycle() -> impl IntoView {
    let choice = RwSignal::new("monthly");
    let monthly_h = RwSignal::new(String::new());
    let monthly_v = RwSignal::new(true);
    view! {
        <Group aria_label="Billing" row=true>
            <ControlLabel
                control=view! {
                    <Input
                        r#type="radio"
                        id="lp-billing-monthly"
                        name="lp-billing"
                        otp_mode=true
                        handle=monthly_h.split()
                        valid_handle=monthly_v.split()
                        validate_function=|_| true
                    />
                }.into_any()
                label=Some(view! {
                    <span>"Monthly"</span>
                }.into_any())
            />
        </Group>
    }
}"#}</pre>
                        <ExampleGroupRow />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Custom Color Themes"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::Color;
use form_rs::leptos::{Form, Field};
use leptos::prelude::*;

// Colors: Primary, Secondary,
// Success, Info, Warning, Error
#[component]
pub fn ColoredForm() -> impl IntoView {
    let v = RwSignal::new(String::new());
    let vv = RwSignal::new(true);
    view! {
        <Form>
            <Field id="url" r#type="url"
                label="Website (Success)"
                color=Color::Success
                handle=v valid_handle=vv
                validate_function=|v: String|
                    v.starts_with("https://")
            />
        </Form>
    }
}"#}</pre>
                        <ExampleCustomColors />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Server-Side Error"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::ValidationState;
use form_rs::leptos::{Form, Field};
use leptos::prelude::*;

#[component]
pub fn ServerErrorForm() -> impl IntoView {
    let v = RwSignal::new(String::new());
    let vv = RwSignal::new(true);
    view! {
        <Form>
            <Field id="srv-email" r#type="email"
                label="Email"
                validation_state=
                    ValidationState::Invalid(
                        "Email already registered."
                            .to_string()
                    )
                handle=v valid_handle=vv
                validate_function=|v: String|
                    v.contains('@')
            />
        </Form>
    }
}"#}</pre>
                        <ExampleServerError />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"FormLabel Showcase"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::leptos::{
    Control, FormLabel, Helper
};
use leptos::prelude::*;

#[component]
pub fn LabelShowcase() -> impl IntoView {
    view! {
        <Control input_id="l1">
            <FormLabel html_for="l1">"Normal"</FormLabel>
        </Control>
        <Control input_id="l2" required=true>
            <FormLabel html_for="l2" required=true>
                "Required"
            </FormLabel>
        </Control>
        <Control input_id="l3" error=true>
            <FormLabel html_for="l3" error=true>
                "Error label"
            </FormLabel>
            <Helper error=true>
                "Error message here"
            </Helper>
        </Control>
    }
}"#}</pre>
                        <ExampleFormLabels />
                    </article>

                    <article class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black">
                        <h3 class="text-xl font-bold mb-2">"Small Size Fields"</h3>
                        <pre class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre">{r#"use form_rs::Size;
use form_rs::leptos::{Form, Field};
use leptos::prelude::*;

#[component]
pub fn SmallForm() -> impl IntoView {
    let v = RwSignal::new(String::new());
    let vv = RwSignal::new(true);
    view! {
        <Form>
            <Field id="first"
                label="First name"
                size=Size::Small
                handle=v valid_handle=vv
                validate_function=|v: String|
                    !v.is_empty()
            />
        </Form>
    }
}"#}</pre>
                        <ExampleSmallSize />
                    </article>

                </div>
            </section>
        </div>
    }
}
