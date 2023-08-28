use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// #[server(FakeServerFnVec, "/api")]
// pub async fn fake_server_function_vec(input: String) -> Result<Vec<char>, ServerFnError> {
//     Ok(input.chars().collect())
// }

#[server(FakeServerFnString, "/api")]
pub async fn fake_server_function_string(input: String) -> Result<String, ServerFnError> {
    Ok(format!("The input was \"{}\"", input))
}

#[derive(Params, PartialEq, Clone)]
struct QueryParams {
    input: Option<String>,
}

#[component]
pub fn Main(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let query_result = use_query::<QueryParams>(cx);
    let query = Signal::derive(cx, move || {
        query_result.with(|query| {
            query
                .as_ref()
                .expect("Failed to parse query params")
                .clone()
        })
    });
    let input = Signal::derive(cx, move || query.with(|query| query.input.clone()));

    let resource_string = create_resource(
        cx,
        move || input(),
        |input| fake_server_function_string(input.unwrap_or_default()),
    );

    // let resource_vec = create_resource(
    //     cx,
    //     move || input(),
    //     |input| fake_server_function_vec(input.unwrap_or_default()),
    // );

    view! { cx,
        <>
            <Form method="GET" action="">
                <label>
                    Input
                    <input name="input" type="text" value=input />
                </label>
                <button type="submit">Submit</button>
            </Form>
            <Transition fallback=move || view! { cx, <span>Loading</span> }>
                <div class="result">
                    {move || resource_string.with(cx, |result| match result {
                        Ok(result) => view! {
                            cx,
                            <>
                                The result is:
                                <output>{result}</output>
                            </>
                        },
                        Err(err) => view! { cx, <>The resource failed to load with: <output><pre>{format!("\"{:?}\"", err)}</pre></output></> },
                    })}
                </div>
            </Transition>
            // <Transition fallback=move || view! { cx, <span>Loading</span> }>
            //     <div class="result">
            //         {move || resource_vec.with(cx, |result| match result {
            //             Ok(result) => view! {
            //                 cx,
            //                 <>
            //                     The result is:
            //                     <output>{result.iter().collect::<String>()}</output>
            //                 </>
            //             },
            //             Err(err) => view! { cx, <>The resource failed to load with: <output><pre>{format!("\"{:?}\"", err)}</pre></output></> },
            //         })}
            //     </div>
            // </Transition>
        </>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <main>
            <Router>
                <Main />
            </Router>
        </main>
    }
}
