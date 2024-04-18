use leptos::*;

#[component]
pub fn MarkdownInput(value: RwSignal<String>) -> impl IntoView {
    let handle_input = move |e| {
        value.set(event_target_value(&e));
    };
    view! {
        <div class="card mb-3">
            <div class="card-header">
                <ul class="nav nav-tabs">
                    <li class="nav-item">
                        <button
                            class="nav-link active"
                            type="button"
                            data-bs-toggle="tab"
                            data-bs-target="#write"
                        >
                            Write
                        </button>
                    </li>
                    <li class="nav-item">
                        <button
                            class="nav-link"
                            type="button"
                            data-bs-toggle="tab"
                            data-bs-target="#preview"
                        >
                            Preview
                        </button>
                    </li>
                </ul>
            </div>
            <div class="card-body">
                <div class="tab-content">
                    <div class="tab-pane active" id="write">
                        <textarea
                            class="form-control"
                            on:input=handle_input
                            style="min-height: 25vh"
                        >
                            {value}
                        </textarea>
                    </div>
                    <div class="tab-pane" id="preview" style="min-height: 25vh">
                        <zero-md>
                            <script type="text/markdown">{value}</script>
                        </zero-md>
                    </div>
                </div>
            </div>
        </div>
    }
}
