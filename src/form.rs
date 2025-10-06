use hypertext::prelude::*;

///Checkbox
///
///```
///# use hypertext::rsx;
///# use orbit::form::Checkbox;
///# fn main(){rsx!{
/// <Checkbox id="1" text="Check me!" description=(None)>
/// <Checkbox id="2" text="Look, I've got a description!" description=(Some("This is a really cool box to check!"))>
///# };}
/// ```
#[component(pub)]
fn checkbox<'a>(id: &'a str, text: &'a str, description: Option<&'a str>) -> impl Renderable {
    rsx! {
    <label for=(id) class="inline-flex items-start gap-3">
        <input type="checkbox" id=(id) name=(id) class="rounded accent-red-600" />

        <div>
            <span class="text-gray-700">(text)</span>

            @if description.is_some(){
            <p class="text-sm text-gray-700">(description)</p>
            }
        </div>
    </label>
    }
}

///Toggle
///
///```
///# use hypertext::rsx;
///# use orbit::form::Toggle;
///# fn main(){rsx!{
/// <Toggle id="1">
///# };}
/// ```
#[component(pub)]
fn toggle<'a>(id: &'a str) -> impl Renderable {
    rsx! {
    <label for=(id) class="group relative block h-8 w-14 rounded-full inset-shadow-xs bg-gray-300 transition-colors has-checked:bg-green-500">
        <input type="checkbox" id=(id) name=(id) class="peer sr-only" />

        <span
        class="absolute inset-y-0 start-0 m-1 grid size-6 place-content-center rounded-full bg-white text-gray-700 transition-[inset-inline-start] peer-checked:start-6 peer-checked:*:first:hidden *:last:hidden peer-checked:*:last:block select-none"
        >
            <p>"✕"</p>
            <p>"✓"</p>
        </span>
    </label>
    }
}

///Submit
///
///```
///# use hypertext::rsx;
///# use orbit::form::Submit;
///# fn main(){rsx!{
/// <Submit text="Submit">
///# };}
/// ```
#[component(pub)]
fn submit<'a>(text: &'a str) -> impl Renderable {
    rsx! {
    <input type="submit" value=(text)>
    }
}

///Form
///
///```
///# use hypertext::prelude::*;
///# use orbit::form::{Form, FormMethod, Toggle};
///# fn main(){rsx!{
/// <Form method=(FormMethod::Get) action="/example">
///   <Toggle id="1" />
/// </Form>
///# };}
/// ```
#[component(pub)]
fn form<'a, R: Renderable>(method: FormMethod, action: &'a str, children: &R) -> impl Renderable {
    rsx! {
        <form method=(method.to_string()) action=(action)>
            (children)
        </form>
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FormMethod {
    Get,
    Post,
}

impl std::fmt::Display for FormMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormMethod::Get => write!(f, "get"),
            FormMethod::Post => write!(f, "post"),
        }
    }
}
