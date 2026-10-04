// Copyright 2026 Open SASS Core Maintainers.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use form_rs::yew::{Control, ControlLabel, Field, Form, FormLabel, Group, Helper, Input};
use form_rs::{Color, LabelPlacement, Method, ValidationBehavior, ValidationState, Variant};
use yew::prelude::*;

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

#[function_component(ExampleBasicLogin)]
pub fn example_basic_login() -> Html {
    let email = use_state(String::new);
    let email_valid = use_state(|| true);
    let password = use_state(String::new);
    let password_valid = use_state(|| true);
    let submitted = use_state(|| false);

    let on_submit = {
        let submitted = submitted.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            submitted.set(true);
        })
    };

    html! {
        <Form method={Method::Post} on_submit={on_submit} aria_label="Basic login form">
            <Field
                id="login-email"
                name="email"
                r#type="email"
                label="Email address"
                placeholder="ferris@opensass.org"
                helper_text="Enter your registered email."
                required=true
                full_width=true
                handle={email.clone()}
                valid_handle={email_valid.clone()}
                validate_function={Callback::from(validate_email)}
            />
            <Field
                id="login-password"
                name="password"
                r#type="password"
                label="Password"
                placeholder="*******"
                required=true
                full_width=true
                handle={password.clone()}
                valid_handle={password_valid.clone()}
                validate_function={Callback::from(validate_password)}
            />
            <button
                type="submit"
                style="width:100%;padding:10px 0;border-radius:8px;\
                       background:#7c3aed;color:white;border:none;cursor:pointer;\
                       font-size:15px;font-weight:600;transition:opacity 0.2s;"
            >
                { "Sign in" }
            </button>
            if *submitted {
                <p style="color:#22c55e;font-size:12px;text-align:center;margin:0;">
                    { "✓ Form submitted check console for data." }
                </p>
            }
        </Form>
    }
}

#[function_component(ExampleRegistration)]
pub fn example_registration() -> Html {
    let username = use_state(String::new);
    let username_valid = use_state(|| true);
    let email = use_state(String::new);
    let email_valid = use_state(|| true);
    let password = use_state(String::new);
    let password_valid = use_state(|| true);
    let done = use_state(|| false);

    let on_submit = {
        let done = done.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            done.set(true);
        })
    };

    html! {
        <Form method={Method::Post} on_submit={on_submit} aria_label="Registration form">
            <Field
                id="reg-username"
                name="username"
                label="Username"
                placeholder="opensass_org"
                required=true
                full_width=true
                pattern="[a-zA-Z0-9_]{3,20}"
                handle={username.clone()}
                valid_handle={username_valid.clone()}
                validate_function={Callback::from(validate_required)}
            />
            <Field
                id="reg-email"
                name="email"
                r#type="email"
                label="Email"
                placeholder="ferris@opensass.org"
                required=true
                full_width=true
                handle={email.clone()}
                valid_handle={email_valid.clone()}
                validate_function={Callback::from(validate_email)}
            />
            <Field
                id="reg-password"
                name="password"
                r#type="password"
                label="Password"
                placeholder="At least 8 characters"
                helper_text="Must be 8+ characters."
                required=true
                full_width=true
                minlength={Some(8)}
                handle={password.clone()}
                valid_handle={password_valid.clone()}
                validate_function={Callback::from(validate_password)}
            />
            <button
                type="submit"
                style="width:100%;padding:10px 0;border-radius:8px;\
                       background:#7c3aed;color:white;border:none;cursor:pointer;\
                       font-size:15px;font-weight:600;"
            >
                { "Create account" }
            </button>
            if *done {
                <p style="color:#22c55e;font-size:12px;text-align:center;margin:0;">
                    { "✓ Account created successfully!" }
                </p>
            }
        </Form>
    }
}

#[function_component(ExampleValidationStates)]
pub fn example_validation_states() -> Html {
    let v = use_state(String::new);
    let vv = use_state(|| true);
    html! {
        <div style="display:flex;flex-direction:column;gap:12px;width:100%;">
            <Control input_id="vs-valid" error=false>
                <FormLabel html_for="vs-valid" focused=false>{ "Valid field" }</FormLabel>
                <div
                    style="border:1.5px solid #16a34a;border-radius:8px;padding:10px 14px;\
                            background:transparent;font-size:15px;"
                >
                    { "ferris@opensass.org" }
                </div>
                <Helper valid=true>{ "Looks great!" }</Helper>
            </Control>
            <Control input_id="vs-error" error=true>
                <FormLabel html_for="vs-error" error=true>{ "Error field" }</FormLabel>
                <div
                    style="border:1.5px solid #dc2626;border-radius:8px;padding:10px 14px;\
                            background:transparent;font-size:15px;box-shadow:0 0 0 3px rgba(220,38,38,0.18);"
                >
                    { "bad-email" }
                </div>
                <Helper error=true>{ "Not a valid email address." }</Helper>
            </Control>
            <Field
                id="vs-input"
                label="Try me"
                placeholder="Type something..."
                full_width=true
                handle={v.clone()}
                valid_handle={vv.clone()}
                validate_function={Callback::from(validate_any)}
            />
        </div>
    }
}

#[function_component(ExampleAriaValidation)]
pub fn example_aria_validation() -> Html {
    let email = use_state(String::new);
    let email_valid = use_state(|| true);
    html! {
        <Form
            validation_behavior={ValidationBehavior::Aria}
            aria_label="Aria-mode validation form"
        >
            <Field
                id="aria-email"
                name="email"
                r#type="email"
                label="Email (ARIA mode)"
                placeholder="ferris@opensass.org"
                helper_text="Errors appear as you type, submit is never blocked."
                required=true
                full_width=true
                validation_state={if !(*email).is_empty() && !validate_email((*email).clone()) {
                        ValidationState::Invalid("Must be a valid email.".to_string())
                    } else if validate_email((*email).clone()) {
                        ValidationState::Valid
                    } else {
                        ValidationState::None
                    }}
                handle={email.clone()}
                valid_handle={email_valid.clone()}
                validate_function={Callback::from(validate_email)}
            />
        </Form>
    }
}

#[function_component(ExampleDisabledForm)]
pub fn example_disabled_form() -> Html {
    let v = use_state(|| "ferris@opensass.org".to_string());
    let vv = use_state(|| true);
    let pv = use_state(|| "secretpassword".to_string());
    let pvv = use_state(|| true);
    html! {
        <Form aria_label="Disabled form example">
            <Field
                id="dis-email"
                name="email"
                r#type="email"
                label="Email"
                disabled=true
                full_width=true
                handle={v.clone()}
                valid_handle={vv.clone()}
                validate_function={Callback::from(validate_email)}
            />
            <Field
                id="dis-password"
                name="password"
                r#type="password"
                label="Password"
                disabled=true
                full_width=true
                handle={pv.clone()}
                valid_handle={pvv.clone()}
                validate_function={Callback::from(validate_password)}
            />
            <button
                disabled=true
                style="width:100%;padding:10px 0;border-radius:8px;\
                       background:#7c3aed;color:white;border:none;\
                       font-size:15px;font-weight:600;opacity:0.4;cursor:not-allowed;"
            >
                { "Disabled Submit" }
            </button>
        </Form>
    }
}

#[function_component(ExampleFilledVariant)]
pub fn example_filled_variant() -> Html {
    let email = use_state(String::new);
    let email_valid = use_state(|| true);
    let msg = use_state(String::new);
    let msg_valid = use_state(|| true);
    html! {
        <Form aria_label="Filled variant form">
            <Field
                id="fill-email"
                name="email"
                r#type="email"
                label="Email address"
                placeholder="ferris@opensass.org"
                variant={Variant::Filled}
                color={Color::Secondary}
                full_width=true
                handle={email.clone()}
                valid_handle={email_valid.clone()}
                validate_function={Callback::from(validate_email)}
            />
            <Field
                id="fill-msg"
                name="message"
                label="Message"
                placeholder="Your message..."
                variant={Variant::Filled}
                color={Color::Secondary}
                full_width=true
                handle={msg.clone()}
                valid_handle={msg_valid.clone()}
                validate_function={Callback::from(validate_required)}
            />
        </Form>
    }
}

#[function_component(ExampleStandardVariant)]
pub fn example_standard_variant() -> Html {
    let v = use_state(String::new);
    let vv = use_state(|| true);
    html! {
        <Form aria_label="Standard (underline) variant form">
            <Field
                id="std-name"
                name="name"
                label="Full name"
                placeholder="Ferris Prophet"
                variant={Variant::Standard}
                full_width=true
                handle={v.clone()}
                valid_handle={vv.clone()}
                validate_function={Callback::from(validate_required)}
            />
        </Form>
    }
}

#[function_component(ExampleContactForm)]
pub fn example_contact_form() -> Html {
    let name = use_state(String::new);
    let name_valid = use_state(|| true);
    let email = use_state(String::new);
    let email_valid = use_state(|| true);
    let subject = use_state(String::new);
    let subject_valid = use_state(|| true);
    let sent = use_state(|| false);

    let on_submit = {
        let sent = sent.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            sent.set(true);
        })
    };

    html! {
        <Form method={Method::Post} on_submit={on_submit} aria_label="Contact form">
            <Field
                id="ct-name"
                name="name"
                label="Full name"
                placeholder="Ferris Prophet"
                required=true
                full_width=true
                handle={name.clone()}
                valid_handle={name_valid.clone()}
                validate_function={Callback::from(validate_required)}
            />
            <Field
                id="ct-email"
                name="email"
                r#type="email"
                label="Email"
                placeholder="ferris@opensass.org"
                required=true
                full_width=true
                handle={email.clone()}
                valid_handle={email_valid.clone()}
                validate_function={Callback::from(validate_email)}
            />
            <Field
                id="ct-subject"
                name="subject"
                label="Subject"
                placeholder="How can we help?"
                required=true
                full_width=true
                handle={subject.clone()}
                valid_handle={subject_valid.clone()}
                validate_function={Callback::from(validate_required)}
            />
            <button
                type="submit"
                style="width:100%;padding:10px 0;border-radius:8px;\
                       background:#7c3aed;color:white;border:none;cursor:pointer;\
                       font-size:15px;font-weight:600;"
            >
                { "Send message" }
            </button>
            if *sent {
                <p style="color:#22c55e;font-size:12px;text-align:center;margin:0;">
                    { "✓ Message sent! We'll get back to you shortly." }
                </p>
            }
        </Form>
    }
}

#[function_component(ExampleGroup)]
pub fn example_form_group() -> Html {
    let email_notify = use_state(String::new);
    let email_valid = use_state(|| true);
    let sms_notify = use_state(String::new);
    let sms_valid = use_state(|| true);
    let push_notify = use_state(String::new);
    let push_valid = use_state(|| true);
    let email_ref = use_node_ref();
    let sms_ref = use_node_ref();
    let push_ref = use_node_ref();

    html! {
        <div style="width:100%;">
            <p style="font-size:13px;color:#a1a1aa;margin:0 0 12px;">
                { "Choose your notification preferences:" }
            </p>
            <Group aria_label="Notification preferences">
                <ControlLabel
                    control={html! {
                        <Input
                            r#ref={email_ref}
                            r#type="checkbox"
                            id="notif-email"
                            otp_mode=true
                            input_style="width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;"
                            handle={email_notify.clone()}
                            valid_handle={email_valid.clone()}
                            validate_function={Callback::from(|_: String| true)}
                        />
                    }}
                    label={html! { <span>{"Email notifications"}</span> }}
                    label_placement={LabelPlacement::End}
                />
                <ControlLabel
                    control={html! {
                        <Input
                            r#ref={sms_ref}
                            r#type="checkbox"
                            id="notif-sms"
                            otp_mode=true
                            input_style="width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;"
                            handle={sms_notify.clone()}
                            valid_handle={sms_valid.clone()}
                            validate_function={Callback::from(|_: String| true)}
                        />
                    }}
                    label={html! { <span>{"SMS notifications"}</span> }}
                    label_placement={LabelPlacement::End}
                />
                <ControlLabel
                    control={html! {
                        <Input
                            r#ref={push_ref}
                            r#type="checkbox"
                            id="notif-push"
                            otp_mode=true
                            input_style="width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;"
                            handle={push_notify.clone()}
                            valid_handle={push_valid.clone()}
                            validate_function={Callback::from(|_: String| true)}
                        />
                    }}
                    label={html! { <span>{"Push notifications"}</span> }}
                    label_placement={LabelPlacement::End}
                />
            </Group>
            <p style="font-size:11px;color:#71717a;margin:12px 0 0;">
                { format!("Selected: email={}, sms={}, push={}", *email_notify, *sms_notify, *push_notify) }
            </p>
        </div>
    }
}

#[function_component(ExampleGroupRow)]
pub fn example_form_group_row() -> Html {
    let choice = use_state(|| "monthly");
    let monthly_h = use_state(|| "monthly".to_string());
    let monthly_v = use_state(|| true);
    let quarterly_h = use_state(|| "quarterly".to_string());
    let quarterly_v = use_state(|| true);
    let yearly_h = use_state(|| "yearly".to_string());
    let yearly_v = use_state(|| true);
    let monthly_ref = use_node_ref();
    let quarterly_ref = use_node_ref();
    let yearly_ref = use_node_ref();

    html! {
        <div style="width:100%;">
            <p style="font-size:13px;color:#a1a1aa;margin:0 0 12px;">{ "Billing cycle:" }</p>
            <Group aria_label="Billing cycle options" row=true>
                <ControlLabel
                    control={html! {
                        <Input
                            r#ref={monthly_ref}
                            r#type="radio"
                            id="billing-monthly"
                            name="billing"
                            otp_mode=true
                            input_style="width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;"
                            handle={monthly_h.clone()}
                            valid_handle={monthly_v.clone()}
                            validate_function={Callback::from(|_: String| true)}
                        />
                    }}
                    label={html! { <span>{"MONTHLY"}</span> }}
                />
                <ControlLabel
                    control={html! {
                        <Input
                            r#ref={quarterly_ref}
                            r#type="radio"
                            id="billing-quarterly"
                            name="billing"
                            otp_mode=true
                            input_style="width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;"
                            handle={quarterly_h.clone()}
                            valid_handle={quarterly_v.clone()}
                            validate_function={Callback::from(|_: String| true)}
                        />
                    }}
                    label={html! { <span>{"QUARTERLY"}</span> }}
                />
                <ControlLabel
                    control={html! {
                        <Input
                            r#ref={yearly_ref}
                            r#type="radio"
                            id="billing-yearly"
                            name="billing"
                            otp_mode=true
                            input_style="width:16px;height:16px;accent-color:#7c3aed;cursor:pointer;"
                            handle={yearly_h.clone()}
                            valid_handle={yearly_v.clone()}
                            validate_function={Callback::from(|_: String| true)}
                        />
                    }}
                    label={html! { <span>{"YEARLY"}</span> }}
                />
            </Group>
            <p style="font-size:11px;color:#71717a;margin:12px 0 0;">
                { format!("Selected: {}", *choice) }
            </p>
        </div>
    }
}

#[function_component(ExampleCustomColors)]
pub fn example_custom_colors() -> Html {
    let v = use_state(String::new);
    let vv = use_state(|| true);
    let v2 = use_state(String::new);
    let vv2 = use_state(|| true);
    html! {
        <Form aria_label="Custom color form">
            <Field
                id="cc-success"
                name="website"
                r#type="url"
                label="Website (green focus)"
                placeholder="https://opensass.org"
                color={Color::Success}
                full_width=true
                handle={v.clone()}
                valid_handle={vv.clone()}
                validate_function={Callback::from(validate_url)}
            />
            <Field
                id="cc-info"
                name="twitter"
                label="Twitter handle (cyan focus)"
                placeholder="@opensass"
                color={Color::Info}
                full_width=true
                handle={v2.clone()}
                valid_handle={vv2.clone()}
                validate_function={Callback::from(validate_any)}
            />
        </Form>
    }
}

#[function_component(ExampleServerError)]
pub fn example_server_error() -> Html {
    let email = use_state(String::new);
    let email_valid = use_state(|| true);
    html! {
        <Form aria_label="Server-side error example">
            <Field
                id="srv-email"
                name="email"
                r#type="email"
                label="Email"
                placeholder="ferris@opensass.org"
                full_width=true
                validation_state={ValidationState::Invalid("This email is already registered.".to_string())}
                handle={email.clone()}
                valid_handle={email_valid.clone()}
                validate_function={Callback::from(validate_email)}
            />
            <button
                disabled=true
                style="width:100%;padding:10px 0;border-radius:8px;\
                       background:#dc2626;color:white;border:none;\
                       font-size:15px;font-weight:600;opacity:0.7;cursor:not-allowed;"
            >
                { "Continue (blocked by server error)" }
            </button>
        </Form>
    }
}

#[function_component(ExampleCustomStyle)]
pub fn example_custom_style() -> Html {
    let v = use_state(String::new);
    let vv = use_state(|| true);
    html! {
        <Form aria_label="Headless custom-styled form">
            <Control input_id="cs-name" full_width=true>
                <FormLabel
                    html_for="cs-name"
                    style="font-size:11px;text-transform:uppercase;letter-spacing:0.1em;color:#7c3aed;"
                >
                    { "Name" }
                </FormLabel>
                <Field
                    id="cs-name"
                    name="name"
                    label=""
                    placeholder="Your full name"
                    full_width=true
                    variant={Variant::Standard}
                    color={Color::Primary}
                    style="border-bottom: 2px solid #7c3aed;"
                    handle={v.clone()}
                    valid_handle={vv.clone()}
                    validate_function={Callback::from(validate_required)}
                />
                <Helper style="font-size:10px;text-align:right;color:#7c3aed;">
                    { "Required" }
                </Helper>
            </Control>
        </Form>
    }
}

#[function_component(ExampleSmallSize)]
pub fn example_small_size() -> Html {
    let v = use_state(String::new);
    let vv = use_state(|| true);
    let v2 = use_state(String::new);
    let vv2 = use_state(|| true);
    html! {
        <Form aria_label="Small-size fields">
            <Field
                id="sm-first"
                name="first_name"
                label="First name"
                placeholder="Ferris"
                size={form_rs::Size::Small}
                full_width=true
                handle={v.clone()}
                valid_handle={vv.clone()}
                validate_function={Callback::from(validate_required)}
            />
            <Field
                id="sm-last"
                name="last_name"
                label="Last name"
                placeholder="Prophet"
                size={form_rs::Size::Small}
                full_width=true
                handle={v2.clone()}
                valid_handle={vv2.clone()}
                validate_function={Callback::from(validate_required)}
            />
        </Form>
    }
}

#[function_component(ExampleFormLabels)]
pub fn example_form_labels() -> Html {
    html! {
        <div style="display:flex;flex-direction:column;gap:16px;width:100%;">
            <Control input_id="lbl-normal">
                <FormLabel html_for="lbl-normal">{ "Normal label" }</FormLabel>
                <div
                    style="border:1.5px solid #3f3f46;border-radius:8px;padding:10px 14px;color:#71717a;font-size:15px;"
                >
                    { "Input placeholder" }
                </div>
            </Control>
            <Control input_id="lbl-required" required=true>
                <FormLabel html_for="lbl-required" required=true>{ "Required label" }</FormLabel>
                <div
                    style="border:1.5px solid #3f3f46;border-radius:8px;padding:10px 14px;color:#71717a;font-size:15px;"
                >
                    { "Input placeholder" }
                </div>
            </Control>
            <Control input_id="lbl-error" error=true>
                <FormLabel html_for="lbl-error" error=true>{ "Error label" }</FormLabel>
                <div
                    style="border:1.5px solid #dc2626;border-radius:8px;padding:10px 14px;color:#71717a;font-size:15px;box-shadow:0 0 0 3px rgba(220,38,38,0.18);"
                >
                    { "Invalid input" }
                </div>
                <Helper error=true>{ "This field has an error." }</Helper>
            </Control>
            <Control input_id="lbl-disabled" disabled=true>
                <FormLabel html_for="lbl-disabled" disabled=true>{ "Disabled label" }</FormLabel>
                <div
                    style="border:1.5px solid #3f3f46;border-radius:8px;padding:10px 14px;color:#71717a;font-size:15px;opacity:0.5;"
                >
                    { "Disabled input" }
                </div>
            </Control>
        </div>
    }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center ">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Form RS Yew Examples" }</h1>
            <section aria-labelledby="form-heading" class="w-full max-w-6xl mb-12">
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Basic Login Form" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::yew::{Form, Field};
use form_rs::Method;
use yew::prelude::*;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let email = use_state(String::new);
    let valid = use_state(|| true);
    let on_submit = Callback::from(|e: SubmitEvent| {
        e.prevent_default();
    });
    html! {
        <Form method={Method::Post} on_submit={on_submit}>
            <Field id="email" r#type="email"
                label="Email" required=true
                handle={email.clone()} valid_handle={valid.clone()}
                validate_function={|v: String| v.contains('@')}
            />
            <button type="submit">{"Sign in"}</button>
        </Form>
    }
}"# }
                        </pre>
                        <ExampleBasicLogin />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Registration Form" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::yew::{Form, Field};
use yew::prelude::*;

#[function_component(RegForm)]
pub fn reg_form() -> Html {
    let email = use_state(String::new);
    let ev = use_state(|| true);
    let pass  = use_state(String::new);
    let pv = use_state(|| true);
    html! {
        <Form aria_label="Register">
            <Field id="email" r#type="email"
                label="Email" required=true
                handle={email.clone()} valid_handle={ev.clone()}
                validate_function={|v: String| v.contains('@')}
            />
            <Field id="password" r#type="password"
                label="Password" minlength={Some(8)}
                handle={pass.clone()} valid_handle={pv.clone()}
                validate_function={|v: String| v.len() >= 8}
            />
            <button type="submit">{"Register"}</button>
        </Form>
    }
}"# }
                        </pre>
                        <ExampleRegistration />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Validation States" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::yew::{
    Control, FormLabel, Helper,
};
use yew::prelude::*;

#[function_component(States)]
pub fn states() -> Html {
    html! {
        <>
            <Control input_id="valid-field">
                <FormLabel html_for="valid-field">
                    {"Valid field"}
                </FormLabel>
                <Helper valid=true>
                    {"Looks great!"}
                </Helper>
            </Control>
            <Control input_id="err-field" error=true>
                <FormLabel html_for="err-field" error=true>
                    {"Error field"}
                </FormLabel>
                <Helper error=true>
                    {"Not a valid email."}
                </Helper>
            </Control>
        </>
    }
}"# }
                        </pre>
                        <ExampleValidationStates />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "ARIA Realtime Validation" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::{ValidationBehavior, ValidationState};
use form_rs::yew::{Form, Field};
use yew::prelude::*;

#[function_component(AriaForm)]
pub fn aria_form() -> Html {
    let email = use_state(String::new);
    let ev = use_state(|| true);
    html! {
        <Form validation_behavior={
            ValidationBehavior::Aria
        }>
            <Field id="e" r#type="email"
                label="Email (ARIA mode)"
                validation_state={
                    if !(*email).is_empty()
                        && !(*email).contains('@') {
                        ValidationState::Invalid(
                            "Invalid email".to_string())
                    } else { ValidationState::None }
                }
                handle={email.clone()}
                valid_handle={ev.clone()}
                validate_function={|v: String|
                    v.contains('@')}
            />
        </Form>
    }
}"# }
                        </pre>
                        <ExampleAriaValidation />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Disabled Form" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::yew::{Form, Field};
use yew::prelude::*;

#[function_component(DisabledForm)]
pub fn disabled_form() -> Html {
    let v = use_state(|| "ferris@opensass.org".to_string());
    let vv = use_state(|| true);
    html! {
        <Form aria_label="Disabled form">
            <Field id="dis-email"
                r#type="email"
                label="Email"
                disabled=true
                handle={v.clone()}
                valid_handle={vv.clone()}
                validate_function={|v: String| v.contains('@')}
            />
            <button disabled=true>{"Disabled"}</button>
        </Form>
    }
}"# }
                        </pre>
                        <ExampleDisabledForm />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Filled Variant" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::{Color, Variant};
use form_rs::yew::{Form, Field};
use yew::prelude::*;

#[function_component(FilledForm)]
pub fn filled_form() -> Html {
    let v = use_state(String::new);
    let vv = use_state(|| true);
    html! {
        <Form>
            <Field id="filled-email"
                r#type="email"
                label="Email address"
                variant={Variant::Filled}
                color={Color::Secondary}
                full_width=true
                handle={v.clone()}
                valid_handle={vv.clone()}
                validate_function={|v: String| v.contains('@')}
            />
        </Form>
    }
}"# }
                        </pre>
                        <ExampleFilledVariant />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Standard (Underline) Variant" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::Variant;
use form_rs::yew::{Form, Field};
use yew::prelude::*;

#[function_component(StandardForm)]
pub fn standard_form() -> Html {
    let v = use_state(String::new);
    let vv = use_state(|| true);
    html! {
        <Form>
            <Field id="std-name"
                label="Full name"
                variant={Variant::Standard}
                full_width=true
                handle={v.clone()}
                valid_handle={vv.clone()}
                validate_function={|v: String| !v.is_empty()}
            />
        </Form>
    }
}"# }
                        </pre>
                        <ExampleStandardVariant />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Contact Form" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::yew::{Form, Field};
use form_rs::Method;
use yew::prelude::*;

#[function_component(ContactForm)]
pub fn contact_form() -> Html {
    let name = use_state(String::new);
    let nv = use_state(|| true);
    let email = use_state(String::new);
    let ev = use_state(|| true);
    let subject = use_state(String::new);
    let sv = use_state(|| true);
    let sent = use_state(|| false);
    let on_submit = Callback::from({
        let sent = sent.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            sent.set(true);
        }
    });
    html! {
        <Form method={Method::Post} on_submit={on_submit}>
            <Field id="name" label="Name"
                handle={name.clone()} valid_handle={nv.clone()}
                validate_function={|v: String| !v.is_empty()}
            />
            <Field id="email" r#type="email" label="Email"
                handle={email.clone()} valid_handle={ev.clone()}
                validate_function={|v: String| v.contains('@')}
            />
            <Field id="subject" label="Subject"
                handle={subject.clone()} valid_handle={sv.clone()}
                validate_function={|v: String| !v.is_empty()}
            />
            <button type="submit">{"Send message"}</button>
        </Form>
    }
}"# }
                        </pre>
                        <ExampleContactForm />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Group Checkboxes" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::yew::{Group, ControlLabel, Input};
use yew::prelude::*;

#[function_component(NotifPrefs)]
pub fn notif_prefs() -> Html {
    let email = use_state(String::new);
    let ev = use_state(|| true);
    let email_ref = use_node_ref();
    html! {
        <Group aria_label="Notifications">
            <ControlLabel
                control={html! {
                    <Input
                        r#ref={email_ref}
                        r#type="checkbox"
                        id="notif-email"
                        otp_mode=true
                        handle={email.clone()}
                        valid_handle={ev.clone()}
                        validate_function={
                            Callback::from(|_: String| true)
                        }
                    />
                }}
                label={html!{ <span>{"Email"}</span> }}
            />
        </Group>
    }
}"# }
                        </pre>
                        <ExampleGroup />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Group Radio Row" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::yew::{Group, ControlLabel, Input};
use yew::prelude::*;

#[function_component(BillingCycle)]
pub fn billing_cycle() -> Html {
    let choice = use_state(|| "monthly");
    let monthly_h = use_state(String::new);
    let monthly_v = use_state(|| true);
    let monthly_ref = use_node_ref();
    html! {
        <Group aria_label="Billing" row=true>
            <ControlLabel
                control={html! {
                    <Input
                        r#ref={monthly_ref}
                        r#type="radio"
                        id="billing-monthly"
                        name="billing"
                        otp_mode=true
                        handle={monthly_h.clone()}
                        valid_handle={monthly_v.clone()}
                        validate_function={
                            Callback::from(|_: String| true)
                        }
                    />
                }}
                label={html!{ <span>{"Monthly"}</span> }}
            />
        </Group>
    }
}"# }
                        </pre>
                        <ExampleGroupRow />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Custom Color Themes" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::{Color};
use form_rs::yew::{Form, Field};
use yew::prelude::*;

// Color variants: Primary, Secondary,
// Success, Info, Warning, Error
#[function_component(ColoredForm)]
pub fn colored_form() -> Html {
    let v = use_state(String::new);
    let vv = use_state(|| true);
    html! {
        <Form>
            <Field id="url"
                r#type="url"
                label="Website (Success)"
                color={Color::Success}
                handle={v.clone()}
                valid_handle={vv.clone()}
                validate_function={|v: String|
                    v.starts_with("https://")}
            />
        </Form>
    }
}"# }
                        </pre>
                        <ExampleCustomColors />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Server-Side Error" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::ValidationState;
use form_rs::yew::{Form, Field};
use yew::prelude::*;

// Pass in server error via ValidationState::Invalid
#[function_component(ServerErrorForm)]
pub fn server_error_form() -> Html {
    let v = use_state(String::new);
    let vv = use_state(|| true);
    html! {
        <Form>
            <Field id="srv-email"
                r#type="email"
                label="Email"
                validation_state={
                    ValidationState::Invalid(
                        "Email already registered."
                            .to_string()
                    )
                }
                handle={v.clone()}
                valid_handle={vv.clone()}
                validate_function={|v: String|
                    v.contains('@')}
            />
        </Form>
    }
}"# }
                        </pre>
                        <ExampleServerError />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "FormLabel Showcase" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::yew::{
    Control, FormLabel, Helper
};
use yew::prelude::*;

// All label + helper states in one view
#[function_component(LabelShowcase)]
pub fn label_showcase() -> Html {
    html! {
        <>
            <Control input_id="l1">
                <FormLabel html_for="l1">
                    {"Normal"}
                </FormLabel>
            </Control>
            <Control input_id="l2" required=true>
                <FormLabel html_for="l2" required=true>
                    {"Required"}
                </FormLabel>
            </Control>
            <Control input_id="l3" error=true>
                <FormLabel html_for="l3" error=true>
                    {"Error label"}
                </FormLabel>
                <Helper error=true>
                    {"Error message here"}
                </Helper>
            </Control>
        </>
    }
}"# }
                        </pre>
                        <ExampleFormLabels />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Small Size Fields" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::Size;
use form_rs::yew::{Form, Field};
use yew::prelude::*;

#[function_component(SmallForm)]
pub fn small_form() -> Html {
    let v = use_state(String::new);
    let vv = use_state(|| true);
    html! {
        <Form>
            <Field id="first"
                label="First name"
                size={Size::Small}
                handle={v.clone()}
                valid_handle={vv.clone()}
                validate_function={|v: String| !v.is_empty()}
            />
        </Form>
    }
}"# }
                        </pre>
                        <ExampleSmallSize />
                    </article>
                    <article
                        class="flex flex-col items-start bg-gray-200 p-4 rounded-lg shadow-md text-black"
                    >
                        <h3 class="text-xl font-bold mb-2">{ "Custom Styled (Headless)" }</h3>
                        <pre
                            class="font-mono text-xs text-white p-4 bg-gray-800 mb-6 rounded-md w-full overflow-x-auto whitespace-pre"
                        >
                            { r#"use form_rs::Variant;
use form_rs::yew::{
    Form, Control, FormLabel,
    Helper, Field,
};
use yew::prelude::*;

// Pass style to any component for full control
#[function_component(HeadlessForm)]
pub fn headless_form() -> Html {
    let v = use_state(String::new);
    let vv = use_state(|| true);
    html! {
        <Form>
            <Control input_id="cs-name" full_width=true>
                <FormLabel html_for="cs-name"
                    style="text-transform:uppercase;color:#7c3aed;">
                    {"Name"}
                </FormLabel>
                <Field id="cs-name"
                    variant={Variant::Standard}
                    handle={v.clone()} valid_handle={vv.clone()}
                    validate_function={|v: String| !v.is_empty()}
                />
                <Helper style="text-align:right;">
                    {"Required"}
                </Helper>
            </Control>
        </Form>
    }
}"# }
                        </pre>
                        <ExampleCustomStyle />
                    </article>
                </div>
            </section>
        </div>
    }
}
