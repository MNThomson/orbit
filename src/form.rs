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
fn checkbox<'a>(
    id: &'a str,
    text: &'a str,
    description: Option<&'a str>,
) -> impl Renderable + use<'a> {
    rsx! {
    <label for=(id) class="inline-flex items-start gap-3">
      <input type="checkbox" id=(id) class="rounded accent-red-600" />
      <div>
        <span class="text-gray-700">(text)</span>

        @if description.is_some(){
        <p class="text-sm text-gray-700">(description)</p>
        }
      </div>
    </label>
    }
}
