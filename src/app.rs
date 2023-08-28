use leptos::*;

#[component]
pub fn Main(cx: Scope) -> impl IntoView {
    let (input, set_input) = create_signal(cx, "".to_string());

    let resource_string = create_resource(
        cx,
        move || input(),
        |input| async move { format!("The input was \"{}\"", input) },
    );

    view! { cx,
        <>
            <form on:submit=|ev| ev.prevent_default()>
                <label>
                    Input
                    <input name="input" type="text" value=input on:input=move |ev| set_input(event_target_value(&ev)) />
                </label>
            </form>
            <Transition fallback=move || view! { cx, <span>Loading</span> }>
                <div class="result">
                    {move || resource_string.with(cx, |result| view! { cx, <>
                        The result is: <output>{result}</output>
                    </> })}
                </div>
            </Transition>
        </>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <link rel="stylesheet" href="/pkg/leptos_start.css"/>

        // content for this welcome page
        <main>
            <Main />
        </main>
    }
}
