use icondata::{LuEye, LuEyeOff};
use leptos::prelude::*;
use leptos_icons::Icon;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;

/*
 * title: Login Form Card
*/

#[component]
pub fn Login() -> impl IntoView {
    let show_password = RwSignal::new(false);
    let password_input_ref = NodeRef::<leptos::html::Input>::new();

    let toggle_password_visibility = move |_| {
        show_password.update(|value| *value = !*value);
        if let Some(input) = password_input_ref.get() {
            input.set_type(if show_password.get_untracked() {
                "text"
            } else {
                "password"
            });
        }
    };

    view! {
        <div class="flex justify-center items-center p-6 w-full md:p-10 min-h-svh">
            <div class="w-full max-w-sm">
                <div class="flex flex-col gap-6">
                    <Card>
                        <CardHeader>
                            <CardTitle>"请登录您的账户。"</CardTitle>
                            <CardDescription>
                                "在下方输入您的电子邮件以登录账户"
                            </CardDescription>
                        </CardHeader>
                        <CardContent>
                            <form>
                                <div class="flex flex-col gap-6">
                                    <div class="grid gap-3">
                                        <Label html_for="email">Email</Label>
                                        <Input
                                            attr:r#type="email"
                                            attr:id="email"
                                            autocomplete="username"
                                            attr:placeholder="m@example.com"
                                            attr:required=true
                                        />
                                    </div>
                                    <div class="grid gap-3">
                                        <div class="flex items-center">
                                            <Label html_for="password">Password</Label>
                                            <a
                                                attr:href="#"
                                                class="inline-block ml-auto text-sm hover:underline underline-offset-4"
                                            >
                                                "忘记密码了吗？"
                                            </a>
                                        </div>
                                        <div class="relative">
                                            <Input
                                                node_ref=password_input_ref
                                                attr:r#type="password"
                                                attr:id="password"
                                                autocomplete="current-password"
                                                minlength=8
                                                attr:required=true
                                                class="pr-10"
                                            />
                                            <button
                                                type="button"
                                                class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                                                attr:aria-label=move || {
                                                    if show_password.get() {
                                                        "Show password"
                                                    } else {
                                                        "Hide password"
                                                    }
                                                }
                                                on:click=toggle_password_visibility
                                            >
                                                {move || {
                                                    if show_password.get() {
                                                        view! {
                                                            <div class="flex items-center gap-1">
                                                                <span class="size-4"><Icon icon=LuEye /></span>
                                                                <span class="sr-only">Show password</span>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    } else {
                                                        view! {
                                                            <div class="flex items-center gap-1">
                                                                <span class="size-4"><Icon icon=LuEyeOff /></span>
                                                                <span class="sr-only">Hide password</span>
                                                            </div>
                                                        }
                                                            .into_any()
                                                    }
                                                }}
                                            </button>
                                        </div>
                                    </div>
                                    <div class="flex flex-col gap-3">
                                        <Button class="w-full">"登录"</Button>
                                        <Button variant=ButtonVariant::Outline class="w-full">
                                            "使用 Google 登录"
                                        </Button>
                                    </div>
                                </div>
                                <div class="mt-4 text-sm text-center">
                                    "还没有账户？"
                                    <a href="#" class="underline underline-offset-4">
                                        "注册"
                                    </a>
                                </div>
                            </form>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </div>
    }
}
