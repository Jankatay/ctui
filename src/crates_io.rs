use crates_io_api::*;

// start from here. Just make a client and search using that client.
pub fn init_client() -> Option<SyncClient> {
    // make a client
    SyncClient::new(
        "jne (barjanay@proton.me)",
        std::time::Duration::from_millis(100)
    ).ok()
}

// starting at `page`*100, get 100 entries for `search_string` from `client`
pub fn search(client: &SyncClient, search_string: String, page: u64) -> Vec<Crate> {
    // setup a default query
    let mut query = CratesQuery::default();
    query.set_sort(Sort::Relevance);
    query.set_page_size(20);

    // add function options
    query.set_search(Some(search_string));
    query.set_page(page);

    // get matching crates from client
    if let Ok(crates_page) = client.crates(query) {
        return crates_page.crates;
    } 

    // on error just return empty vector
    return vec![];
}
