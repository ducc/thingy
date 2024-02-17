use leptos::*;

use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use {
    futures::stream::{StreamExt, TryStreamExt},
    mongodb::{
        bson::{doc, Document},
        Client,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    #[serde(rename = "_id")]
    id: String,
    entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    description: String,
    #[serde(rename = "servingSize")]
    serving_size: Option<ServingSize>,
    #[serde(rename = "timeEaten")]
    time_eaten: String,
    #[serde(rename = "imageUrl")]
    image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServingSize {
    measure: String,
    size: f64,
}

#[server]
pub async fn get_diary_entries() -> Result<Vec<Response>, ServerFnError> {
    let client = use_context::<Client>().unwrap();

    let result: Vec<Document> = client
        .database("calories")
        .collection::<Document>("nutracheck_entries")
        .aggregate(
            [
                doc! {
                    "$sort": doc! {
                        "_id": -1
                    }
                },
                doc! {
                    "$limit": 6
                },
                doc! {
                    "$project": doc! {
                        "entries": "$data.entries"
                    }
                },
            ],
            None,
        )
        .await?
        .try_collect()
        .await?;

    Ok(result
        .into_iter()
        .map(|doc| bson::from_bson(doc.into()).unwrap())
        .collect::<Vec<Response>>())
}

#[component]
pub fn DiaryTable() -> impl IntoView {
    let diary_entries = create_resource(|| (), |_| get_diary_entries());

    view! {
        <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
            <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
                <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                    <tr>
                        <th scope="col" class="p-4">
                            <div class="flex items-center">
                                <input
                                    id="checkbox-all-search"
                                    type="checkbox"
                                    class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                                />
                                <label for="checkbox-all-search" class="sr-only">
                                    checkbox
                                </label>
                            </div>
                        </th>
                        <th scope="col" class="px-6 py-3">
                            "Image"
                        </th>
                        <th scope="col" class="px-6 py-3">
                            "Time"
                        </th>
                        <th scope="col" class="px-6 py-3">
                            "Product name"
                        </th>
                        <th scope="col" class="px-6 py-3">
                            "Quantity"
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <Transition>
                        {move || match diary_entries.get() {
                            None => view! { <p>"Loading..."</p> }.into_view(),
                            Some(Err(e)) => {
                                view! { <p>"Error! " {format!("{:?}", e)}</p> }.into_view()
                            }
                            Some(Ok(data)) => {
                                data.into_iter()
                                    .map(|resp| resp.entries.into_iter().rev().collect::<Vec<_>>())
                                    .flatten()
                                    .map(|entry| {
                                        view! {
                                            <DiaryRow
                                                image_url=entry.image_url
                                                time_eaten=entry.time_eaten
                                                product_name=entry.description
                                                quantity=format!(
                                                    "{} {}",
                                                    entry.serving_size.clone().map(|s| s.size).unwrap_or(0.0),
                                                    entry
                                                        .serving_size
                                                        .clone()
                                                        .map(|s| s.measure)
                                                        .unwrap_or("?".into()),
                                                )
                                            />
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .into_view()
                            }
                        }}

                    </Transition>
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn DiaryRow(
    image_url: Option<String>,
    time_eaten: String,
    product_name: String,
    quantity: String,
) -> impl IntoView {
    let (expand_row, set_expand_row) = create_signal(false);

    view! {
        <tr 
            class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600"
            on:click=move |_| {
                set_expand_row.set(!(expand_row.get()));
            }
        >
            <td class="w-4 p-4">
                <div class="flex items-center">
                    <input
                        id="checkbox-table-search-1"
                        type="checkbox"
                        class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                    />
                    <label for="checkbox-table-search-1" class="sr-only">
                        checkbox
                    </label>
                </div>
            </td>
            <th
                scope="row"
                class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
            >
                <img width="50px" height="50px" src={image_url.clone()} />
            </th>
            <td class="px-6 py-4">{time_eaten}</td>
            <td class="px-6 py-4">{product_name}</td>
            <td class="px-6 py-4">{quantity}</td>
        </tr>
        <tr class={move || format!("bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 {}", if !expand_row() { "hidden" } else { ""})}>
            <td />
            <td colspan=4 class="p-4">
                <div class="flex gap-4">
                    <div>
                        <img width="250px" height="250px" src={image_url} />
                    </div>
                    <div>
                        <Button name="Decrease stock count" />
                    </div>
                </div>
                // <div class="grid grid-cols-2 gap-4">
                //     <div class="">
                //         <img width="250px" height="250px" src={image_url} />
                //     </div>
                //     <div class="">
                //         <p>"Stock level: 3150g"</p>
                //         <Button name="Decrease stock count" />

                //     </div>

                // </div>
            </td>
        </tr>
    }
}

#[component]
fn Button(name: &'static str) -> impl IntoView {
    view! {
        <div>
            <button 
                type="button" 
                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800"
            >
                {name}
            </button>
        </div>
    }
}