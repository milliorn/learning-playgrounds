fn main() {
    println!("Hello, world!");
    /* To display a web page in the browser, the browser (client) sends an HTTP request to the server, which responds with the source code of the web page. The browser then renders this code.

    ...you’ll need to mimic the behavior by sending an HTTP GET request to IMDb.

    ..reqwest provides the features of an HTTP client. It can do a lot of the things, ...such as open pages, log in, and store cookies. */
    let response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();

    /* ...scraper library works by parsing the HTML document into a tree-like structure. You can use CSS selectors to query the elements you’re interested in.
     */
    let document = scraper::Html::parse_document(&response);

    /* ...find and select the parts you need. Check the website’s code and find a collection of CSS selectors that uniquely identifies those items.
     */
    /*
       <h3 class="lister-item-header">
       <span class="lister-item-index unbold text-primary">1.</span>
       <a href="/title/tt0111161/?ref_=adv_li_tt">The Shawshank Redemption</a>
       <span class="lister-item-year text-muted unbold">(1994)</span>
       </h3>
    */
    let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();

    /* Now you can apply this query to your parsed document with the select method, ...you’ll map each HTML element to the HTML that’s inside it:
     */
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    /* titles is now an iterator holding the names of all the top one hundred titles.

    ...print out names. First zip your title list with the numbers 1 to 100. Call for_each method on the iterator, which prints each item in iterator.*/
    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}
